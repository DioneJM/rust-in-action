use serde_derive::Serialize;

#[derive(Serialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}

fn main() {
    let calabar = City {
        name: String::from("Calabar"),
        population: 470_000,
        latitude:  4.95,
        longitude: 8.33
    };

    let as_json = serde_json::to_string(&calabar).unwrap();
    let as_cbor = serde_cbor::to_vec(&calabar).unwrap();
    let as_bincode = bincode::serialize(&calabar).unwrap();

    println!("json:\n{}\n", &as_json);
    println!("cbor:\n{:?}\n", &as_cbor);
    println!("bincode:\n{:?}\n", &as_bincode);
    println!("json as UTF-8:\n{}\n", String::from_utf8_lossy(as_json.as_bytes()));
    println!("cbor as UTF-8:\n{}\n", String::from_utf8_lossy(as_cbor.as_slice()));
    println!("bincode as UTF-8:\n{}\n", String::from_utf8_lossy(as_bincode.as_slice()));
}
