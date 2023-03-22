pub fn collatz(num: u64) -> Option<u64> {
    let mut n = num.clone();
    let mut count = 0;

    while n != 1 {
        if n == 0 || n >= (u64::MAX - 1) / 3 {
            return None;
        }
        if n % 2 != 0 {
            n = n * 3 + 1;
            count += 1;
        } else {
            n = n / 2;
            count += 1;
        }
    }
    Some(count)
}
