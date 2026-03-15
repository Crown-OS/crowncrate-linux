use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::Ipv4Addr};

use crate::communication::Method;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub client: Ipv4Addr,
    pub method: Method,
    pub body: HashMap<String, String>,
}
