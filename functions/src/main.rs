fn main() {
another_function(3);
}

fn another_function(x: i32) {
    let x = five();
    println!("Hello, world! {x}");
}

fn five() -> i32 {
    5
}