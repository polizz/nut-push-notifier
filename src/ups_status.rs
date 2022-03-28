use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpsStatus {
    Online,
    OnBattery,
    Startup,
    Unknown(String),
}

impl fmt::Display for UpsStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Online => write!(f, "Online"),
            Self::OnBattery => write!(f, "On Battery"),
            Self::Startup => write!(f, "Monitoring Started"),
            Self::Unknown(status) => write!(f, "Unknown UPS Status, {}", status)
        }
    }
}

impl From<String> for UpsStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "OL" => UpsStatus::Online,
            "OB" => UpsStatus::OnBattery,
            str => UpsStatus::Unknown(String::from(str)),
        }
    }
}