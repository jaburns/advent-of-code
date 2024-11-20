#include "main.h"

enumdef(Day12Dir, u8){
    D12_UP, D12_DOWN, D12_LEFT, D12_RIGHT
};
readonly_global ivec2 D12_DIRECTIONS[4] = {
    IVEC2_DOWN, IVEC2_UP, IVEC2_LEFT, IVEC2_RIGHT
};

structdef(Day12Cell) {
    bool can_go[4];
    bool in_open_set;
    u8   height;

    ivec2 coord;
    u32   g_score;
    u32   f_score;
    ivec2 came_from;
};
typedef Day12Cell* Day12Cell_ptr;
DefArrayTypes(Day12Cell_ptr);

internal DayResult day12(Arena* arena, Str input) {
    Day12Cell* cells = arena_alloc(arena, Kb(8) * sizeof(Day12Cell));

    ivec2 start = {0}, end = {0};

    u32   width = 0;
    ivec2 read  = {0};
    for (u32 char_idx = 0; char_idx < input.count; ++char_idx) {
        char ch = input.items[char_idx];

        if (ch == '\n') {
            width  = read.x;
            read.x = 0;
            ++read.y;
            continue;
        }

        Day12Cell* cell = &cells[read.x + width * read.y];
        cell->coord     = read;
        cell->came_from = (ivec2){-1, -1};
        cell->g_score   = UINT32_MAX;

        if (ch == 'S') {
            start         = read;
            cell->g_score = 0;
        } else if (ch == 'E') {
            end          = read;
            cell->height = 'z' - 'a';
        } else {
            cell->height = ch - 'a';
        }

        ++read.x;
    }
    u32 height = read.y + 1;

    for (u32 y = 0; y < height; ++y) {
        u32 yoff = width * y;
        for (u32 x = 0; x < width; ++x) {
            Day12Cell* cell         = &cells[x + yoff];
            cell->can_go[D12_LEFT]  = x > 0 && cells[x - 1 + yoff].height <= cell->height + 1;
            cell->can_go[D12_UP]    = y > 0 && cells[x + yoff - width].height <= cell->height + 1;
            cell->can_go[D12_RIGHT] = x < width - 1 && cells[x + 1 + yoff].height <= cell->height + 1;
            cell->can_go[D12_DOWN]  = y < height - 1 && cells[x + yoff + width].height <= cell->height + 1;
        }
    }

#define CoordToIdx(v) (v.x + width * v.y)

    Vec_Day12Cell_ptr open_set = VecAlloc(Day12Cell_ptr, arena, 512);

#define OpenSetPush(x)    VecMinHeapPush(Day12Cell_ptr, open_set, ->f_score, (x))
#define OpenSetPopInto(x) VecMinHeapPopInto(Day12Cell_ptr, open_set, ->f_score, (x))
#define Heuristic(x)      ivec2_manhattan(ivec2_sub((x), end));

    u32 start_idx = CoordToIdx(start);
    u32 end_idx   = CoordToIdx(end);

    cells[start_idx].in_open_set = true;
    OpenSetPush(&cells[start_idx]);

    cells[start_idx].f_score = Heuristic(start);

    while (open_set.count > 0) {
        Day12Cell* cell;
        OpenSetPopInto(cell);
        cell->in_open_set = false;

        if (ivec2_eq(cell->coord, end)) {
            goto success;
        }

        for (u32 i = 0; i < 4; ++i) {
            if (!cell->can_go[i]) continue;

            ivec2      step     = D12_DIRECTIONS[i];
            ivec2      ncoord   = ivec2_add(cell->coord, step);
            Day12Cell* neighbor = &cells[CoordToIdx(ncoord)];

            u32 tentative_g = cell->g_score + 1;
            if (tentative_g < neighbor->g_score) {
                neighbor->came_from = cell->coord;
                neighbor->g_score   = tentative_g;
                neighbor->f_score   = tentative_g + Heuristic(ncoord);

                if (!neighbor->in_open_set) {
                    neighbor->in_open_set = true;
                    OpenSetPush(neighbor);
                }
            }
        }
    }

    Panic("Failed");

success: {}

    u32        count     = 0;
    Day12Cell* path_cell = &cells[end_idx];
    while (path_cell->came_from.x >= 0) {
        ++count;
        path_cell = &cells[CoordToIdx(path_cell->came_from)];
    }

    return (DayResult){
        (DayResultPart){
            .is_str = false,
            .as_u64 = count,
        },
        (DayResultPart){
            .is_str = false,
            .as_u64 = 0,
        },
    };

#undef CoordToIdx
#undef OpenSetPush
#undef OpenSetPopInto
}