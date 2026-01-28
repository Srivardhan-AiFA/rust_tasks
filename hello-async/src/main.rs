// use trpl::Html;

// fn main() {
//     trpl::block_on(async {
//         let title_of_the_page = page_title("https://axna.vercel.app/").await;
//         println!("The Title iof the given URL is: {}", title_of_the_page);
//     });
// }
// async fn page_title(url: &str) -> String {
//     let response = trpl::get(url).await;
//     let response_text = response.text().await;
//     match Html::parse(&response_text)
//         .select_first("title")
//         .map(|title| title.inner_html())
//     {
//         Some(title) => title,
//         _ => format!("There is an isssue with the url"),
//     }
// }

// Racing Two URLs Against Each Other Concurrently

use trpl::{Either, Html};

fn main() {
    let url_1 = "https://axna.vercel.app/";
    let url_2 = "https://codefe.vercel.app/";

    trpl::block_on(async {
        let title_one = page_title(url_1);
        let title_two = page_title(url_2);

        let (url, maybe_title) = match trpl::select(title_one, title_two).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");

        match maybe_title {
            Some(title) => println!("first returned title: {title}"),
            None => println!("It had no title"),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}
