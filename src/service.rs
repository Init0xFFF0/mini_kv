use crate::db::Database;
use crate::error::KvResult;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream, db: &mut Database) -> KvResult<()> {
    let mut query = String::new();
    {
        let mut buf_read = BufReader::new(&mut stream);
        buf_read.read_line(&mut query)?;
    }
    let parse: Vec<&str> = query.trim().splitn(3, ' ').collect();
    match parse.as_slice() {
        ["GET", key] => match db.get(key) {
            Some(value) => stream.write_all(format!("值是{}\n", value).as_bytes())?,
            None => stream.write_all(b"NOT FOUND\n")?,
        },
        ["SET", key, value] => {
            db.insert(key.to_string(), value.to_string());
            db.save()?;
            stream.write_all(b"Ok\n")?;
        }
        _ => stream.write_all(b"INVALID COMMAND\n")?,
    }
    Ok(())
}
