#include "main.h"

enumdef(Day11MonkeyOperation, u8){
    DAY11_MONKEY_OP_ADD,
    DAY11_MONKEY_OP_MULTIPLY,
    DAY11_MONKEY_OP_SQUARE,
};

structdef(Day11Monkey) {
    u32 inspections;
    Day11MonkeyOperation operation;
    u8 true_target;
    u8 false_target;
    u32 operand;
    u32 test_divisor;
    StaticVec(u64, 64) items;
};
DefArrayTypes(Day11Monkey);

internal u64 day11_monkey_eval_item(Day11Monkey* monkey, u32 idx) {
    u64 worry = monkey->items.items[idx];

    switch (monkey->operation) {
        case DAY11_MONKEY_OP_ADD: {
            worry += monkey->operand;
            break;
        }
        case DAY11_MONKEY_OP_MULTIPLY: {
            worry *= monkey->operand;
            break;
        }
        case DAY11_MONKEY_OP_SQUARE: {
            worry *= worry;
            break;
        }
    }

    return worry;
}

internal DayResult day11(Arena* arena, Str input) {
    ArenaArray proto_monkeys = arena_array_begin(arena, sizeof(Day11Monkey));

    Day11Monkey monkey = {0};
    u32 monkey_parse_line = 0;
    for (StrSplitIter it = str_split_iter('\n', input); !it.done; ++monkey_parse_line, str_split_iter_next(&it)) {
        if (monkey_parse_line == 7) monkey_parse_line = 0;

        switch (monkey_parse_line) {
            case 1: {
                Str item_list = str_after_last_index(':', it.item);
                for (StrSplitIter item_it = str_split_iter(',', item_list); !item_it.done; str_split_iter_next(&item_it)) {
                    i32 item = str_atoi(str_trim(item_it.item));
                    *StaticVecPush(monkey.items) = item;
                }
                break;
            }
            case 2: {
                Str full_op = str_substr_from(it.item, CstrLen("  Operation: new = old "));
                char op = full_op.items[0];
                Str operand = str_substr_from(full_op, 2);

                if (op == '+') {
                    monkey.operation = DAY11_MONKEY_OP_ADD;
                    monkey.operand = str_atoi(operand);
                } else {
                    if (operand.items[0] == 'o') {
                        monkey.operation = DAY11_MONKEY_OP_SQUARE;
                    } else {
                        monkey.operation = DAY11_MONKEY_OP_MULTIPLY;
                        monkey.operand = str_atoi(operand);
                    }
                }
                break;
            }
            case 3: {
                Str divisor = str_substr_from(it.item, CstrLen("  Test: divisible by "));
                monkey.test_divisor = str_atoi(divisor);
                break;
            }
            case 4: {
                Str target = str_substr_from(it.item, CstrLen("    If true: throw to monkey "));
                monkey.true_target = str_atoi(target);
                break;
            }
            case 5: {
                Str target = str_substr_from(it.item, CstrLen("    If false: throw to monkey "));
                monkey.false_target = str_atoi(target);

                *(Day11Monkey*)arena_array_push(&proto_monkeys) = monkey;
                ZeroStruct(&monkey);
                break;
            }
        }
    }

    Slice_Day11Monkey monkeys = {
        .items = arena_alloc(arena, sizeof(Day11Monkey) * proto_monkeys.count),
        .count = proto_monkeys.count,
    };
    SliceCopy(monkeys.items, proto_monkeys.items, 0, proto_monkeys.count);

    for (u32 round = 0; round < 20; ++round) {
        for (u32 i = 0; i < monkeys.count; ++i) {
            Day11Monkey* monkey = &monkeys.items[i];

            for (u32 j = 0; j < monkey->items.count; ++j) {
                u64 worry = day11_monkey_eval_item(monkey, j);

                worry /= 3;

                u8 target_idx = worry % monkey->test_divisor == 0 ? monkey->true_target : monkey->false_target;
                *StaticVecPush(monkeys.items[target_idx].items) = worry;
            }

            monkey->inspections += monkey->items.count;
            monkey->items.count = 0;
        }
    }

    SliceSort(monkeys, sort_fn_i32_desc);
    u64 part1 = monkeys.items[0].inspections * monkeys.items[1].inspections;

    monkeys.items = proto_monkeys.items;

    u64 common_multiple = 1;
    for (u32 i = 0; i < monkeys.count; ++i) {
        common_multiple *= monkeys.items[i].test_divisor;
    }

    for (u32 round = 0; round < 10000; ++round) {
        for (u32 i = 0; i < monkeys.count; ++i) {
            Day11Monkey* monkey = &monkeys.items[i];

            for (u32 j = 0; j < monkey->items.count; ++j) {
                u64 worry = day11_monkey_eval_item(monkey, j);

                worry %= common_multiple;

                u8 target_idx = worry % monkey->test_divisor == 0 ? monkey->true_target : monkey->false_target;
                *StaticVecPush(monkeys.items[target_idx].items) = worry;
            }

            monkey->inspections += monkey->items.count;
            monkey->items.count = 0;
        }
    }

    SliceSort(monkeys, sort_fn_i32_desc);
    u64 part2 = (u64)monkeys.items[0].inspections * (u64)monkeys.items[1].inspections;

    return (DayResult){
        (DayResultPart){
            .is_str = false,
            .as_i64 = part1,
        },
        (DayResultPart){
            .is_str = false,
            .as_i64 = part2,
        },
    };
}