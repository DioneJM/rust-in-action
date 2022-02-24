use std::collections::HashMap;
use libactionkv::{ActionKV, ByteStr, ByteString};

#[cfg(target_os="windows")]
const USAGE: &str = r#"
Usage:
    akv_disk.exe FILE get KEY
    akv_disk.exe FILE delete KEY
    akv_disk.exe FILE insert KEY VALUE
    akv_disk.exe FILE update KEY VALUE
"#;

#[cfg(not(target_os="windows"))]
const USAGE: &str = r#"
Usage:
    akv_disk FILE get KEY
    akv_disk FILE delete KEY
    akv_disk FILE insert KEY VALUE
    akv_disk FILE update KEY VALUE
"#;

fn store_index_on_disk(store: &mut ActionKV, index_key: &ByteStr) {
    store.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&store.index).unwrap();
    store.index = HashMap::new();
    store.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
    const INDEX_KEY: &ByteStr = b"+index";
    let args: Vec<String> = std::env::args().collect();
    let file_name = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_bytes();
    let value = args.get(4);

    let path = std::path::Path::new(&file_name);
    let mut store = ActionKV::open(path).expect("Unable to open file");
    store.load().expect("Unable to load data from store");

    match action {
        "get" => {
            let index_as_bytes = store.get(&INDEX_KEY)
                .unwrap().unwrap();
            let index_decoded = bincode::deserialize(index_as_bytes.as_slice());
            let index: HashMap<ByteString, u64> = index_decoded.unwrap();

            match index.get(key) {
                None => eprintln!("{:?} not found", key),
                Some(&i) => {
                    let kv = store.get_at(i).unwrap();
                    println!("{}", String::from_utf8_lossy(kv.value.as_slice()))
                }
            }
        },
        "delete" => store.delete(key).unwrap(),
        "insert" => {
            let value = value.expect(&USAGE).as_bytes();
            store.insert(key, value).unwrap();
            store_index_on_disk(&mut store, INDEX_KEY);
        },
        "update" => {
            let value = value.expect(&USAGE).as_bytes();
            store.update(key, value).unwrap();
        },
        _ => eprintln!("{}", &USAGE),
    }
}