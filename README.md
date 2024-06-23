# Echo Netowrking

Echo networking is a basic echo server and client in rust using
synchronous IO. The server reads bytes from the client application
and writes those same bytes to the application. 

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
listening on [::]:3000
```

You can also build the executable which you can execute directly.
```console
$ cargo build --bin echo_client --release
$ find . -name echo_client
./target/release/echo_client
```

## License

Echo Networking is distributed under the MIT-style license found in the 
[LICENSE](LICENSE) file.
