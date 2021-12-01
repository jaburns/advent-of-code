use modinverse::modinverse;
use num::cast::ToPrimitive;
use num::BigInt;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Technique {
    Reverse,
    Cut(i64),
    Deal(i64),
}

fn parse_technique_line(line: &str) -> Technique {
    const CUT_OFFSET: usize = 1 + "cut".len();
    const DEAL_OFFSET: usize = 1 + "deal with increment".len();

    if line == "deal into new stack" {
        Technique::Reverse
    } else if line.starts_with("cut") {
        Technique::Cut(line[CUT_OFFSET..].parse::<i64>().unwrap())
    } else {
        Technique::Deal(line[DEAL_OFFSET..].parse::<i64>().unwrap())
    }
}

#[derive(Debug, Clone)]
struct ModularTransformation {
    pub a: BigInt,
    pub b: BigInt,
    pub m: BigInt,
}

impl ModularTransformation {
    pub fn then(&self, other: &ModularTransformation) -> ModularTransformation {
        if self.m != other.m {
            panic!("Cannot combine operations of different moduli");
        }
        ModularTransformation {
            a: (&self.a * &other.a) % &self.m,
            b: (&other.a * &self.b + &other.b) % &self.m,
            m: self.m.clone(),
        }
    }

    pub fn apply(&self, x: &BigInt) -> BigInt {
        ((&self.a * x + &self.b) % &self.m + &self.m) % &self.m
    }

    pub fn identity(m: BigInt) -> ModularTransformation {
        ModularTransformation {
            a: BigInt::from(1),
            b: BigInt::from(0),
            m: m,
        }
    }
}

fn get_reverse_transformation(num_cards: u64, technique: &Technique) -> ModularTransformation {
    let (a, b) = match technique {
        Technique::Reverse => (BigInt::from(-1), BigInt::from(num_cards - 1)),
        Technique::Cut(offset) => (BigInt::from(1), BigInt::from(num_cards as i64 + *offset)),
        Technique::Deal(offset) => (
            BigInt::from(modinverse(*offset as i64, num_cards as i64).unwrap()),
            BigInt::from(0),
        ),
    };
    ModularTransformation {
        a: a,
        b: b,
        m: BigInt::from(num_cards),
    }
}

fn compile_reverse_program(num_cards: u64, program: &[Technique]) -> ModularTransformation {
    let mut transformation = ModularTransformation::identity(BigInt::from(num_cards));

    for line in program.iter().rev() {
        transformation = transformation.then(&get_reverse_transformation(num_cards, line));
    }

    transformation
}

fn find_position_after_one_run(num_cards: u64, desired_card: u64, program: &[Technique]) -> u64 {
    let reverse_program = compile_reverse_program(num_cards, &program);
    let card = BigInt::from(desired_card);
    let mut result = 0;

    for i in 0..num_cards {
        if &reverse_program.apply(&BigInt::from(i)) == &card {
            result = i;
            break;
        }
    }

    result
}

fn get_bits(num: u64) -> u32 {
    let mut work = num;
    let mut bit_count = 0u32;

    while work > 0 {
        work /= 2;
        bit_count += 1;
    }

    bit_count
}

fn find_card_after_many_runs(
    num_cards: u64,
    num_runs: u64,
    card_index: u64,
    program: &[Technique],
) -> u64 {
    let mut transform = compile_reverse_program(num_cards, &program);
    let mut mask = 1u64;
    let mut result = BigInt::from(card_index);

    for _ in 0..get_bits(num_runs) {
        if mask & num_runs != 0 {
            result = transform.apply(&result);
        }
        transform = transform.then(&transform);
        mask <<= 1;
    }

    result.to_u64().unwrap()
}

pub fn main() {
    let program: Vec<Technique> = std::fs::read_to_string("data/day22.txt")
        .unwrap()
        .lines()
        .map(parse_technique_line)
        .collect();

    let result0 = find_position_after_one_run(10007, 2019, &program);
    let result1 = find_card_after_many_runs(119315717514047, 101741582076661, 2020, &program);

    println!("{} {}", result0, result1);
}
