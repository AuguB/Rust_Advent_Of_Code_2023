mod day1;
#[allow(dead_code)]
fn main() {
    match day1::run() {
        Ok(_) => println!("Finished successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
