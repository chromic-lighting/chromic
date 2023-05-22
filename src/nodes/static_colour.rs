//! TODO!: A static colour.

use crate::graph;
use crate::graph::data_types::Colour;

/// Contains a colour as defined in the CIE XYZ Colour space.
pub struct StaticColour(Colour);

#[async_trait::async_trait]
impl graph::Node for StaticColour {}

/// Black by default
impl Default for StaticColour {
    fn default() -> Self {
        Self(Colour {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })
    }
}
