use smol_str::SmolStr;
use std::collections::HashMap;

/// Contains a colour as defined in the CIE XYZ Colour space.
#[derive(Clone, Copy, Debug)]
pub struct Colour {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// A potition defined in 3d space, (fixture positions need to be defined for this to be particularly useful (TODO: Add way to calibrate fixture position))
#[derive(Debug, Clone, Copy)]
pub struct Position3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// A position defined by a conventional pan and tilt value
#[derive(Clone, Copy, Debug)]
pub struct PositionPanTilt {
    pub pan: f64,
    pub tilt: f64,
}

/// A position
#[derive(Clone, Copy, Debug)]
pub enum Position {
    PanTilt(PositionPanTilt),
    Pos3d(Position3d),
}

mod fixture_data_types {
    #[derive(Clone, Copy, Debug)]
    pub struct Zoom(f64);
}

#[derive(Clone, Copy, Debug)]
pub struct Universe {
    pub data: [u8; 512],
}

#[derive(Clone, Debug)]
pub enum FixtureSpec {
    Manufacturer(SmolStr),
    FixtureType(SmolStr),
    GroupName(SmolStr),
    FixtureId(SmolStr),
}

#[derive(Clone, Copy, Debug)]
pub enum FixtureDatum {
    Colour(Colour),
    Intensity(f64),
    Position(Position),
    Zoom(fixture_data_types::Zoom),
}

#[derive(Clone, Debug)]
pub struct FixtureData(HashMap<FixtureSpec, FixtureDatum>);

#[derive(Clone, Debug)]
pub enum Data {
    Colour(Colour),
    Position(Position),
    Float(f64),
    VecFloat(Vec<f64>),
    Int(i64),
    VecInt(Vec<i64>),
    FixtureData(FixtureData),
    Universe(Box<Universe>),
    MultiUniverse(Vec<Universe>),
    GroupName(SmolStr),
}
