use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpsStatus {
    Online,
    Charging,
    OnBattery,
    Startup,
    None(String),
}

impl fmt::Display for UpsStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Online => write!(f, "Online"),
            Self::Charging => write!(f, "Charging"),
            Self::OnBattery => write!(f, "On Battery"),
            Self::Startup => write!(f, "Monitoring Started"),
            Self::None(status) => write!(f, "Unknown UPS Status, {}", status)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpsState<'main> {
    pub online_status_spec: &'main str,
    pub charge_status_spec: &'main str,
    pub discharge_status_spec: &'main str,
    pub status: UpsStatus,
    pub verbose_online_status: bool,
    state_changed: bool,
}

impl<'main> UpsState<'main> {
    pub fn new(online_status_spec: &'main str, charge_status_spec: &'main str, discharge_status_spec: &'main str, verbose_online_status: bool) -> Self {
        UpsState {
            online_status_spec,
            charge_status_spec,
            discharge_status_spec,
            state_changed: false,
            status: UpsStatus::Startup,
            verbose_online_status,
        }
    }

    pub fn update_status_from_str(&mut self, s: String) -> () {
        let next_status = match s {
            s if s == self.online_status_spec => UpsStatus::Online,
            s if s == self.charge_status_spec => {
                match self.verbose_online_status {
                    true => UpsStatus::Charging,
                    _ => UpsStatus::Online
                }
            },
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

#[cfg(test)]
mod tests {
    use super::*;

    struct UpsStateTester<'main> {
        ups_state: UpsState<'main>,
    }

    impl<'main> UpsStateTester<'main> {
        fn new(verbose: bool) -> Self {
            let online = "ONLINE";
            let charge = "CHARGE";
            let onbatt = "ONBATTERY";

            let ups_state = UpsState::new(online, charge, onbatt, verbose);

            UpsStateTester {
                ups_state
            }
        }

        fn set_input_str(&mut self, input: String) {
            self.ups_state.update_status_from_str(input)
        }

        fn get_state(&self) -> &UpsState {
            &self.ups_state
        }
    }

    #[test]
    fn can_create_ups_state() {
        let tester = UpsStateTester::new(false);

        assert_eq!(tester.get_state().status, UpsStatus::Startup);
        assert_eq!(tester.get_state().is_state_changed(), false);
    }

    #[test]
    fn can_update_status_from_str() {
        let online = "ONLINE".to_string();
        let mut tester = UpsStateTester::new(false);
        
        tester.set_input_str(online);

        assert_eq!(tester.get_state().status, UpsStatus::Online);
        assert_eq!(tester.get_state().is_state_changed(), true);
    }

    #[test]
    fn state_changed_resets_after_no_change() {
        let online = "ONLINE".to_string();
        let mut tester = UpsStateTester::new(false);

        tester.set_input_str(online.clone());
        tester.set_input_str(online);

        assert_eq!(tester.get_state().status, UpsStatus::Online);
        assert_eq!(tester.get_state().is_state_changed(), false);
    }

    #[test]
    fn state_change_for_verbose_online_to_charging() {
        let online = "ONLINE".to_string();
        let charge = "CHARGE".to_string();
        let mut tester = UpsStateTester::new(true);

        tester.set_input_str(online);
        tester.set_input_str(charge);

        assert_eq!(tester.get_state().status, UpsStatus::Charging);
        assert_eq!(tester.get_state().is_state_changed(), true);
    }

    #[test]
    fn state_change_for_no_verbose_online() {
        let online = "ONLINE".to_string();
        let charge = "CHARGE".to_string();
        let mut tester = UpsStateTester::new(false);

        tester.set_input_str(online);
        tester.set_input_str(charge);

        assert_eq!(tester.get_state().status, UpsStatus::Online);
        assert_eq!(tester.get_state().is_state_changed(), false);
    }

    #[test]
    fn unknown_state_text_from_ups() {
        let unknown = "UNKNOWN STATE".to_string();
        let mut tester = UpsStateTester::new(false);

        tester.set_input_str(unknown.clone());

        assert_eq!(tester.get_state().status, UpsStatus::None(unknown));
        assert_eq!(tester.get_state().is_state_changed(), true);
    }
}