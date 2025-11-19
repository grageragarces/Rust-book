
smart pointer is a box: boxes allow you to store data on the heap rather than the stack (on the stack remains the pointer to the heap data)
no performance overhead, other than storing their data on the heap instead of on the stack
don’t have many extra capabilities either

use them in these situations:
-> When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
-> When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
-> When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

### bytes:
A box contains a pointer, which is 8 bytes on a 64-bit architecture. 
An array of 4 pointers is therefore at least 4 * 8 = 32 bytes.

## Deref
Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type