fn main() {
    let n = 42.42_f32;

    let n_bits = n.to_bits();

    // sign 1b [32, 31)
    let sign_bit = n_bits >> 31;
    let sign = if sign_bit == 0 { 1.0_f32 } else { -1.0_f32 };

    // exp 8b [31, 23)
    let exp_bit = (n_bits >> 23) & 0xff;
    let exp = ((exp_bit as i32) - 127) as f32; // [-127, 128]

    // man 23b [23, 0]
    let mut man = 1.0_f32;
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        if one_at_bit_i != 0 {
            man += 2_f32.powf(i as f32 - 23.0);
        }
    }

    // result
    let result = sign * 2.0_f32.powf(exp) * man;
    println!("origin: {}", n);
    println!("sign: {}", sign);
    println!("exp: {}", exp);
    println!("2^exp: {}", 2.0_f32.powf(exp));
    println!("man: {}", man);
    println!("result: {}", result);
}
