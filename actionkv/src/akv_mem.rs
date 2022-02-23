use libactionkv::ActionKV;

#[cfg(target_os="windows")]
const USAGE: &str = r#"
Usage:
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY
    akv_mem.exe FILE insert KEY VALUE
    akv_mem.exe FILE update KEY VALUE
"#;

#[cfg(not(target_os="windows"))]
const USAGE: &str = r#"
Usage:
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
"#;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let value = args.get(4);

    let path = std::path::Path::new(&file_name);
    let mut store = ActionKV::open(path).expect("Unable to open file");
    store.load.expect("Unable to load data from store");

    match action {
        "get" => match store.get(key).unwrap {
            None => eprintln!("{:?} not found", key),
            Some(value) => println!("{:?}", value)
        },
        "delete" => store.delete(key).unwrap(),
        "insert" => {
            let value = value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap()
        },
        "update" => {
            let value = value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap()
        },
        _ => eprintln!("{}", &USAGE),
    }
}