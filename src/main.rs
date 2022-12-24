use clap::Parser;
use std::io::{Read};
use std::net::{Shutdown, SocketAddr, TcpListener, UdpSocket};
use syslog_loose::parse_message;

//https://datatracker.ietf.org/doc/html/rfc5424#section-6

const UDP_PORT: u16 = 514;
const TCP_PORT: u16 = 601; //TODO: verify if the port is right

/// Simple server using syslog
#[derive(Parser, Debug)]
#[command(name = "rslogd-rust")]
#[command(author = "Federico Zotti")]
#[command(version = "1.1")]
// #[command(about = "Simple server using syslog")]
#[command(about = "This is a server that accept incoming syslog messages using tcp and udp.\nAll the messages should consist of only one packet.")]
#[command(long_about = None)]
struct Args {
    /// Use the tcp protocol instead of udp
    #[arg(short, long)]
    tcp: bool,
}

//remove trailing \n or \r from string
fn trim_newline(s: &mut String) {
    //remove \0 from the tcp stream message
    *s = s.chars().filter(|&char| {char != '\0'}).collect::<String>();
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
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
    if let Some(msgid) = message.msgid {
        println!("   msg id:    {}", msgid);
    }

    //remove trailing \r or \n
    let mut msg = message.msg.to_string();
    trim_newline(&mut msg);
    println!("   payload:   {}", msg);
}

fn main() {
    let args = Args::parse();
    if args.tcp {
        //listen on all interfaces
        let listener = TcpListener::bind(format!("127.0.0.1:{TCP_PORT}")).unwrap();
        println!("Server running (TCP)...\n");

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    //do something with the stream
                    let mut buf = [0 as u8; 2048];

                    match stream.read(&mut buf) {
                        Ok(_size) => {
                            // only accept a single segment
                            print_message(&buf, stream.peer_addr().unwrap());
                            stream.shutdown(Shutdown::Both).unwrap();
                        }
                        Err(_) => {
                            println!(
                                "An error occurred, terminating connection with {}",
                                stream.peer_addr().unwrap()
                            );
                            stream.shutdown(Shutdown::Both).unwrap();
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    /* connection failed */
                }
            }
        }
        // close the socket server
        drop(listener);
    } else {
        //open the socket on all interfaces
        let socket = UdpSocket::bind(format!("0.0.0.0:{UDP_PORT}")).unwrap();
        println!("Server running (UDP)...\n");

        loop {
            // Receives a single datagram message on the socket. If `buf` is too small to hold
            // the message, it will be cut off.
            let mut buf = [0; 2048];
            let (_amt, src) = socket.recv_from(&mut buf).unwrap();

            print_message(&buf, src);
        }
    }
}
