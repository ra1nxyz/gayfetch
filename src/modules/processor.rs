use std::fs;

#[derive(Debug)]
pub struct CPUInfo {
    pub cpu_name: Option<String>,
    // pub core_count: Option<usize>,
    pub thread_count: Option<usize>,
    pub cpu_frequency: Option<f64>,
}

pub fn get_processor() -> CPUInfo {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").ok();
    let syscpu = "/sys/devices/system/cpu";

    match cpuinfo {
        Some(inforead) => {
            return CPUInfo {
                cpu_name: get_cpu_model(&inforead),
                thread_count: Some(get_cpu_threads(&inforead)),
                cpu_frequency: get_cpu_frequency(syscpu),
            }
        }
        None => {
            return CPUInfo {
                cpu_name: None,
                thread_count: None,
                cpu_frequency: get_cpu_frequency(syscpu), // assuming the comment down, this should
                                                          // work anyways in any linux scenario,
                                                          // idk about just returning freq even
                                                          // before looking at other methods
            }
        }
    }
}

fn get_cpu_model(cpuinfo: &String) -> Option<String> {
    for line in cpuinfo.lines() {
        if let Some(name) = line.strip_prefix("model name\t: ") {
            return Some(name.to_string());
        }
    }
    return None;
}

fn get_cpu_threads(cpuinfo: &String) -> usize {
    return cpuinfo
        .lines()
        .filter(|line| line.starts_with("processor"))
        .count();
}

fn get_cpu_frequency(syscpu: &str) -> Option<f64> {
    let mut max_freq: Option<u64> = None;

    for entry in fs::read_dir(syscpu).unwrap() { // pretty sure this is present in every linux
                                                 // distro but read further for this later
        let entry = entry.unwrap();
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if !name.starts_with("cpu") {
            continue; // skip files not related to cpu counts
        }

        let path = entry.path().join("cpufreq/scaling_max_freq");

        if let Ok(freq) = fs::read_to_string(path) {
            if let Ok(freq) = freq.trim().parse::<u64>() {
                max_freq = Some(max_freq.map_or(freq, |m| m.max(freq)));
            }
        }
    }
    return max_freq.map(|f| f as f64 / 1000.0);
}

// cpufreq tends to be missing in things such as TrueNAS/NixOS vms (only ones ive tested), will add
// a fallback option later, for now changed handling of max_freq to an Option directly as any sort
// of operations even when cpufreq doesnt exist returns 0 not None otherwise

