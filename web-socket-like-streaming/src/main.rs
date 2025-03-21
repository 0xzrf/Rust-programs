use trpl::{ReceiverStream, Stream, StreamExt};
use std::{pin::pin, time::Duration};

fn main() {

    trpl::run(async {
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals);
        let mut stream = pin!(merged);
        

        while let Some(result) = stream.next().await {
            match result {
                Ok(msg) => println!("{msg}"),
                Err(_) => println!("The message took too long to arrive"),   
            }
        }

    })

}

fn get_messages() -> impl Stream<Item=String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        
    let messages = ['a', 'b', 'c', 'd', 'e', 'd', 'e', 'd', 'e'];

    for (index, message) in messages.into_iter().enumerate() {
        let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
        trpl::sleep(Duration::from_millis(time_to_sleep)).await;

        if let Err(send_error) = tx.send(format!("Message: {message}")){
            eprintln!("Cannot send message '{message}': {send_error}");
            break
        }
    }

    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item=u32> {

    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;

        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            tx.send(count).unwrap();
        }
    });

    ReceiverStream::new(rx)

}
