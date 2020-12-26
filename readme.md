# rs2dart-proto

Rust Struct to Dart class converter using [proc-macro](https://doc.rust-lang.org/reference/procedural-macros.html) and [syn](https://github.com/dtolnay/syn).

My attempt to learn procedural macros in rust. I haven't seen any rust to dart converter anywhere so I decided to make one. I've seen rust to typescript but never Dart. Took a reference from [here](https://github.com/dtolnay/syn/issues/516) a lot.

By running `cargo run`, the code will generate a file called `model.dart` which will be generated if you use the `#[gen_dart]` procedural macro attribute on a struct. (Example in `src/main.rs`). 

It transforms something like this:
```rs
#[gen_dart]
struct MyTestStruct {
    id: i32,
    my_string: String,
    my_double: f32
}
```
to:
```dart
class MyStructTest {
  int id;
  String my_string;
  double my_double;
}
```

`cargo clean` must be used sometime in order to show the `println!` output in the converter code. I do not understand why.
