pub mod summarize {
    pub trait Summarizable {
        fn summary(&self) -> String {
            String::from("Read more...")
        }
    }
}

pub mod news {
    use super::summarize::Summarizable;

    #[derive(Debug)]
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    // if we didn't have an actual implementation, we would
    // use the below line to utilize the default
    // impl Summarizable for NewsArticle {}

    impl Summarizable for NewsArticle {
        fn summary(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    #[derive(Debug)]
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summarizable for Tweet {
        fn summary(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
}
