// futures というクレートに、非同期プログラムを実行するための基盤が用意されている。
// サンプルコードではそれを利用する。
// cargo add futures で依存を追加出来る。
use futures::executor;

struct User {

}

struct UserId(u32);

struct Db {}

impl Db {
    async fn find_by_user_id(&self, user_id: UserId) -> Option<User> {
        // DBに接続するなどの実装が追加される想定
    }
}

async fn find_user_by_id(db: Db, user_id: UserId) -> Option<UserId> {
    // dbはデータアクセスを示す。Option<User>型を返すものとする。
    db.find_by_user_id(user_id).await
}

fn main() {
    // find_user_by_id関数を実行する
    executor::block_on(find_user_by_id(Db {}, UserId(1)));
}