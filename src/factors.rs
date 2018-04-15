
// return a vector of prime factors for the number provided
pub fn prime_factors( mut number: u64 ) -> Vec<u64>
{
    let mut result = Vec::new();
    let mut i : u64 = 3;

    // handle 0
    if number == 0
    {
        return result;
    }

    // deal with even prime factors
    while ( number & 0x01 ) == 0
    {
        result.push(2);
        number = number>>1;
    }

    // find odd prime factors
    while number > 1
    {
        while ( number%i ) == 0
        {
            result.push(i);
            number = number/i;
        }

        i = i+2; // all prime factors after 2 are odd, so save time by skipping even numbers
    }
    return result;
}



// return a vector of factors
pub fn factors( number: u64 ) -> Vec<u64>
{
    let mut result = Vec::new();
    let mut i : u64 = 2;

    // deal with a 0 input
    if number == 0
    {
        return result;
    }
    result.push(1);
    // deal with a 1 input
    if number == 1
    {
        return result;
    }
    result.push(number);

    let mut sqrt : u64 = (number as f64).sqrt() as u64;

    // check if we should add the square root
    if ( sqrt > 1 ) && ( sqrt * sqrt == number )
    {
        result.push(sqrt);
        sqrt -= 1;
    }

    // deal with factors
    while i <= sqrt
    {
        if number % i == 0
        {
            result.push(i);
            result.push(number/i);
        }
        i += 1;
    }

    return result;
}

// return the number of divisors the provided number has
pub fn divisor_count( number : u64 ) -> u64
{
    let mut count = 2;
    let mut i :u64 = 2;
    let mut sqrt : u64 = (number as f64).sqrt() as u64;
    //deal with 0
    if number == 0
    {
        return 0;
    }
    if number == 1
    {
        return 1;
    }

    // deal with perfect squares
    if ( sqrt > 1 ) && ( sqrt * sqrt == number )
    {
        count += 1;
        sqrt -= 1;
    }

    // deal with other factors
    while i <= sqrt
    {
        if number %i == 0
        {
            count +=2;
        }
        i += 1;
    }
    // check if we should add the square root

    count
}


pub fn proper_divisors( number: u64 ) -> Vec<u64>
{
    let mut result = Vec::new();
    let mut i : u64 = 2;
    let mut sqrt : u64 = (number as f64).sqrt() as u64;

    if number > 1
    {
        result.push(1);
    }

    // deal with a perfect square
    if sqrt > 1 && sqrt * sqrt == number
    {
        result.push(sqrt);
        sqrt -= 1;
    }

    // deal with factors
    while i <= sqrt
    {
        if number % i == 0
        {
            result.push(i);
            result.push(number/i);
        }
        i += 1;
    }

    return result;
}



// return the sum of a numbers proper divisors ie all the proper factors
// factors other than itself
pub fn sum_of_proper_divisors( number: u32 ) -> u64
{
    let mut sum_of_factors : u32 = 0;
    let mut i : u32 = 2;
    let mut sqrt : u32 = (number as f64).sqrt() as u32;

    // handle zero and 1
    if number <= 1
    {
        return 0;
    }
    sum_of_factors += 1;

    // handle the case of a perfect square
    if ( sqrt > 1 ) && ( sqrt * sqrt == number )
    {
        sum_of_factors += sqrt;
        sqrt -= 1;
    }

    // deal with factors
    while i <= sqrt
    {
        if number % i == 0
        {
            sum_of_factors += i + number/i;
        }
        i += 1;
    }

    return sum_of_factors as u64;
}

#[cfg(test)]
mod test
{
    use super::*;
    #[test]

    fn test_prime_factors()
    {
        assert_eq!(prime_factors(1), vec![]);
        assert_eq!(prime_factors(2), vec![2]);
        assert_eq!(prime_factors(3), vec![3]);
        assert_eq!(prime_factors(4), vec![2,2]);
        assert_eq!(prime_factors(5), vec![5]);
        assert_eq!(prime_factors(6), vec![2,3]);
        assert_eq!(prime_factors(12), vec![2,2,3]);
        assert_eq!(prime_factors(21), vec![3,7]);
        assert_eq!(prime_factors(30), vec![2,3,5]);
    }

    fn test_factors()
    {
        assert_eq!(factors(1), vec![1]);
        assert_eq!(factors(2), vec![1,2]);
        assert_eq!(factors(3), vec![1,3]);
        assert_eq!(factors(4), vec![1,4,2]);
        assert_eq!(factors(6), vec![1,6,2,3]);
        assert_eq!(factors(12), vec![1,12,2,6,3,4]);
    }

    fn test_divisor_count()
    {
        assert_eq!(divisor_count(1), 1);
        assert_eq!(divisor_count(3), 2);
        assert_eq!(divisor_count(6), 4);
        assert_eq!(divisor_count(10), 4);
        assert_eq!(divisor_count(15), 4);
        assert_eq!(divisor_count(21), 4);
        assert_eq!(divisor_count(28), 6);
    }

    fn test_proper_divisors()
    {
        assert_eq!(proper_divisors(1), vec![]);
        assert_eq!(proper_divisors(2), vec![1]);
        assert_eq!(proper_divisors(3), vec![1]);
        assert_eq!(proper_divisors(4), vec![1,2]);
        assert_eq!(proper_divisors(5), vec![1]);
        assert_eq!(proper_divisors(6), vec![1,2,3]);
        assert_eq!(proper_divisors(7), vec![1]);
        assert_eq!(proper_divisors(12), vec![1,2,6,3,4]);
    }

    fn test_sum_of_proper_divisors()
    {
        assert!(sum_of_proper_divisors(1)==0);
        assert!(sum_of_proper_divisors(2)==1);
        assert!(sum_of_proper_divisors(3)==1);
        assert!(sum_of_proper_divisors(4)==3);
        assert!(sum_of_proper_divisors(5)==1);
        assert!(sum_of_proper_divisors(6)==6);
        assert!(sum_of_proper_divisors(28)==28);
        assert!(sum_of_proper_divisors(12)==16);
    }
}



pub fn prime_factor_count( mut number : usize ) -> usize
{
    let mut i = 3;
    let mut count = 0;

    // handle 0
    if number == 0
    {
        return 0;
    }

    // deal with even prime factors
    if ( number % 2 ) == 0
    {
        while ( number % 2 ) == 0
        {
            number = number/2;
        }
        count += 1;
    }

    // find odd prime factors
    while i <= (f64::sqrt(number as f64).floor()) as usize
    {
        if ( number%i ) == 0
        {
            while ( number%i ) == 0
            {
                number = number/i;
            }
            count += 1;
        }

        i = i+2; // all prime factors after 2 are odd, so save time by skipping even numbers
    }

    if number > 1 
    {
        count += 1;
    }

    return count;
}