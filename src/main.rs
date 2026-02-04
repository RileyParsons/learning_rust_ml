use linfa::prelude::Records;
use std::fs::File;
use std::path::Path;
use std::env;
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

// static variable is valid for the entire duration of the program
// fixed memory location and initilized at begining of program
// this is not needed but good to keep in mind as an option
// const is an alternitive for other scenarios
static USE_CSV_DATA: bool = false;
enum IrisNames{
    Setosa,
    Versicolor,
    Virginica

}
fn main() {
    println!("Playing with Rust ML with iris dataset");

    println!("Using CSV file: {}", USE_CSV_DATA);

    get_data();

    

}

// to fix error from non numeric valies in data
fn update_csv(csv: &Path){

    let clean_csv = "TODO";


    return clean_csv:;
}

fn get_data(){
    let mut rng = rand::thread_rng();

    if !USE_CSV_DATA {
        //need to create my own random number generator for rust to use and have it passed via mutale reference
        let original_data = linfa_datasets::iris().shuffle(&mut rng);
        let (train, test) = original_data.split_with_ratio(0.8);
        // print via formated string
        // {:?} and {:#?} are debug print
        println!("Train: {:#?}",train);
        println!("test: {:#?}",test);
        println!("{:?}", train.nsamples());
        println!("{:?}", test.nsamples());
    }
    else {
        let data_path = Path::new("./data/iris.csv");
        println!("Current dir: {:?}", env::current_dir().unwrap());
        if data_path.exists() {
            println!("path '{:?}' found to exist.", data_path);   
        }
        else {
            println!("path '{:?}' NOT found to exist.", data_path);
        }
        if data_path.is_file(){
            println!("file found at locaiton");
            let values = update_csv(data_path);


            let csv_data = File::open(data_path).unwrap();
            let data_array = linfa_datasets::array_from_csv(csv_data, true, b',');
        // testing read in
        println!("{:#?}",data_array);
        }
        else {
            println!("NO file found at locaiton");
        }
    }

}
