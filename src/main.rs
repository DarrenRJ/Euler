mod solutions;
mod factors;
mod palindrome;
mod choose;
mod util;

extern crate time;
extern crate num;
extern crate primal;

use time::PreciseTime;
use solutions::*;

fn main()
{

    let u64_problems : Vec<fn()->u64> = vec![p1,  p2,  p3,  p4,  p5,  p6,  p7,  p8,
                                             p9,  p10, p11, p12, p13, p14, p15, p16,
                                             p17, p18, p19, p20, p21, p22, p23, p24,
                                             p25, p26, skip, p28, p29, p30, p31, p32,
                                             p33, p34, p35, p36, p37, p38, p39, p40,
                                             p41, p42, p43, p44, p45, p46, p47, p48,
                                             p49, p50 ];
    let i64_problems : Vec<fn()->i64> = vec![p27];
    let start_time = PreciseTime::now();
    let mut total_time : time::Duration = start_time.to(start_time);

    for i in 0..u64_problems.len()
    {
        let start = PreciseTime::now();
        let result = u64_problems[i]();
        let end = PreciseTime::now();
        total_time = total_time.checked_add( &start.to(end) ).unwrap();
        println!("Answer for problem {} is {} in {:?}", i+1, result, start.to(end) );
    }

    for i in 0..i64_problems.len()
    {
        let start = PreciseTime::now();
        let result = i64_problems[i]();
        let end = PreciseTime::now();
        total_time = total_time.checked_add( &start.to(end) ).unwrap();
        println!("Answer for problem 27 is {} in {:?}", result, start.to(end) );
    }

    println!( "total time = {:?}", total_time );
}
