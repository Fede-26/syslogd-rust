# Syslod-rust

This is a simple server for syslog.

It supports incoming connection via udp or tcp.

It listens on all interfaces but can't use both protocols at the same time (you should run 2 servers).

# Build

```bash
cargo build --release
```

# Run

```bash
sudo target/release/syslogd-rust --help
```