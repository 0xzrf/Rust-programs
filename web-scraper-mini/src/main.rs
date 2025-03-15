use trpl::{Html, Either};
use std::env::args;

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await.text().await;
    let title = Html::parse(&response)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}

fn main() {
    let arg: Vec<String> = args().collect();

    trpl::run(async {
        let url_1 = page_title(&arg[1]);
        let url_2 = page_title(&arg[2]);
        
        let (url, maybe_title) = match trpl::race(url_1, url_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right
        };


        println!("{url} returned first");

        match maybe_title {
            Some(title) => println!("The title for url: {url} is: {title}"),
            None => println!("No title found for url: {url}")
        }
    })        

}

