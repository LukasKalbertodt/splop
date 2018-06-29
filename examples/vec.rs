extern crate splop;

use splop::IterStatusExt;

fn main() {
    let v = vec!['a', 'b', 'c', 'd', 'e'];
    for (c, status) in v.iter().with_status() {
        if status.is_first() {
            print!("┏");
        }

        if status.is_in_between() {
            print!("┃");
        }

        if status.is_last() {
            print!("┗");
        }
        println!(" {}", c);
    }
}
