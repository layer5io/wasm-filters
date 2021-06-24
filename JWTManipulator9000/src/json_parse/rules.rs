use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd)]
pub struct ConfigJwt {
    pub add_header: Vec<(String, String)>,
    pub del_header: Vec<String>,
    pub add_payload: Vec<(String, String)>,
    pub del_payload: Vec<String>,
    pub payload_to_header: Vec<String>,
    pub header_to_payload: Vec<String>,
}

impl ConfigJwt {
    pub fn new() -> Self {
        ConfigJwt {
            add_header: Vec::new(),
            add_payload: Vec::new(),
            del_header: Vec::new(),
            del_payload: Vec::new(),
            payload_to_header: Vec::new(),
            header_to_payload: Vec::new(),
        }
    }
}