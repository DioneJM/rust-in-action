use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;
use clap::{App, Arg};
use trust_dns::op::{Message, MessageType, OpCode, Query};
use trust_dns::rr::{Name, RecordType};
use trust_dns::serialize::binary::{BinEncodable, BinEncoder};

fn main() {
    let app = App::new("resolve")
    .about("Simple DNS resolve CLI")
        .arg(Arg::with_name("dns-server").short("s").default_value("1.1.1.1"))
        .arg(Arg::with_name("domain-name").required(true))
        .get_matches();

    let domain_name_raw = app.value_of("domain-name").unwrap();
    let domain_name = Name::from_ascii(&domain_name_raw).unwrap();

    let dns_server_raw = app.value_of("dns-server").unwrap();
    let dns_server: SocketAddr = format!("{}:53", dns_server_raw)
        .parse()
        .expect("Invalid Address");

    let mut request_buffer: Vec<u8> = Vec::with_capacity(512);
    let mut response_buffer: Vec<u8> = vec![0; 512];

    let mut request = Message::new();
    request.add_query(Query::query(domain_name, RecordType::A));
    request
        .set_id(rand::random::<u16>())
        .set_message_type(MessageType::Query)
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    let localhost = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind localhost");
    let timeout = Duration::from_secs(3);
    localhost.set_read_timeout(Some(timeout)).unwrap();
    localhost.set_nonblocking(false).unwrap();

    let mut encoder = BinEncoder::new(&mut request_buffer);
    request.emit(&mut encoder).unwrap();

    let _amt = localhost.send_to(&request_buffer, dns_server)
        .expect("socket misconfigured");

    loop {
        let (_amt, remote_port) = localhost
            .recv_from(&mut response_buffer)
            .expect("timeout reached");

        if remote_port == dns_server {
            break;
        }
    }

    let dns_message = Message::from_vec(&response_buffer)
        .expect("Failed to parse response");

    for answer in dns_message.answers() {
        if answer.record_type() == RecordType::A {
            let resource = answer.rdata();
            let ip = resource.to_ip_addr()
                .expect("invalid IP address received");
            println!("{}", ip.to_string())
        }
    }
}
