

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more...")
    }
}

pub struct NewsArticle {
    pub topic: String,
    pub author: String,
    pub location: String,
    pub content: String
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{} by {} at {}",self.topic, self.author, self.location)
    // }
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet: bool,
    pub reply: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} by {}", self.content, self.username)
    }
    fn summarize_author(&self) -> String{
        format!("{}",self.username)
    }
}

// fn notify(item: &impl Summary){
//     println!("Top Hacker: {}", item.summarize_author());
// }

fn notify<T: Summary>(item: &T){
    println!("Breaking news:{}", item.summarize());
}

fn main() {
    let tweet = Tweet{
        username: String::from("jero"),
        content: String::from("learning rust is sha hard ooo"),
        retweet: false,
        reply: true
    };
    let article = NewsArticle{
        topic: String::from("Web dev for rust"),
        content: String::from("just compile to wasm"),
        author: String::from("James"),
        location: String::from("Jos")
    };

    println!("1 new tweet: {} from {}",tweet.summarize(), tweet.summarize_author());
    println!("1 new article: {} from {}",article.summarize(), article.summarize_author());

    notify(&tweet);
    
}
