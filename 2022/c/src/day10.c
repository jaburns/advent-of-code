#include "main.h"

internal DayResult day10(Arena* arena, Str input) {
    i64 reg = 1;

    i64 part1 = 0;
    i64 part1_cycle = 0;
    i64 part1_wrap = 20;

    i64 part2_scan = 0;
    char* part2 = arena_alloc(arena, 512);
    char* part2_write = part2;

    *part2_write++ = '\n';
    *part2_write++ = '\n';

    for (StrSplitIter it = str_split_iter('\n', input); !it.done; str_split_iter_next(&it)) {
        Str item = it.item;
        bool is_add = str_starts_with_cstr("addx", item);

        for (i32 i = 0; i <= is_add; ++i) {
            ++part1_cycle;

            *part2_write++ = part2_scan >= reg - 1 && part2_scan <= reg + 1 ? '#' : '.';

            if (--part1_wrap == 0) {
                part1_wrap = 40;
                part1 += part1_cycle * reg;
            }
            if (++part2_scan == 40) {
                part2_scan = 0;
                *part2_write++ = '\n';
            }
        }

        if (is_add) {
            i32 val = str_atoi(str_substr_from(item, 5));
            reg += val;
        }
    }

    return (DayResult){
        (DayResultPart){
            .is_str = false,
            .as_i64 = part1
        },
        (DayResultPart){
            .is_str = true,
            .as_str = (Str){.items = part2, .count = part2_write - part2}
        },
    };
}