use publishing_a_crate_to_crates_io::kinds::PrimaryColor;
use publishing_a_crate_to_crates_io::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}