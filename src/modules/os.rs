use crate::modules::libc::uname::call_uname;

#[derive(Debug)] // for outputting in bulk on testing, remove once clearcut 
pub struct OSInfo {
    pub distro: Option<String>,
    pub kernel: Option<String>,
    pub kernel_release: Option<String>,
    pub arch: Option<String>,
}

pub fn get_os() -> OSInfo {
    let release: String = std::fs::read_to_string("/etc/os-release").unwrap();
    let mut distrorel: String = String::new();
    for line in release.lines() {
        if let Some(val) = line.strip_prefix("PRETTY_NAME=") {
            distrorel = val.trim_matches('"').into();
        }
    }

    let os_info = call_uname();

    return OSInfo {
        distro: Some(distrorel),
        kernel: Some(os_info.sysname),
        kernel_release: Some(os_info.release),
        arch: Some(os_info.machine),
    }
}
