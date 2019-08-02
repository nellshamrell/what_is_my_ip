use dns_lookup::{lookup_host, get_hostname};

extern crate pnet;
use pnet::datalink;

fn main() {
    dns_lookup();
    pnet_lookup();
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