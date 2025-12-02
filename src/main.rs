use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
// 1. 引入 anyhow 的 Context 和 Result
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Database {
        Database {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    // 🔴 修正点：去掉 std::io::Error，只写 Result<()>
    fn save(&self) -> Result<()> {
        let file = File::create("kv.db")?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    // 这里也可以简写为 Result<Database>，因为前面 use 了
    fn load() -> Result<Database> {
        let file = File::open("kv.db");
        match file {
            Ok(f) => {
                let reader = std::io::BufReader::new(f);
                let db = serde_json::from_reader(reader).context("Failed to parse kv.db")?;
                Ok(db)
            }
            Err(_) => {
                println!("no existing db, creating new one");
                Ok(Database::new())
            }
        }
    }
}

fn main() -> Result<()> {
    let mut db = Database::load()?;
    println!("Init Data:{:?}", db);

    db.insert("mobile".to_string(), "iphone".to_string());
    db.save()?;
    println!("Data saved");
    Ok(())
}
