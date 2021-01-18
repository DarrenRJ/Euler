mod choose;
mod factors;
mod palindrome;
mod problem54;
mod solutions;
mod util;

extern crate num;
extern crate primal;
extern crate time;

use problem54::p54;
use solutions::*;
use time::PreciseTime;

#[derive(Debug)]
pub enum Func {
    U64(fn() -> u64),
    I64(fn() -> i64),
}

fn main() {
    let solutions: Vec<Func> = vec![
        Func::U64(p1),
        Func::U64(p2),
        Func::U64(p3),
        Func::U64(p4),
        Func::U64(p5),
        Func::U64(p6),
        Func::U64(p7),
        Func::U64(p8),
        Func::U64(p9),
        Func::U64(p10),
        Func::U64(p11),
        Func::U64(p12),
        Func::U64(p13),
        Func::U64(p14),
        Func::U64(p15),
        Func::U64(p16),
        Func::U64(p17),
        Func::U64(p18),
        Func::U64(p19),
        Func::U64(p20),
        Func::U64(p21),
        Func::U64(p22),
        Func::U64(p23),
        Func::U64(p24),
        Func::U64(p25),
        Func::U64(p26),
        Func::I64(p27),
        Func::U64(p28),
        Func::U64(p29),
        Func::U64(p30),
        Func::U64(p31),
        Func::U64(p32),
        Func::U64(p33),
        Func::U64(p34),
        Func::U64(p35),
        Func::U64(p36),
        Func::U64(p37),
        Func::U64(p38),
        Func::U64(p39),
        Func::U64(p40),
        Func::U64(p41),
        Func::U64(p42),
        Func::U64(p43),
        Func::U64(p44),
        Func::U64(p45),
        Func::U64(p46),
        Func::U64(p47),
        Func::U64(p48),
        Func::U64(p49),
        Func::U64(p50),
        Func::U64(p51),
        Func::U64(p52),
        Func::U64(p53),
        Func::U64(p54),
        Func::U64(p55),
        Func::U64(p56),
        Func::U64(p57),
        Func::U64(p58),
        Func::U64(p59),
        Func::U64(p60),
        Func::U64(p61),
    ];
    let start_time = PreciseTime::now();
    let mut total_time: time::Duration = start_time.to(start_time);

    for i in 0..solutions.len() {
        match solutions[i] {
            Func::U64(f) => {
                let start = PreciseTime::now();
                let result = f();
                let end = PreciseTime::now();
                total_time = total_time.checked_add(&start.to(end)).unwrap();
                println!(
                    "Answer for problem {} is {} in {:?}",
                    i + 1,
                    result,
                    start.to(end)
                );
            }
            Func::I64(f) => {
                let start = PreciseTime::now();
                let result = f();
                let end = PreciseTime::now();
                total_time = total_time.checked_add(&start.to(end)).unwrap();
                println!(
                    "Answer for problem {} is {} in {:?}",
                    i + 1,
                    result,
                    start.to(end)
                );
            }
        }
    }

    println!("total time = {:?}", total_time);
}
