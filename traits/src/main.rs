#![allow(unused)]
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("More about that later") // default behavior
    }
}

pub trait OtherSummary {
    fn summarize_author(&self) -> String;

    fn summarize_other
    (&self) -> String {
        format!("read more from {}", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline:   String,
    pub location:   String,
    pub author:     String,
    pub content:    String,  
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }   
}

impl OtherSummary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username:   String,
    pub content:    String,
    pub reply:      bool,
    pub retweet:    bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("something somethings {} ", item.summarize())
}

fn main() {
    println!("Hello, world!");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("my long ass tweet"),
        reply: false,
        retweet: false,
    };

    println!("summarization of tweet is {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize_author())


}
