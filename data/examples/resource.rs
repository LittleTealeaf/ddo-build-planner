//! Demonstrates obtaining data dynamically stored at build time
use data::get_test_data;

/// Main Function
pub fn main() {
    let data = get_test_data().unwrap();
    println!("Data returned: {}", data);
}
