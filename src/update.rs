use crate::graph;

impl graph::Graph {
    pub fn render(&mut self, t: std::time::Duration) -> Result<(), anyhow::Error> {
        let needs_updating = self.get_nodes().filter(|n| n.requires_update(t));

        needs_updating.for_each(|n| update_node(self, n));

        Ok(())
    }
}

fn update_node(g: graph::Graph, n: &dyn graph::Node) {}
