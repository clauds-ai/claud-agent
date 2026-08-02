use sysinfo::{System, CpuRefreshKind, MemoryRefreshKind, DiskRefreshKind, NetworkRefreshKind};
use std::time::Duration;
use crate::SystemMetrics;

pub struct SystemMetricsCollector {
    system: System,
}

impl SystemMetricsCollector {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self { system }
    }

    pub fn collect(&mut self) -> SystemMetrics {
        // Оновлюємо дані системи
        self.system.refresh_cpu();
        self.system.refresh_memory();
        self.system.refresh_disks();
        self.system.refresh_networks();

        // Збираємо метрики CPU
        let cpu_usage = {
            let cpu_usage = self.system.global_cpu_info().cpu_usage();
            cpu_usage * 100.0
        };

        let load_avg = self.system.load_average();
        let load_avg_1min = load_avg.one;
        let load_avg_5min = load_avg.five;
        let load_avg_15min = load_avg.fifteen;

        // Збираємо метрики RAM
        let total_memory_mb = self.system.total_memory() / 1024 / 1024;
        let used_memory_mb = self.system.used_memory() / 1024 / 1024;
        let free_memory_mb = total_memory_mb - used_memory_mb;
        let swap_usage_percent = {
            let total_swap = self.system.total_swap();
            let used_swap = self.system.used_swap();
            if total_swap > 0 {
                (used_swap as f32 / total_swap as f32) * 100.0
            } else {
                0.0
            }
        };

        // Збираємо метрики диску (сумарно по всіх дисках)
        let mut total_disk_usage = 0.0;
        let mut total_disk_read = 0.0;
        let mut total_disk_write = 0.0;
        let mut disk_count = 0;

        for disk in self.system.disks() {
            total_disk_usage += disk.used_space as f32 / disk.total_space as f32 * 100.0;
            total_disk_read += disk.read_bytes as f32 / 1024.0; // KB
            total_disk_write += disk.written_bytes as f32 / 1024.0; // KB
            disk_count += 1;
        }

        let disk_usage_percent = if disk_count > 0 {
            total_disk_usage / disk_count as f32
        } else {
            0.0
        };

        // Збираємо метрики мережі (сумарно по всіх інтерфейсах)
        let mut total_rx = 0.0;
        let mut total_tx = 0.0;
        let mut network_count = 0;

        for (_, network) in self.system.networks() {
            total_rx += network.incoming.as_f32() / 1024.0; // KB
            total_tx += network.outgoing.as_f32() / 1024.0; // KB
            network_count += 1;
        }

        let network_rx_kbps = if network_count > 0 {
            total_rx / network_count as f32
        } else {
            0.0
        };

        let network_tx_kbps = if network_count > 0 {
            total_tx / network_count as f32
        } else {
            0.0
        };

        // Збираємо метрики процесів
        let process_count = self.system.processes().len() as u32;
        let thread_count = self.system.processes().iter().map(|(_, p)| p.threads.len() as u32).sum();

        // Збираємо температуру CPU (якщо доступно)
        let cpu_temperature = {
            // Спробуємо прочитати температуру з /sys/class/thermal/
            // Це працює на більшості Linux-систем
            std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
                .ok()
                .and_then(|temp| temp.trim().parse::<f32>().ok())
                .map(|temp| temp / 1000.0) // Конвертуємо з міліградусів у градуси
                .unwrap_or(0.0)
        };

        // Збираємо uptime
        let uptime_sec = {
            std::fs::read_to_string("/proc/uptime")
                .ok()
                .and_then(|uptime| uptime.split_whitespace().next().and_then(|s| s.parse::<f64>().ok()))
                .map(|uptime| uptime as u64)
                .unwrap_or(0)
        };

        // Збираємо hostname
        let hostname = std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string());

        SystemMetrics {
            cpu_usage,
            load_avg_1min,
            load_avg_5min,
            load_avg_15min,
            total_memory_mb,
            used_memory_mb,
            free_memory_mb,
            swap_usage_percent,
            disk_usage_percent,
            disk_read_kbps: total_disk_read,
            disk_write_kbps: total_disk_write,
            network_rx_kbps,
            network_tx_kbps,
            process_count,
            thread_count,
            cpu_temperature,
            uptime_sec,
            hostname,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}