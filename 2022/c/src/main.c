#define JBC_MEMORY_COMMIT_BLOCK_SIZE (32ULL << 20)
#define JBC_ALWAYS_SMALL_ENUMS

#include "../jaburns_c/base/inc.c"

#include "day10.c"
#include "day11.c"
#include "day12.c"

#define DAY_NUMBER 12
#define INPUT_TYPE "main"

#define DayFn(x)  Concatenate(day, x)
#define DayStr(x) Stringify(x)

internal void print_result_part(DayResultPart* part) {
    if (part->is_str) {
        printf("%.*s", StrPrintfArgs(part->as_str));
    } else {
        printf("%llu", part->as_u64);
    }
}

internal void print_time(Arena* arena, u64 nanos) {
    char* units = "ns";
    if (nanos > 1000000000) {
        nanos /= 1000000;
        units  = "ms";
    } else if (nanos > 1000000) {
        nanos /= 1000;
        units  = "μs";
    }
    printf("%s %s", u64_print_with_commas(arena, nanos), units);
}

internal void run_day(void) {
    ArenaTemp scratch = scratch_acquire(NULL, 0);
    Str       input   = str_read_file(scratch.arena, "inputs/day" DayStr(DAY_NUMBER) "-" INPUT_TYPE ".txt");

    u64       start  = timing_get_nanos_since_start();
    DayResult result = DayFn(DAY_NUMBER)(scratch.arena, input);
    u64       delta  = timing_get_nanos_since_start() - start;

    printf("\n");
    printf("-- DAY " DayStr(DAY_NUMBER) " --\n");
    printf("   Time: ");
    print_time(scratch.arena, delta);
    printf("\n");
    printf(" Part 1: ");
    print_result_part(&result.parts[0]);
    printf("\n");
    printf(" Part 2: ");
    print_result_part(&result.parts[1]);
    printf("\n\n");
}

i32 main(int argc, char** argv) {
    scratch_thread_local_create(&GLOBAL_ALLOCATOR);
    timing_global_init();

    run_day();

    return 0;
}
