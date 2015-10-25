#![feature(negate_unsigned)]
#![feature(convert)]
#![allow(dead_code)]
#![allow(unused_variables)]
pub extern crate pcap;

pub mod net;
pub mod common;

use std::sync::{Arc, Mutex, Condvar};
use net::TcpSocketMessage;



//#[test]
fn test_net_tcp_http() {
	/*
		Start the network.
	*/
	let net_instance = net::new_net(vec![0xec, 0x55, 0xf9, 0x7a, 0x54, 0x70], 0xc0a8020c, vec![0x94, 0x10, 0x3e, 0xfd, 0x55, 0x69]);

	/*
		Create a TCP socket and connect to a remote machine.
	*/
	let mut socket = net_instance.new_tcp_socket();

	//socket.connect(vec![0x94, 0x10, 0x3e, 0xfc, 0xc6, 0xf2], 0, 0xc0a80101, 80);
	socket.connect(0x689cf603, 80);

	match socket.sys_recv() {
		TcpSocketMessage::Connected => println!("connected!!"),
		_ => panic!("unexpected message; instead of tcp connected"),
	}

	socket.send("GET / HTTP/1.1\x0d\x0aConnection: keep-alive\x0d\x0aAccept: */*\x0d\x0aHost: kmcg3413.net\x0d\x0a\x0d\x0a".bytes().collect());

   println!("[test_net_tcp_http] waiting on data reply from HTTP server");
   socket.recv();
   println!("[test_net_tcp_http] received data reply from HTTP server");
   
   net_instance.shutdown();
}

fn main() {
    test_net_tcp_http();
}