use std::time::Duration;
use trpl;

fn main() {
    trpl::block_on(async {
        // let handle = trpl::spawn_task(async {
        //     for i in 1..10 {
        //         println!("hi number {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // });

        // for i in 1..5 {
        //     println!("hi number {i} from the second task!");
        //     trpl::sleep(Duration::from_millis(500)).await;
        // }

        // handle.await.unwrap();

        // -------------------------------

        //     let fut1 = async {
        //         for i in 1..10 {
        //             println!("hi number {i} from the first task!");
        //             trpl::sleep(Duration::from_millis(500)).await;
        //         }
        //     };

        //     let fut2 = async {
        //         for i in 1..5 {
        //             println!("hi number {i} from the second task!");
        //             trpl::sleep(Duration::from_millis(500)).await;
        //         }
        //     };

        //     println!("FIRST STATEMENT");
        //     trpl::join(fut1, fut2).await;
        //     println!("LAST STATEMENT");

        // -------------------------------

        // let (tx, mut rx) = trpl::channel();

        // let val = String::from("hi");
        // tx.send(val).unwrap();

        // let received = rx.recv().await.unwrap();
        // println!("received '{received}'");

        // -------------------------------

        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    })
}
