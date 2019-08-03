use dns_lookup::{lookup_host, get_hostname};

extern crate pnet;
use pnet::datalink;

use std::net::{IpAddr,
               UdpSocket};


fn main() {
    dns_lookup();
    pnet_lookup();
    udp_method_lookup();
}

fn dns_lookup() {
    println!("Using the dns_lookup method");

    let hostname = get_hostname().unwrap();
    println!("{:?}", hostname);

    let ip_addr = lookup_host(&hostname).unwrap();
    println!("{:?}", ip_addr);
}

fn pnet_lookup() {
    println!("using pnet crate");
    
    for iface in datalink::interfaces() {
        println!("{:?}", iface.ips);
    }
}

static GOOGLE_DNS: &'static str = "8.8.8.8:53";

// Based on https://github.com/habitat-sh/habitat/blob/4527c32e25d60d77ece99cb7ac8cd1182cb85279/components/core/src/util/sys.rs#L9-L16
fn udp_method_lookup() {
    println!("udp_method_lookup");

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect(GOOGLE_DNS);

    let addr = socket.local_addr().unwrap();

    let ip = addr.ip();
}