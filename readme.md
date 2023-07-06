# command line rust

Learn Rust by following along with the book Command-Line Rust

Book repo - https://github.com/kyclark/command-line-rust/tree/main

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

### Hints

* Testss are not run in order by default, they can run concurrently. To run in order use `cargo test -- --test-threads=1`

## Echo

Args - https://doc.rust-lang.org/stable/std/env/fn.args.html

Separating the program arguments from cargo run arguments - use the double dash, e.g. `--`

For example

```
cargo run -- -n Hello world
```

Crate clap - for processing command line args - https://docs.rs/clap/2.33.3/clap/

Underscore var names, e.g. _matches - tells the compiler you don't intend to use the variable now, to avoid
unused variable warnings.

Options - standard functional Some or None - see https://doc.rust-lang.org/std/option/enum.Option.html

Values - https://docs.rs/clap/2.33.3/clap/struct.Values.html
Vec - https://doc.rust-lang.org/std/vec/struct.Vec.html
String - https://doc.rust-lang.org/std/string/struct.String.html

Error -     https://doc.rust-lang.org/std/error/trait.Error.html
Box - https://doc.rust-lang.org/std/boxed/struct.Box.html
dyn - https://doc.rust-lang.org/std/keyword.dyn.html

Idiom - instead of returning a value, omit the semi colon on the last expression to implicitly 
return that item.

Question mark ? operator - unpack an Ok value or propagate the Error as the result

Slice - https://doc.rust-lang.org/std/primitive.slice.html

## Cat

