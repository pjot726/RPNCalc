/* This is a Reverse Polish Calculator that supports
    the four basic functions. */

use std::env;

fn main()
{
    // Declare a stack for use in storing ints.
    let mut int_stack = std::collections::LinkedList::<i32>::new();

    // Store results of input in vector
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    // Parses each line in the input
    for argument in args
    {
        // Attempt to parse argument as an integer.
        let _parsed_int = match argument.parse::<i32>()
        {
            // If successful, push int onto stack
            Ok(parsed_int) => int_stack.push_back(parsed_int),
            // Otherwise...
            Err(_e) =>
            {
                /* If the argument is an operation, pop the top two
                   elements from the stack and perform the operation,
                   pushing the result back onto the stack. */
                if argument == "+" {
                    let i1 = int_stack.pop_back().unwrap();
                    let i2 = int_stack.pop_back().unwrap();
                    int_stack.push_back(i1 + i2);
                }
                else if argument == "-" {
                    let i1 = int_stack.pop_back().unwrap();
                    let i2 = int_stack.pop_back().unwrap();
                    int_stack.push_back(i1 - i2);
                }
                else if argument == "*" {
                    let i1 = int_stack.pop_back().unwrap();
                    let i2 = int_stack.pop_back().unwrap();
                    int_stack.push_back(i1 * i2);
                }
                else if argument == "/" {
                    let i1 = int_stack.pop_back().unwrap();
                    let i2 = int_stack.pop_back().unwrap();
                    int_stack.push_back(i1 / i2);
                }
                // If the argument is not an integer or operator,
                // it is invalid.
                else
                {
                    eprintln!("Invalid input '{}'", argument);
                    std::process::exit(1);
                }
            }
        };
    }
    // At this point, there should be only one element on the stack.
    if int_stack.len() == 1
    {
        println!("Result: {}", int_stack.pop_back().unwrap());
        std::process::exit(0);
    }
    // Otherwise, the user's input was wrong.
    else
    {
        eprintln!("Sorry, there was an error in the input.");
        std::process::exit(1);
    }
}
