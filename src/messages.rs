use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coordinates {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Shape {
    Square {
        size: f32,
        ctr: Coordinates,
    },
    Circle {
        radius: f32,
        ctr: Coordinates,
    },
    Cross {
        size: f32,
        line_width: f32,
        ctr: Coordinates,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    shape: Shape,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let msg = Message {
            shape: Shape::Square {
                size: 10.0,
                ctr: Coordinates { x: 0.0, y: 0.0 },
            },
        };
        let serialized = serde_json::to_string(&msg).unwrap();
        println!("{}", serialized);
    }
}
