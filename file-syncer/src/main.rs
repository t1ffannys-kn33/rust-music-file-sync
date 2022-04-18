use std::{env, fs, io};
use glob::glob;

pub struct Pathandhash { 
    hash:String,
    path:String
}

fn main() {
    println!("Rust music syncer!");
    println!("is this the computer sending the files?");
    let mut issender:bool = false;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    println!("you typed {}",buffer);
    match &buffer as &str {
        "yes" | "y" => issender=true,
        "n" => issender=false,
        _ => println!("not y or n"),
    }

    if issender == true { // we are the syncer
        println!("{:?}", issender);
    } else { // we are the syncee
        println!("{:?}", issender);

    }
}
/*   broken code       
let mut hashes:Vec<Pathandhash> = Vec::new();
        let mut wheretoread = String::new;
        let mut stdin = io::stdin(); // We get `Stdin` here.

        stdin().read_line(&mut wheretoread)?;
        println!({}, wheretoread);
        // if we are the sender, get a vec of all the 
        // hashes of the files in the specified dir
        println!("current dir is {:?}", currentdir);
        //cool, we know where we are located, so we can start walking over all the sub
        //directorys, and hash each file
        for entry in fs::read_dir(currentdir)? {
            println!()
            */

fn hashfileadd(){} 