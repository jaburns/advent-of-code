#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

#DEBUG_BUILD=1

WARNINGS='
    -Wall
    -Werror
    -Wswitch-enum
    -Wvla
    -Wno-unused-function
    -Wno-logical-op-parentheses
    -Wno-unused-variable
    -Wno-unused-but-set-variable
'

if [[ -n "$DEBUG_BUILD" ]]; then
    OPTS='
        -g 
        -O0 
        -DDEBUG=1 
        -fsanitize=undefined,unsigned-integer-overflow 
        -fno-omit-frame-pointer
    '
else
    OPTS='-O3'
fi


mkdir -p bin

clang -std=c11 src/day20171.s src/main.c $WARNINGS $OPTS -o bin/aoc2024