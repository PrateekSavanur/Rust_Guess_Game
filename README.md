# Rust Basics

> Did you know toml stands for Tom's Obvious, Minimal Language ?

1. Variables in Rust are immutable by default. Once a value is bound to a name, you can't change that value unless you explicitly make the variable mutable using the mut keyword.

2. Constants in Rust are always immutable and are declared using the const keyword. They must have their type annotated and can only be set to a constant expression, not a value computed at runtime.

3. Rust allows variable shadowing, which means you can declare a new variable with the same name as a previous variable. This is different from mutability as it creates a new variable and allows you to change the type while reusing the same name.

4. The unused variables are denoted with `_variableName` to avoid the warning by compiler.

5. Compund types include tuples and arrays. Everything about array is similar to other languages. Tuples are couple of variables with same name. Can be accessed by assigning them to other variables.

6. 1. Statements are instructions that perform some action and do not return a value.
   2. Expressions evaluate to a resultant value.

```rust
    {
        let x = 3;
        x + 1
    }
    // The above expression returns 4.
```

7. 1. `fn plus_one(x: &str) -> &str` defines a function named plus_one that takes a string slice (&str) as an input parameter and returns a string slice.
   2. The body of the function { x } simply returns the input parameter x

8. loop don't stop until explicitly mentioned, "while" can be used ( Similar to other)

` `

# Ownership in rust

It enables Rust to make memory safety guarantees without needing a garbage collector, so it’s important to understand how ownership works.

> Each value in Rust has an owner.
> There can only be one owner at a time.
> When the owner goes out of scope, the value will be dropped.

Consider the snippet below:

```rust
    let s1 = String::from("hello");
    let s2 = s1;
```

When we do `let s2 = s1 ` it copies the pointer of s1 to s2. When both s2 and s1 are out of scope, rust calls "drop" function for both s2 and s1. it is called **Double free error**. This is considered as a security vulnerability. So rust makes s1 out of scope as soon as it is assigned to s2.

This does not happen with integer variables as they are stored directly on stack but string content is stored in the heap.

Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the stack, as integers are . If a type implements the `Copy` trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

> When a variable is passed to another function, it's ownership is transferred

## References and Borrowing

1. References allow you to refer to a value without taking ownership of it.
2. References are created using the & symbol.
3. Borrowing is the act of creating a reference to a value.
4. By default, references are immutable, meaning you can't modify the borrowed value.
5. Mutable references (&mut) allow you to modify the borrowed value.
6. Rust enforces strict rules for references to prevent data races:

   1. You can have either one mutable reference or any number of immutable references to a value at a time.
   2. References must always be valid (no dangling references).
