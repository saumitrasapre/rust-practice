fn main() {
    /*
     * VARIABLES:
     * By default, variables in rust are immutable. Once a value is set to them, it can't be changed
     */
    let x = 5;
    println!("The value of x is {x}");
    // As a result, running the following piece of code will give a compilation error -
    // x = 6;
    // println!("The value of x is {x}");

    // We can declare a variable to be mutable by adding the `mut` keyword
    let mut y = 10;
    println!("The value of y is {y}");
    y = 20;
    println!("The value of y is {y}");

    /*
     * CONSTANTS:
     * A `constant` is similar to an immutable variable, but with some subtle differences -
     * 1. One cannot use `mut` with constants. Constants aren’t just immutable by default — they’re always immutable.
     * 2. We declare constants using the `const` keyword instead of the `let` keyword.
     * 3. The type of the value MUST be annotated. Rust can infer the types of variables.
     * 4. Constants can be declared in any scope, including the global scope. Variables can only be declared in local scopes
     * 5. Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
     * 6. Constants cannot be shadowed in the same scope
     */
    const HUNDRED: u32 = 100;
    println!("The value of constant HUNDRED is {HUNDRED}");

    /*
     * SHADOWING:
     * One can declare a new variable with the same name as a previous variable.
     * When this happens, we say that the first variable is "shadowed" by the second, which means
     * that the second variable is what the compiler will see when you use the name of the variable.

     * In simple words, the second variable "overshadows" the first, taking any uses of the variable name
     * to itself until either it itself is shadowed or the scope ends. Shadowing is done by re-declaring the variable
     * using the `mut` keyword
     * 
     * Shadowing is different from marking a variable as `mut` because we’ll get a compilation error 
     * if we accidentally try to reassign to this variable without using the `let` keyword. 
     * By using let, we can perform a few transformations on a value but have the variable be immutable after 
     * those transformations have completed.
     * 
     * The other difference between mut and shadowing is that because we’re effectively creating a new variable 
     * when we use the let keyword again, we can change the type of the value but reuse the same name.
     */
    let var = 5;
    let var = var + 1; // `var` is shadowed here

    {
        let var = var * 2; // `var` is again shadowed here
        println!("The value of var in the inner scope is {var}");
    }

    println!("The value of var is : {var}"); // Scope of the earlier shadow ends when the scoped block terminates
}
