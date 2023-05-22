//! TODO!: Implements a single DMX Universe

use crate::graph::Node;

pub struct Universe {}

#[async_trait::async_trait]
impl Node for Universe {}

impl Default for Universe {
    fn default() -> Self {
        todo!()
    }
}
