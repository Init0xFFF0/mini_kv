use mini_kv::service::handle_client;
use mini_kv::{Database, KvResult};
use std::net::TcpListener;
// 给那一长串复杂的类型起个名字

fn main() -> KvResult<()> {
    let mut db = Database::load()?;
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("开始监听127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream?;
        handle_client(stream, &mut db)?;
    }

    Ok(())
}
