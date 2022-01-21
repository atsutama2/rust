// FizzBuzz
// 1から整数を数える
// 3で割り切れる = Fizz
// 5で割り切れる = Buzz
// 3でも5でも割り切れる = FizzBuzz
fn main() {
    for n in 1..=30 {
        // pattern1
        let mut str = String::new();
        if (n % 3 == 0) && (n % 5 == 0) {
            str = "FizzBuzz".to_string();
        } else if n % 3 == 0 {
            str = "Fizz".to_string();
        } else if n % 5 == 0 {
            str = "Buzz".to_string();
        } else {
            let s: String = n.to_string();
            str = s;
        }
        println!("{}", str);
    }
}
