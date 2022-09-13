use rust_dylib_serde_issue::test_serde_json::Cat;

fn main() {
    println!("Hello, world!");

    let cat = Cat::new();
    let cat_json = serde_json::to_string(&cat);
    println!("cat_json: {:?}", cat_json);
}
