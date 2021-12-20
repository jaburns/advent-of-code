use std::fmt::Write;

const TYPE_SUM: usize = 0;
const TYPE_PRODUCT: usize = 1;
const TYPE_MINIMUM: usize = 2;
const TYPE_MAXIMUM: usize = 3;
const TYPE_LITERAL: usize = 4;
const TYPE_GREATER_THAN: usize = 5;
const TYPE_LESS_THAN: usize = 6;
const TYPE_EQUAL_TO: usize = 7;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut bits = Bits::new(lines[0]);
    let value = bits.parse().1;
    write!(out, "{}  {}", bits.version_sum, value).unwrap();
}

struct Bits {
    version_sum: usize,
    idx: usize,
    bits: Vec<bool>,
}

impl Bits {
    fn new(hex: &str) -> Self {
        Self {
            version_sum: 0,
            idx: 0,
            bits: hex
                .chars()
                .flat_map(|x| {
                    let num = x.to_digit(16).unwrap();
                    [
                        num & 0b1000 != 0,
                        num & 0b0100 != 0,
                        num & 0b0010 != 0,
                        num & 0b0001 != 0,
                    ]
                })
                .collect(),
        }
    }

    fn chomp(&mut self, num_bits: usize) -> usize {
        let mut mask = 1_usize;
        let mut ret = 0_usize;

        self.idx += num_bits;

        for i in 1..=num_bits {
            if self.bits[self.idx - i] {
                ret |= mask;
            }
            mask <<= 1;
        }

        ret
    }

    fn chomp_literal(&mut self) -> usize {
        let mut chunks = vec![];

        loop {
            let keep_going = self.chomp(1) != 0;
            chunks.push(self.chomp(4));
            if !keep_going {
                break;
            }
        }

        let mut value = 0;
        let mut mask = 1;
        while !chunks.is_empty() {
            value += mask * chunks.pop().unwrap();
            mask *= 16;
        }

        value
    }

    fn chomp_operator(&mut self, type_: usize) -> usize {
        let length_type = self.chomp(1);
        let mut values = vec![];

        if length_type == 0 {
            let mut total_bits = self.chomp(15) as isize;
            while total_bits > 0 {
                let (bits, value) = self.parse();
                total_bits -= bits as isize;
                values.push(value);
            }
        } else {
            let total_packets = self.chomp(11);
            for _ in 0..total_packets {
                let (_, value) = self.parse();
                values.push(value);
            }
        }

        match type_ {
            TYPE_SUM => values.iter().sum(),
            TYPE_PRODUCT => values.iter().product(),
            TYPE_MINIMUM => *values.iter().min().unwrap(),
            TYPE_MAXIMUM => *values.iter().max().unwrap(),
            TYPE_GREATER_THAN => (values[0] > values[1]) as usize,
            TYPE_LESS_THAN => (values[0] < values[1]) as usize,
            TYPE_EQUAL_TO => (values[0] == values[1]) as usize,
            _ => panic!(),
        }
    }

    fn parse(&mut self) -> (usize, usize) {
        let start_idx = self.idx;

        let version = self.chomp(3);
        let type_ = self.chomp(3);

        self.version_sum += version;

        let value = if type_ == TYPE_LITERAL {
            self.chomp_literal()
        } else {
            self.chomp_operator(type_)
        };

        (self.idx - start_idx, value)
    }
}
