// futures というクレートに、非同期プログラムを実行するための基盤が用意されている。
// サンプルコードではそれを利用する。
// cargo add futures で依存を追加出来る。
use futures::executor;

// struct User {

// }

// struct UserId(u32);

// struct Db {}

// impl Db {
//     async fn find_by_user_id(&self, user_id: UserId) -> Option<User> {
//         // DBに接続するなどの実装が追加される想定
//     }
// }

// async fn find_user_by_id(db: Db, user_id: UserId) -> Option<UserId> {
//     // dbはデータアクセスを示す。Option<User>型を返すものとする。
//     db.find_by_user_id(user_id).await
// }

async fn async_add(left: i32, ringht: i32) -> i32 {
    left + ringht
}

async fn something_great_async_function(num1: i32, num2: i32) -> i32 {
    let ans1 = async_add(num1, num2).await; // この時点で5という値を取り出せる。
    println!("ans1: {}", ans1);
    let ans2 = async_add(num1 + 1, num2 + 1).await; // この時点で5という値を取り出せる。
    println!("ans2: {}", ans2);
    let ans3 = async_add(num1 + 2, num2 + 2).await; // この時点で5という値を取り出せる。
    println!("ans3: {}", ans3);
    // 何か処理を挟むことも出来る
    let res = ans1 + ans2 + ans3;
    println!("{}", res);
    res
}

fn main() {
    // find_user_by_id関数を実行する
    // executor::block_on(find_user_by_id(Db {}, UserId(1)));

    // 関数を実行する。5が出力される
    executor::block_on(something_great_async_function(2,3));
}