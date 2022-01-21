fn main() {
    println!("オセロしますよ！");

    let size: usize;
    loop {
        println!("盤面サイズを4以上の偶数で入力して下さい。Returnキーで確定します。");

        let mut size_string = String::new();
        std::io::stdin().read_line(&mut size_string).ok();
        if let Ok(n) = size_string.trim().parse::<usize>() {
            if (n >= 4) && (n % 2 == 0) {
                size = n;
                break;
            } else {
                err_input();
            }
        } else {
            err_not_int();
        }
    }
}

/// 整数の入力が不正である旨のメッセージ
fn err_not_int() {
    println!("半角数字で整数を入力してください．");
}

/// 入力が不適切な旨のメッセージ
fn err_input() {
    println!("入力が不適切です．");
}
