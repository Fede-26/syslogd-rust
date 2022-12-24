# Syslod-rust

This is a simple server for syslog.

It supports incoming connection via udp (514) or tcp (601) _without tls_.

It listens on all interfaces but can't use both protocols at the same time (you should run 2 servers).

# Build

```bash
cargo build --release
```

# Run

```bash
sudo target/release/syslogd-rust --help
```

# "The Flow"

Depending on the flag used, the port is bound and the process waits for connections.
For udp it defines the socket (with the bind of the port) and enters an infinite loop in which every incoming connection is accepted.
After which the transmitted content is passed to the `print_message()` function, which takes care of parsing and printing the payload.

For tcp instead a listener is created, which is an infinite iterator.
A for loop loops through all incoming connections.
Each connection is then accepted and read to extract its payload.
As before it is passed to `print_message()`.

`print_message()` uses [syslog_loose](https://crates.io/crates/syslog_loose) for parsing and then has a series of conditional prints (only print if the value is present).

Using the `--raw` flag you can see the original payload.

For parsing command line arguments, [clap](https://crates.io/crates/clap) is used instead.

I don't know why yet, but using tcp appends a `\n` to the end.
This doesn't happen with udp.
To make the outputs equal I trim the last character if it's a `\n` or `\r`.

# External links

- [rfc5424](https://datatracker.ietf.org/doc/html/rfc5424)
- [abnf syntax of syslog](https://datatracker.ietf.org/doc/html/rfc5424#section-6)
- [Wikipedia: syslog](https://en.wikipedia.org/wiki/Syslog)
- Stack overflow