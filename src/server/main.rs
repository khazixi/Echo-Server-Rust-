use std::{
    fmt::write,
    io::{self, stdout, Read, Write, Stdout},
    net::{self, TcpListener, TcpStream},
};

fn main() -> std::io::Result<()> {
    let listener = net::TcpListener::bind("127.0.0.1:42069")?;

    let mut stdout_handle = io::stdout().lock();

    let mut buf: [u8; 8192] = [0; 8192];

    for stream in listener.incoming() {
        let bytes_read = stream?.read(&buf)?;

        stdout_handle.write(&buf);

        stream?.write(&buf);

        stdout_handle.flush();
        stream?.flush();
    }

    Ok(())
}
