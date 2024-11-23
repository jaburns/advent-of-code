.section __TEXT,__text

; DayResult day20171(Arena* arena, Str input);
; x0 -> Arena*      arena
; x1 -> char*       input.items
; x2 -> size_t      input.count
; x8 -> DayResult*  result
.global _day20171
_day20171:
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