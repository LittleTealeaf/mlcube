use neuralnet::Network;

mod neuralnet;

fn main() {
    let network = Network::new(10, vec![10, 10, 10, 10], 2);
    println!(
        "{:?}",
        network
            .feed_forward(vec![0.9,0.8,0.7,0.6,0.5,0.4,0.3,0.2,0.1,0.0])
            .unwrap()
    );
}
