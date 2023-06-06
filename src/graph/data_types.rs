/// Contains a colour as defined in the CIE XYZ Colour space.
#[derive(Clone, Copy, Debug)]
pub struct Colour {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Universe {
    pub data: [u8; 512],
}

#[derive(Clone, Debug)]
pub enum Data {
    Colour(Colour),
    Universe(Box<Universe>),
    MultiUniverse(Vec<Universe>),
}
