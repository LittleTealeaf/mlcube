use neuralnet::Network;

mod neuralnet;

fn main() {
    let mut network = Network::new(3, vec![10], 3);

    let values = [
        (vec![1f64, 0f64, 0f64], vec![0f64, 1f64, 1f64]),
        (vec![0f64, 1f64, 0f64], vec![1f64, 0f64, 1f64]),
        (vec![0f64, 0f64, 1f64], vec![1f64, 1f64, 0f64]),
        (vec![1f64, 0f64, 1f64], vec![0f64, 1f64, 0f64]),
        (vec![0f64, 1f64, 1f64], vec![0f64, 1f64, 0f64]),
    ];

    for i in 0..100000 {
        for (input, output) in values.clone() {
            let feed = network.feed_forward(input.clone());
            println!("{:?} => {:?} | {:?}", input, output, feed);
            let _ = network.back_propagate(input, output, 0.1 * 0.999999_f64.powi(i));
        }
    }

    // let network = Network::new(400, vec![400, 350, 300, 250, 200, 150, 100, 50], 18);
    // println!("{:?}", network.feed_forward(vec![1f64; 400]).unwrap());
}
