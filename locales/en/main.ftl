## UI Labels
label-rank = Rank
label-target = Target
label-ip = Ip
label-last-rtt = Last Rtt
label-avg-rtt = Avg Rtt
label-max = Max
label-min = Min
label-jitter = Jitter
label-loss = Loss

## Table View
table-view-title = PingWatch Table View

## Point View
point-view-title = PingWatch Point View
point-view-legend = Healthy
point-view-high-latency = High Latency (over 80% of max)
point-view-timeout = Timeout

## Sparkline View
sparkline-view-title = PingWatch SparkLine View
sparkline-view-note = Blank area means timeout or error

## Graph View
graph-view-title = PingWatch Graph View

## Error Messages
error-target-required = Error: target IP address or hostname is required
error-output-exists = Output file already exists: {$path}
error-unable-shutdown = Unable to listen for shutdown signal: {$error}
error-ping-init-failed = host({$host}) ping err, reason: ping init failed, err: {$error}
error-ping-unknown = host({$host}) ping err, reason: unknown, err: {$error}
error-ping-recv-failed = host({$host}) ping err, reason: recv failed, err: {$error}
error-encode-metrics = Error encoding metrics: {$error}
error-serve-connection = Error serving connection: {$error}
error-accept-connection = Failed to accept connection: {$error}
error-restore-terminal = Failed to restore terminal: {$error}

## Metrics and Units
unit-ms = ms
unit-percent = %
metric-less-than = < 0.01ms
metric-zero = 0.0ms

## Exporter Mode
exporter-mode-title = PingWatch Exporter Mode
exporter-interval-help = Interval in seconds between pings
exporter-port-help = Prometheus metrics HTTP port

## Command Line Help
arg-target-help = target IP address or hostname to ping
arg-count-help = Number of pings to send
arg-interval-help = Interval in seconds between pings
arg-ipv6-help = Force using IPv6
arg-multiple-help = Specify the maximum number of target addresses, Only works on one target address
arg-view-help = View mode graph/table/point/sparkline
arg-output-help = Output file to save ping results

## Rank Medals
rank-first = 🥇
rank-second = 🥈
rank-third = 🥉
rank-top-10 = 🏆
rank-slow = 🐢

## General
app-about = 🏎  PingWatch - A Ping Tool in Rust with Real-Time Data and Visualizations
app-version = v0.6.0
app-author = hanshuaikang<https://github.com/hanshuaikang>
