use std::fmt::Display;

fn main() {
    let x = "will it be this";
    let y = "or will it be that";

    let longest = longest_with_an_announcement(x, y, "testing generics, lifetimes and traits together");

    println!("And the longest is: {}", longest);
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
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
