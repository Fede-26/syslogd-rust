use clap::Parser;
use std::net::{SocketAddr, UdpSocket};
use syslog_loose::parse_message;

//https://datatracker.ietf.org/doc/html/rfc5424#section-6

const UDP_PORT: u16 = 514;

/// Simple server using syslog
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Use the tcp protocol instead of udp
    #[arg(short, long)]
    tcp: bool,
}

//parse and print the message from the buffer
fn print_message(buf: &[u8], src_ip: SocketAddr) {
    let buf_str = String::from_utf8(buf.to_vec()).unwrap();
    println!("\n\n?> {}", &buf_str);

    let message = parse_message(&buf_str);

    println!("\n>> MESSAGE RECEIVED:");
    println!("   source IP: {}", src_ip);
    println!("   protocol:  {:?}", &message.protocol);
    if let Some(timestamp) = &message.timestamp {
        println!("   timestamp: {}", timestamp);
    }
    if let Some(severity) = &message.severity {
        println!("   severity:  {}", severity.as_str());
    }
    if let Some(facility) = &message.facility {
        println!("   facility:  {}", facility.as_str());
    }
    if let Some(appname) = &message.appname {
        println!("   appname:   {}", appname);
    }
    if let Some(pid) = &message.procid {
        println!("   pid:       {}", pid);
    }
    if let Some(msgid) = &message.msgid {
        println!("   msg id:    {}", msgid);
    }
    println!("   payload:   {}", &message.msg);
}

fn main() {
    let args = Args::parse();
    let socket;
    if args.tcp {
        println!("Server running (TCP)...\n");
        todo!("implement tcp protocol")
    } else {
        println!("Server running (UDP)...\n");
        socket = UdpSocket::bind(format!("0.0.0.0:{UDP_PORT}")).unwrap();
    }
    //open the socket on all interfaces

    loop {
        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 2048];
        let (_amt, src) = socket.recv_from(&mut buf).unwrap();

        print_message(&buf, src);
    }
}
