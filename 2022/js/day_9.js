let start_time = Date.now();

#include "day_9.input"

#define TRUE  1
#define FALSE 0

#define ARENA_SIZE (96 << 10)

#define ARENA_PUSH(n) arena_p ; { arena_p += (n); if (arena_p >= ARENA_SIZE) throw new Error("out of mem");  }
let arena_u32 = new Uint32Array(ARENA_SIZE);
let arena_i32 = new Int32Array(arena_u32.buffer);
let arena_p = 1;

#define Cell_SIZE         6
#define Cell_full(p)      arena_u32[ (p)     ]
#define Cell_next(p)      arena_u32[ (p) + 1 ]
#define Cell_x(p)         arena_i32[ (p) + 2 ]
#define Cell_y(p)         arena_i32[ (p) + 3 ]
#define Cell_visited1(p)  arena_u32[ (p) + 4 ]
#define Cell_visited9(p)  arena_u32[ (p) + 5 ]

#define HASHMAP_SIZE 4096

let hashmap_cells_ = ARENA_PUSH(Cell_SIZE * HASHMAP_SIZE);
#define hashmap_cells(i) ( hashmap_cells_ + Cell_SIZE*(i) )

function hashmap_insert(x, y) {
    // djb2 hash
    let hash = 5381;
    hash = ((hash << 5) + hash) + (x >>> 0);
    hash = ((hash << 5) + hash) + (y >>> 0);
    if (hash === 0) hash = 1;

    let idx = hash & (HASHMAP_SIZE - 1);
    let cell_ptr = hashmap_cells(idx);

    for (;;) {
        if (!Cell_full(cell_ptr)) {
            Cell_full(cell_ptr) = TRUE;
            Cell_x(cell_ptr) = x;
            Cell_y(cell_ptr) = y;
            return cell_ptr;
        }

        let cx = Cell_x(cell_ptr);
        let cy = Cell_y(cell_ptr);
        if (cx === x && cy === y) {
            return cell_ptr;
        }

        let next = Cell_next(cell_ptr);
        if (next === 0) {
            let new_node_ptr = ARENA_PUSH(Cell_SIZE);
            Cell_next(cell_ptr) = new_node_ptr;
            cell_ptr = new_node_ptr;
        } else {
            cell_ptr = next;
        }
    }
}

#define Knot_SIZE  2
#define Knot_x(p)  arena_i32[ (p)     ]
#define Knot_y(p)  arena_i32[ (p) + 1 ]

#define KNOT_COUNT 10
let knots_ = ARENA_PUSH(Knot_SIZE * KNOT_COUNT)
#define knots(i) ( knots_ + Knot_SIZE*(i) )

#define UP    0x55
#define DOWN  0x44
#define LEFT  0x4C
#define RIGHT 0x52

function knot_follow(head, tail) {
    let moved = false;
    if      (Knot_y(tail) < Knot_y(head) - 1) { ++Knot_y(tail); moved = true; }
    else if (Knot_y(tail) > Knot_y(head) + 1) { --Knot_y(tail); moved = true; }

    if (moved) {
        if      (Knot_x(tail) < Knot_x(head)) ++Knot_x(tail);
        else if (Knot_x(tail) > Knot_x(head)) --Knot_x(tail);
        return true;
    }

    moved = false;
    if      (Knot_x(tail) < Knot_x(head) - 1) { ++Knot_x(tail); moved = true; }
    else if (Knot_x(tail) > Knot_x(head) + 1) { --Knot_x(tail); moved = true; }

    if (moved) {
        if      (Knot_y(tail) < Knot_y(head)) ++Knot_y(tail);
        else if (Knot_y(tail) > Knot_y(head)) --Knot_y(tail);
        return true;
    }

    return false;
}

function pull_knots() {
    let j0 = knots(0);
    let j1 = knots(1);
    if (!knot_follow(j0, j1)) return;
    let cell = hashmap_insert(Knot_x(j1), Knot_y(j1));
    Cell_visited1(cell) = TRUE;

    for (let j = 1; j < KNOT_COUNT - 2; ++j) {
        j0 = knots(j);
        j1 = knots(j + 1);
        if (!knot_follow(j0, j1)) return;
        hashmap_insert(Knot_x(j1), Knot_y(j1));
    }

    j0 = knots(KNOT_COUNT - 2);
    j1 = knots(KNOT_COUNT - 1);
    if (!knot_follow(j0, j1)) return;
    cell = hashmap_insert(Knot_x(j1), Knot_y(j1));
    Cell_visited9(cell) = TRUE;
}

let head_x = 0, head_y = 0;
let tail_x = 0, tail_y = 0;
let read = 0, newline = 0;

let start_cell = hashmap_insert(0, 0);
Cell_visited1(start_cell) = TRUE;
Cell_visited9(start_cell) = TRUE;

while (newline < input.length) {
    let dir, count; {
        dir = input.charCodeAt(read);
        read += 2;
        newline = input.indexOf('\n', read) >>> 0;
        count = parseInt(input.substring(read, newline));
        read = newline + 1;
    }

    switch (dir) {
        case UP: {
            for (let i = 0; i < count; ++i) {
                ++Knot_y(knots(0));
                pull_knots();
            }
            break;
        }
        case DOWN: {
            for (let i = 0; i < count; ++i) {
                --Knot_y(knots(0));
                pull_knots();
            }
            break;
        }
        case RIGHT: {
            for (let i = 0; i < count; ++i) {
                ++Knot_x(knots(0));
                pull_knots();
            }
            break;
        }
        case LEFT: {
            for (let i = 0; i < count; ++i) {
                --Knot_x(knots(0));
                pull_knots();
            }
            break;
        }
    }
}

let part1 = 0, part2 = 0;

for (let i = 0; i < HASHMAP_SIZE; ++i) {
    let cell_ptr = hashmap_cells(i);
    part1 += Cell_visited1(cell_ptr);
    part2 += Cell_visited9(cell_ptr);
    let next = cell_ptr;
    while (next = Cell_next(next)) {
        part1 += Cell_visited1(next);
        part2 += Cell_visited9(next);
    }
}

console.log(`Time: ${Date.now() - start_time} ms`);
console.log(`Part 1: ${part1}   Part 2: ${part2}`)
