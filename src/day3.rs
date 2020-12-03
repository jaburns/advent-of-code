#[derive(Debug, Clone)]
struct Forest {
    width: usize,
    rows: Vec<String>,
}

const TREE_CHAR: char = '#';

impl Forest {
    pub fn new(rows: Vec<String>) -> Self {
        Self {
            width: rows[0].len(),
            rows: rows,
        }
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn tree_at(&self, x: usize, y: usize) -> bool {
        self.rows[y].chars().nth(x % self.width).unwrap() == TREE_CHAR
    }
}

fn find_trees_along_path(forest: &Forest, dx: usize, dy: usize) -> usize {
    let mut trees = 0usize;
    let mut x = 0usize;
    let mut y = 0usize;
    loop {
        x += dx;
        y += dy;
        if y >= forest.height() {
            break;
        }
        if forest.tree_at(x, y) {
            trees += 1;
        }
    }
    trees
}

fn multiply_trees_along_paths(forest: &Forest, paths: &[(usize, usize)]) -> usize {
    paths.iter().fold(1usize, |a, &(dx, dy)| {
        a * find_trees_along_path(forest, dx, dy)
    })
}

pub fn main() {
    let forest = Forest::new(
        std::fs::read_to_string("data/day3.txt")
            .unwrap()
            .lines()
            .map(|x| String::from(x.trim()))
            .collect(),
    );

    let part1 = find_trees_along_path(&forest, 3, 1);
    let part2 = multiply_trees_along_paths(&forest, &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]);

    println!("{} {}", part1, part2);
}
