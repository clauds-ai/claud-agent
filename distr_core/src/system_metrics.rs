use sysinfo::System;

pub fn collect_system_metrics() -> serde_json::Value {
    let mut system = System::new_all();
    system.refresh_all();

    serde_json::json!({
        "cpu_usage": system.global_cpu_info().cpu_usage() * 100.0,
        "total_memory_mb": system.total_memory() / 1024 / 1024,
        "used_memory_mb": system.used_memory() / 1024 / 1024,
        "hostname": std::env::var("HOSTNAME").unwrap_or_default(),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    })
}