
// specialised n choose k that tries not to overflow a u64 on larger numbers
pub fn n_choose_k( n :u64, k :u64 ) -> u64
{
    // n!/(k!*(n-k)!)
    // we assume n-k > k
    let mut n_on_n_sub_k_fact :u64 = 1;
    let mut k_fact = Vec::new();
    for i in 2..k+1
    {
        k_fact.push(i);
    }
    // calculate n_on_n_sub_k_fact but also
    // divide where possible to limit max zise
    for i in (n-k)+1..n+1
    {
        n_on_n_sub_k_fact *= i;
        for j in 0..k_fact.len()
        {
            if n_on_n_sub_k_fact % k_fact[j] == 0
            {
                n_on_n_sub_k_fact /= k_fact.remove(j);
                break; // we just busted our itorator so bail the loop here
            }
        }
    }
    // if there is anything left in the vector divide it out now
    let mut divider = 1;
    for j in k_fact
    {
        divider *= j;
    }

    return n_on_n_sub_k_fact/divider;
}

#[cfg(test)]
mod test
{
    use super::*;
    #[test]
    fn test_choose()
    {
        assert!(n_choose_k(3,3)==1);
        assert!(n_choose_k(3,2)==3);
        assert!(n_choose_k(4,4)==1);
        assert!(n_choose_k(4,3)==4);
        assert!(n_choose_k(4,2)==6);
    }
}
