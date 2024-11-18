#include "main.h"

structdef(Day12Cell) {
    u8   height;
    bool left, right, up, down;
};

internal DayResult day12(Arena* arena, Str input) {
    return (DayResult){
        (DayResultPart){
            .is_str = false,
            .as_u64 = 0
        },
        (DayResultPart){
            .is_str = false,
            .as_u64 = 0
        },
    };
}