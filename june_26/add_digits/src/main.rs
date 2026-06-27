/*
   class Solution {
   public:
       int addDigits(int num) {

       }
   };

   Given an integer num, repeatedly add all its digits until the result has only one digit, and return it.

   Example 1:

   Input: num = 38
   Output: 2
   Explanation: The process is
   38 --> 3 + 8 --> 11
   11 --> 1 + 1 --> 2
   Since 2 has only one digit, return it.
   Example 2:

   Input: num = 0
   Output: 0


   Constraints:

   0 <= num <= 2^31 - 1

*/
fn main() {
    //let number: u32 = 2147483648 - 1;
    let number = 2345237;
    println!("{}", add_digits(number));
}

fn add_digits(num: i32) -> i32 {
    let mut num = num;
    loop {
        let mut new_number = 0;
        for num_sliced in num.clone().to_string().chars() {
            new_number += num_sliced.to_digit(10).unwrap() as i32;
        }
        if new_number < 10 {
            return new_number;
        } else {
            num = new_number;
        }
    }
}
