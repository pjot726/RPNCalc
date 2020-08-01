/* This is a Reverse Polish Calculator that supports
    the four basic functions. */

use std::env;

fn main()
{
    // Parses each line in the input
    for argument in env::args() {
        println!("{}", argument);
    }
}
