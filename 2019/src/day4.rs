fn get_digits(n: i32) -> [i32; 6] {
    [
        (n / 100000) % 10,
        (n / 10000) % 10,
        (n / 1000) % 10,
        (n / 100) % 10,
        (n / 10) % 10,
        (n / 1) % 10,
    ]
}

fn same_neighbors_criterion(digits: [i32; 6]) -> bool {
    digits[0] == digits[1]
        || digits[1] == digits[2]
        || digits[2] == digits[3]
        || digits[3] == digits[4]
        || digits[4] == digits[5]
}

fn never_decrease_criterion(digits: [i32; 6]) -> bool {
    digits[0] <= digits[1]
        && digits[1] <= digits[2]
        && digits[2] <= digits[3]
        && digits[3] <= digits[4]
        && digits[4] <= digits[5]
}

fn extra_criterion(digits: [i32; 6]) -> bool {
    let mut result = false;
    for i in 0..5 {
        let is_double = digits[i] == digits[i + 1];
        let no_left_match = i == 0 || digits[i - 1] != digits[i];
        let no_right_match = i == 4 || digits[i + 2] != digits[i];
        result = result || is_double && no_left_match && no_right_match;
    }
    result
}

pub fn main() {
    let mut result0 = 0i32;
    let mut result1 = 0i32;

    for i in 153517..630395 {
        let digits = get_digits(i);
        if same_neighbors_criterion(digits) && never_decrease_criterion(digits) {
            result0 += 1;

            if extra_criterion(digits) {
                result1 += 1;
            }
        }
    }

    println!("{} {}", result0, result1);
}
