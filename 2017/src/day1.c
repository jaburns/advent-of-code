#include "main.h"

internal u8 day1_sum_equal_lanes(u8x16 vec0, u8x16 vec1) {
    u8x16 equal  = u8x16_equal(vec0, vec1);
    u8x16 values = u8x16_and(u8x16_and(vec0, equal), u8x16_splat(0x0f));
    return u8x16_sum(values);
}

internal no_inline DayResult day1_c(Arena* arena, Str input) {
    char* end = input.items + input.count;

    u8x16_store((u8*)end, u8x16_load((u8*)input.items));

    u32 iterations         = input.count / 16 - 1;
    u32 halfway_iterations = input.count / 32;

    u32 part1 = 0;
    u32 part2 = 0;

    u8* walk    = (u8*)input.items;
    u8* halfway = walk + input.count / 2;

    for (u32 i = 0; i <= iterations; ++i) {
        if (i == halfway_iterations) halfway -= input.count;

        u8x16 vec_at_walk = u8x16_load(walk);

        part2 += day1_sum_equal_lanes(vec_at_walk, u8x16_load(halfway));
        part1 += day1_sum_equal_lanes(vec_at_walk, u8x16_load(walk + 1));

        walk    += 16;
        halfway += 16;
    }

    u8x16_store((u8*)end, u8x16_splat('0'));
    part2 += day1_sum_equal_lanes(u8x16_load(walk), u8x16_load(halfway));

    *end   = input.items[0];
    part1 += day1_sum_equal_lanes(u8x16_load(walk), u8x16_load(walk + 1));

    DayResult result;
    result.parts[0].is_str = false;
    result.parts[0].as_u64 = part1;
    result.parts[1].is_str = false;
    result.parts[1].as_u64 = part2;
    return result;
}