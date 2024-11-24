.section __TEXT,__text

; ======================================================================================================================
; DayResult day20171(Arena* arena, Str input);
; x0 -> Arena*      arena
; x1 -> char*       input.items
; x2 -> size_t      input.count
; x8 -> DayResult*  result
.global _day20171
_day20171:
    ; b old_day20171

    ; preamble
        stp x29, x30, [sp, #-16]!
        mov x29, sp

    ; --- setup ----------------------------------------------------------
    ; v7 <- mask of 0F0F0F.. to grab ascii digit values

        movi v7.16b, #0x0f

    ; --- part 1 ---------------------------------------------------------

    ; pad the end of the input string with one repeat of the first character, then 16 ascii zeroes
        ; x3 <- first char
        ; x4 <- end of string
        ; v0 <- ascii zeros
        ldrb w3, [x1]
        add  x4, x1, x2
        str  x3, [x4]
        add  x4, x4, 1
        movi v0.16b, #0x30
        st1 {v0.16b}, [x4]

    ; figure out the number of iterations
    ; x3 <- iteration count
        lsr x3, x2, #4
        add x3, x3, #1

    ; rip over the string
    ; x4 <- result
        ; x6     <- string walking pointer
        ; v0, v1, x5 <- temp
        mov  x4, #0
        mov  x6, x1
        1:  ld1  {v0.16b}, [x6]
            add  x6, x6, #1
            ld1  {v1.16b}, [x6]
            cmeq v1.16b, v0.16b, v1.16b
            and  v1.16b, v0.16b, v1.16b
            and  v1.16b, v1.16b, v7.16b
            addv b1, v1.16b
            umov w5, v1.b[0]
            add  x4, x4, x5
            add  x6, x6, #15
            subs x3, x3, 1
            bne  1b

    ; --- part 2 ---------------------------------------------------------
    ; x4 <- carrying part 1 result forwards

    ; pad the end of the input string with the first 16 bytes repeated again
        ; x3 <- end of string
        ; v0 <- temp
        ld1  {v0.16b}, [x1]
        add  x3, x1, x2
        st1  {v0.16b}, [x3]

    ; figure out the number of iterations
    ; x3  <- iteration count
    ; x10 <- iterations until resetting the halfway-around pointer
        lsr x3, x2, #4
        lsr x10, x3, #1
        sub x3, x3, #1

    ; rip over the string
    ; x7 <- result
    ; x6 <- main string walking pointer
    ; x9 <- halfway-around string walking pointer
        ; v0, v1, x5 <- temp
        mov  x7, #0
        add  x9, x1, x2, lsr #1
        mov  x6, x1
        b    1f
        3:  sub  x9, x9, x2
        1:  ld1  {v0.16b}, [x6]
            ld1  {v1.16b}, [x9]
            cmeq v1.16b, v0.16b, v1.16b
            and  v1.16b, v0.16b, v1.16b
            and  v1.16b, v1.16b, v7.16b
            addv b1, v1.16b
            umov w5, v1.b[0]
            add  x7, x7, x5
            add  x6, x6, #16
            add  x9, x9, #16
            subs x10, x10, 1
            beq  3b
            subs x3, x3, 1
            bne  1b

    ; do the last iteration (replacing the end with zeroes to not over-count)
        ; v0, v1, x5 <- temp
        movi v0.16b, #0
        add  x3, x1, x2
        st1  {v0.16b}, [x3]
        ld1  {v0.16b}, [x6]
        ld1  {v1.16b}, [x9]
        cmeq v1.16b, v0.16b, v1.16b
        and  v1.16b, v0.16b, v1.16b
        and  v1.16b, v1.16b, v7.16b
        addv b1, v1.16b
        umov w5, v1.b[0]
        add  x7, x7, x5

    ; --------------------------------------------------------------------

    ; write result into the returned struct
        str xzr, [x8       ]
        str x4,  [x8, #0x08]
        str xzr, [x8, #0x18]
        str x7,  [x8, #0x20]

    ; postamble
        ldp x29, x30, [sp], #16
        ret

; ======================================================================================================================
old_scalar_day20171:
    ; preamble
        stp x29, x30, [sp, #-16]!
        mov x29, sp

    ; x9  <- previous digit, start by setting to final char in input str
    add  x9, x1, x2
    sub  x9, x9, 1
    ldrb w9, [x9]
    sub  x9, x9, #48

    lsr x14, x2, #1   ; x14 <- part 2 countdown until pointer should wrap
    add x13, x1, x14  ; x13 <- part 2 pointer, start half-way through the string
    mov x5,  x1       ; x5  <- string walking pointer
    
    mov  x11, #0      ; x11 <- part 1 result
    mov  x15, #0      ; x15 <- part 2 result
    b    1f

    ; x10     <- current digit
    ; x4, x12 <- temp

    2:  sub  x10, x10, #48   
        add  x12, x10, x11  
        cmp  x10, x9
        csel x11, x12, x11, eq

        ldrb w4, [x13]

        sub  x4, x4, #48   
        add  x12, x10, x15
        cmp  x10, x4
        csel x15, x12, x15, eq

        add  x5, x5, #1
        mov  x9, x10

        add  x13, x13, #1
        subs x14, x14, #1
        csel x13, x1, x13, eq

    1:  ldrb w10, [x5]
        cbnz x10, 2b

    ; write result into the returned struct
    str xzr, [x8       ]
    str x11, [x8, #0x08]
    str xzr, [x8, #0x18]
    str x15, [x8, #0x20]

    ; postamble
    ldp x29, x30, [sp], #16
    ret