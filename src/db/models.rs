use serde_derive::Serialize;
use super::schema::hostname_mac_address;

#[derive(Serialize, Queryable)]
pub struct HostnameMacAdrress {
    pub hostname: String,
    pub mac_address: String,
}