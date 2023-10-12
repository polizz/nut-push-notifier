use crate::ups::UpsStatus;

pub type NoticeParam = Vec<(String, String)>;

pub struct StatusEvent {
    pub ups_status: UpsStatus,
    pub changed: bool,
}
