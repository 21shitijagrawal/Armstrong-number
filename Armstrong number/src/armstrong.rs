use std::io;	

fn main() {
    let mut input_text = String::new();	// Declaring / Creating variable to get input .
    println!("Enter the Number :  ");	// Ask user to input number . 
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");	// Getting input from user by calling input stream 
										// And applying read_line() function to it to get the input.
    let trimmed = input_text.trim();	// Removing White Spaces from Sides of input and assigning the result to the variable trimmed.
    match trimmed.parse::<i32>() {	// Parsing / Converting the trimmed string into number and checking for err and result with match statement. 
        Ok(i) => {			// If successfully Parsed.
		if  is_armstrong(i) == true{	// Checking either the number is armstrong or not.
			println!("Yes it is a armstrong number");	// If condition is true then the number is armstrong. 
		}else{
			println!("No it is not a armstrong number");	// Else the number is not armstrong.
		}
	},
        Err(..) => println!("this was not an integer: {}", trimmed),	// If any error occurs then the trimmed variable does not contain the integer value. 
    };

}
pub fn is_armstrong(num:i32) -> bool {
    let mut digits = Vec::new();	// Creating a new vector for storing the digits of number.
    let mut remainder = num;		// Initializing the remainder with num.
    let mut length = 0;			// Initializing the length to zero.
    while remainder > 0 {
        digits.push(remainder % 10);	// Pushing Last digit of a number to the vector by taking mod of remainder with 10 ( EX : 123 mod 10 = 3 ) . 
        remainder = remainder / 10;	// Dividing remainder by itself for removing last digit of number ( EX:  123 / 10 = 12 ) .
        length = length + 1;		// Increasing the length by one .
    }
    num == digits.iter().map(|d| d.pow(length)).sum()	// Traversing through the vector and calculating the power of each digit by length (the number of digits) and then    
							// Adding that result and comparing with num and then returning the result of comparison   
}
