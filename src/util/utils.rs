pub fn exit_if_key_not_exist(key: &str) {
    if std::env::var(key).is_err() {
        log::error!("The key {} does not exist in the .env file.", key);
        std::process::exit(1);
    }
}
pub fn exit_if_keys_not_exist(keys: &[&str]) {
    for key in keys {
        exit_if_key_not_exist(key);
    }
}
pub fn get_key(key_name: &str) -> String {
    std::env::var(key_name).unwrap()
}
