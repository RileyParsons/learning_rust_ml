use linfa::prelude::Records;

// links and references
// https://docs.rs/linfa/latest/linfa/
// https://docs.rs/rand/latest/rand/

// cargo build once dependencies have been added to cargo.toml file
// cargo run to build and compile
// cargo check to compile code without creating exe. good for error checking

// iris
// samples: 150
// features: 4
// target: 1

fn main() {
    println!("Playing with Rust ML with iris dataset");

    let mut rng = rand::thread_rng();
    // need to create my own random number generator for rust to use and have it passed via mutale reference
    let original_data = linfa_datasets::iris().shuffle(&mut rng);
    let (train, test) = original_data.split_with_ratio(0.8);

    // print via formated string
    // {:?} and {:#?} are debug print
    // println!("Train: {:#?}",train);
    // println!("test: {:#?}",test);

    println!("{:?}", train.nsamples());
    println!("{:?}", test.nsamples());

}
