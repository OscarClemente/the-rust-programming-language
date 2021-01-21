fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let cls = returns_closure();
    println!("{}", cls(1));
    println!("{}", returns_closure()(2));
}
