extern crate markov;
#[cfg(feature = "graph")]
extern crate petgraph;

#[cfg(feature = "graph")]
fn main() {
    let mut chain = markov::Chain::of_order(2);
    chain.feed(vec![
        'e', 'r', 't', 'r', 't', 'y', 'r', 't', 'e', 'r', 't', 'y', 't', 'r',
    ]);
    let graph = chain.graph();

    println!("{:?}", petgraph::dot::Dot::new(&graph));
}

#[cfg(not(feature = "graph"))]
fn main() {
    println!("graph example must be compiled with graph enabled.")
}
