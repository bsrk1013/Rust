extern crate generic_trait_lifetime;

use crate::generic_trait_lifetime::Summarizable;
use generic_trait_lifetime::NewsArticle;
use generic_trait_lifetime::Tweet;
use std::fmt::Display;
use std::fmt::Formatter;

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    largest_test2();
    largest_test();
    generic_test();
    trait_test();
    lifetime_test();
    lifetime_test2();
}

fn lifetime_test() {
    let string1 = String::from("abcdefg");
    let string2 = String::from("xyz");
    {
        let result = longest(string1.as_str(), string2.as_str());
        println!("longest:{}", result);

        let result = longest_with_an_announcement(string1.as_str(), string2.as_str(), result);
        println!("longest:{}", result);
    }

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

fn lifetime_test2() {
    struct ImportantExcept<'a> {
        part: &'a str,
    }
    impl<'a> ImportantExcept<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    impl Display for ImportantExcept<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}:{}", self.part, self.level())
        }
    }

    let novel = String::from("Call me Ishmael. Some yearse agoo...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcept {
        part: first_sentence,
    };
    println!("{}", i);
}

fn trait_test() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let news = NewsArticle {
        headline: String::from("속보"),
        author: String::from("백종환"),
        location: String::from("서울"),
        content: String::from(""),
    };

    println!("1 new twee:{}", tweet.summary());
    println!("news:{}", news.summary());
}

fn generic_test() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point {
        x: "Hello",
        y: "World",
    };

    let p3 = p1.mixup(p2);

    println!("x:{}, y:{}", p3.x, p3.y);
}

fn largest_test() {
    let numbers = vec![34, 50, 25, 100, 65];
    let largest = find_largest(&numbers);

    println!("{:?}", numbers);

    println!("largest:{}", largest);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest = find_largest(&numbers);
    println!("largest:{}", largest);

    fn find_largest(datas: &Vec<i32>) -> i32 {
        if datas.len() <= 0 {
            panic!("no data.");
        }
        let mut largest: i32 = datas[0];
        for &data in datas {
            if data > largest {
                largest = data;
            }
        }
        largest
    }
}

fn largest_test2() {
    let datas = vec![1, 4214, 53, 21, 5431, 1, 124];
    let largest = find_largest(&datas);
    println!("largest2:{}", largest);

    let datas = vec!["Hello", "World", "Rust"];
    let largest = find_largest(&datas);
    println!("largest2:{}", largest);
    // let datas: Vec<f64> = Vec::new();
    // let largest = find_largest2(&datas);
    // println!("largest2:{}", largest);

    fn find_largest<T>(datas: &Vec<T>) -> T
    where
        T: PartialOrd + Copy,
    {
        if datas.len() <= 0 {
            panic!("no data.");
        }

        let mut largest = datas[0];
        for &data in datas {
            if data > largest {
                largest = data;
            }
        }

        largest
    }
}
