use std::net;
use std::io::{Write, Read};
fn main() -> std::io::Result<()> {
    let mut connection = net::TcpStream::connect("127.0.0.1:42069")?;

    let mut input_handle = std::io::stdin();

    let mut buf = [0u8; 8192];
    
    loop {
        input_handle.read(&mut buf)?;

        if buf.starts_with(b"quit") {
            println!("Exiting");
            break;
        }

        connection.write(&buf)?;

        connection.flush()?;
    }

    Ok(())
}
