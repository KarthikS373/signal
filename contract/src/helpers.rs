use cosmwasm_std::Addr;

fn generate_anonymous_id(addr: &Addr) -> String {
    // let mut hasher = Sha256::new();
    // let mut hasher = String::from("");
    // hasher.update(addr.as_bytes());
    // format!("anon_{}", hex::encode(hasher.finalize()))
    "".to_string()
}
