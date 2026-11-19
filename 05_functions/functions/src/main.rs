/*
 * FUNCTIONS:
 * We define a function by entering `fn`` followed by a function name and a set of parentheses.
 * The curly brackets tell the compiler where the function body begins and ends.
 * We can call any function we’ve defined by entering its name followed by a set of parentheses.
 *
 */

fn main() {
    println!("Hello, world!");
    another_function();
    parameterized_function(10);
    another_parameterized_function(5, 'a');
    statements_and_expressions();
    let retval_1 = function_with_return_value();
    println!("The value returned by the function is - {retval_1}");
    let retval_2 = parameterized_function_with_return_value(5);
    println!("The value returned by the parameterized function is - {retval_2}");
}

/*
 * Note that in the below example, we defined `another_function` AFTER `main`.
 * We could have defined it before as well.
 * Rust doesn’t care where you define your functions, only that they’re defined somewhere in a
 * scope that can be seen by the caller.
 */
fn another_function() {
    println!("Another function!")
}

/*
 * We can define functions to have parameters. When a function has parameters, you can provide it with
 * concrete values for those parameters.
 * In function signatures, you MUST declare the type of each parameter.
 */
fn parameterized_function(x: i32) {
    println!("Parameterized function. Value of x is - {x}")
}
/*
 * When defining multiple parameters, separate the parameter declarations with commas.
 */
fn another_parameterized_function(x: i32, y: char) {
    println!("Parameterized function. Value of x is - {x}. Value of y is - {y}.");
}

//------------------------------------------------------------------------------------------------------------------------------------------

/*
 * STATEMENTS AND EXPRESSIONS:
 * - Statements are instructions that perform some action and do not return a value.
 * - Expressions evaluate to a resultant value.
 */
fn statements_and_expressions() {
    let x = 6; // This is a statement. The 'let' keyword doesnt return any value.
    println!("Statement. The value of x is - {x}");
    let y = {
        let x = 3;
        x + 1 // This is an expression - it has a return value. Notice how this doesn't have an ending semicolon?
    };
    /*
     * This entire block -
     * {
     *  let x = 3;
     *  x + 1
     * }
     * is an expression.
     * Expressions do not include ending semicolons.
     * If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
     */
    println!("Expression. The value of y is - {y}");
}
//------------------------------------------------------------------------------------------------------------------------------------------

/*
 * FUNCTIONS WITH RETURN VALUES -
 * Functions can return values to the code that calls them.
 * We don’t name return values, but we must declare their type after an arrow (->).
 * The return value of the function is synonymous with the value of the final expression in the block of the
 * body of a function.
 * You can return early from a function by using the return keyword and specifying a value,
 * but most functions return the last expression implicitly.
 *
 * If we don't return anything from a function, the implicit return value is `()` which is the unit tuple
 */
fn function_with_return_value() -> i32 {
    5 // This is a valid expression. Since this is the final expression in the body of this function, it is the return value.
}

fn parameterized_function_with_return_value(x: i32) -> i32 {
    x + 1 // Again, this is a valid expression, and this being the final line of the function, it is also the return value
}
