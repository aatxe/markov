extern crate markov;
extern crate petgraph;

fn main() {
    let mut chain = markov::Chain::new();
    chain.order(2);
    chain.feed(vec!('e', 'r', 't', 'r', 't', 'y', 'r', 't', 'e', 'r', 't', 'y', 't', 'r'));
    let graph = chain.graph();

    println!("{:?}", petgraph::dot::Dot::new(&graph));
}
