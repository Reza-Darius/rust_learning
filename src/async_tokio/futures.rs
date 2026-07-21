use std::{pin::pin, time::Duration, vec};

#[tokio::main]
async fn main() {
    // async fn always creates !Unpin anonymous futures
    let mut f = pin!(hello());

    // does not require pinning because MyFuture is Unpin
    let mut my_f = MyFuture;

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;

        tokio::select! {
            // removing the &mut consumes the future and therefore doesnt compile with a loop
            n = &mut my_f => {
                println!("my future prints {n}")
            }

            // this causes a panic on second iteration because we run a completed async fn twice
            n = &mut f => {
                println!("{n}");
            }
        }
    }
}

// this doesnt compile, because anonymous futures are distinct types
// fn return_future() -> impl Future<Output = usize> {
//     let condition = true;
//     if condition {
//         hello()
//     } else {
//         bye()
//     }
//
// }

// this doesnt work either
// fn add_futures() {
//     let mut v = vec![];
//     v.push(bye());
//     v.push(hello());
// }

async fn bye() -> usize {
    42
}

async fn hello() -> usize {
    42
}

struct MyFuture;

impl Future for MyFuture {
    type Output = i32;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::task::Poll::Ready(42)
    }
}
