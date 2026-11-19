fn main() {
    // BRANCHES:
    let number = 3;

    /*
     * An `if`` expression allows you to branch your code depending on conditions.
     * All if expressions start with the keyword `if``, followed by a condition.
     * We place the block of code to execute if the condition is true immediately after the condition inside curly brackets.
     * Blocks of code associated with the conditions in if expressions are sometimes called "arms"
     */
    if number < 5 {
        println!("condition was true");
    } else {
        /*
         * Optionally, we can also include an `else` expression, to give the program
         * an alternative block of code to execute should the condition evaluate to false.
         */
        println!("condition was false");
    }
    /*
     * It’s also worth noting that the condition in this code must be a bool.
     * If the condition isn’t a bool, we’ll get an error.
     * Unlike other languages, Rust will NOT automatically try to convert non-Boolean types to a Boolean.
     */

    //------------------------------------------------------------------------------------------------------------------------------------------

    /*
     * MULTIPLE CONDITIONS WITH else if
     * You can use multiple conditions by combining if and else in an else if expression.
     */
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //------------------------------------------------------------------------------------------------------------------------------------------

    /*
     * USING if IN A let STATEMENT
     * Because `if` is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
     *
     * Remember that blocks of code evaluate to the last expression in them - and numbers by themselves are also expressions.
     * In this case, the value of the whole if expression depends on which block of code executes.
     * This means the values that have the potential to be results from each arm of the if MUST be of the same type
     */
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
