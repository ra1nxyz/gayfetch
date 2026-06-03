use std::mem::MaybeUninit;

#[repr(C)]
struct UtsName {
    sys: [libc::c_char; 65],
    node: [libc::c_char; 65],
    release: [libc::c_char; 65],
    version: [libc::c_char; 65],
    machine: [libc::c_char; 65],
    domain: [libc::c_char; 65],
}

unsafe extern "C" {
    fn uname(buf: *mut UtsName) -> i32;
}

fn c_str_convert(arr: &[libc::c_char]) -> String {
    let bytes: Vec<u8> = arr
        .iter()
        .map(|&c| c as u8)
        .take_while(|&b| b != 0)
        .collect();

    return String::from_utf8_lossy(&bytes).to_string();
}

pub struct UnameInfo {
    pub sysname: String,
    pub release: String,
    pub machine: String,
}

pub fn call_uname() -> UnameInfo {
    let mut uts = MaybeUninit::<UtsName>::uninit();

    unsafe {
        uname(uts.as_mut_ptr());
        let uts = uts.assume_init();

        return UnameInfo {
            sysname: c_str_convert(&uts.sys),
            release: c_str_convert(&uts.release),
            machine: c_str_convert(&uts.machine),
        }
    }
}

