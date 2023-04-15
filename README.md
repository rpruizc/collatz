# Collatz Conjecture Project

This project demonstrates the Collatz Conjecture, also known as the 3n + 1 Conjecture, using a Rust program. The conjecture is defined as follows:

1. Take any positive integer n.
2. If n is even, divide it by 2.
3. If n is odd, multiply it by 3 and add 1.
4. Repeat the process indefinitely.

The conjecture states that no matter what value of n you start with, you will always eventually reach 1.

## Setup and Usage

1. Install Rust on your system.

2. Create a new Rust project using Cargo:

```
cargo new collatz_conjecture
```

3. Change to the new project directory:

```
cd collatz_conjecture
```
4. Replace the content of the src/main.rs file with the provided Rust code for the Collatz Conjecture.

5. Build and run the project with the desired initial number as the argument:

```
cargo run <initial_number>
```
Replace <initial_number> with a positive integer.

6. The program will output each step of the Collatz Conjecture until it reaches 1.

## Example
```
cargo run 6
```
Output:
```
Initial number: 6
Step 1: 3
Step 2: 10
Step 3: 5
Step 4: 16
Step 5: 8
Step 6: 4
Step 7: 2
Step 8: 1
```

## License

This project is licensed under the MIT License.