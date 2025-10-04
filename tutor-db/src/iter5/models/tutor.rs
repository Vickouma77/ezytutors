use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tutor {
    tutor_id: i32,
    tutor_name: String,
    tutor_pic_url: String,
    tutor_profile: String
}