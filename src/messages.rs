use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    shape: String,
    x_ctr: f32,
    y_ctr: f32,
}
