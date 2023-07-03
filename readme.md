# command line rust

Learn Rust by following along with the book Command-Line Rust

## Hello world

Yeah yeah...

```
rustc hello.rs
./hello
```

Better to...

```
cargo new hello
cd hello
cargo run
```

### Tests

Convention tests directory in the same directory as the src directory

For testing programs in the current create, use the create assert command. Add the dependency to Cargo.toml as a dev dependency.

## True / False

```
cargo run --bin true
echo $?
```

## Hints

* Testss are not run in order by default, they can run concurrently. To run in order use `cargo test -- --test-threads=1`