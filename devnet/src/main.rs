use std::fs::{self, File};
use std::path::Path;
use sha2::{Sha256, Digest};
use serde::Deserialize;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct Genesis {
    treasury_address: String,
    initial_validators: Vec<String>,
    dao_addresses: Vec<String>,
    modules: Vec<ModuleInit>
}

#[derive(Debug, Deserialize)]
struct ModuleInit {
    name: String,
    path: String,
    upgradeable: bool
}

fn hash_wasm(path: &str) -> [u8; 32] {
    let mut file = File::open(path).expect("WASM not found");
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    let hash = Sha256::digest(&data);
    let mut out = [0u8; 32];
    out.copy_from_slice(&hash[..]);
    out
}

fn main() {
    let genesis_path = "../genesis/genesis.json";
    let raw = fs::read_to_string(genesis_path).expect("Missing genesis.json");
    let config: Genesis = serde_json::from_str(&raw).unwrap();

    println!("[Genesis] Treasury Address: {}", config.treasury_address);
    println!("[Genesis] DAO Controllers: {:?}", config.dao_addresses);
    println!("[Genesis] Validators: {:?}", config.initial_validators);

    for module in config.modules {
        let hash = hash_wasm(&module.path);
        println!(
            "[Module] {} (Upgradeable: {}): 0x{}",
            module.name,
            module.upgradeable,
            hex::encode(hash)
        );
    }

    println!("[DevNet] All runtime modules loaded. Validator setup ready.");
}
