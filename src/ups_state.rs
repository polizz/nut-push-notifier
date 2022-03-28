use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpsState {
    pub charge_status_spec: String,
    pub discharge_status_spec: String,
    pub status: UpsStatus,
    state_changed: bool,
}

impl UpsState {
    pub fn new(charge_status_spec: String, discharge_status_spec: String) -> Self {
        UpsState {
            charge_status_spec,
            discharge_status_spec,
            state_changed: false,
            status: UpsStatus::Startup,
        }
    }

    pub fn update_status_from_str(&mut self, s: String) -> () {
        let next_status = match s {
            s if s == self.charge_status_spec => UpsStatus::Online,
            s if s == self.discharge_status_spec => UpsStatus::OnBattery,
            str => UpsStatus::None(str),
        };

        if next_status != self.status {
            self.state_changed = true;
            self.status = next_status;
        } else {
            self.state_changed = false;
        }
    }

    pub fn is_state_changed(&self) -> bool {
        self.state_changed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpsStatus {
    Online,
    OnBattery,
    Startup,
    None(String),
}

impl fmt::Display for UpsStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Online => write!(f, "Online"),
            Self::OnBattery => write!(f, "On Battery"),
            Self::Startup => write!(f, "Monitoring Started"),
            Self::None(status) => write!(f, "Unknown UPS Status, {}", status)
        }
    }
}