- Declaration of standard libraries

use std::io;
use std::io::prelude::*;

- Starts the code block of the main function

fn main

- We initialize and declare a mutable variable of type String

let mut name = String::new();

- We print a string to the screen requesting input

print!("Enter your name: ");

-  We call the stdout() output method -> it directs the output to the standard device (monitor)
-  We call the read_line() function to read the value into the &mut name string
-  We call the expect function to handle potential errors
-  We call the flush() method to ensure immediate output
-  We call the unwrap() method to ensure the return of the value reference

if let Some('\n') = name.chars().next_back();

-    It checks the condition, meaning it checks what occurs after the enter key is pressed, which, when triggered, performs two operational actions:

    a) Line feed or newline ('\n')
    b) Carriage return ('\r')

- If both if let conditions are true, the name.pop() instruction will be executed, removing the characters '\n' and '\r' from the heap memory, leaving only what matters to be assigned to the variable name.

Some.('\code') = name.chars().next_back();

We use the Some function to locate the mentioned special characters.

Conclusion:

Rust presents an interesting syntax, although still difficult to understand at first, but practice resolves with time. It is an expressive language, much like C++, but as we know, it has more interesting features than C++.