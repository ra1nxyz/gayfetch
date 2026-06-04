use std::fs;

pub struct MemInfo {
    mem_total: Option<f64>,
    mem_used: Option<f64>, // take from memavailable against total
    mem_used_percent: Option<u32>,
    swap_total: Option<f64>,
    swap_used: Option<f64>, // take from swapfree against total
}

struct RawMemory {
    total_mb: Option<f64>,
    available_mb: Option<f64>,
}

pub fn get_host_memory() -> MemInfo {
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap(); // same issue as processor.rs

    // call rawmemory and unwrap then use to calc used from available or something

    return MemInfo {

    }

}

fn get_raw_memory(meminfo: &String) -> Option<RawMemory> {
    let mut total: Option<f64> = None;
    let mut available: Option<f64> = None;

    for line in meminfo.lines() {
        if let Some(totalmem) = line.strip_prefix("MemTotal:") {
            total = totalmem
                .split_whitespace()
                .next()
                .and_then(|x: &str| x.parse::<f64>().ok());
        }
        if let Some(avail) = line.strip_prefix("MemAvailable:") {
            available = avail
                .split_whitespace()
                .next()
                .and_then(|x: &str| x.parse::<f64>().ok()); 
        }
    }

    Some(RawMemory {
        total_mb: total,
        available_mb: available,
    })
    
}

