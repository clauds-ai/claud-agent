use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub temperature: f32,
    pub vibration: f32,
    pub motor_status: f32,
    pub voltage: f32,
    pub timestamp: String,
}

impl Metrics {
    pub fn new(temperature: f32, vibration: f32, motor_status: f32, voltage: f32) -> Self {
        Self {
            temperature,
            vibration,
            motor_status,
            voltage,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
