use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32, // Відсоток використання CPU (0.0 - 100.0)
    //    pub load_avg_1min: f32,      // Середнє навантаження за 1 хв
    //    pub load_avg_5min: f32,      // Середнє навантаження за 5 хв
    //    pub load_avg_15min: f32,     // Середнє навантаження за 15 хв
    pub total_memory_mb: u64,    // Загальний обсяг RAM (MB)
    pub used_memory_mb: u64,     // Використана пам'ять (MB)
    pub free_memory_mb: u64,     // Вільна пам'ять (MB)
    pub swap_usage_percent: f32, // Використання swap (%)
    //   pub disk_usage_percent: f32, // Використання диску (%)
    // pub disk_read_kbps: f32,  // Швидкість читання (KB/s)
    // pub disk_write_kbps: f32, // Швидкість запису (KB/s)
    //    pub network_rx_kbps: f32,    // Швидкість отримання (KB/s)
    //    pub network_tx_kbps: f32,    // Швидкість відправлення (KB/s)
    pub process_count: u32,   // Кількість процесів
    pub cpu_temperature: f32, // Температура CPU (°C)
    pub uptime_sec: u64,      // Час роботи системи (секунди)
    pub hostname: String,     // Ім'я хоста
    pub timestamp: String,    // Часовий штамп
}

impl SystemMetrics {
    pub fn new() -> Self {
        Self {
            cpu_usage: 0.0,
            //        load_avg_1min: 0.0,
            //        load_avg_5min: 0.0,
            //        load_avg_15min: 0.0,
            total_memory_mb: 0,
            used_memory_mb: 0,
            free_memory_mb: 0,
            swap_usage_percent: 0.0,
            //       disk_usage_percent: 0.0,
            //       disk_read_kbps: 0.0,
            //       disk_write_kbps: 0.0,
            //       network_rx_kbps: 0.0,
            //       network_tx_kbps: 0.0,
            process_count: 0,
            //       thread_count: 0,
            cpu_temperature: 0.0,
            uptime_sec: 0,
            hostname: String::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
