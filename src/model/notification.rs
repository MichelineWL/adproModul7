use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Notification {
    pub product_title: String,
    pub product_type: String,
    pub product_url: String,
    pub subscriber_name: String,
    pub status: String
}