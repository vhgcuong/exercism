pub fn nth(n: u32) -> u32 {
    let mut nth_primes: Vec<u32> = Vec::new();
    let mut index = 0;

    loop {
        if !nth_primes.is_empty() && nth_primes.len() == (n + 1) as usize {
            return *nth_primes.get((n) as usize).unwrap();
        }

        if is_prime(index) {
            nth_primes.push(index);
        }
        index += 1;
    }
}


///
/// kiểm tra các số k có tính chất giống với tính chất của số nguyên tố, có thể
/// sử dụng một trong hai tính chất ñơn giản sau của số nguyên tố:
/// 1) Trừ số 2 và các số nguyên tố là số lẻ.
/// 2) Trừ số 2, số 3 các số nguyên tố có dạng 6K ± 1 (vì số có dạng 6K ± 2 thì
/// chia hết cho 2, số có dạng 6K ± 3 thì chia hết cho 3).
///
pub fn is_prime(number: u32) -> bool {
    if number == 2 || number == 3 {
        return true;
    }

    if number == 1 || number % 2 == 0 || number % 3 == 0 {
        return false;
    }

    let sqrt_n = (number as f64).sqrt() as u32;
    let mut k = 5;
    let mut result = true;
    while k <= sqrt_n {
        if number % k == 0 || number % (k + 2) == 0 {
            result = false;
            break;
        }
        k += 6;
    }
    result
}

// pub fn is_prime(number: u32) -> bool {
//     if number <= 1 {
//         return false;
//     }
//
//     2u32.pow(number) % number == 2
// }


// pub fn is_prime(number: u32) -> bool {
//     let number = number as f64;
//     if number < 2.0 {
//         return false;
//     }
//
//     let square_number = number.sqrt() as u32 + 1;
//     for i in 2..square_number {
//         if number as u32 % i == 0 {
//             return false;
//         }
//     }
//
//     true
// }
