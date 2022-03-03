use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Command {
    Draw(Shape),
    Clear([f64; 4]),
}

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
pub enum Message {
    SetShape(Shape),
    SetBgColor([f64; 4]),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let msg = Message::SetShape(Shape::Square {
            size: 0.2,
            ctr: Coordinates { x: 0.0, y: 0.0 },
        });
        let serialized = serde_json::to_string(&msg).unwrap();
        println!("{}", serialized);

        let msg = Message::SetBgColor([0.1, 0.2, 0.3, 1.0]);
        let serialized = serde_json::to_string(&msg).unwrap();
        println!("{}", serialized);
    }
}
