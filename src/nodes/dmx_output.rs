//! TODO!: A DMX output method (Can accept one or several universes)

use crate::graph::Node;

pub struct DMXOutput {}

#[async_trait::async_trait]
impl Node for DMXOutput {}
