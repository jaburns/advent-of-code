let start_time = Date.now();

#define ARENA_PUSH_U8(val) { arena_u8[arena_p++] = val; }
let arena_u8 = new Uint8Array(16 << 10);
let arena_p = 0;

#include "day_8.input"

let grid_p = arena_p;

let width = 0, height = 1;
for (width = 0; input[width] != '\n'; ++width);

for (let i = 0; i < input.length; ++i) {
    let code = input.charCodeAt(i);
    if (code === 0x0A) ++height;
    else ARENA_PUSH_U8(code - 0x30);
}

#define GRID(x, y) arena_u8[grid_p + width * y + x]

let part1 = 0;
let part2 = 0;

for (let x = 0; x < width; ++x) {
    for (let y = 0; y < height; ++y) {
        let this_height = GRID(x, y);
        let hits = 0;
        let a = 0, b = 0, c = 0, d = 0;

        for (let j = x - 1; j >= 0; --j) {
            ++a;
            if (GRID(j, y) >= this_height) { hits++; break; }
        }

        for (let j = x + 1; j < width; ++j) {
            ++b;
            if (GRID(j, y) >= this_height) { hits++; break; }
        }

        for (let j = y - 1; j >= 0; --j) {
            ++c;
            if (GRID(x, j) >= this_height) { hits++; break; }
        }

        for (let j = y + 1; j < height; ++j) {
            ++d;
            if (GRID(x, j) >= this_height) { hits++; break; }
        }

        part2 = Math.max(part2, a * b * c * d);
        if (hits < 4) part1++;
    }
}

console.log(`Time: ${Date.now() - start_time} ms`);
console.log(`Part 1: ${part1}   Part 2: ${part2}`)
