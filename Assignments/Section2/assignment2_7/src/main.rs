// Assignment 2.7: Graph Algorithm Framework

trait Graph {
    type Node;
    type Edge;
    fn init(n: Self::Node, e: Self::Edge) -> Self;
    fn describe(&self);
}

struct CityGraph {
    root_city: String,
    distance: u32,
}

impl Graph for CityGraph {
    type Node = String;
    type Edge = u32;

    fn init(n: Self::Node, e: Self::Edge) -> Self {
        Self {
            root_city: n,
            distance: e,
        }
    }

    fn describe(&self) {
        println!(
            "City Graph: {} with distance {}",
            self.root_city, self.distance
        )
    }
}

// Dynamically building a graph using implemented init function and then describing it.
fn build_and_describe<G: Graph>(n: G::Node, e: G::Edge) {
    G::init(n, e).describe();
}

fn main() {
    build_and_describe::<CityGraph>("London".to_string(), 500);
}
