use std::io; 

fn main() { 

    println!("数当てゲーム"); 

    println!("どの数だとおもう？ = "); 

    let mut guess = String::new(); 

    io::stdin().read_line(&mut guess).expect("読み込み失敗"); 
    
    println!("入力値: {}", guess); 
}