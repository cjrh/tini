extern crate tini;
use tini::Ini;

fn main() {
    let config = Ini::new()
        .section("names")
        .item("first", "John")
        .item("second", "Peter")
        .item("third", "Emily")
        .section("languages")
        .item("list", "c, c++, rust");

    // iterate over config
    for (section, item) in config.iter() {
        println!("section {} with items:", section);
        for (_, value) in item {
            println!("  - {}", value);
        }
    }

    let result = config.to_buffer();
    println!("\n--- serialize to ini ---\n{}---", result);
}
