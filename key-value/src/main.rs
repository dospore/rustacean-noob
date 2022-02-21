fn main() {
    let mut args = std::env::args().skip(1);

    let db = match Database::new() {
        Ok(db) => db,
        Err(_err) => return
    };

    let key = args.next().unwrap();
    let value = match args.next() {
        Some(value) => value,
        None => {
            let value = match db.inner.get_key_value(&key) {
                Some((_key, value)) => value,
                None => {
                    println!("{} does not have a value stored", key);
                    return;
                }
            };
            println!("Key {}, Value {}", key, value);
            return; 
        }
    };

    let contents = format!("{}\t{}\n", key, value);


    std::fs::write("kd.db", contents);

    Database::insert(db, key.clone(), value.clone());

    println!("Key {}, Value {}", key, value);
}

use std::collections::HashMap;
struct Database {
    inner: HashMap<String, String>
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let contents = std::fs::read_to_string("kd.db")?;
        let mut inner = HashMap::new();
        for line in contents.lines() {
            let chunks: Vec<&str> = line.split('\t').collect();

            if chunks.len() != 2 {
                todo!("Return error");
            }
            let key = chunks[0];
            let value = chunks[1];
            inner.insert(key.to_owned(), value.to_owned());

            dbg!(line);
        }
        Ok(
            Database {
                inner: inner
            }
        )
    }

    fn insert(mut database: Database, key: String, value: String) -> () {
        let mut i = database.inner;
        i.insert(key, value);
        database.inner = i;
    }
}
