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
    available_mb: Option<f64>, 
}

pub fn get_host_memory() -> MemInfo {
    let meminfo = fs::read_to_string("/proc/meminfo").ok(); // get result<s,e> instead
                                                                                          
    let mut mem = MemInfo {
        mem_total: None,
        mem_used: None,
        mem_used_percent: None,
        swap_total: None,
        swap_used: None,
        swap_used_percent: None,
    };
    
    match meminfo {
        Some(inforead) => {
            if let Some(r_mem) = get_raw_memory(&inforead) {
                mem.mem_total = r_mem.total_mb; 
                mem.mem_used = r_mem.total_mb
                    .zip(r_mem.available_mb)
                    .map(|(t, a)| t - a);
                mem.mem_used_percent = mem.mem_used
                    .zip(mem.mem_total)
                    .map(|(u, t)| (u / t * 100.0) as u32);
            };
        
            if let Some(r_mem_swap) = get_raw_swap(&inforead) {
                mem.swap_total = r_mem_swap.total_mb;
                mem.swap_used = r_mem_swap.total_mb
                    .zip(r_mem_swap.available_mb)
                    .map(|(t, a)| t - a);
                mem.swap_used_percent = mem.swap_used
                    .zip(mem.swap_total)
                    .map(|(u, t)| (u / t * 100.0) as u32);
            };
        }
        None => {} // nothing necessary done, maybe throw debug! later date
    }

    return mem;
}

fn get_raw_memory(meminfo: &String) -> Option<RawMemory> {
    let mut total: Option<f64> = None;
    let mut available: Option<f64> = None;
    let mut fallback: f64 = 0.0; // available temp storage where memavailable is n/a
                                          
    for line in meminfo.lines() {
        let mut blocks = line.split_whitespace();

        let key = blocks.next();
        let value = blocks.next().and_then(|v: &str| v.parse::<f64>().ok());

        match key {
            Some("MemTotal:") => total = value,
            Some("MemFree:") => { fallback = value.unwrap_or(0.0); }
            Some("Buffers:") => { fallback += value.unwrap_or(0.0); }
            Some("Cached:") => { fallback += value.unwrap_or(0.0); }
            Some("SReclaimable:") => { fallback += value.unwrap_or(0.0); }
            Some("Shmem:") => { fallback -= value.unwrap_or(0.0); }
            Some("MemAvailable:") => available = value,
            _ => {}
        }
    }
    available = Some(available.unwrap_or_else(|| fallback));

    Some(RawMemory {
        total_mb: total.map(|val| (val / 1000.0) / 1.049),
        available_mb: available.map(|val| (val / 1000.0) / 1.049),
    })
}
// these dont do the same thing anymore so! lmao
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
        total_mb: total.map(|val| (val / 1000.0) / 1.049),
        available_mb: available.map(|val| (val / 1000.0) / 1.049), 
    })

}

