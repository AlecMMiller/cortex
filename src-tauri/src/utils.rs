use std::time::SystemTime;

pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("SystemTime reported before UNIX Epoch")
        .as_millis()
        .try_into()
        .expect("SystemTime reported time ridiculously far into the future")
}
