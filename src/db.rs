use crate::error::KvResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{read_to_string, rename, write};
use std::io::ErrorKind;

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    map: HashMap<String, String>,
}
impl Database {
    pub fn new() -> Self {
        Database {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
    pub fn insert(&mut self, key: String, value: String) -> Option<String> {
        self.map.insert(key, value)
    }

    // fn remove(&mut self, key: &str) -> Option<String> {
    //     self.map.remove(key)
    // }
    pub fn save(&self) -> KvResult<()> {
        let tmp_path = "db.kv.tmp";
        let path = "db.kv";
        println!("序列化");
        let to_string_pretty = serde_json::to_string_pretty(self)?;
        println!("写入临时文件");
        write(tmp_path, to_string_pretty)?;
        // 注意这里：加上分号，表示这是一个"语句" (Statement)
        rename(tmp_path, path)?;
        Ok(())
    }
    pub fn load() -> KvResult<Self> {
        let read_to_string = match read_to_string("db.kv") {
            Ok(it) => it,
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    return Ok(Self::new());
                }
                return Err(e.into());
            }
        };
        let database = serde_json::from_str(&read_to_string)?;
        Ok(database)
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
