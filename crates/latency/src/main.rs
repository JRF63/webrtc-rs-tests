mod nalus;

use std::sync::Arc;
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    Mutex, Notify,
};
use webrtc::{
    api::{
        interceptor_registry::register_default_interceptors,
        media_engine::{MediaEngine, MIME_TYPE_H264},
        APIBuilder,
    },
    ice_transport::{
        ice_candidate::RTCIceCandidate, ice_connection_state::RTCIceConnectionState,
        ice_gatherer_state::RTCIceGathererState,
    },
    interceptor::registry::Registry,
    media::Sample,
    peer_connection::{
        configuration::RTCConfiguration, sdp::session_description::RTCSessionDescription,
    },
    rtp_transceiver::{
        rtp_codec::{RTCRtpCodecCapability, RTPCodecType},
        rtp_receiver::RTCRtpReceiver,
    },
    track::{
        track_local::{track_local_static_sample::TrackLocalStaticSample, TrackLocal},
        track_remote::TrackRemote,
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
    registry = register_default_interceptors(registry, &mut m)?;
    let api = APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(registry)
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
            }
        });
    }

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
                #[cfg(debug_assertions)]
                println!("Peer A: Found ICE candidate:\n{:?}", &candidate);
                ice_tx
                    .send(candidate)
                    .expect("Peer A: Unable to send ICE candidate");
            }
            Box::pin(async {})
        }))
        .await;

    let rtp_transceiver = peer_connection
        .add_transceiver_from_track(
            Arc::clone(&video_track) as Arc<dyn TrackLocal + Send + Sync>,
            &[],
        )
        .await?;

    tokio::spawn(async move {
        let mut rtcp_buf = vec![0u8; 1500];
        if let Some(rtp_sender) = rtp_transceiver.sender().await {
            while let Ok((_, _)) = rtp_sender.read(&mut rtcp_buf).await {}
        }
    });

    let pc = peer_connection.clone();
    tokio::spawn(async move {
        while let Some(candidate) = ice_rx.recv().await {
            let candidate = candidate
                .to_json()
                .await
                .expect("Peer A: `to_json` of `RTCIceCandidate` failed");
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
                ice_completion.notified().await;

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
    registry = register_default_interceptors(registry, &mut m)?;
    let api = APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(registry)
        .build();

    let config = RTCConfiguration::default();
    let peer_connection = Arc::new(api.new_peer_connection(config).await?);

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

                    let mut buf = vec![0u8; 1500];

                    let mut frag_starts = 0;
                    let mut frag_ends = 0;

                    while do_read {
                        tokio::select! {
                            _ = closed_notification.notified() => {
                                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                                do_read = false;
                            }
                            v = track.read(&mut buf) => {
                                if let Ok((len, _attributes)) = v {
                                    let rtp_packet = &buf[..len];

                                    // Almost always zero
                                    let csrc_count = rtp_packet[0] & 0b00001111;

                                    let m = 3 + csrc_count as usize;
                                    let nalu_header = rtp_packet[4*m];
                                    let nalu_type = nalu_header & 0b00011111;

                                    match nalu_type {
                                        // Single NAL unit
                                        1..=23 => {
                                            timestamps.push(timer_counter());
                                        }
                                        // Fragmentation unit
                                        28 | 29 => {
                                            let fu_header = rtp_packet[4*m + 1];

                                            // Fragmentation start bit
                                            if fu_header & 0b10000000 != 0 {
                                                frag_starts += 1;
                                            }

                                            // Fragmentation end bit
                                            if fu_header & 0b01000000 != 0 {
                                                timestamps.push(timer_counter());
                                                frag_ends += 1;
                                            }
                                        }
                                        a => println!("nalu_type: {}", a)
                                    }
                                }
                            }
                        }
                    }

                    println!("S: {}", frag_starts);
                    println!("E: {}", frag_ends);
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
                #[cfg(debug_assertions)]
                println!("Peer B: Found ICE candidate:\n{:?}", &candidate);
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
            pc.add_ice_candidate(candidate)
                .await
                .expect("Peer B: Unable to add ICE candidate");
        }
    });

    peer_connection
        .add_transceiver_from_kind(RTPCodecType::Video, &[])
        .await?;

    ice_completion.notified().await;
    closed_notification.notified().await;
    peer_connection.close().await?;
    Ok(())
}

fn main() {
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
