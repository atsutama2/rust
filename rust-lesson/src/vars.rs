pub mod sub_a;
pub mod sub_b;

pub fn run() {
    println!("Here is vars module!!");
    sub_a::fnc_a();
    sub_b::fnc_b();
}
