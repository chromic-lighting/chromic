pub struct Colour {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Universe {
    pub data: [u8; 512],
}

pub enum Data {
    Colour(Colour),
    Universe(Universe),
    MultiUniverse(Vec<Universe>),
}
