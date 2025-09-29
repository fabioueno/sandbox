// Defining public trait
pub trait Summary {
    fn summarize(&self) -> String;

    // Function with default implementation
    fn default_summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarize_author(&self) -> String;

    // Function using another:
    // Since this has a default implementation, only the dependent requires
    // implementation, i.e. summarize_author.
    fn default_summarize_with_author(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// Defining public structs
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Implementing trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Creating function with Trait parameter
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("someone"),
        content: String::from("Nothing to read here."),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The PP once again are the best hockey team in the NHL.")
    };

    println!("New article available! {}", article.default_summarize());
    println!("New article available: {}", article.default_summarize_with_author());

    notify(&article);
}