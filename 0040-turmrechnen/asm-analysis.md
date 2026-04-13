# Assembly analysis (Claude Opus 4.6)

[Assembler Code](code.asm)

Let me focus on just the calculation. This is the most interesting part of the diff vs. the constant version, because LLVM did something genuinely clever.

## The setup: `start` is now in `%edi`

After the parse, `%edi` holds the runtime `start_value`. We're now computing the same 16 tuples as before, but with a variable input. Naively this would be 8 multiplies and 8 divides in a loop. LLVM did neither.

## The multiplication half — strength reduction, not a loop

Look at how each "next" value is computed:

```asm
; i=2: next = start * 2
pslld $1, %xmm1                       ; shift left by 1

; i=3: next = start * 6
leal (%rdi,%rdi), %eax                ; eax = start * 2
leal (%rax,%rax,2), %eax              ; eax = eax * 3 = start * 6

; i=4: next = start * 24
leal (,%rdi,8), %eax                  ; eax = start * 8
leal (%rax,%rax,2), %eax              ; eax = eax * 3 = start * 24

; i=5: start * 120
imull $120, %edi, %eax

; i=6: start * 720
imull $720, %edi, %eax

; i=7: start * 5040
imull $5040, %edi, %eax

; i=8: start * 40320
imull $40320, %edi, %eax

; i=9: start * 362880
imull $362880, %edi, %eax
```

This is the key insight: the loop has been **completely unrolled**, and at each step LLVM recognized that `current` after iteration `i` is just `start * i!`. So instead of computing `current = current * i` sequentially (which would create a dependency chain — each multiply waiting on the previous), it computes each value **independently from the original `start`** using a precomputed factorial constant.

Notice the smallest factorials (`2`, `6`, `24`) use `lea` tricks instead of `imul` because they're cheaper. `120` and up fall back to `imull` with an immediate — still a single instruction, still independent of all the others, so the CPU can issue them in parallel.

## The division half — magic number division

This is even nicer. After the multiply chain, `current = start * 362880` (which is `start * 9!`). The divide loop should produce:

```
start * 362880 / 2     = start * 181440
start * 181440 / 3     = start * 60480
start * 60480  / 4     = start * 15120
...
start * 6      / 9     = start
```

Same trick: each result is just `start * (some factorial-derived constant)`. But LLVM went one step further and computed each division of the *previous result* using **integer division by invariant multiplication** (Granlund-Montgomery magic numbers). It uses `%rax` holding `start * 362880` (sign-extended) as the base and applies a different magic constant for each divisor:

```asm
; First: x / 2  (just an arithmetic shift)
psrad $1, %xmm0

; x / 3  via magic number
imulq $715827883, %rax, %rcx          ; 715827883 ≈ 2^31 / 3
movq  %rcx, %rdx
shrq  $63, %rdx                       ; sign bit
shrq  $32, %rcx                       ; high half
leal  (%rcx,%rdx), %esi               ; result = high + sign

; x / 4  (shift)
sarl  $2, %ecx
addl  %edx, %ecx

; x / 5  via magic
imulq $-2004318071, %rax, %rcx
shrq  $32, %rcx
addl  %eax, %ecx
sarl  $6, %ecx                        ; (the shift amount encodes the divisor)
...

; x / 6  via magic  (-1240768329, sarl $9)
; x / 7  via magic  (-804454191, sarl $12)
; x / 8  via magic  (775669579, sarq $48)
; x / 9  via magic  (the last one, also folded)
```

Each `imulq` + shift sequence is the standard `divide by constant via 64-bit multiply, take the high half, shift, fix up sign` pattern. **There is not a single `idiv` instruction in the entire function.** `idiv` is enormously slow (20–40 cycles on modern x86), so LLVM replaces every constant divisor with a multiply-and-shift that's typically 3–5 cycles.

Worth noting: the divisors aren't applied to the *previous* division's result (which would create a dependency chain again) — they're each computed against `%rax`, the sign-extended `start * 362880`. So divisions 2–9 are also independent and can pipeline freely.

## Building the tuples in SIMD registers

Interleaved with all the arithmetic, you see a lot of this:

```asm
movd     %eax, %xmm4
movd     %esi, %xmm15      ; esi = i (the loop index, baked in)
movdqa   %xmm15, %xmm1
punpckldq %xmm4, %xmm1     ; pack (i, next) into low 64 bits
punpcklqdq %xmm1, %xmm0    ; combine with (op, current) high half
```

Each tuple `(current, op, i, next)` is 16 bytes, exactly one SSE register. LLVM is assembling each row using `punpckldq` (interleave 32-bit lanes) and `punpcklqdq` (interleave 64-bit lanes), so the four `i32`-sized fields get packed into a single `xmm` with two instructions. The `op` value (`42` = `'*'` or `47` = `'/'`) and the loop index `i` are immediate constants since the loops were unrolled — only `current` and `next` come from the actual computation.

Then all 16 xmm registers get spilled to the stack at the end:

```asm
movdqu %xmm2,  200(%rsp)
movdqu %xmm4,  216(%rsp)
movdqu %xmm5,  232(%rsp)
... etc, 16 stores total
```

That stack region is the `[(i32, char, i32, i32); 16]` array, ready for the same `println!` loop you saw in the const version.

## The big picture

What's striking is that the **shape** of the output code is identical to the const-START version: 16 SSE registers built up, dumped to the stack, then a 16-iteration print loop. The only difference is *how* the 16 records get filled in:

- **Const version:** all 16 records are `.rodata` literals, copied with `movaps` from `.LCPI0_*`.
- **Variable version:** each record's `current` and `next` fields are computed from `%edi` using one `imul` (multiplies) or one `imul`+shift (divides), all independently, all parallelizable. The `op` and `i` fields are still immediates because the unrolling exposed them.

Zero loops. Zero idivs. Zero data dependencies between the 16 computations. For a function that, written naively, looks like two sequential 8-iteration loops with a carried dependency on `current`, this is about as good as it gets — LLVM saw through the closure, the loop, and the carried state, and turned it into 16 independent expressions in `start`. 

If you want to see this fall apart, change `start` to be read inside the loop (e.g. read a new number each iteration) — the carried dependency becomes real and the unrolling collapses.