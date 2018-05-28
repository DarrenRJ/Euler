extern crate primal;

use factors::*;


// make a string out of a number
pub fn stringify( in_num : u16 ) -> String
{
    let mut result : String;
    let mut num = in_num;

    result = match num/1000 {
        0 => "",
        1 => "one thousand ",
        2 => "two thousand ",
        3 => "three thousand ",
        4 => "four thousand ",
        5 => "five thousand ",
        6 => "six thousand ",
        7 => "seven thousand ",
        8 => "eight thousand ",
        9 => "nine thousand ",
        _ => "" }.to_string();

    num %= 1000;
    result += match num/100 {
        0 => "",
        1 => "one hundred ",
        2 => "two hundred ",
        3 => "three hundred ",
        4 => "four hundred ",
        5 => "five hundred ",
        6 => "six hundred ",
        7 => "seven hundred ",
        8 => "eight hundred ",
        9 => "nine hundred ",
        _ => "" };
    num %= 100;
    if num!=0 {
        if  in_num/100 != 0 {
            result += "and ";
        }
        // special case the teens
        if num > 9 && num < 20 {
            result += match num%100 {
                10 => "ten",
                11 => "eleven",
                12 => "twelve",
                13 => "thirteen",
                14 => "fourteen",
                15 => "fifteen",
                16 => "sixteen",
                17 => "seventeen",
                18 => "eighteen",
                19 => "nineteen",
                _  => ""
            };
        }
        else
        {
            result += match num/10 {
                0 => "",
                1 => "",
                2 => "twenty ",
                3 => "thirty ",
                4 => "forty ",
                5 => "fifty ",
                6 => "sixty ",
                7 => "seventy ",
                8 => "eighty ",
                9 => "ninety ",
                _ => ""
            };
            result += match num%10 {
                0 => "",
                1 => "one",
                2 => "two",
                3 => "three",
                4 => "four",
                5 => "five",
                6 => "six",
                7 => "seven",
                8 => "eight",
                9 => "nine",
                _ => ""
            };
        }
    }


    return result;
}

// count the number of chars in a string
pub fn count_chars( number : String ) -> u32 {
    let mut sum : u32 = 0;
    for i in number.chars() {
        if i != ' ' {
            sum += 1;
        }
    }
    return sum;
}

// return true if the year is a leap year
pub fn is_leap( year : u16 )-> bool
{
    if (year % 400) == 0
    {
        return true;
    }
    if (year % 100 ) == 0
    {
        return false;
    }
    if (year % 4 ) == 0
    {
        return true;
    }
    return false;
}


pub fn permutate( digits : &mut Vec<u64>, num : u32  )
{
    for _i in 0..num
    {
        // do a permutaion
        // which is to say re-arange the digits to the next largest arangement

        // step 1 is find the right most increasing pair

        // for all the digits right to left
        for j in 1..digits.len()
        {
            let index = digits.len()-j;
            // if they are increasing
            if digits[index]>digits[index-1]
            {
                // swap a new value into index-1
                // this value will be the smallest value to the right
                // that is also larger than the current value
                let current_val = digits[index-1];
                let mut right_chunk : Vec<u64> = digits.drain( index-1.. ).collect();

                right_chunk.sort();
                let result = right_chunk.binary_search( &current_val ).expect("wierd error");
                let left_most = right_chunk.remove( result+1 );
                digits.push(left_most);
                digits.extend(right_chunk);

                // exit the loop
                break;
            }
            // if we have hit the end, start again
            if index == 1
            {
//                println!("rollover at perm {}", i);
/*                println!("digits = {}{}{}{}{}{}{}{}{}{}",
                    digits[0], digits[1], digits[2], digits[3], digits[4],
                    digits[5], digits[6], digits[7], digits[8], digits[9] );*/
                digits.sort();
                break;
            }
        }
/*
        println!("digits = {}{}{}{}{}{}{}{}{}{}",
            digits[0], digits[1], digits[2], digits[3], digits[4],
            digits[5], digits[6], digits[7], digits[8], digits[9] );
*/
    }
}


// function to find if 3 numbers are decimal digit permutations of each other
pub fn is_3_perm( mut a : usize, mut b:usize, mut c: usize ) -> bool
{
    let mut digits_a : Vec<u8> = Vec::new();
    let mut digits_b : Vec<u8> = Vec::new();
    let mut digits_c : Vec<u8> = Vec::new();
    while a > 0
    {
        digits_a.push((a%10) as u8);
        a /= 10;
    }

    while b > 0 
    {
        digits_b.push((b%10) as u8);
        b /= 10;
    }
    while c > 0
    {
        digits_c.push((c%10) as u8);
        c /= 10;
    }
    digits_a.sort();
    digits_b.sort();
    digits_c.sort();

    for i in 0..digits_a.len()
    {
        if digits_a[i] != digits_b[i] ||
           digits_a[i] != digits_c[i]
        {
            return false
        }
    }
    return true;
}


// fucntion to find if two numbers are deciaml digit permutations of each other
pub fn is_perm( mut a :usize, mut b : usize ) -> bool
{
    let mut digits_a : Vec<u8> = Vec::new();
    let mut digits_b : Vec<u8> = Vec::new();

    while a > 0
    {
        digits_a.push((a%10) as u8);
        a /= 10;
    }
    while b > 0
    {
        digits_b.push((b%10) as u8);
        b /= 10;
    }
    digits_a.sort();
    digits_b.sort();

    if digits_a.len() != digits_b.len()
    {
        return false;
    }

    for i in 0..digits_a.len()
    {
        if digits_a[i] != digits_b[i]
        {
            return false
        }
    }

    return true;
}



pub fn quadratic( a : i64, b:i64, n:i64 ) -> u64
{
    let quad = (n*n) +(a*n) + (b);
    if quad < 0
    {
        return 0
    }
    return quad as u64;
}

pub fn reduce_fraction( n :u64, d:u64 ) -> ( u64, u64 )
{
    let mut n_factors = factors(n);
    let mut d_factors = factors(d);
    n_factors.sort();
    d_factors.sort();

    for i in 0..n_factors.len()
    {
        if let Ok(index) = d_factors.binary_search( &(n_factors[(n_factors.len()-(i+1))]) )
        {
            let factor = d_factors[index];
            return (n/factor, d/factor);
        }
    }
    (n,d)
}

// return true if the number is left and right primally truncatable
pub fn is_trunc( number :u32, sieve: &primal::Sieve ) -> bool
{
    let mut digits = Vec::new(); // a vector to fill with decimal digits
    let mut i = number/10; // we know the provided number is prime so start at the first right truncation
    digits.push(number%10);

    // loop around check the number is right truncatable
    // and gather digits for left truncatable test
    while i > 0
    {
        // check left truncation for primality
        if !sieve.is_prime(i as usize)
        {
            return false;
        }
        // stash this digit for later use
        digits.push(i%10);
        // push a decimal digit off the right end of this number
        i /= 10;
    }

    // check the number is left truncatable
    // working from smallest left truncation to largest
    i = 0;
    let mut multiplier = 1;
    for j in 0..digits.len()
    {
        i += digits[j]*multiplier; // add the digit as the high order decimal digit
        multiplier *= 10; // swap to the higher decimal digit
        // check right truncation for primality
        if !sieve.is_prime(i as usize)
        {
            return false;
        }
    }

    return true
}

// return the 1-9 pandigital product of the supplied number
pub fn pandigital_product( input : u64 ) -> Option<u64>
{
    let mut result : u64 = 0;
    let mut digit_used : [bool;9] = [false;9];
    let mut digit_count = 0;
    for i in 1..10
    {
        let product : u64 = i as u64 * input;
        let mut decode_product = product;
        while decode_product > 0
        {
            let digit : usize = (decode_product as usize)%10;
            if digit == 0
            {
                return None;
            }
            if digit_used[digit-1]
            {
                return None; // fail as we reused a digit
            }
            digit_used[digit-1] = true;
            decode_product /= 10;
            digit_count += 1;
        }

        let power = (product as f64).log(10f64) as u32;
        result *= u64::pow(10, power+1);
        result += product;
        if result > 987654321
        {
            return None;
        }
        if digit_count == 9
        {
            return Some(result);
        }
    }
    return None;
}

// find the nth digit in the sequence of decimal integers 1234567891011121314
pub fn digit_num( mut digit : usize ) -> u64
{
    if digit==0
    {
        return 0;
    }
    let mut max = 9;
    let mut cuttoff = 0;
//    println!( "called digit_num with {}", digit );
    digit -= 1;
    for i in 1..10
    {
//        println!( "loop {}", i );
        if digit <= max*i
        {
            let number = cuttoff+(digit/i)+1;
            let offset = digit%i;
            let decimal = number.to_string().into_bytes();
//            println!( "number = {} offset = {} decimal = {:?}", number, offset, decimal );
            return decimal[offset] as u64 - 0x30;
        }
        cuttoff += max;
        digit -= max*i;
        max *= 10;
//        println!( "digit = {} max = {}", digit, max );
    }
    0_u64
}


pub fn is_triangle( value : usize ) -> bool
{
    let tri_root = value*8 + 1;
    let sqrt = f64::sqrt( tri_root as f64 );

    if sqrt.floor() ==  sqrt
    {
        return true;
    }
    return false;
}

pub fn is_pentagonal( value : usize ) -> bool
{
    let root = value*24+1;
    let numerator = f64::sqrt(root as f64) + 1_f64;
    let test = numerator/6_f64;
    if test.floor() == test 
    {
        return true;
    }
    return false;
}

// find a^b mod c
pub fn powmod( value : usize, mut exponent : usize, modulo : usize ) -> usize
{
    let mut result = value;

    while exponent > 1
    {
        if result > modulo
        {
            result %= modulo;
        }
        result *= value;
        exponent -= 1;
    }
    return result%modulo;
}
