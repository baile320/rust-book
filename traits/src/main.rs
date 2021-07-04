mod traits;

use traits::news;
use traits::summarize::Summarizable;

fn main() {
    let tweet = news::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summary());

    let article = news::NewsArticle {
        headline: String::from("Blah is a Blah Blah?"),
        location: String::from("New Yawk"),
        author: String::from("Joe Schmoe"),
        content: String::from("This is a long article about..."),
    };
    println!("1 new news article: {}", article.summary());

    notify(&tweet);
}

pub fn notify(item: &impl Summarizable) {
    println!("Breaking news! {}", item.summary());
}
