#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert_and_get() {
        let mut db = Database::new();
        db.insert("key1".to_string(), "value1".to_string());
        db.insert("key2".to_string(), "value2".to_string());
        assert_eq!(db.get("key1"), Some(&"value1".to_string()));
        assert_eq!(db.get("key2"), Some(&"value2".to_string()));
        assert_eq!(db.get("key3"), None);
    }
    #[test]
    fn test_overwrite() {
        //旧值是否被正确返回”以及“新值是否生效;
        let mut db = Database::new();
        db.insert("k1".to_string(), "v1".to_string());
        let old = db.insert("k1".to_string(), "v2".to_string());
        assert_eq!(old, Some("v1".to_string()));
        assert_eq!(db.get("k1"), Some(&"v2".into()))
    }
}
