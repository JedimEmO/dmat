use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("src/style") {
        let entry = entry.unwrap();
        let path = entry.path().to_str().unwrap();

        if path.ends_with(".scss") {
            println!("cargo-rerun-if-changed:{}", path);
        }
    }
}
