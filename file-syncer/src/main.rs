use std::{env, fs, io};
use glob::glob;

pub struct Pathandhash { 
    hash:String,
    path:String
}

fn main() {
    println!("Rust music syncer!");
    println!("is this the computer sending the files?");
    let mut issender = true;
    if issender == true {
        let mut hashes:Vec<Pathandhash> = Vec::new();
        let currentdir = env::current_dir();
        // if we are the sender, get a vec of all the 
        // hashes of the files in the specified dir
        println!("current dir is {:?}", currentdir);
        //cool, we know where we are located, so we can start walking over all the sub
        //directorys, and hash each file
        for entry in fs::read_dir(currentdir)? {
            println!()
        }
    } else {

    }
}

fn 

fn hashfileadd(){} 