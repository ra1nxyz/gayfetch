use std::fs;

struct Disk {
    model: Option<String>,
    size: Option<u32>,
    used_space: Option<u32>, // probably split this into disk and disk_partition 
    partitions: Option<Vec<Partition>>,
}

struct Partition {
    size: Option<u32>,
    used: Option<u32>,
    label: Option<String>,
    mount: Option<String>,
}


pub fn get_storage() -> Vec<Disk> {
    // do stujff here i dont wanna yet
}

fn enumerate_drives() -> Vec<String> {
    let drives: Vec<String> = Vec::new();
    for folder in fs::read_dir("/sys/block").unwrap() {
        let entry = entry.unwrap().file_name().to_string_lossy();
        drives.Append(entry);
    }
}

fn get_drive_partitions(devblock: &String) -> String {
    for folder in fs::read_dir(format!("/sys/block/{devblock}")).unwrap() {
        let folder = folder.unwrap(); // changing mid enumeration, this is 99% panicing
        let header = folder.file_name().to_string_lossy();

        if header.starts_with(devblock) && header != devblock {
            header;
        }
    }

}


fn get_drive_sectors(devblock: &String) -> u64 {

}








