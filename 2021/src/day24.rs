use hashbrown::HashMap;
use std::fmt::Write;

const NUM_DIGITS: usize = 14;

#[derive(Default, Debug, Clone, Copy)]
struct CompiledDigitRoutine {
    z: i64,
}

fn compile_routine_for_all_inputs(lines: &[&str]) -> HashMap<u8, i64> {
    fn get_var_or_imm(vars: &HashMap<&str, i64>, val: &str) -> i64 {
        vars.get(val)
            .cloned()
            .unwrap_or_else(|| val.parse::<i64>().unwrap())
    }

    let mut map = HashMap::default();

    for in_w in 1..=9 {
        let mut vars = HashMap::<&str, i64>::new();
        vars.insert("x", 0);
        vars.insert("y", 0);
        vars.insert("z", 0);

        for line in lines.iter() {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            match parts[0] {
                "inp" => vars.insert(parts[1], in_w),
                "mul" => vars.insert(parts[1], vars[parts[1]] * get_var_or_imm(&vars, parts[2])),
                "add" => vars.insert(parts[1], vars[parts[1]] + get_var_or_imm(&vars, parts[2])),
                "div" => vars.insert(parts[1], vars[parts[1]] / get_var_or_imm(&vars, parts[2])),
                "mod" => vars.insert(parts[1], vars[parts[1]] % get_var_or_imm(&vars, parts[2])),
                "eql" => vars.insert(
                    parts[1],
                    (vars[parts[1]] == get_var_or_imm(&vars, parts[2])) as i64,
                ),
                _ => panic!(),
            };
        }

        map.insert(in_w as u8, vars["z"]);
    }

    map
}

pub fn part1(lines: &[&str], out: &mut String) {
    let mut routines = vec![];

    let mut cur_routine = vec![lines[0]];
    for line in lines.iter().skip(1) {
        println!("line {}", line);
        if line.starts_with("inp") {
            routines.push(compile_routine_for_all_inputs(&cur_routine));
            break;
            cur_routine.clear();
        }
        cur_routine.push(line);
    }
    //routines.push(compile_routine_for_all_inputs(&cur_routine));

    println!("{:#?}", routines);

    write!(out, "{}", 0).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}", result).unwrap();
}

/*
inp w
mul x 0 const
add x z const
mod x 26 const
div z 26 var B
add x -8 var C
eql x w const
eql x 0 const
mul y 0 const
add y 25 const
mul y x const
add y 1 const
mul z y const
mul y 0 const
add y w const
add y 4 var D
mul y x
add z y

digit(z0, w, B, C, D) {
    let x = (z0 % 26 + C) != w
    let z1 = z0 / B * 25 * x + 1 + (w + D) * x
}
if x == 0 {
    z0 % 26 = w - C
    z1 must = 1
}
if x == 1 {
    z0 = (z1 - D - w - 1) / 25 * B
}

inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 4
mul y x
add z y
*/
