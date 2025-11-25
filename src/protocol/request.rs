use std::collections::BTreeMap;

pub struct Request {
    method: String,
    client: String,
    body: BTreeMap<String, String>
}

