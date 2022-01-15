pub fn run() {
    let st1 = String::from("x");
    let st2 = String::from("y");
    let res1 = get_longest(&st1, &st2);
    // println!("{}", res1);

    let st3 = String::from("bbbbb");
    {
        let st4 = String::from("aaa");
        let res2 = get_longest(&st3, &st4);
        println!("{}", res2);
    }
}

fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() {
        x
    } else {
        y
    }
}
