fn main() {
    if  is_amstrome(9) == true{
        println!("Yes it is a amstrome number");
    }else{
        println!("No it is not a amstrome number");
    }
}
pub fn is_amstrome(num:i32) -> bool {
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
