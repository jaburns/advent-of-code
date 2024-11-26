.section __TEXT,__text

; ======================================================================================================================
; DayResult day1_asm(Arena* arena, Str input);
; x0 -> Arena*      arena
; x1 -> char*       input.items
; x2 -> size_t      input.count
; x8 -> DayResult*  result
.global _day1_asm
_day1_asm:
    ; v7  : mask of 0F0F0F.. to grab ascii digit values
    ; x15 : pointer to end of input string
        movi v7.16b, #0x0f
        add  x15, x1, x2

    ; pad the end of the input string with the first 16 bytes repeated again
    ; so the part2 halfway-around pointer can sample off the end
        ; v0 : temp
        ld1 {v0.16b}, [x1]
        st1 {v0.16b}, [x15]

    ; figure out the number of iterations
    ; x3  : total iteration count
    ; x10 : iterations until resetting the halfway-around pointer
        lsr x3, x2, #4
        lsr x10, x3, #1
        sub x3, x3, #1

    ; rip over the string, do the comparisons, and add up the matches
    ; x4 : part 1 result
    ; x7 : part 2 result
    ; x6 : main string walking pointer
    ; x9 : halfway-around string walking pointer
    ; v0 : holds 16 bytes dereferenced from x6
        ; v1, v2, x5 : temp
        mov x4, #0
        mov x7, #0
        mov x6, x1
        add x9, x1, x2, lsr #1
        b   1f

        2:  sub x9, x9, x2
        1:  ld1 {v0.16b}, [x6]

            ; part 2 comparison
            ld1  {v2.16b}, [x9]
            cmeq v2.16b, v0.16b, v2.16b
            and  v2.16b, v0.16b, v2.16b
            and  v2.16b, v2.16b, v7.16b
            addv b2, v2.16b
            umov w5, v2.b[0]
            add  x7, x7, x5

            ; part 1 comparison
            add  x6, x6, #1
            ld1  {v1.16b}, [x6]
            cmeq v1.16b, v0.16b, v1.16b
            and  v1.16b, v0.16b, v1.16b
            and  v1.16b, v1.16b, v7.16b
            addv b1, v1.16b
            umov w5, v1.b[0]
            add  x4, x4, x5

            ; move pointers
            add  x6, x6, #15
            add  x9, x9, #16
            subs x10, x10, 1
            beq  2b ; wrap the halfway-around pointer if it's time
            subs x3, x3, 1
            bne  1b

    ; do the last iteration of part 2 (replacing the end with ascii zeroes to not over-count)
        ; v0, v2, x5 : temp
        movi v1.16b, #0x30
        st1  {v1.16b}, [x15]

        ld1  {v0.16b}, [x6]
        ld1  {v2.16b}, [x9]
        cmeq v2.16b, v0.16b, v2.16b
        and  v2.16b, v0.16b, v2.16b
        and  v2.16b, v2.16b, v7.16b
        addv b2, v2.16b
        umov w5, v2.b[0]
        add  x7, x7, x5

    ; do the last iteration of part 1 (put the first char at the end of the input)
        ; v0, v1, x5 : temp
        ldrb w5, [x1]
        str  x5, [x15]

        ld1  {v0.16b}, [x6]
        add  x6, x6, #1
        ld1  {v1.16b}, [x6]
        cmeq v1.16b, v0.16b, v1.16b
        and  v1.16b, v0.16b, v1.16b
        and  v1.16b, v1.16b, v7.16b
        addv b1, v1.16b
        umov w5, v1.b[0]
        add  x4, x4, x5

    ; write result into the returned struct and return
        str xzr, [x8       ]  ; part[0].is_str
        str x4,  [x8, #0x08]  ; part[0].as_u64
        str xzr, [x8, #0x18]  ; part[1].is_str
        str x7,  [x8, #0x20]  ; part[1].as_u64
        ret