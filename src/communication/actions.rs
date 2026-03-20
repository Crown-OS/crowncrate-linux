use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize, Hash)]
#[repr(u8)]
pub enum Actions {
    CLIPBOARD,
    MEDIA,
    OPEN,
    OTPSYNC,
    MONITOR,
    VOLUME,
    SHUTDOWN,
}
