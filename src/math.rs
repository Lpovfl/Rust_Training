pub mod math_structs;

use math_structs::Numm;

pub fn subtract(num1: u8, num2: u8) -> u8{
   let x = Numm{
    first: num1,
    second: num2
   };
   x.first - x.second
}

