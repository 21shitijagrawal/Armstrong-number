use std::io;

fn main() {
    let mut input_text = String::new();
    println!("Enter the Number :  ");	
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => {
		if  is_armstrong(i) == true{
			println!("Yes it is a armstrong number");
		}else{
			println!("No it is not a armstrong number");
		}
	},
        Err(..) => println!("this was not an integer: {}", trimmed),
    };

}
pub fn is_armstrong(num:i32) -> bool {
    let mut digits = Vec::new();
    let mut remainder = num;
    let mut length = 0;
    while remainder > 0 {
        digits.push(remainder % 10);
        remainder = remainder / 10;
        length = length + 1;
    }
    num == digits.iter().map(|d| d.pow(length)).sum()
}
