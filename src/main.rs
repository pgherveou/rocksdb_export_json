// use rocksdb::{IteratorMode, DB};
use hex::ToHex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
struct KeyValue {
    key: String,
    value: String,
}
// see https://github.com/paritytech/substrate/blob/master/client/db/src/utils.rs#L37
pub const NUM_COLUMNS: u32 = 13;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Please provide a path to rocksdb folder");

    let db_config = kvdb_rocksdb::DatabaseConfig::with_columns(NUM_COLUMNS);
    let db = kvdb_rocksdb::Database::open(&db_config, path).unwrap();

    let mut key_values = Vec::<KeyValue>::new();
    for i in 0..NUM_COLUMNS {
        let iter = db.iter(i);
        for entry in iter {
            let (key, value) = entry.unwrap();

            // convert bytes to hex string
            let key_str = key.encode_hex::<String>();
            let value_str = value.encode_hex::<String>();

            key_values.push(KeyValue {
                key: key_str.to_string(),
                value: value_str.to_string(),
            });
        }
    }

    let json = serde_json::to_string_pretty(&key_values).unwrap();
    let mut file = File::create("rocksdb_export.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();

    println!("Export completed!");
}
