#include "main.h"

internal no_inline DayResult day1_c(Arena* arena, Str input) {
    u8x16 digit_mask = simde_vdupq_n_u8(0x0f);
    u8x16 zero_ascii = simde_vdupq_n_u8('0');

    char* end = input.items + input.count;

    u8x16 temp = simde_vld1q_u8((u8*)input.items);
    simde_vst1q_u8((u8*)end, temp);

    u32 iterations         = input.count / 16 - 1;
    u32 halfway_iterations = input.count / 32;

    u32 part1 = 0;
    u32 part2 = 0;

    u8* walk    = (u8*)input.items;
    u8* halfway = walk + input.count / 2;

#define Accumulate(out_sum, main_vec, other_vec)             \
    do {                                                     \
        u8x16 equal   = simde_vceqq_u8(main_vec, other_vec); \
        u8x16 masked  = simde_vandq_u8(main_vec, equal);     \
        masked        = simde_vandq_u8(masked, digit_mask);  \
        out_sum      += simde_vaddvq_u8(masked);             \
    } while (0)

    for (u32 i = 0; i <= iterations; ++i) {
        if (i == halfway_iterations) halfway -= input.count;

        u8x16 main_vec = simde_vld1q_u8(walk);

        Accumulate(part2, main_vec, simde_vld1q_u8(halfway));
        Accumulate(part1, main_vec, simde_vld1q_u8(walk + 1));

        walk    += 16;
        halfway += 16;
    }

    simde_vst1q_u8((u8*)end, zero_ascii);
    u8x16 end_vec_part2 = simde_vld1q_u8(walk);
    Accumulate(part2, end_vec_part2, simde_vld1q_u8(halfway));

    *end                = input.items[0];
    u8x16 end_vec_part1 = simde_vld1q_u8(walk);
    Accumulate(part1, end_vec_part1, simde_vld1q_u8(walk + 1));

    DayResult result;
    result.parts[0].is_str = false;
    result.parts[0].as_u64 = part1;
    result.parts[1].is_str = false;
    result.parts[1].as_u64 = part2;
    return result;

#undef Accumulate
}