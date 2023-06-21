use data::get_test_data;

pub fn main() {
    let data = get_test_data().unwrap();
    println!("Data returned: {}", data);
}
