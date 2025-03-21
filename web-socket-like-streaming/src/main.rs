use trpl::{ReceiverStream, Stream, StreamExt};
use std::{pin::pin, time::Duration};

fn main() {

    trpl::run(async {
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
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

        tx.send(format!("Message: {message}")).unwrap();
    }

    });

    ReceiverStream::new(rx)
}
