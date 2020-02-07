
pub trait Summary {
    fn summarize(&self) -> String {
        format!("No summary available at this time!")
    }
    fn test(&self) -> ();
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn test(&self) -> () { }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn test(&self) -> () {
        println!("tweet!");
    }
}

fn main() {
    let _tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let tweet = sm();
    woo(&tweet);

    let article = NewsArticle {
        headline: "Wowwww".to_string(),
        location: "Croydon".to_string(),
        author: "Tiago Castro".to_string(),
        content: "News flash, it's Croydon!".to_string(),
    };

    let _a = &article;
    foo(_a);
    wooo(article);
    wooo(tweet);

    n();
}

fn wooo(sm: impl Summary) {
    //println!("Summary: {}", sm.summarize());
    println!("Test: ");
    sm.test();
}

fn woo<T: Summary>(sm: &T) {
    println!("Summary: {}", sm.summarize());
}

fn foo<T>(sm: &T)
    where T: Summary,
{
    println!("Summary: {}", sm.summarize());
}


fn sm() -> impl Summary {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    tweet
}

//#[derive(Debug, Clone, Copy)]
#[derive(Debug)]
struct NType {
    val: i32,
}

use std::cmp::Ordering;

impl PartialOrd for NType {
    fn partial_cmp(&self, other: &NType) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NType {
    fn cmp(&self, other: &NType) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl Eq for NType {}

impl PartialEq for NType {
    fn eq(&self, other: &NType) -> bool {
        self.val == other.val
    }
}


fn largest<T: Copy + PartialOrd>(v: &[T]) -> T {
    if v.len() == 0 {
        panic!("empty vector!")
    }

    let mut lg = v[0];

    for &i in v.iter() {
        if i > lg {
            lg = i;
        }
    }

    lg
}

fn largest3<T: PartialOrd>(v: &[T]) -> &T {
    if v.len() == 0 {
        panic!("empty vector!")
    }

    let mut lg = &v[0];

    for i in v.iter() {
        if i > lg {
            lg = i;
        }
    }

    lg
}

fn largest2<T: Copy + PartialOrd>(v: &[T]) -> Option<T> {
    if v.len() == 0 {
        return None
    }
    let mut lg = v[0];
    for &i in v.iter() {
        if i > lg {
            lg = i;
        }
    }
    Some(lg)
}

fn n() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest2(&number_list).expect("oops");
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let n_list = vec![NType{val:1}, NType{val:2}, NType{val:0}];
    let result = largest3(&n_list);
    println!("The largest ntype is {:#?}", result);
}

