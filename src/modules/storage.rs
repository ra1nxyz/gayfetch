use std::fs;

struct Disk {
    model: Option<String>,
    size: Option<u32>,
    used_space: Option<u32>, // probably split this into disk and disk_partition 
    mount: Option<String>,
}


pub fn get_storage() -> Vec<Disk> {
    // do stujff here i dont wanna yet
}











