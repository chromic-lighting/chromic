// Public to prevent unused code warnings
// TODO: Make private again when no longer needed
pub mod graph;

fn main() -> anyhow::Result<()> {
    let _ = graph::Graph::new();
    Ok(())
}
