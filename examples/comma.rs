extern crate splop;

use splop::IterStatusExt;

fn main() {

    print!("[");
    for (i, status) in (1..13).filter(|i| i % 2 == 0).with_status() {
        if !status.is_first() {
            print!(", ");
        }

        print!("{}", i);
    }

    println!("]");
}
