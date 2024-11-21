#include "main.h"

enumdef(Day12Flag, u8){
    D12_FLAG_UP    = 1,
    D12_FLAG_DOWN  = 2,
    D12_FLAG_LEFT  = 4,
    D12_FLAG_RIGHT = 8,
};

structdef(Day12Cell) {
    Day12Flag flags;
    u8        elevation;
    u16       idx;
    u16       distance;
};
typedef Day12Cell* Day12Cell_ptr;
DefArrayTypes(Day12Cell_ptr);

#define D12_GRID_WIDTH 256

internal DayResult day12(Arena* arena, Str input) {
    Day12Cell* cells = arena_alloc_nz(arena, Kb(16) * sizeof(Day12Cell));

    // --- parsing ---

    ivec2 start = {0};
    ivec2 end   = {0};
    u16   width = 0;
    ivec2 read  = {0};

    for (u16 char_idx = 0; char_idx < input.count; ++char_idx) {
        char ch = input.items[char_idx];

        if (ch == '\n') {
            width  = read.x;
            read.x = 0;
            ++read.y;
            continue;
        }

        u16        idx  = read.x + D12_GRID_WIDTH * read.y;
        Day12Cell* cell = &cells[idx];
        cell->flags     = 0;
        cell->idx       = idx;
        cell->distance  = 0;

        if (ch == 'S') {
            start           = read;
            cell->elevation = 0;
        } else if (ch == 'E') {
            end             = read;
            cell->elevation = 'z' - 'a';
        } else {
            cell->elevation = ch - 'a';
        }

        ++read.x;
    }

    u16 height = read.y + 1;

    for (u16 y = 0; y < height; ++y) {
        u16 yoff = D12_GRID_WIDTH * y;
        for (u16 x = 0; x < width; ++x) {
            Day12Cell* cell = &cells[x + yoff];
            // clang-format off
            cell->flags |= (x > 0          && cells[ x - 1 + yoff                  ].elevation >= cell->elevation - 1) * D12_FLAG_LEFT  ;
            cell->flags |= (y > 0          && cells[ x     + yoff - D12_GRID_WIDTH ].elevation >= cell->elevation - 1) * D12_FLAG_UP    ;
            cell->flags |= (x < width - 1  && cells[ x + 1 + yoff                  ].elevation >= cell->elevation - 1) * D12_FLAG_RIGHT ;
            cell->flags |= (y < height - 1 && cells[ x     + yoff + D12_GRID_WIDTH ].elevation >= cell->elevation - 1) * D12_FLAG_DOWN  ;
            // clang-format on
        }
    }

    // --- breadth-first search ---

    u16 part1 = 0;
    u16 part2 = 0;

    Vec_Day12Cell_ptr open      = VecAllocNZ(Day12Cell_ptr, arena, 512);
    Vec_Day12Cell_ptr open_next = VecAllocNZ(Day12Cell_ptr, arena, 512);

    u16 start_idx  = start.x + D12_GRID_WIDTH * start.y;
    *VecPush(open) = &cells[end.x + D12_GRID_WIDTH * end.y];

    for (;;) {
        while (open.count) {
            Day12Cell* cell = *VecPop(open);

            if (part2 == 0 && cell->elevation == 0) {
                part2 = cell->distance;
            }
            if (cell->idx == start_idx) {
                part1 = cell->distance;
                goto end;
            }

            if (cell->flags & D12_FLAG_LEFT) {
                Day12Cell* neighbor = &cells[cell->idx - 1];
                if (!neighbor->distance) {
                    neighbor->distance  = cell->distance + 1;
                    *VecPush(open_next) = neighbor;
                }
            }
            if (cell->flags & D12_FLAG_RIGHT) {
                Day12Cell* neighbor = &cells[cell->idx + 1];
                if (!neighbor->distance) {
                    neighbor->distance  = cell->distance + 1;
                    *VecPush(open_next) = neighbor;
                }
            }
            if (cell->flags & D12_FLAG_UP) {
                Day12Cell* neighbor = &cells[cell->idx - D12_GRID_WIDTH];
                if (!neighbor->distance) {
                    neighbor->distance  = cell->distance + 1;
                    *VecPush(open_next) = neighbor;
                }
            }
            if (cell->flags & D12_FLAG_DOWN) {
                Day12Cell* neighbor = &cells[cell->idx + D12_GRID_WIDTH];
                if (!neighbor->distance) {
                    neighbor->distance  = cell->distance + 1;
                    *VecPush(open_next) = neighbor;
                }
            }
        }

        if (!open_next.count) {
            Panic("Part 2 search failed");
        }

        Swap(Vec_Day12Cell_ptr, open, open_next);
    }

    // ---

end:
    return (DayResult){
        (DayResultPart){
            .is_str = false,
            .as_u64 = part1,
        },
        (DayResultPart){
            .is_str = false,
            .as_u64 = part2,
        },
    };
}

// ------------------------------------------------------------------------------------------------------------------
// --- original part 1 :: A* search from start to end ---
// this ends up being slower than just brute-force breadth-first searching.
// the heap management dominates the runtime
//
// #define D12_GRID_WIDTH_BITS 8
// #define D12_GRID_WIDTH      (1 << D12_GRID_WIDTH_BITS)
// #define D12_GRID_WIDTH_MASK (D12_GRID_WIDTH - 1)
// u16 part1 = 0;
//     {
//         Vec_Day12Cell_ptr open_set = VecAllocNZ(Day12Cell_ptr, arena, 512);
//
// #define OpenSetPush(x)    VecMinHeapPush(Day12Cell_ptr, open_set, ->f_score, (x))
// #define OpenSetPopInto(x) VecMinHeapPopInto(Day12Cell_ptr, open_set, ->f_score, (x))
// #define Heuristic(x)      ivec2_manhattan(ivec2_sub((x), end));
//
//         cells[start_idx].flags |= D12_FLAG_IN_OPEN_SET;
//         OpenSetPush(&cells[start_idx]);
//
//         cells[start_idx].f_score = Heuristic(start);
//
//         while (open_set.count) {
//             Day12Cell* cell;
//             OpenSetPopInto(cell);
//             cell->flags &= ~D12_FLAG_IN_OPEN_SET;
//
//             if (cell->idx == end_idx) {
//                 goto success1;
//             }
//
//             for (u16 i = 0; i < 4; ++i) {
//                 if (!(cell->flags & (1 << i))) continue;
//
//                 u16 nidx;
//                 switch (i) {
//                     case 0: {
//                         nidx = cell->idx - D12_GRID_WIDTH;
//                         break;
//                     }
//                     case 1: {
//                         nidx = cell->idx + D12_GRID_WIDTH;
//                         break;
//                     }
//                     case 2: {
//                         nidx = cell->idx - 1;
//                         break;
//                     }
//                     case 3: {
//                         nidx = cell->idx + 1;
//                         break;
//                     }
//                 }
//
//                 Day12Cell* neighbor    = &cells[nidx];
//                 u16        tentative_g = cell->g_score + 1;
//                 if (tentative_g < neighbor->g_score) {
//                     ivec2 ncoord = (ivec2){nidx & D12_GRID_WIDTH_MASK, nidx >> D12_GRID_WIDTH_BITS};
//
//                     neighbor->came_from_idx = cell->idx;
//                     neighbor->g_score       = tentative_g;
//                     neighbor->f_score       = tentative_g + Heuristic(ncoord);
//
//                     if (!(neighbor->flags & D12_FLAG_IN_OPEN_SET)) {
//                         neighbor->flags |= D12_FLAG_IN_OPEN_SET;
//                         OpenSetPush(neighbor);
//                     }
//                 }
//             }
//         }
//
//         Panic("Part 1 search failed");
//
//     success1: {}
//
//         Day12Cell* path_cell = &cells[end_idx];
//         while (path_cell->came_from_idx < UINT16_MAX) {
//             ++part1;
//             path_cell = &cells[path_cell->came_from_idx];
//         }
//
// #undef OpenSetPush
// #undef OpenSetPopInto
// #undef Heuristic
//     }