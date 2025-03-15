use trpl::Html;
use std::env::args;

async fn page_title(url: &String) -> Option<String> {
    let response = trpl::get(url).await.text().await;
    Html::parse(&response)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}

fn main() {

    trpl::run(async {
        let urls = match Urls::build(args()) {
            Ok(urls) => urls,
            Err(err) => return eprint!("Error parsing the urls: {err}")
        };

        match page_title(&urls.url_1).await {
            Some(title) => println!("The title for {} is:\n{title}", &urls.url_1),
            None => println!("provided url had no title")
        };

    })    

}


struct Urls {
    pub url_1: String,
    pub url_2: String
}


impl Urls {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Urls, &'static str> {
        args.next();

        let url_1 = match args.next() {
            Some(url) => url,
            None => return Err("1st URL not provided")
        };

        let url_2 = match args.next() {
            Some(url) => url,
            None => return Err("2nd URL not provided")
        };

        Ok(Urls {
            url_1: url_1,
            url_2: url_2
        })

    }
}