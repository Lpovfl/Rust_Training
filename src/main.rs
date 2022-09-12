use calculator::math;
use math::math_structs::Numm;


fn main() {
    // let y:u8 = math::subtract(3,2);
    let x = Numm{
        first: 7,
        second: 5
    };
    println!("{}", math::subtract(x.first,x.second))
}