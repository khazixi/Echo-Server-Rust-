use std::{io::{self, Read, Write}, net};
fn main() -> std::io::Result<()> {
    let mut connection = net::TcpStream::connect("127.0.0.1:42069")?;

    let mut input_handle = io::stdin();

    let mut stdout_handle = io::stdout().lock();

    let mut buf = [0u8; 8192];
    
    loop {
        stdout_handle.write(b"> ")?;
        stdout_handle.flush()?;

        input_handle.read(&mut buf)?;

        if buf.starts_with(b"quit") {
            println!("Exiting");
            break;
        }

        connection.write(&buf)?;

        connection.read(&mut buf)?;

        print!("[SERVER] ");
        stdout_handle.write(&buf)?;

        connection.flush()?;
    }

    Ok(())
}
