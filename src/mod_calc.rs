const MOD: i64 = 1000000007;

pub fn power(x: i64, y: i64) -> i64 {
    if y == 0 {
        1
    } else if y == 1 {
        x % MOD
    } else if y % 2 == 0 {
        power(x, y / 2).pow(2) % MOD
    } else {
        power(x, y / 2).pow(2) * x % MOD
    }
}

pub fn div(a: i64, b: i64) -> i64 {
    mul(a, power(b, MOD - 2))
}

pub fn add(a: i64, b: i64) -> i64 {
    (a + b) % MOD
}

pub fn mul(a: i64, b: i64) -> i64 {
    ((a % MOD) * (b % MOD)) % MOD
}
