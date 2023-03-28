pub fn is_armstrong_number(num: u32) -> bool {
    let mut n = num;
    let mut count = 0;
    let mut arr = vec![];
    let mut sum = 0;

    while n > 0 {
        let digit = n % 10;
        arr.push(digit);
        count += 1;
        n = n / 10;
    }

    for i in arr {
        sum = sum.(i.pow(count));
    }

    sum == num
}
