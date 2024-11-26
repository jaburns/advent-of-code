.section __TEXT,__text

; ======================================================================================================================
; DayResult day2(Arena* arena, Str input);
; x0 -> Arena*      arena
; x1 -> char*       input.items
; x2 -> size_t      input.count
; x8 -> DayResult*  result
.global _day2
_day2:
    ; preamble
        stp x29, x30, [sp, #-16]!
        mov x29, sp

    ; v1 : ascii tab
    ; v4 : ascii newline
    ; x6 : pointer to end of input string
        movi v1.16b, #9
        movi v4.16b, #10
        add  x6, x1, x2

    ; count columns
    ; x5 : number of columns
        ; v2, x3 : temp
        ; x4 : string walk pointer
        mov x4, x1
        mov x5, #1
        1:  ld1   {v0.16b}, [x4]          ; load 16 bytes
            cmeq  v2.16b, v0.16b, v4.16b  ; are any of them a newline?
            umaxv b2, v2.16b
            umov  w3, v2.b[0]
            cbnz  x3, 2f                  ; if yes, jump into the wrap-up loop

            cmeq  v2.16b, v0.16b, v1.16b  ; add up the number of tab characters
            addv  b2, v2.16b
            smov  w3, v2.b[0]
            neg   w3, w3
            add   w5, w5, w3
            add   x4, x4, #16  ; bump the read pointer and restart the loop
            b     1b

        2:  ldrb  w3, [x4]    ; load a single byte
            cmp   x3, #10     ; if it's a newline we're done
            beq   3f
            cmp   x3, #9      ; count it if it's a tab
            cinc  x5, x5, eq
            add   x4, x4, #1
            b     2b
        3:

    ; parse the digits into the memory immediately after the input string (pre-allocated arena)
    ; x1 : mutated, walks the string
    ; x9 : write pointer start
    ; x7 : write pointer current / final
        ; v0 : substring
        ; v2, v3 : temp
        ; x3 : 4x number of digits in loaded number
        ; x4 : temp
        add x7, x6, #16                  ; 16-byte align the write pointer into the post-input-data arena
        and x7, x7, 0xfffffffffffffff0
        mov x9, x7

        1:  ld1  {v0.16b}, [x1]          ; load 16 byte chunk and set each non-digit separator byte
            cmle v2.16b, v0.16b, v4.16b
            shrn v2.8b, v2.8h, #4        ; use shift+narrow to compress vector into a u64 bit field of
            umov x3, v2.d[0]             ;   4 bits per comparison result, bit-reverse the u64, and
            rbit x3, x3                  ;   count leading zeroes to get four times the number of digits
            clz  x3, x3

            adr x4, value_mask           ; mask out the digit values from the ascii bytes
            add x4, x4, x3
            ld1 {v2.8h}, [x4]
            and v3.16b, v0.16b, v2.16b
            uxtl v3.8h, v3.8b            ; extend the bytes into u16s

            adr x4, digit_values         ; multiply each digit u16 by its base10 placement value
            sub x4, x4, x3, lsr #1
            ld1 {v2.8h}, [x4]
            mul v3.8h, v3.8h, v2.8h

            addv h3, v3.4h    ; sum up the digit values to get the final parsed number
            umov w4, v3.h[0]  ;   and store it at the write pointer
            strh w4, [x7]

            add  x3, x3, #4          ; bump pointers/counters and loop if we're not at the end
            add  x1, x1, x3, lsr #2
            add  x7, x7, 2
            cmp  x1, x6
            blt 1b

;   ; asserting here that the number of columns fits neatly into vector registers ( x5 % 8 == 0 )
;       and x0, x5, #0xfffffffffffffff8
;       cmp x0, x5
;       beq 1f
;       brk #1
;   1:

    ; --- at this point the only scratch registers with values we care about are: ---
    ; x9 : pointer start of data (u16)
    ; x7 : pointer to end of data
    ; x5 : number of items per row
    ; ---

    ; part 1 - loop over rows and find the sum of min/max difference across columns
    ; x0 : result
        ; x0 : read pointer
        ; x1 : chunks left in this row
        ; v0, v1, v2 : temp
        ; h3, h4 : cumulative min/max respectively
        ; h5 : cumulative result
        mov  x0, x9
        movi v5.16b, #0

        2:  lsr   x1, x5, #3          ; init per-row state
            movi  v3.16b, #0xff
            movi  v4.16b, #0
        1:  ld1   {v0.16b}, [x0]      ; load a chunk and accumulate min/max,
            uminv h1, v0.8h           ;   looping if there's more chunks
            umaxv h2, v0.8h
            umin  v3.8h, v3.8h, v1.8h
            umax  v4.8h, v4.8h, v2.8h
            add   x0, x0, #16
            subs  x1, x1, #1
            bne   1b

            sub   v3.8h, v4.8h, v3.8h  ; at the end of the row, add up the min/max
            add   v5.8h, v5.8h, v3.8h  ;   difference into the final result
            cmp   x0, x7
            blt   2b

        umov w0, v5.h[0]

    ; part 2 - loop over rows searching for evenly divisible pairs
    ; x9  : mutated to walk the data
    ; x13 : result
        ; x1, x2 : inner loop pointers
        ; x3, x4, x10, x11, x12 : temp

        mov x13, #0

        1:  add x1, x9, #2
            mov x2, x9

                ; for (x1 = &row[1];        ; ++x1)
                ; for (x2 = &row[0]; x2 < x1; ++x2)
            2:  ldrh w3, [x1]           ; load two u16s and check if they divide evenly
                ldrh w4, [x2]
                cmp  w3, w4
                csel w10, w3, w4, gt
                csel w11, w3, w4, lt
                udiv w6, w10, w11
                msub w12, w6, w11, w10
                cmp  w12, #0
                beq  3f                 ; if so, break out of the x1/x2 loop

                add  x2, x2, #2  ; inner loop
                cmp  x2, x1
                blt  2b
                add  x1, x1, #2  ; outer loop
                mov  x2, x9
                b    2b

        3:  add  x13, x13, x6        ; add current row's quotient (x6) to result and
            add  x9, x9, x5, lsl #1  ;   check outer loop condition
            cmp  x9, x7
            bne  1b

    ; write result into the returned struct
        str xzr, [x8       ]  ; part[0].is_str
        str x0,  [x8, #0x08]  ; part[0].as_u64
        str xzr, [x8, #0x18]  ; part[1].is_str
        str x13, [x8, #0x20]  ; part[1].as_u64

    ; postamble
        ldp x29, x30, [sp], #16
        ret

.align 16
    .hword   1000
    .hword    100
    .hword     10
    .hword      1
digit_values:
    .hword      0

.align 16
value_mask:
    .word  0x00000000
    .word  0x0000000f
    .word  0x00000f0f
    .word  0x000f0f0f
    .word  0x0f0f0f0f