# erase-output

Erase the last output from the terminal.

## Example

```rs
use erase_output::Erase;

fn main() {
    let mut old = String::new();
    for i in 0..100 {
        let new: String = std::iter::once("🌱")
            .chain(std::iter::once("🦩").cycle().take(i % 80))
            .chain(std::iter::once("🌱\n"))
            .chain(std::iter::once("🐎").cycle().take((i + 2) % 4))
            .chain(std::iter::once("\n"))
            .collect();
        print!("{}{new}", Erase(&old));
        old = new;
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/example`
🌱🦩🦩🦩🦩🦩🦩🦩🦩🦩🦩🦩🦩🦩🦩🦩🦩🦩🦩🦩🌱
🐎
```
