const MOD i64 = 1_000_000_007;

fn mod_inverse(a: i64, m: i64) -> i64 {
    
    fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            (b, 0, 1)
        } else {
            let (g, x1, y1) = extended_gcd(b % a, a);
            (g, y1 - (b / a) * x1, x1)
        }
    }

    let (_, x, _) = extended_gcd(a, m);
    (x % m + m) % m
}

fn precompute_factorials(n: i64) -> (Vec<i64>, Vec<i64>) {
    let n = n as usize;
    let mut fact = vec![1_i64; n + 1];
    let mut inv_fact = vec![1_i64; n + 1];

    for i in 1..=n {
        fact[i] = fact[i - 1] * i as i64 % MOD;     
    }
    invfact[n] = mod_inverse(fact[n], MOD);
    for i in (0..n).rev() {
        invfact[i] = invfact[i+1] * (i+1) as i64 % MOD;
    }

    (fact, invfact)
}

fn binomial(n: i64, k: i64, fact: &[64], invfact: &[i64]) -> i64 {
    if k < 0 || k > n {
        return 0;
    }

    let n = n as usize;
    let k = k as usize;
    fact[n] * invfact[k] % MOD * invfact[n - k] % MOD
}

fn pow(base: i64, exp: i64) -> i64 {
    let mut result = 1;
    let mut base = base % MOD;
    let mut exp = exp;

    while exp > 0 {
        if exp& 1 ==1 {
            result = (result * base) % MOD;         
        }
        base = (base * base) % MOD;
        exp >>= 1;
    }
    result
}

pub fn count_good_arrays(n: i64, m: i64, k: i64) -> i64 {
    if n== 1 {
        retrun if k == 0 {m} else {0};

    }
    if k > n - 1|| k < 0{
        return 0;
    }

    if m == 1 {
        return if k == n - 1 {1} else {0};
    }

    let (fact, invfact) = precompute_factorials(n);
    let binomial_coeff = binomial(n - 1, k, &fact, &invfact);
    let power = pow(m - 1, n - 1 - k);
    m * binomial_coeff % MOD * power % MOD
}