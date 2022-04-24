use std::{env, fs, io};
use glob::glob;
use std::path::Path;

#[derive(Debug)]
pub struct Pathandhash { 
    hash:String,
    path:String
}


fn main() {

    let mut hashes:Vec<Pathandhash> = Vec::new(); 


    println!("Rust music syncer!");
    println!("is this the computer sending the files?");
    let mut issender:bool = false;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    println!("you typed {}",buffer);
    match &buffer as &str {
        "yes\n" | "y\n" => issender=true,
        "n\n" => issender=false,
        _ => panic!("not y or n"),
    }

    if issender == true { // we are the syncer
        for entry in glob("/Users/tiffany/Music/Soulseek Downloads/**").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => hashes.push(Pathandhash {hash:path.display().to_string(), path:path.display().to_string(),}),
                Err(e) => println!("{:?}", e),
            }
        }
        println!("files assembled, ready to start pairing.")

    } else { // we are the syncee
        println!("you picked that this computer is not the sender");
    }

}

