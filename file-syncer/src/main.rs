use std::{env, fs, io};

pub struct Pathandhash { 
    hash: String,
    path:String
}

fn main() {
    println!("Rust music syncer!");
    println!("is this the computer sending the files?");
    let mut issender = true;
    if issender == true {
        let mut hashes: Vec<Pathandhash> = vec![];
        let currentdir = env::current_dir();
        // if we are the sender, get a vec of all the 
        // hashes of the files in the specified dir
        println!("current dir is {:?}", currentdir);


    } else {

    }
}

fn hashfileadd() {
  
}