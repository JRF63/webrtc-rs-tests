mod nalus;

use std::sync::Arc;
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    Mutex, Notify,
};
use webrtc::{
    api::{
        interceptor_registry::{configure_nack, configure_twcc},
        media_engine::{MediaEngine, MIME_TYPE_H264},
        setting_engine::SettingEngine,
        APIBuilder,
    },
    ice::{mdns::MulticastDnsMode, network_type::NetworkType},
    ice_transport::{
        ice_candidate::RTCIceCandidate, ice_connection_state::RTCIceConnectionState,
        ice_gatherer_state::RTCIceGathererState,
    },
    interceptor::registry::Registry,
    media::Sample,
    peer_connection::{
        configuration::RTCConfiguration, peer_connection_state::RTCPeerConnectionState,
        sdp::session_description::RTCSessionDescription,
    },
    rtcp::transport_feedbacks::transport_layer_cc::{SymbolTypeTcc, TransportLayerCc},
    rtp::{codecs::h264::H264Packet, packetizer::Depacketizer},
    rtp_transceiver::{
        rtp_codec::{RTCRtpCodecCapability, RTPCodecType},
        rtp_receiver::RTCRtpReceiver,
        rtp_transceiver_direction::RTCRtpTransceiverDirection,
        RTCRtpTransceiverInit,
    },
    track::{
        track_local::track_local_static_sample::TrackLocalStaticSample, track_remote::TrackRemote,
    },
};

/// Peer A sends the H.264 stream
async fn peer_a(
    offer_tx: UnboundedSender<RTCSessionDescription>,
    mut answer_rx: UnboundedReceiver<RTCSessionDescription>,
    ice_tx: UnboundedSender<RTCIceCandidate>,
    mut ice_rx: UnboundedReceiver<RTCIceCandidate>,
    timestamps: Arc<Mutex<Vec<i64>>>,
) -> Result<(), webrtc::Error> {
    let mut m = MediaEngine::default();
    m.register_default_codecs()?;

    let mut registry = Registry::new();
    registry = configure_nack(registry, &mut m);
    registry = configure_twcc(registry, &mut m)?;

    let mut setting_engine = SettingEngine::default();
    // setting_engine.set_ice_multicast_dns_mode(MulticastDnsMode::QueryAndGather);

    let api = APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(registry)
        .with_setting_engine(setting_engine)
        .build();

    let config = RTCConfiguration::default();
    let peer_connection = Arc::new(api.new_peer_connection(config).await?);
    let video_track = Arc::new(TrackLocalStaticSample::new(
        RTCRtpCodecCapability {
            mime_type: MIME_TYPE_H264.to_owned(),
            ..Default::default()
        },
        "video".to_owned(),
        "webrtc-rs".to_owned(),
    ));

    let has_remote_sdp = Arc::new(Notify::new());
    let has_remote_sdp_clone = has_remote_sdp.clone();
    {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        peer_connection
            .on_negotiation_needed(Box::new(move || {
                let _ = tx.send(());
                Box::pin(async move {})
            }))
            .await;

        let pc = peer_connection.clone();
        tokio::spawn(async move {
            while let Some(_) = rx.recv().await {
                let offer = pc.create_offer(None).await.unwrap();
                offer_tx.send(offer.clone()).unwrap();
                pc.set_local_description(offer).await.unwrap();
                let answer = answer_rx.recv().await.unwrap();
                pc.set_remote_description(answer).await.unwrap();
                has_remote_sdp_clone.notify_one();
            }
        });
    }

    peer_connection
        .on_ice_candidate(Box::new(move |candidate| {
            if let Some(candidate) = candidate {
                log::info!("Peer A: Found ICE candidate:\n{:?}", &candidate);
                ice_tx
                    .send(candidate)
                    .expect("Peer A: Unable to send ICE candidate");
            }
            Box::pin(async {})
        }))
        .await;

    let peer_connected = Arc::new(Notify::new());
    let peer_connected_clone = peer_connected.clone();
    peer_connection
        .on_peer_connection_state_change(Box::new(move |state| {
            if state == RTCPeerConnectionState::Connected {
                peer_connected_clone.notify_one();
            }
            Box::pin(async {})
        }))
        .await;

    let rtp_sender = peer_connection.add_track(video_track.clone() as _).await?;

    tokio::spawn(async move {
        let mut arrival_times: Vec<i64> = Vec::new();

        while let Ok((packets, _)) = rtp_sender.read_rtcp().await {
            for packet in packets {
                let packet = packet.as_any();

                // Can be any of:
                //
                // SenderReport
                // ReceiverReport
                // SourceDescription
                // Goodbye
                // TransportLayerNack
                // RapidResynchronizationRequest
                // TransportLayerCc
                // PictureLossIndication
                // SliceLossIndication
                // ReceiverEstimatedMaximumBitrate
                // FullIntraRequest
                // ExtendedReport

                // println!("{:?}", packet.type_id());

                if let Some(tcc) = packet.downcast_ref::<TransportLayerCc>() {
                    let mut arrival_time = (tcc.reference_time * 64000) as i64;

                    for recv_delta in tcc.recv_deltas.iter() {
                        match recv_delta.type_tcc_packet {
                            SymbolTypeTcc::PacketReceivedSmallDelta => {
                                arrival_time += recv_delta.delta;
                                arrival_times.push(arrival_time);
                            }
                            SymbolTypeTcc::PacketReceivedLargeDelta => {
                                arrival_time += recv_delta.delta - 8192000;
                                arrival_times.push(arrival_time);
                            }
                            _ => (),
                        }
                    }
                }
            }
        }

        // dbg!(arrival_times);
        // let sum: i64 = recv_deltas.iter().sum();
        // println!("Ave recv delta: {}", sum as f64 / recv_deltas.len() as f64);
    });

    let pc = peer_connection.clone();
    tokio::spawn(async move {
        while let Some(candidate) = ice_rx.recv().await {
            let candidate = candidate
                .to_json()
                .await
                .expect("Peer A: `to_json` of `RTCIceCandidate` failed");
            has_remote_sdp.notified().await;
            pc.add_ice_candidate(candidate)
                .await
                .expect("Peer A: Unable to add ICE candidate");
        }
    });

    let send_completion = Arc::new(Notify::new());
    let send_complete = send_completion.clone();

    std::thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                peer_connected.notified().await;

                let dur = std::time::Duration::from_nanos(16666667);
                let mut ticker = tokio::time::interval(dur);

                let mut timestamps = timestamps.lock().await;

                for &data in &nalus::NALUS {
                    let _ = ticker.tick().await;
                    let ts = timer_counter();
                    video_track
                        .write_sample(&Sample {
                            data: bytes::Bytes::copy_from_slice(data),
                            duration: dur,
                            ..Default::default()
                        })
                        .await?;
                    timestamps.push(ts);
                }

                send_complete.notify_waiters();
                Result::<(), webrtc::Error>::Ok(())
            })
    });

    send_completion.notified().await;
    peer_connection.close().await?;
    Ok(())
}

/// Peer B receives the H.264 stream
async fn peer_b(
    answer_tx: UnboundedSender<RTCSessionDescription>,
    mut offer_rx: UnboundedReceiver<RTCSessionDescription>,
    ice_tx: UnboundedSender<RTCIceCandidate>,
    mut ice_rx: UnboundedReceiver<RTCIceCandidate>,
    timestamps: Arc<Mutex<Vec<i64>>>,
) -> Result<(), webrtc::Error> {
    let mut m = MediaEngine::default();
    m.register_default_codecs()?;

    let mut registry = Registry::new();
    registry = configure_nack(registry, &mut m);
    registry = configure_twcc(registry, &mut m)?;

    let mut setting_engine = SettingEngine::default();
    // setting_engine.set_ice_multicast_dns_mode(MulticastDnsMode::QueryAndGather);

    let api = APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(registry)
        .with_setting_engine(setting_engine)
        .build();

    let config = RTCConfiguration::default();
    let peer_connection = Arc::new(api.new_peer_connection(config).await?);

    let has_remote_sdp = Arc::new(Notify::new());
    let has_remote_sdp_clone = has_remote_sdp.clone();
    {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        peer_connection
            .on_negotiation_needed(Box::new(move || {
                let _ = tx.send(());
                Box::pin(async move {})
            }))
            .await;

        let pc = peer_connection.clone();
        tokio::spawn(async move {
            while let Some(_) = rx.recv().await {
                let offer = offer_rx.recv().await.unwrap();
                pc.set_remote_description(offer).await.unwrap();
                has_remote_sdp_clone.notify_one();
                let answer = pc.create_answer(None).await.unwrap();
                pc.set_local_description(answer).await.unwrap();
                let local_desc = pc.local_description().await.unwrap();
                answer_tx.send(local_desc).unwrap();
            }
        });
    }

    let closed_notification = Arc::new(Notify::new());
    let closed_notify = closed_notification.clone();
    peer_connection
        .on_ice_connection_state_change(Box::new(move |connection_state| {
            match connection_state {
                RTCIceConnectionState::Disconnected
                | RTCIceConnectionState::Failed
                | RTCIceConnectionState::Closed => closed_notify.notify_waiters(),
                _ => (),
            }
            Box::pin(async {})
        }))
        .await;

    let notification = closed_notification.clone();
    peer_connection
        .on_track(Box::new(
            move |track: Option<Arc<TrackRemote>>, _receiver: Option<Arc<RTCRtpReceiver>>| {
                let closed_notification = notification.clone();
                let timestamps = timestamps.clone();
                Box::pin(async move {
                    let track = track.unwrap();

                    let mut do_read = true;
                    let mut timestamps = timestamps.lock().await;
                    let mut seq_nums = Vec::new();

                    let mut h264_packet = H264Packet::default();

                    while do_read {
                        tokio::select! {
                            _ = closed_notification.notified() => {
                                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                                do_read = false;
                            }
                            rtp = track.read_rtp() => {
                                if let Ok((packet, _attributes)) = rtp {
                                    seq_nums.push(packet.header.sequence_number);
                                    if let Ok(bytes) = h264_packet.depacketize(&packet.payload) {
                                        if !bytes.is_empty() {
                                            timestamps.push(timer_counter());
                                        }
                                    }
                                }
                            }
                        }
                    }

                    let mut iter = seq_nums.iter();
                    let mut prev = iter.next().unwrap();
                    while let Some(sn) = iter.next() {
                        if sn - prev != 1 {
                            println!("{}", sn);
                        }
                        prev = sn;
                    } 
                })
            },
        ))
        .await;

    let ice_completion = Arc::new(Notify::new());
    let ice_complete = ice_completion.clone();
    peer_connection
        .on_ice_gathering_state_change(Box::new(move |state| {
            if state == RTCIceGathererState::Complete {
                ice_complete.notify_one();
            }
            Box::pin(async {})
        }))
        .await;

    peer_connection
        .on_ice_candidate(Box::new(move |candidate| {
            if let Some(candidate) = candidate {
                log::info!("Peer B: Found ICE candidate:\n{:?}", &candidate);
                ice_tx
                    .send(candidate)
                    .expect("Peer B: Unable to send ICE candidate");
            }
            Box::pin(async {})
        }))
        .await;

    let pc = peer_connection.clone();
    tokio::spawn(async move {
        while let Some(candidate) = ice_rx.recv().await {
            let candidate = candidate
                .to_json()
                .await
                .expect("Peer B: `to_json` of `RTCIceCandidate` failed");
            has_remote_sdp.notified().await;
            pc.add_ice_candidate(candidate)
                .await
                .expect("Peer B: Unable to add ICE candidate");
        }
    });

    peer_connection
        .add_transceiver_from_kind(
            RTPCodecType::Video,
            &[RTCRtpTransceiverInit {
                direction: RTCRtpTransceiverDirection::Recvonly,
                send_encodings: Vec::new(),
            }],
        )
        .await?;

    ice_completion.notified().await;
    closed_notification.notified().await;
    peer_connection.close().await?;
    Ok(())
}

fn main() {
    env_logger::init();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .build()
        .unwrap()
        .block_on(async {
            let (tx1, rx1) = tokio::sync::mpsc::unbounded_channel();
            let (tx2, rx2) = tokio::sync::mpsc::unbounded_channel();
            let (tx3, rx3) = tokio::sync::mpsc::unbounded_channel();
            let (tx4, rx4) = tokio::sync::mpsc::unbounded_channel();

            let ts1 = Arc::new(Mutex::new(Vec::with_capacity(nalus::NALUS.len())));
            let ts2 = Arc::new(Mutex::new(Vec::with_capacity(nalus::NALUS.len())));

            let (result_a, result_b) = tokio::join!(
                peer_a(tx1, rx2, tx3, rx4, ts1.clone()),
                peer_b(tx2, rx1, tx4, rx3, ts2.clone())
            );

            if let Err(e) = result_a {
                panic!("Peer A error: {}", e);
            }
            if let Err(e) = result_b {
                panic!("Peer B error: {}", e);
            }

            let ts1 = ts1.lock().await;
            let ts2 = ts2.lock().await;
            let div = timer_frequency() as u64 / 1000000;

            println!(
                "Lost packets: {} ({} sent, {} received)",
                ts1.len() - ts2.len(),
                ts1.len(),
                ts2.len()
            );

            let mut deltas = Vec::with_capacity(nalus::NALUS.len());
            for (&a, &b) in ts1.iter().zip(ts2.iter()) {
                let diff = (b as u64 - a as u64) / div;
                deltas.push(diff);
            }

            let mut peer_a_diffs = Vec::with_capacity(nalus::NALUS.len());
            for i in 1..ts1.len() {
                peer_a_diffs.push((ts1[i] as u64 - ts1[i - 1] as u64) / div);
            }
            println!("\n|Sender frame delta|us|");
            println!("| :--- | ---:|");
            print_stats(&peer_a_diffs);

            if deltas.len() == nalus::NALUS.len() {
                println!("\n|Latency|us|");
                println!("| :--- | ---:|");
                print_stats(&deltas);
            }
        });
}

fn print_stats(deltas: &[u64]) {
    let sum: f64 = deltas.iter().map(|&x| x as f64).sum();
    let ave = sum / deltas.len() as f64;
    let sum_sqdiff: f64 = deltas
        .iter()
        .map(|&x| {
            let diff = x as f64 - ave;
            diff * diff
        })
        .sum();
    let stddev = (sum_sqdiff / deltas.len() as f64).sqrt();
    println!("|Average|{}|", ave);
    println!("|Stddev|{}|", stddev);
}

fn timer_counter() -> i64 {
    let mut now = 0;
    unsafe {
        windows::Win32::System::Performance::QueryPerformanceCounter(&mut now);
        now
    }
}

fn timer_frequency() -> i64 {
    let mut freq = 0;
    unsafe {
        windows::Win32::System::Performance::QueryPerformanceFrequency(&mut freq);
        freq
    }
}
