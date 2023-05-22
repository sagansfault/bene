# bene
Bene is a library for parsing command line arguments efficiently and elegantly.

Example:  
The command line arguments:
```
-f 30 --length 5 --lib
```
Would be parsed as:
```rust
let mut frames: usize = 30;
let mut length = 3; // inferred i32
let mut lib = false;

// let input = "-f 30 --length 5 --lib"
bene::Intake::new()
    .arg('f', "frames", &mut frames)
    .arg('l', "length", &mut length)
    .arg('L', "lib", &mut lib) // case sensitive!
    .process(input);

assert_eq!(frames, 20);
assert_eq!(length, 5);
assert_eq!(lib, true);
```
