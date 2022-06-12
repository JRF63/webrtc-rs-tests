# RTP latency test

Modified from the [play-from-disk-h264](https://github.com/webrtc-rs/examples/tree/0f7511bfcf19c016809170efda5342b117146ad2/examples/play-from-disk-h264) and [play-from-disk-h264](https://github.com/webrtc-rs/examples/tree/0f7511bfcf19c016809170efda5342b117146ad2/examples/play-from-disk-h264), adding proper (I think) ICE trickle and timing data.

### Running

Windows only, sorry. I'm using `QueryPerformanceCounter` for the timestamps.

```cargo run -p latency``` on the workspace root.

### Results

On my local Wi-Fi connection, running it with `--release` I get

|Sender frame delta|us|
| :--- | ---:|
|Average|16566.193277310926|
|Stddev|6356.967117189646|

|Latency|us|
| :--- | ---:|
|Average|1082.5583333333334|
|Stddev|708.6143379374679|

That's only a smidge slower than a ping.

### TODO

Build time is an atrocious 2 minutes. Which crate's fault is that?

Make this work on loopback. There's annoying packet loss on Wi-Fi.