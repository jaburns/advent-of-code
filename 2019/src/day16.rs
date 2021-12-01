fn fft(digits: &[u8], offset: usize) -> Vec<u8> {
    let mut result = Vec::<u8>::new();
    let num_digits = digits.len();

    'outer: for out_digit in 0..num_digits {
        let mut sum = 0i32;
        let mut done = false;
        let mut upper = out_digit;

        loop {
            // Block of ones
            let lower = upper;
            upper = lower + offset + out_digit + 1;
            if upper >= num_digits {
                upper = num_digits;
                done = true;
            }
            for i in lower..upper {
                sum += digits[i] as i32;
            }
            if done {
                break;
            }

            // First block of zeroes
            let lower = upper;
            upper = lower + offset + out_digit + 1;
            if upper >= num_digits {
                break;
            }

            // Block of negative ones
            let lower = upper;
            upper = lower + offset + out_digit + 1;
            if upper >= num_digits {
                upper = num_digits;
                done = true;
            }
            for i in lower..upper {
                sum -= digits[i] as i32;
            }
            if done {
                break;
            }

            // Second block of zeroes
            let lower = upper;
            upper = lower + offset + out_digit + 1;
            if upper >= num_digits {
                break;
            }
        }

        result.push((sum.abs() % 10) as u8);
    }

    result
}

fn multiphase_fft(digits: &[u8], phases: u32, offset: usize, show_log: bool) -> Vec<u8> {
    let mut buffer = Vec::from(digits);

    for i in 0..phases {
        if show_log {
            println!("Computing phase {} of {}...", i + 1, phases);
        }
        buffer = fft(&buffer, offset);
    }

    buffer
}

fn digits_to_string(digits: &[u8]) -> String {
    let mut result = String::new();

    for d in digits {
        result.push_str(&d.to_string());
    }

    result
}

fn get_offset_and_digits_for_part2(digits: &[u8]) -> (usize, Vec<u8>) {
    let offset = digits[6] as usize
        + 10 * digits[5] as usize
        + 100 * digits[4] as usize
        + 1000 * digits[3] as usize
        + 10000 * digits[2] as usize
        + 100000 * digits[1] as usize
        + 1000000 * digits[0] as usize;

    let mut result = Vec::<u8>::new();

    for i in offset..(10_000 * digits.len()) {
        result.push(digits[i % digits.len()]);
    }

    (offset, result)
}

pub fn main() {
    let digits: Vec<u8> = std::fs::read_to_string("data/day16.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect();

    let (part2_offset, part2_digits) = get_offset_and_digits_for_part2(&digits);

    let fft0 = &multiphase_fft(&digits, 100, 0, false)[0..8];
    let fft1 = &multiphase_fft(&part2_digits, 100, part2_offset, true)[0..8];

    let result0 = digits_to_string(fft0);
    let result1 = digits_to_string(fft1);

    println!("{} {}", result0, result1);
}
