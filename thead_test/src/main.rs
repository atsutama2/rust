// use std::sync::{Arc, Mutex};
// or
// use std::sync::Arc;
// use std::sync::Mutex;
use std::sync::mpsc;

// use std::sync::Arc;
// use std::rc::Rc;
use std::thread;

fn main() {
    // ==========================================
    // メッセージパッシング(10要素配列Ver)
    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut snd_channels = Vec::new();
    let mut rcv_channels = Vec::new();

    for _ in 0..10 {
        // mainから各スレッドへのチャンネル
        let (snd_tx, snd_rx) = mpsc::channel();

        // 各スレッドからmainチャンネル
        let (rcv_tx, rcv_rx) = mpsc::channel();

        snd_channels.push(snd_tx);
        rcv_channels.push(rcv_rx);

        handles.push(thread::spawn(move || {
            let mut data = snd_rx.recv().unwrap();
            data += 1;
            let _ = rcv_tx.send(data);
        }));
    }

    // 各スレッドにdataの値を送信
    for x in 0..10 {
        let _ = snd_channels[x].send(data[x]);
    }

    // 各スレッドから結果をdataに格納
    for x in 0..10 {
        data[x] = rcv_channels[x].recv().unwrap();
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);

    // ==========================================
    // メッセージパッシング
    // let (tx, rx) = mpsc::channel();
    // let handle = thread::spawn(move || {
    //     let data = rx.recv().unwrap();
    //     println!("{}", data);
    // });

    // let _ = tx.send("Hello, world!");

    // let _ = handle.join();

    // ==========================================
    // スレッド間の情報共有
    // let mut handles = Vec::new();
    // // let mut data = Rc::new(vec![1; 10]);
    // // let mut data = Arc::new(vec![1; 10]);
    // let mut data = Arc::new(Mutex::new(vec![1; 10]));

    // for x in 0..10 {
    //     let data_ref = data.clone();
    //     handles.push(thread::spawn(move || {
    //         // lockを使ってdataへ可変参照を得る
    //         let mut data = data_ref.lock().unwrap();
    //         data[x] += 1;
    //     }));
    // }

    // for handle in handles {
    //     let _ = handle.join();
    // }

    // dbg!(data);

    // ==========================================
    // スレッド作成
    // let mut handles = Vec::new();

    // for x in 0..10 {
    //     handles.push(thread::spawn(move|| {
    //         println!("Hello Thread!: {}", x);
    //     }));
    // }

    // for handle in handles {
    //     let _ = handle.join();
    // }
}
