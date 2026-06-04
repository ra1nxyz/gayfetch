use std::fs;

#[derive(Debug)]
pub struct MemInfo {
    mem_total: Option<f64>,
    mem_used: Option<f64>, // take from memavailable against total
    mem_used_percent: Option<u32>,
    swap_total: Option<f64>,
    swap_used: Option<f64>, // take from swapfree against total
    swap_used_percent: Option<u32>,
}

struct RawMemory {
    total_mb: Option<f64>,
    available_mb: Option<f64>, // add buffers and etc into this later
}

pub fn get_host_memory() -> MemInfo {
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap(); // same issue as processor.rs
                                                                // for the love of god fix this
                                                                // already
    let mut mem = MemInfo {
        mem_total: None,
        mem_used: None,
        mem_used_percent: None,
        swap_total: None,
        swap_used: None,
        swap_used_percent: None,
    };

    if let Some(r_mem) = get_raw_memory(&meminfo) {
        mem.mem_total = r_mem.total_mb; 
        mem.mem_used = r_mem.total_mb
            .zip(r_mem.available_mb)
            .map(|(t, a)| t - a); 
        mem.mem_used_percent = mem.mem_used
            .zip(mem.mem_total)
            .map(|(u, t)| (u / t * 100.0) as u32);
    };
    if let Some(r_mem_swap) = get_raw_swap(&meminfo) {
        mem.swap_total = r_mem_swap.total_mb;
        mem.swap_used = r_mem_swap.total_mb
            .zip(r_mem_swap.available_mb)
            .map(|(t, a)| t - a);
        mem.swap_used_percent = mem.swap_used
            .zip(mem.swap_total)
            .map(|(u, t)| (u / t * 100.0) as u32);
    };

    return mem;
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
        total_mb: total.map(|val| val / 1048.576),
        available_mb: available.map(|val| val / 1048.576),
    })
}
// im not happy with these two functions doing the same thing but only with different prefix on the
// parsing, i also dont wanna reconfig rawmem function to use some sort of swapfetch flag or smn
// tho
fn get_raw_swap(meminfo: &String) -> Option<RawMemory> {
    let mut total: Option<f64> = None;
    let mut available: Option<f64> = None;

    for line in meminfo.lines() {
        if let Some(totalmem) = line.strip_prefix("SwapTotal:") {
            total = totalmem
                .split_whitespace()
                .next()
                .and_then(|x: &str| x.parse::<f64>().ok());
        }
        if let Some(avail) = line.strip_prefix("SwapFree:") {
            available = avail
                .split_whitespace()
                .next()
                .and_then(|x: &str| x.parse::<f64>().ok()); 
        }
    }

    Some(RawMemory {
        total_mb: total.map(|val| val / 1048.576),
        available_mb: available.map(|val| val / 1048.576), // data conversion test
    })

}

