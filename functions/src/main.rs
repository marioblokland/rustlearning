fn main() {
    another_function(5, 10);
    println!("Increasing 5 by one: {}", plus_one(5));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    // This is an expression insead of a statement,
    // since there is a missing semicolon, thus the
    // value of the expression will be returned implicitly
    x + 1
}