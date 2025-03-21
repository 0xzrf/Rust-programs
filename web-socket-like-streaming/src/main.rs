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

    let messages = ['a', 'b', 'c', 'd', 'e'];

    for message in messages {
        tx.send(format!("Message: {message}")).unwrap();
    }

    ReceiverStream::new(rx)
}
