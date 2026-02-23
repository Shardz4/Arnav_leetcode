fn ways_to_distribute(n: i32, k: i32) -> i32 {
    fn comb(k: i32) -> i32 {
        if k<2{
            0
        } else {
            (k * (k - 1)) / 2
        }
    }

    if n > 3 * k {
        return 0;
    }

    let total  =comb(n+2);
    let over limit = 3 * comb(n - k + 1) - 3 * comb(n - 2 * k) + comb(n - 3 * k - 1);

    total - over_limit
}