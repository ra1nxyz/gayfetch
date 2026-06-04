use std::fs;

#[derive(Debug)]
pub struct CPUInfo {
    pub cpu_name: Option<String>,
    // pub core_count: Option<usize>,
    pub thread_count: Option<usize>,
    pub cpu_frequency: Option<f64>,
}

pub fn get_processor() -> CPUInfo {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap(); // panic if error, use expect or
                                                                // smn later
    let syscpu = "/sys/devices/system/cpu";

    return CPUInfo {
        cpu_name: get_cpu_model(&cpuinfo),
        thread_count: Some(get_cpu_threads(&cpuinfo)),
        cpu_frequency: get_cpu_frequency(syscpu),
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
    let mut max_freq = 0u64;

    for entry in fs::read_dir(syscpu).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if !name.starts_with("cpu") {
            continue; // skip files not related to cpu counts
        }

        let path = entry.path().join("cpufreq/scaling_max_freq");

        if let Ok(freq) = fs::read_to_string(path) {
            if let Ok(freq) = freq.trim().parse::<u64>() {
                max_freq = max_freq.max(freq);
            }
        }
    }
    return Some((max_freq / 1000) as f64);
}
