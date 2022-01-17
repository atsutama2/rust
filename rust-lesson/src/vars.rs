pub mod sub_a;
pub mod sub_b;

// 定数
const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("Here is vars module!!");
    // sub_a::fnc_a();
    // sub_b::fnc_b();

    // 上書き可能変数
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    // 「_」を変数につけると意図的に使ってない定義のWARNINGを除外できる
    let _i1 = 3;
    let _f1 = 0.1;

    // 環境のサイズ
    println!("{}", usize::BITS);

    // メモリのアドレスの表示
    println!("Memory address of const is: {:p}", &MAX_POINTS);

    // スタックに積まれてるか確認
    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Stack address of i2 is: {:p}", &i2);
    println!("Stack address of i3 is: {:p}", &i3);

    // シャドーイング ローカル内で有効
    let y = 5;
    println!("Stack address of y is: {:p}", &y);
    let y = y + 1;
    println!("Stack address of y is: {:p}", &y);
    let y = y * 2;
    println!("Stack address of y is: {:p}", &y);
    println!("The value of y is: {}", y);
    {
        let y = 0;
        println!("The value of y is: {}", y);
        println!("Stack address of y is: {:p}", &y);
    }
    println!("The value of y is: {}", y);
    println!("Stack address of y is: {:p}", &y);

    // タプル
    let t1 = (500, 6.4, "dummy");
    let (_x, _y, _z) = t1;
    println!("The value of t1 is: {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("{:?}", t2);

    // 配列
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);

    let s1 = "helloこんにちは挨拶";
    let s2 = "hello";
    println!("Stack address of s1 is: {:p}", &s1);
    println!("Stack address of s2 is: {:p}", &s2);
    // 静的領域に格納されている値に紐付いているアドレスの値の取得
    println!("Static memory address of s1 {:?}", s1.as_ptr());
    println!("Static memory address of s2 {:?}", s2.as_ptr());
    // 実データのbyte数
    println!("Len of s1 is {}", s1.len());
    println!("Len of s2 is {}", s2.len());

    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");
    println!("Static address of s1 {:p}", &s1);
    println!("Static address of s2 {:p}", &s2);
    println!("Heap memory address of s1 {:?}", s1.as_ptr());
    println!("Heap memory address of s2 {:?}", s2.as_ptr());
    println!("Len of s1 is {}", s1.len());
    println!("Len of s2 is {}", s2.len());
    println!("Capacity of s1 is {}", s1.capacity());
    println!("Capacity of s2 is {}", s2.capacity());

    s1.push_str("_new1");
    s2.push_str("_new2");
    println!("{} {}", s1, s2)
}
