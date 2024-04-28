use std::arch::aarch64::float32x2_t;
use std::net::{Ipv4Addr, UdpSocket};
use std::ptr::read;
use log::{error, info, warn, debug};
use log4rs;

/*
The structure of a raw packet is the wire representation of the DNS query and response
as documented by RFC 1035. A 12-byte DNS header is followed by either a question section for the query,
or by a variable number (can be 0) of records for the response. If TCP is used, then the raw packet must be prefixed with a 2-byte length field.
You can use this API to apply host NRPT rules, or to perform encrypted DNS queries, among other things.
 */
static DNS_HEADER_LEN: usize = 12;

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

        dns.connect("1.1.1.1:53").unwrap();
        dns.send(&mut buf[..n]).unwrap();
        
        let domain_len = buf[DNS_HEADER_LEN] as usize;
        let domain = String::from_utf8(buf[DNS_HEADER_LEN+1..DNS_HEADER_LEN+domain_len+1].to_vec()).unwrap();

        let tld_len = buf[DNS_HEADER_LEN+domain_len+1] as usize;
        let tld = String::from_utf8(buf[DNS_HEADER_LEN+domain_len+1..DNS_HEADER_LEN+domain_len+1+tld_len+1].to_vec()).unwrap();
        // TODO support subdomain

        let target = format!(
            "{}.{}", domain, tld
        );
        // https://en.wikipedia.org/wiki/List_of_DNS_record_types
        let query = match buf[DNS_HEADER_LEN+domain_len+1+tld_len+3] {
            1 => "A",
            28 => "AAAA",
            5 => "CNAME",
            15 => "MX",
            16 => "TXT",
            _ => todo!()
        };


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

        let mut buf2: [u8; 4096] = [0; 4096];
        let (n2, _) = dns.recv_from(&mut buf2).unwrap();
        let amp: f32 = n2 as f32 / n as f32;
        info!(
            "{}" ,format!(
                " {} dport={}, recv={}, send={}\n{} {} || Amplification: {:.3}%",
                src.ip(), src.port(), n, n2,
                query, target, amp
            ).as_str()
        );
        stream.send_to(
            &buf2[..n2], &src
        ).unwrap(); // Forward dns Query Packet

    }
}
