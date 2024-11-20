#include "main.h"

enumdef(Day12Flag, u8){
    D12_FLAG_UP          = 1,
    D12_FLAG_DOWN        = 2,
    D12_FLAG_LEFT        = 4,
    D12_FLAG_RIGHT       = 8,
    D12_FLAG_IN_OPEN_SET = 16,
};

structdef(Day12Cell) {
    Day12Flag flags;
    u8        elevation;

    u16 idx;
    u16 came_from_idx;
    u16 g_score;
    u16 f_score;

    u16 part2_distance;
};
typedef Day12Cell* Day12Cell_ptr;
DefArrayTypes(Day12Cell_ptr);

#define D12_GRID_WIDTH_BITS 8
#define D12_GRID_WIDTH      (1 << D12_GRID_WIDTH_BITS)
#define D12_GRID_WIDTH_MASK (D12_GRID_WIDTH - 1)

internal DayResult day12(Arena* arena, Str input) {
    Day12Cell* cells = arena_alloc_nz(arena, Kb(16) * sizeof(Day12Cell));

    // --- parsing ---

    ivec2 start = {0}, end = {0};

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

        u16        idx      = read.x + D12_GRID_WIDTH * read.y;
        Day12Cell* cell     = &cells[idx];
        cell->idx           = idx;
        cell->came_from_idx = UINT16_MAX;
        cell->g_score       = UINT16_MAX;

        if (ch == 'S') {
            start         = read;
            cell->g_score = 0;
        } else if (ch == 'E') {
            end                  = read;
            cell->elevation      = 'z' - 'a';
            cell->part2_distance = 1;
        } else {
            cell->elevation = ch - 'a';
        }

        ++read.x;
    }
    u16 height = read.y + 1;

    for (u16 y = 0; y < height; ++y) {
        u16 yoff = D12_GRID_WIDTH * y;
        for (u16 x = 0; x < width; ++x) {
            Day12Cell* cell  = &cells[x + yoff];
            cell->flags     |= (x > 0 && cells[x - 1 + yoff].elevation <= cell->elevation + 1) ? D12_FLAG_LEFT : 0;
            cell->flags     |= (y > 0 && cells[x + yoff - D12_GRID_WIDTH].elevation <= cell->elevation + 1) ? D12_FLAG_UP : 0;
            cell->flags     |= (x < width - 1 && cells[x + 1 + yoff].elevation <= cell->elevation + 1) ? D12_FLAG_RIGHT : 0;
            cell->flags     |= (y < height - 1 && cells[x + yoff + D12_GRID_WIDTH].elevation <= cell->elevation + 1) ? D12_FLAG_DOWN : 0;
        }
    }

    u16 start_idx = start.x + D12_GRID_WIDTH * start.y;
    u16 end_idx   = end.x + D12_GRID_WIDTH * end.y;

    ArenaMark part1_mark = arena_mark(arena);

    // --- part 1, A* search from start to end ---

    u16 part1 = 0;
    {
        Vec_Day12Cell_ptr open_set = VecAllocNZ(Day12Cell_ptr, arena, 512);

#define OpenSetPush(x)    VecMinHeapPush(Day12Cell_ptr, open_set, ->f_score, (x))
#define OpenSetPopInto(x) VecMinHeapPopInto(Day12Cell_ptr, open_set, ->f_score, (x))
#define Heuristic(x)      ivec2_manhattan(ivec2_sub((x), end));

        cells[start_idx].flags |= D12_FLAG_IN_OPEN_SET;
        OpenSetPush(&cells[start_idx]);

        cells[start_idx].f_score = Heuristic(start);

        while (open_set.count) {
            Day12Cell* cell;
            OpenSetPopInto(cell);
            cell->flags &= ~D12_FLAG_IN_OPEN_SET;

            if (cell->idx == end_idx) {
                goto success1;
            }

            for (u16 i = 0; i < 4; ++i) {
                if (!(cell->flags & (1 << i))) continue;

                u16 nidx;
                switch (i) {
                    case 0: {
                        nidx = cell->idx - D12_GRID_WIDTH;
                        break;
                    }
                    case 1: {
                        nidx = cell->idx + D12_GRID_WIDTH;
                        break;
                    }
                    case 2: {
                        nidx = cell->idx - 1;
                        break;
                    }
                    case 3: {
                        nidx = cell->idx + 1;
                        break;
                    }
                }

                Day12Cell* neighbor    = &cells[nidx];
                u16        tentative_g = cell->g_score + 1;
                if (tentative_g < neighbor->g_score) {
                    ivec2 ncoord = (ivec2){nidx & D12_GRID_WIDTH_MASK, nidx >> D12_GRID_WIDTH_BITS};

                    neighbor->came_from_idx = cell->idx;
                    neighbor->g_score       = tentative_g;
                    neighbor->f_score       = tentative_g + Heuristic(ncoord);

                    if (!(neighbor->flags & D12_FLAG_IN_OPEN_SET)) {
                        neighbor->flags |= D12_FLAG_IN_OPEN_SET;
                        OpenSetPush(neighbor);
                    }
                }
            }
        }

        Panic("Part 1 search failed");

    success1: {}

        Day12Cell* path_cell = &cells[end_idx];
        while (path_cell->came_from_idx < UINT16_MAX) {
            ++part1;
            path_cell = &cells[path_cell->came_from_idx];
        }

#undef OpenSetPush
#undef OpenSetPopInto
#undef Heuristic
    }

    arena_restore(arena, part1_mark);

    // --- part 2, breadth first search from end to elevation 0 ---

    u16 part2 = 0;
    {
        Vec_Day12Cell_ptr open      = VecAllocNZ(Day12Cell_ptr, arena, 512);
        Vec_Day12Cell_ptr open_next = VecAllocNZ(Day12Cell_ptr, arena, 512);

        *VecPush(open) = &cells[end_idx];

        while (open.count) {
            while (open.count) {
                Day12Cell* cell  = *VecPop(open);
                ivec2      coord = (ivec2){cell->idx & D12_GRID_WIDTH_MASK, cell->idx >> D12_GRID_WIDTH_BITS};

                if (cell->elevation == 0) {
                    part2 = cell->part2_distance - 1;
                    goto success2;
                }

                if (coord.x > 0) {
                    Day12Cell* neighbor = &cells[cell->idx - 1];
                    if (!neighbor->part2_distance && (neighbor->flags & D12_FLAG_RIGHT)) {
                        neighbor->part2_distance = cell->part2_distance + 1;
                        neighbor->came_from_idx  = cell->idx;
                        *VecPush(open_next)      = neighbor;
                    }
                }
                if (coord.x < width - 1) {
                    Day12Cell* neighbor = &cells[cell->idx + 1];
                    if (!neighbor->part2_distance && (neighbor->flags & D12_FLAG_LEFT)) {
                        neighbor->part2_distance = cell->part2_distance + 1;
                        neighbor->came_from_idx  = cell->idx;
                        *VecPush(open_next)      = neighbor;
                    }
                }
                if (coord.y > 0) {
                    Day12Cell* neighbor = &cells[cell->idx - D12_GRID_WIDTH];
                    if (!neighbor->part2_distance && (neighbor->flags & D12_FLAG_DOWN)) {
                        neighbor->part2_distance = cell->part2_distance + 1;
                        neighbor->came_from_idx  = cell->idx;
                        *VecPush(open_next)      = neighbor;
                    }
                }
                if (coord.y < height - 1) {
                    Day12Cell* neighbor = &cells[cell->idx + D12_GRID_WIDTH];
                    if (!neighbor->part2_distance && (neighbor->flags & D12_FLAG_UP)) {
                        neighbor->part2_distance = cell->part2_distance + 1;
                        neighbor->came_from_idx  = cell->idx;
                        *VecPush(open_next)      = neighbor;
                    }
                }
            }

            Swap(Vec_Day12Cell_ptr, open, open_next);
        }

        Panic("Part 2 search failed");

    success2: {}
    }

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