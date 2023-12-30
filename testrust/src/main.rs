#[warn(clippy::all, clippy::pedantic)]
fn main() {
    let my_list = ["ONE", "TWO", "THREE"];
    for i in my_list {
        println!("{i}");
    }
}
