use std::{
    env,
    io::{self, Read, Write},
    net,
};
fn main() -> io::Result<()> {
    let flag = match env::args().nth(0) {
        Some(data) => data.parse::<u16>().unwrap_or(42069),
        None => 42069,
    };

    let ip = net::Ipv4Addr::new(127, 0, 0, 1);
    let addr = net::SocketAddrV4::new(ip, flag);

    let mut connection = net::TcpStream::connect(addr)?;

    let mut input_handle = io::stdin();

    let mut stdout_handle = io::stdout().lock();

    let mut buf = [0u8; 8192];

    loop {
        stdout_handle.write(b"> ")?;
        stdout_handle.flush()?;

        let size = input_handle.read(&mut buf)?;

        println!("[BYTES] {}", size);

        if buf.starts_with(b"quit") {
            println!("Exiting");
            break;
        }

        connection.write(&buf[0..size])?;

        connection.read(&mut buf)?;

        print!("[SERVER] ");
        stdout_handle.write(&buf)?;

        buf[0..size].fill(0);

        connection.flush()?;
    }

    Ok(())
}
