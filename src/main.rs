use std::net::{Ipv4Addr, UdpSocket};
use log::{error, info, warn, debug};
use log4rs;

fn main() {
    log4rs::init_file("./config/log4rs.yaml", Default::default()).unwrap();

    let stream = UdpSocket::bind("127.0.0.1:53").unwrap();
    loop {
        let mut buf: [u8; 4096] = [0; 4096];
        let (n, src) = stream.recv_from(&mut buf).unwrap();
        let dns = UdpSocket::bind(
            format!(
                "{}:{}", Ipv4Addr::UNSPECIFIED.to_string(), "0"
            )
        ).unwrap();
        dns.connect("8.8.8.8:53").unwrap();
        dns.send(&mut buf[..n]).unwrap();

        // https://www.ietf.org/rfc/rfc1035.txt
        /*
                                        1  1  1  1  1  1
            0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
        +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        |                                               |
        /                     QNAME                     /
        /                                               /
        +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        |                     QTYPE                     |
        +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        |                     QCLASS                    |
        +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
         */

        info!("src: {:#?} raw bytes: {:#?}", src, &buf[..n]);

        let mut buf2: [u8; 4096] = [0; 4096];
        let (n, _) = dns.recv_from(&mut buf2).unwrap();

        stream.send_to(&buf2[..n], &src).unwrap(); // Forward dns Query Packet

    }
}
