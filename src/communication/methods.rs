use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum Method {
    CLIPBOARD,
    MEDIA,
    OPEN,
    OTPSYNC,
    MONITOR,
    VOLUME,
    SHUTDOWN,
}
