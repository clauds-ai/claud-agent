use std::fmt;
use std::fs::File;
use std::io::Write;
use std::process::Command;

#[derive(Debug)]
pub enum SystemdError {
    IoError(std::io::Error),
    SystemctlFailure,
    ValidationError(String),
}

impl fmt::Display for SystemdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SystemdError::IoError(e) => write!(f, "I/O error: {}", e),
            SystemdError::SystemctlFailure => write!(f, "systemctl command failed"),
            SystemdError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for SystemdError {}

impl From<std::io::Error> for SystemdError {
    fn from(err: std::io::Error) -> Self {
        SystemdError::IoError(err)
    }
}

pub struct SystemdService {
    pub service_name: String,
    pub executable_path: String,
    pub user: String,
    pub group: String,
    pub working_directory: String,
    pub systemd_dir: String,
}

impl SystemdService {
    pub fn new(
        service_name: String,
        executable_path: String,
        user: String,
        group: String,
        working_directory: String,
        systemd_dir: String,
    ) -> Result<Self, SystemdError> {
        if service_name.is_empty() {
            return Err(SystemdError::ValidationError(
                "Service name cannot be empty".to_string(),
            ));
        }
        if !std::path::Path::new(&executable_path).exists() {
            return Err(SystemdError::ValidationError(format!(
                "Executable {} does not exist",
                executable_path
            )));
        }
        Ok(Self {
            service_name,
            executable_path,
            user,
            group,
            working_directory,
            systemd_dir,
        })
    }

    pub fn generate_service_file(&self) -> String {
        format!(
            r#"[Unit]
Description={} Service
After=network.target

[Service]
Type=simple
User={}
Group={}
WorkingDirectory={}
ExecStart={}
Restart=always
RestartSec=5s
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
"#,
            self.service_name, self.user, self.group, self.working_directory, self.executable_path
        )
    }

    pub fn install(&self) -> Result<(), SystemdError> {
        let service_file_path = format!("{}/{}.service", self.systemd_dir, self.service_name);
        let mut file = File::create(&service_file_path)?;
        file.write_all(self.generate_service_file().as_bytes())?;

        fn run_systemctl(args: &[&str]) -> Result<(), SystemdError> {
            let status = Command::new("systemctl").args(args).status()?;
            if !status.success() {
                return Err(SystemdError::SystemctlFailure);
            }
            Ok(())
        }

        run_systemctl(&["daemon-reload"])?;
        run_systemctl(&["enable", &self.service_name])?;
        run_systemctl(&["start", &self.service_name])?;

        Ok(())
    }
}
