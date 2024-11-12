fn main() {
    println!("Hello, world!");

    another_function();

    add(12, 44);
}

fn another_function() {
    println!("Another function.");
}

fn add(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}
