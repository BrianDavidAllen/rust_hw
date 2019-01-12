# Brian Allen Homework #1
## What I did

* Added onto the GCD function from the second chapter of our book.
* In addition, a LCM, product, and sum function were added along with tests for each. Each function take in a borrowed reference to a vector created in main.
* Iterated over a created vector to get input from command line and parse the args
* Added string matching to main for each function name "lcm, gcd, .. etc"
* Added assertions to ensure inputs were not zero nor negative.  
* Test are minimal at best. Just asserted an equality that was sane given the inputs.

## How it Went

I enjoyed the process quite a bit. Was a bit worried about the borrow checker and the oddities of Rust. Other than a little bit of finagling to check a vec slice against a u64 0 everything went well.  

## Testing

Other than the test functions, where asserteq!() were used, the majority of testing was done manually by,
* running each function and checking the output
* passing in invalid args like 0 or -1
* pass in just a function name with no args
* did not try to overflow the args
* thought about writing a driver program to throw random numbers and function calls-- thought about it.
* used rust formatting tool that appeared to do nothing. This hopefully means code is in Rust format. 
