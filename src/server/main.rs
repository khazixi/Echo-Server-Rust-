use std::{
    env,
    io::{self, Read, Write},
    net,
};

fn handle_connection(
    echo_stream: &mut net::TcpStream,
    output_stream: &mut io::StdoutLock,
) -> io::Result<()> {
    let mut buf: [u8; 8192] = [0; 8192];

    while echo_stream.read(&mut buf).is_ok() {
        let size = echo_stream.write(&buf)?;

        output_stream.write(b"[CLIENT] ")?;
        output_stream.write(&buf)?;
        output_stream.flush()?;

        buf[0..size].fill(0);
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

    let mut stdout_handle = io::stdout().lock();

    for stream in listener.incoming() {
        if handle_connection(&mut stream?, &mut stdout_handle).is_err() {};
    }

    Ok(())
}
