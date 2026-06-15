use x86::cpuid::CpuId;

use x86_msr::{HwpCapabilities, HwpRequest, SpeedShift};

fn main() {
    let cpuid = CpuId::new();

    cpuid.get_feature_info().map(|f| dbg!(f));

    match cpuid.get_thermal_power_info() {
        Some(info) => {
            println!("Speed Shift (HWP): {}", info.has_hwp());

            // Дополнительные возможности Speed Shift
            if info.has_hwp() {
                println!("\t- HWP Notification: {}", info.has_hwp_notification());
                println!(
                    "\t- HWP Activity Window: {}",
                    info.has_hwp_activity_window()
                );
                println!(
                    "\t- HWP EPP: {}",
                    info.has_hwp_energy_performance_preference()
                );
                println!("\t- HWP Package: {}", info.has_hwp_package_level_request());
            }
        }
        None => {
            println!("Thermal/Power info not supported");
        }
    }

    // Example: reading capabilities if SpeedShift is available
    match SpeedShift::new() {
        Ok(ss) => {
            let caps = ss.read_capabilities();
            println!("HWP Capabilities: {:?}", caps);
        }
        Err(e) => {
            eprintln!("Cannot use SpeedShift: {}", e);
        }
    }
}

#[test]
pub fn bench_cpu_frequency_with_speed_shift() {
    let ss = SpeedShift::new().expect("Failed to initialize SpeedShift");

    for epp in [0, 32, 64, 96, 128, 160, 192, 224, 255] {
        ss.set_epp(epp);

        // Даём время на применение
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Запускаем нагрузку на короткое время для измерения
        let freq = measure_under_load();

        let actual_epp = ss.get_epp();
        println!(
            "EPP {:3} | Actual EPP: {:3} | Freq: {:4.0} MHz | {}",
            epp,
            actual_epp,
            freq,
            epp_bar(epp)
        );
    }
}

fn measure_actual_frequency_mhz() -> Option<u64> {
    std::fs::read_to_string("/sys/devices/cpu/cpu0/cpufreq/scaling_cur_freq")
        .expect("Failed to read current frequency")
        .trim()
        .parse::<u64>()
        .ok()
        .map(|f| f / 1000)
}

fn measure_under_load() -> u64 {
    // Короткая нагрузка чтобы процессор вышел на нужную частоту
    let start = std::time::Instant::now();
    let mut _dummy = 0u64;

    // ~50ms нагрузки
    while start.elapsed() < std::time::Duration::from_millis(50) {
        _dummy = _dummy.wrapping_add(1);
        _dummy = _dummy.wrapping_mul(3);
        std::hint::black_box(_dummy);
    }

    measure_actual_frequency_mhz().unwrap_or_default()
}

fn epp_bar(epp: u8) -> String {
    let filled = (epp as usize + 1) / 4;
    let empty = 64 - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}
