use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::Ipv4Addr};

use crate::communication::Actions;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub client: Ipv4Addr,
    pub method: Actions,
    pub body: HashMap<String, String>,
}
