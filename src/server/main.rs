use std::{
    env,
    io::{self, Read, Write},
    net, thread,
};

fn handle_connection(
    mut echo_stream: net::TcpStream,
) -> io::Result<()> {
    let mut buf: [u8; 8192] = [0; 8192];

    while let Ok(size) = echo_stream.read(&mut buf) {
        echo_stream.write(&buf)?;

        println!("[BYTES] {}", size);

        let s = String::from_utf8(buf[0..size].to_vec());
        match s {
            Ok(stringg) if stringg.len() == 0 => (),
            Ok(stringg) => print!("[CLIENT] {}", stringg),
            Err(_) => println!("[ERROR] Invalid UTF-8 String")
        }

        buf.fill(0);
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let flag = match env::args().nth(0) {
        Some(data) => data.parse::<u16>().unwrap_or(42069),
        None => 42069,
    };

    let ip = net::Ipv4Addr::new(127, 0, 0, 1);
    let addr = net::SocketAddrV4::new(ip, flag);

    println!("listening on [::]:{}", flag);

    let listener = net::TcpListener::bind(addr)?;

    for stream in listener.incoming() {
        if let Ok(good_stream) = stream {
            thread::spawn(move || handle_connection(good_stream));
        };
    }

    Ok(())
}
