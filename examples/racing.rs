extern crate splop;

use splop::IterStatusExt;

fn main() {
    let names = ["anna", "peter", "brigitte", "bob"];

    for (name, status) in names.iter().with_status() {
        print!("{}", name);

        if status.is_first() {
            print!(" <-- winner (ᵔᴥᵔ)");
        }
        if status.is_last_only() {
            print!(" ... ʘ︵ʘ");
        }

        println!();
    }
}
