fn main() {
    let apple = "apple";
    let banana = "banan";
    let orange = "🤞";
    let fruits = [ apple, banana, orange, "" ];
    for fruit in fruits.iter() {
        greet(fruit);
    }
}

fn greet(fruit:&str) {
    println!("{}", fruit);
}
