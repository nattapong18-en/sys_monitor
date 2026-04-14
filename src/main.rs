use std::fs;
use std::thread;
use std::time::Duration;
fn main() {
    let my_os = fs::read_to_string("/etc/os-release").expect("Falild to read /etc/os-release");
    let mut os_name = String::from("Unknown OS");
    for line in my_os.lines() {
        if line.starts_with("PRETTY_NAME=") {
            os_name = line.replace("PRETTY_NAME=", "").replace("\"", "");
        }
    }
    let my_cpuinfo = fs::read_to_string("/proc/cpuinfo").expect("Falild to read");
    let mut my_cpu_name = String::from("Unknown CPU");
    for line in my_cpuinfo.lines() {
        if line.starts_with("model name") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                my_cpu_name = parts[1].trim().to_string();
                break;
            }
        }
    }

    loop {
        let meminfo = fs::read_to_string("/proc/meminfo")
            .expect("Falild to read /proc/meminfo (Are you on Linux?");

        println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("--- 🐧 {} | 💻 {} System Monitor ---", os_name, my_cpu_name);

        let mut mem_total: f64 = 0.0;
        let mut mem_avaliable: f64 = 0.0;
        let mut swap_total: f64 = 0.0;
        let mut swap_free: f64 = 0.0;

        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                mem_total = extract_kb(line);
            } else if line.starts_with("MemAvaliable:")
                || (line.starts_with("MemFree:") && mem_avaliable == 0.0)
            {
                mem_avaliable = extract_kb(line);
            } else if line.starts_with("SwapTotal:") {
                swap_total = extract_kb(line);
            } else if line.starts_with("SwapFree:") {
                swap_free = extract_kb(line);
            }
        }

        let mem_used = mem_total - mem_avaliable;
        let percent_used = (mem_used / mem_total) * 100.0;
        let gb_divisor = 1024.0 * 1024.0;
        let swap_used = swap_total - swap_free;
        let swap_percent = (swap_used / swap_total) * 100.0;
        println!("RAM Total: {:.2} GB", mem_total / gb_divisor);
        println!(
            "RAM Used: {:.2} GB ({:.1}%)",
            mem_used / gb_divisor,
            percent_used
        );
        // println!("(Debug) RAM Total (Raw):{}", mem_total);
        // println!("(Debug) RAM Avaliable (Raw):{}", mem_avaliable);
        println!(
            "Swap used: {:.2} GB ({:.1}%)",
            swap_used / gb_divisor,
            swap_percent
        );

        thread::sleep(Duration::from_secs(1));
    }
}
fn extract_kb(line: &str) -> f64 {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() >= 2
        && let Ok(kb) = parts[1].parse::<f64>()
    {
        return kb;
    }
    0.0
}
