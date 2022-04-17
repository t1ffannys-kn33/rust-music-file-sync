use walkdir::WalkDir;

fn main() {
    println!("Hello, world!");
    for entry in WalkDir::new("foo").into_iter().filter_map(|e| e.ok()) {
      println!("{}", entry.path().display());
  }
}
