# Echo Networking

Echo networking is a basic echo server and client in rust using
synchronous IO. The server reads bytes from the client application
and writes those same bytes to the client. 

## Limitations

The current iteration of this server uses a single thread with
blocking I/O. As a result, it is currently only able to echo
one TCP connection at a time.

## Purpose

This project was made in an effort by myself to learn rust.

## Running the server

You can run the server using cargo using the following command.

```console
$ cargo run --bin echo_server --release 3000
listening on [::]:3000
```

You can call the server without a port in the arguments. The
default server port is 42069

```console
$ cargo run --bin echo_server --release
listening on [::]:42069
```

You can also build the executable which you can execute directly.
```console
$ cargo build --bin echo_server --release
$ find . -name echo_server
./target/release/echo_server
```

## Running the client

You can run the client using cargo using the following command.

```console
$ cargo run --bin echo_client --release 3000
> 
```

You can call the client without a port in the arguments.
The default connection port is 42069.

```console
$ cargo run --bin echo_client --release
> 
```

You can type "quit" to exit stop the client. You can type any
other message to send the message to the server.

```console
> hello
[SERVER] hello
> quit
Exiting
```

You can also build the executable which you can execute directly.

```console
$ cargo build --bin echo_client --release
$ find . -name echo_client
./target/release/echo_client
> 
```

## License

Echo Networking is distributed under the MIT-style license found in the 
[LICENSE](LICENSE) file.
