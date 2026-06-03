use std::fs;

pub fn get_host() -> String {
    return fs::read_to_string("/etc/hostname").unwrap();
}


