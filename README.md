# Advent of Code 2020
> https://adventofcode.com/2020

## Running my solutions
1. Install rust
2. Clone this repo
3. Navigate to the cloned directory
4. Update the global `static COOKIE_VALUE`  
4a. This is done by logging to https://adventofcode.com/2020 and opening the developer tools
4b. In the network tag, you will see on all the requests a header `cookie`
4c. Copy this value to the `static COOKIE_VALUE` global
5. Run using `cargo`
```bash
$ cargo run
```