extern crate splop;

use splop::SkipFirst;

fn main() {
    let mut comma = SkipFirst::new();

    print!("[");
    for name in &["banana", "melon", "kiwi"] {
        comma.skip_first(|| print!(", "));
        print!("{}", name);
    }

    println!("]");
}
