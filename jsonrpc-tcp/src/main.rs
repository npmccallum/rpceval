use jsonrpc_tcp_server::jsonrpc_core::*;
use jsonrpc_tcp_server::*;

fn main() {
    let mut io = IoHandler::default();
    io.add_method("say_hello", |_params| Ok(Value::String("hello".to_owned())));

    let server = ServerBuilder::new(io)
        .start(&"0.0.0.0:3030".parse().unwrap())
        .expect("Server must start with no issues");

    server.wait()
}
