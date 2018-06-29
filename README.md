Splop
=====

This tiny crate contains functions and types to help you do something special
when repeating for the first or last time (or in between!). This crate offers
two distinct features:

- `IterStatusExt::with_status`: a new method for **iterators**, that
  creates a new iterator which yields the item paired with information to
  tell you if this is the first/last item.
- `SkipFirst`: a simple struct to help you always do something, except on
  the first repetition. Works without iterators, too!


### Examples

*(also see [`examples/`](examples/))*

```rust
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
```

The old programming problem – commas in between elements:

```rust
let mut comma = SkipFirst::new();

print!("[");
for name in &["banana", "melon", "kiwi"] {
    comma.skip_first(|| print!(", "));
    print!("{}", name);
}

println!("]");
```


---

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
