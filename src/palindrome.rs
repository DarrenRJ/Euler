
// return true if the number is a palendrome in decimal
pub fn is_dec_palendrome( x: u32 ) ->bool
{
    let mut reverse : u32 = 0;
    let mut temp = x;

    while temp > 0
    {
        reverse *= 10;
        reverse += temp%10;
        temp /= 10;
    }
    reverse == x
}


// return true if the number is a palendrome in binary
pub fn is_bin_palandrome( num : u32 ) -> bool
{
    let num_bits = 32 - num.leading_zeros();

    for i in 0..num_bits/2
    {
        if ( (num>>i) & 1 ) != ( (num>>(num_bits-(i+1))) & 1)
        {
            return false;
        }
    }
    return true;
}
