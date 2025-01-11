#[allow(unused_variables)]

fn main() {
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();

    let mut mantissa: f32 = 1.0;

    let sign_bit = n_bits >> 31;
    let exp_ = n_bits >> 23;
    let exp_ = exp_ & 0xff;
    let exp = (exp_ as i32) - 127;

    //println!("{} {} {}", n, n_bits, sign_bit);
    //println!("{} {} {}", n, exp_, exp);
    
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let w = 2_f32.powf(i_ - 23.0);
            mantissa += w;
            println!("{} {} {} {} ", one_at_bit_i, i_, w, mantissa);
        }
    }


}
