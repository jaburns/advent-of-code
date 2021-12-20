use std::fmt::Write;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SnailNumElement {
    Push,
    Value(u32),
    Pop,
}

impl SnailNumElement {
    fn to_value(self) -> u32 {
        if let SnailNumElement::Value(v) = self {
            v
        } else {
            panic!()
        }
    }
}

#[derive(Debug)]
struct SnailNum(Vec<SnailNumElement>);

impl SnailNum {
    fn parse(repr: &str) -> Self {
        let mut result = vec![];

        for ch in repr.chars() {
            match ch {
                '[' => result.push(SnailNumElement::Push),
                ']' => result.push(SnailNumElement::Pop),
                ',' => {}
                '0'..='9' => result.push(SnailNumElement::Value(ch.to_digit(10).unwrap())),
                _ => panic!(),
            }
        }

        Self(result)
    }

    fn explode_at(&mut self, idx: usize) {
        let lval = self.0[idx].to_value();
        let rval = self.0[idx + 1].to_value();

        for i in (0..idx).rev() {
            if let SnailNumElement::Value(v) = self.0.get_mut(i).unwrap() {
                *v += lval;
                break;
            }
        }

        for i in (idx + 3)..self.0.len() {
            if let SnailNumElement::Value(v) = self.0.get_mut(i).unwrap() {
                *v += rval;
                break;
            }
        }

        self.0[idx] = SnailNumElement::Value(0);
        self.0.remove(idx - 1);
        self.0.remove(idx);
        self.0.remove(idx);
    }

    fn split_at(&mut self, idx: usize) {
        let val = self.0[idx].to_value();
        let lval = val >> 1;
        let rval = lval + (val & 1 != 0) as u32;

        self.0.insert(idx, SnailNumElement::Push);
        self.0[idx + 1] = SnailNumElement::Value(lval);
        self.0.insert(idx + 2, SnailNumElement::Value(rval));
        self.0.insert(idx + 3, SnailNumElement::Pop);
    }

    fn step_reduce(&mut self) -> bool {
        let mut depth = 0;
        for (i, elem) in self.0.iter().enumerate() {
            match &elem {
                SnailNumElement::Push => {
                    depth += 1;
                }
                SnailNumElement::Pop => {
                    depth -= 1;
                }
                SnailNumElement::Value(_) => {
                    if depth > 4 && matches!(self.0[i + 1], SnailNumElement::Value(_)) {
                        self.explode_at(i);
                        return false;
                    }
                }
            }
        }

        for (i, elem) in self.0.iter().enumerate() {
            if let SnailNumElement::Value(v) = elem {
                if *v >= 10 {
                    self.split_at(i);
                    return false;
                }
            }
        }

        true
    }

    fn evaluate_magnitude(mut self) -> u32 {
        'outer: while self.0.len() > 1 {
            for i in 0..(self.0.len() - 3) {
                let a = self.0[i];
                let b = self.0[i + 1];
                let c = self.0[i + 2];
                let d = self.0[i + 3];

                if a == SnailNumElement::Push && d == SnailNumElement::Pop {
                    if let (SnailNumElement::Value(b), SnailNumElement::Value(c)) = (b, c) {
                        self.0.remove(i);
                        self.0.remove(i);
                        self.0.remove(i);
                        *self.0.get_mut(i).unwrap() = SnailNumElement::Value(3 * b + 2 * c);
                        continue 'outer;
                    }
                }
            }
            break;
        }
        self.0[0].to_value()
    }

    fn add_and_reduce(&self, rhs: &Self) -> Self {
        let mut result = vec![SnailNumElement::Push];
        result.extend(self.0.iter());
        result.extend(rhs.0.iter());
        result.push(SnailNumElement::Pop);
        let mut result = Self(result);

        while !result.step_reduce() {}

        result
    }
}

pub fn part1(lines: &[&str], out: &mut String) {
    let mut nums: Vec<SnailNum> = lines.iter().map(|&x| SnailNum::parse(x)).rev().collect();
    let mut result = nums.pop().unwrap();

    while !nums.is_empty() {
        result = SnailNum::add_and_reduce(&result, &nums.pop().unwrap());
    }

    let result = result.evaluate_magnitude();

    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let nums: Vec<SnailNum> = lines.iter().map(|&x| SnailNum::parse(x)).rev().collect();

    let mut max_value = 0_u32;

    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }
            max_value =
                max_value.max(SnailNum::add_and_reduce(&nums[i], &nums[j]).evaluate_magnitude());
        }
    }

    write!(out, "{}", max_value).unwrap();
}
