use std::fmt::format;
use std::io::{Cursor, Read, Write};
use std::net::TcpStream;
use byteorder::{BigEndian, ReadBytesExt};

fn main() {
	let mut stream = TcpStream::connect("127.0.0.1:7878");
	let hello = b"\"Hello\"";
	let mut size = 7_u32.to_be_bytes();
	let mut buf: [u8; 255] = [0; 255];
	if stream.is_ok() {
		let mut s = stream.unwrap();
		s.write(&size).expect("PANIKKK");
		s.write(hello);
		let mut handle = s.try_clone().unwrap().take(4);
		handle.read(&mut buf);
		let mut rdr = Cursor::new(buf);
		let size = rdr.read_u32::<BigEndian>().unwrap();
		println!("{}", size);
		handle = s.take(size as u64);
		handle.read(&mut buf);
		buf[size as usize] = 0;
		println!("{}", std::str::from_utf8(&buf).unwrap());
		serde::de::Deserialize
	}
}
