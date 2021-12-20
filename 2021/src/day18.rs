use std::fmt::Write;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SnailNumElement {
    Push,
    Value(u8),
    Pop,
}

impl SnailNumElement {
    fn to_value(self) -> u8 {
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
                '0'..='9' => result.push(SnailNumElement::Value(ch.to_digit(10).unwrap() as u8)),
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
        let rval = lval + (val & 1 != 0) as u8;

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

    fn evaluate_magnitude(&self) -> u32 {
        0
    }
}

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut nums: Vec<SnailNum> = lines.iter().map(|&x| SnailNum::parse(x)).rev().collect();
    let mut result = nums.pop().unwrap();

    while !nums.is_empty() {
        let mut next_num = nums.pop().unwrap();
        let mut new_result = vec![SnailNumElement::Push];
        new_result.append(&mut result.0);
        new_result.append(&mut next_num.0);
        new_result.push(SnailNumElement::Pop);
        result = SnailNum(new_result);

        while !result.step_reduce() {}
    }

    write!(out, "{:?}", result).unwrap();
}
