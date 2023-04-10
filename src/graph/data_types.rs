/// Contains a colour as defined in the CIE XYZ Colour space.
#[derive(Clone, Copy)]
pub struct Colour {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy)]
pub struct Universe {
    pub data: [u8; 512],
}

#[derive(Clone)]
pub enum Data {
    Colour(Colour),
    Universe(Universe),
    MultiUniverse(Vec<Universe>),
}
