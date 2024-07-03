## 3-1 Variables and Mutability

1. **Variable Declaration**:
   - By default, variables in Rust are _immutable_.
   - Use the `let` keyword to declare a variable.
     ```rust
     let x = 5;
     ```

2. **Mutable Variables**:
   - To make a variable mutable, use the `mut` keyword.
     ```rust
     let mut x = 5;
     x = 6;
     ```
   - Mutable variables can be changed, whereas immutable variables cannot be changed once assigned.

3. **Constants**:
   - Constants are declared using the `const` keyword.
   - Constants must always have a _type specified_ and are always _immutable_.
   - It is conventional to use _uppercase letters_ for constant names.
     ```rust
     const MAX_POINTS: u32 = 100_000;
     ```

4. **Variable Scope and Shadowing**:
   - Variables are _valid within the scope_ they are declared.
   - Shadowing allows you to _redeclare a variable with the same name_, effectively overshadowing the previous value.
     ```rust
     let x = 5;
     let x = x + 1;
     {
         let x = x * 2;
         println!("The value of x in the inner scope is: {}", x);
     }
     println!("The value of x is: {}", x);
     ```
  - You can _change the type_ of the value using Shadowing
    ```rust
    // Allowed
    let spaces = "   ";
    let spaces = spaces.len();

    // Not allowed
    let mut spaces = "   ";
    spaces = spaces.len();
    ```

5. **Data Types**:
   - Variables can have various data types, and Rust supports type inference.
   - Basic data types include integers, floating-point numbers, booleans, and characters.
   - You can explicitly specify a variable's type.
     ```rust
     let guess: u32 = "42".parse().expect("Not a number!");
     ```

### Example Code

```rust
fn main() {
    let x = 5; // Declare immutable variable x
    println!("The value of x is: {}", x);

    let mut y = 10; // Declare mutable variable y
    println!("The value of y is: {}", y);
    y = 15; // Change the value of y
    println!("The value of y is now: {}", y);

    const MAX_POINTS: u32 = 100_000; // Declare constant MAX_POINTS
    println!("The maximum points are: {}", MAX_POINTS);

    let x = x + 1; // Shadowing
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
        {
            let x = "hello";
            println!("The {x} in the second scope ");
        }
    }
    println!("The value of x is: {}", x);
}
```
