# Dangling Pointer Bug in Rust

This repository demonstrates a common error in Rust involving dangling pointers when using vectors.  The `bug.rs` file contains code that creates a reference to an element of a vector, then modifies the vector in a way that invalidates the reference. This leads to undefined behavior. The `bugSolution.rs` file provides a corrected version.

## Running the Code

1. Clone this repository.
2. Navigate to the directory.
3. Compile and run `bug.rs` using `rustc bug.rs && ./bug` and observe the error. 
4. Compile and run `bugSolution.rs` using `rustc bugSolution.rs && ./bugSolution` and see that the corrected version works correctly.

## Bug Explanation

The core issue is that when you push a new element into the vector, the vector's capacity might be exceeded. When this happens, Rust will reallocate the entire vector to a new memory location, causing the memory address where the previous value was stored to become invalid. The solution involves avoiding the modification of the vector after creating a reference to an element.