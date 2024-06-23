use std::{
    io::{self, Read, Write},
    net::{self},
};

fn handle_connection(
    echo_stream: &mut net::TcpStream,
    output_stream: &mut io::StdoutLock,
) -> io::Result<()> {
    let mut buf: [u8; 8192] = [0; 8192];

    while echo_stream.read(&mut buf).is_ok() {
        echo_stream.write(&buf)?;

        output_stream.write(&buf)?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = net::TcpListener::bind("127.0.0.1:42069")?;

    let mut stdout_handle = io::stdout().lock();

    for stream in listener.incoming() {
        if handle_connection(&mut stream?, &mut stdout_handle).is_err() {
            ();
        };
    }

    Ok(())
}
