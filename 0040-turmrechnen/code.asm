playground::main:
	pushq	%rbp
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	subq	$424, %rsp
	movq	$0, 24(%rsp)
	movq	$1, 32(%rsp)
	movq	$0, 40(%rsp)
	callq	*std::io::stdio::stdin@GOTPCREL(%rip)
	movq	%rax, 48(%rsp)
	leaq	48(%rsp), %rdi
	leaq	24(%rsp), %rsi
	callq	*std::io::stdio::Stdin::read_line@GOTPCREL(%rip)
	testb	$1, %al
	jne	.LBB0_99
	movq	32(%rsp), %rax
	movq	40(%rsp), %rcx
	leaq	(%rax,%rcx), %rsi
	testq	%rcx, %rcx
	je	.LBB0_27
	movq	core::unicode::unicode_data::white_space::WHITESPACE_MAP@GOTPCREL(%rip), %r8
	xorl	%edx, %edx
	movq	%rax, %rdi
	jmp	.LBB0_8

.LBB0_5:
	movzbl	%r9b, %r9d
	movzbl	(%r8,%r9), %r9d
	shrb	%r9b

.LBB0_6:
	testb	$1, %r9b
	je	.LBB0_28

.LBB0_7:
	cmpq	%rsi, %rdi
	je	.LBB0_26

.LBB0_8:
	movq	%rdi, %r10
	movq	%rdx, %rcx
	movzbl	(%rdi), %r9d
	testb	%r9b, %r9b
	js	.LBB0_10
	leaq	1(%r10), %rdi
	jmp	.LBB0_15

.LBB0_10:
	movl	%r9d, %edx
	andl	$31, %edx
	movzbl	1(%r10), %ebx
	andl	$63, %ebx
	cmpb	$-33, %r9b
	jbe	.LBB0_13
	movzbl	2(%r10), %r11d
	shll	$6, %ebx
	andl	$63, %r11d
	orl	%ebx, %r11d
	cmpb	$-16, %r9b
	jb	.LBB0_14
	leaq	4(%r10), %rdi
	movzbl	3(%r10), %r9d
	andl	$7, %edx
	shll	$18, %edx
	shll	$6, %r11d
	andl	$63, %r9d
	orl	%r11d, %r9d
	orl	%edx, %r9d
	jmp	.LBB0_15

.LBB0_13:
	leaq	2(%r10), %rdi
	shll	$6, %edx
	orl	%ebx, %edx
	movl	%edx, %r9d
	jmp	.LBB0_15

.LBB0_14:
	leaq	3(%r10), %rdi
	shll	$12, %edx
	orl	%edx, %r11d
	movl	%r11d, %r9d

.LBB0_15:
	movq	%rdi, %rdx
	subq	%r10, %rdx
	addq	%rcx, %rdx
	leal	-9(%r9), %r10d
	cmpl	$5, %r10d
	jb	.LBB0_7
	cmpl	$32, %r9d
	je	.LBB0_7
	cmpl	$128, %r9d
	jb	.LBB0_28
	movl	%r9d, %r10d
	shrl	$8, %r10d
	cmpl	$31, %r10d
	jg	.LBB0_22
	testl	%r10d, %r10d
	je	.LBB0_25
	cmpl	$22, %r10d
	jne	.LBB0_28
	cmpl	$5760, %r9d
	sete	%r9b
	jmp	.LBB0_6

.LBB0_22:
	cmpl	$32, %r10d
	je	.LBB0_5
	cmpl	$48, %r10d
	jne	.LBB0_28
	cmpl	$12288, %r9d
	sete	%r9b
	jmp	.LBB0_6

.LBB0_25:
	movzbl	%r9b, %r9d
	movzbl	(%r8,%r9), %r9d
	jmp	.LBB0_6

.LBB0_26:
	xorl	%ecx, %ecx
	xorl	%edx, %edx
	jmp	.LBB0_53

.LBB0_27:
	xorl	%edx, %edx
	movq	%rax, %rdi
	xorl	%ecx, %ecx

.LBB0_28:
	cmpq	%rsi, %rdi
	je	.LBB0_53
	movq	core::unicode::unicode_data::white_space::WHITESPACE_MAP@GOTPCREL(%rip), %r8
	jmp	.LBB0_33

.LBB0_30:
	movzbl	%r10b, %r10d
	movzbl	(%r8,%r10), %r10d
	shrb	%r10b

.LBB0_31:
	testb	$1, %r10b
	je	.LBB0_52

.LBB0_32:
	cmpq	%rsi, %rdi
	je	.LBB0_53

.LBB0_33:
	movq	%rsi, %r9
	movsbl	-1(%rsi), %r10d
	testl	%r10d, %r10d
	js	.LBB0_35
	leaq	-1(%r9), %rsi
	leal	-9(%r10), %r11d
	cmpl	$5, %r11d
	jb	.LBB0_32
	jmp	.LBB0_42

.LBB0_35:
	movzbl	-2(%r9), %r11d
	cmpb	$-64, %r11b
	jge	.LBB0_38
	movzbl	-3(%r9), %ebx
	cmpb	$-64, %bl
	jge	.LBB0_39
	leaq	-4(%r9), %rsi
	movzbl	-4(%r9), %ebp
	andl	$7, %ebp
	shll	$6, %ebp
	andl	$63, %ebx
	orl	%ebp, %ebx
	jmp	.LBB0_40

.LBB0_38:
	leaq	-2(%r9), %rsi
	andl	$31, %r11d
	jmp	.LBB0_41

.LBB0_39:
	leaq	-3(%r9), %rsi
	andl	$15, %ebx

.LBB0_40:
	shll	$6, %ebx
	andl	$63, %r11d
	orl	%ebx, %r11d

.LBB0_41:
	shll	$6, %r11d
	andl	$63, %r10d
	orl	%r11d, %r10d
	leal	-9(%r10), %r11d
	cmpl	$5, %r11d
	jb	.LBB0_32

.LBB0_42:
	cmpl	$32, %r10d
	je	.LBB0_32
	cmpl	$128, %r10d
	jb	.LBB0_52
	movl	%r10d, %r11d
	shrl	$8, %r11d
	cmpl	$31, %r11d
	jg	.LBB0_48
	testl	%r11d, %r11d
	je	.LBB0_51
	cmpl	$22, %r11d
	jne	.LBB0_52
	cmpl	$5760, %r10d
	sete	%r10b
	jmp	.LBB0_31

.LBB0_48:
	cmpl	$32, %r11d
	je	.LBB0_30
	cmpl	$48, %r11d
	jne	.LBB0_52
	cmpl	$12288, %r10d
	sete	%r10b
	jmp	.LBB0_31

.LBB0_51:
	movzbl	%r10b, %r10d
	movzbl	(%r8,%r10), %r10d
	jmp	.LBB0_31

.LBB0_52:
	subq	%rdi, %rdx
	addq	%r9, %rdx

.LBB0_53:
	movq	%rdx, %r8
	movl	$3, %esi
	movl	$3, %edi
	subq	%rcx, %r8
	je	.LBB0_109
	leaq	(%rax,%rcx), %r9
	cmpq	$1, %r8
	jne	.LBB0_65
	movzbl	(%r9), %r10d
	movl	$3, %edi
	cmpl	$43, %r10d
	je	.LBB0_109
	movl	$3, %edi
	cmpl	$45, %r10d
	je	.LBB0_109
	cmpb	$45, %r10b
	je	.LBB0_66

.LBB0_58:
	movzbl	%r10b, %eax
	cmpl	$43, %eax
	jne	.LBB0_60
	incq	%r9
	decq	%r8

.LBB0_60:
	cmpq	$8, %r8
	jae	.LBB0_71
	testq	%r8, %r8
	je	.LBB0_84
	movzbl	(%r9), %eax
	addl	$-48, %eax
	movl	$3, %edi
	cmpl	$9, %eax
	ja	.LBB0_109
	cmpq	$1, %r8
	jne	.LBB0_86

.LBB0_64:
	movl	%eax, %edi
	jmp	.LBB0_109

.LBB0_65:
	movzbl	(%r9), %r10d
	cmpb	$45, %r10b
	jne	.LBB0_58

.LBB0_66:
	cmpq	$9, %r8
	jae	.LBB0_77
	decq	%r8
	je	.LBB0_84
	incq	%rcx
	xorl	%edi, %edi

.LBB0_69:
	movzbl	(%rax,%rcx), %r8d
	addl	$-48, %r8d
	cmpl	$9, %r8d
	ja	.LBB0_95
	addl	%edi, %edi
	leal	(%rdi,%rdi,4), %edi
	subl	%r8d, %edi
	incq	%rcx
	cmpq	%rcx, %rdx
	jne	.LBB0_69
	jmp	.LBB0_109

.LBB0_71:
	xorl	%eax, %eax
	xorl	%edi, %edi

.LBB0_72:
	imull	$10, %edi, %edi
	jo	.LBB0_95
	movzbl	(%r9,%rax), %ecx
	addl	$-48, %ecx
	cmpl	$9, %ecx
	ja	.LBB0_95
	addl	%ecx, %edi
	jo	.LBB0_95
	incq	%rax
	cmpq	%rax, %r8
	jne	.LBB0_72
	jmp	.LBB0_109

.LBB0_84:
	xorl	%edi, %edi
	jmp	.LBB0_109

.LBB0_77:
	incq	%rcx
	xorl	%edi, %edi

.LBB0_78:
	imull	$10, %edi, %edi
	jo	.LBB0_95
	movzbl	(%rax,%rcx), %r8d
	addl	$-48, %r8d
	cmpl	$9, %r8d
	ja	.LBB0_95
	subl	%r8d, %edi
	jo	.LBB0_95
	incq	%rcx
	cmpq	%rcx, %rdx
	jne	.LBB0_78
	jmp	.LBB0_109

.LBB0_95:
	movl	$3, %edi

.LBB0_109:
	movd	%edi, %xmm1
	movl	$42, %eax
	movd	%eax, %xmm3
	movdqa	%xmm3, %xmm0
	punpckldq	%xmm1, %xmm0
	pslld	$1, %xmm1
	movl	$2, %eax
	movd	%eax, %xmm13
	movdqa	%xmm13, %xmm2
	punpckldq	%xmm1, %xmm2
	punpcklqdq	%xmm2, %xmm0
	movdqa	%xmm0, 128(%rsp)
	leal	(%rdi,%rdi), %eax
	leal	(%rax,%rax,2), %eax
	movdqa	%xmm3, %xmm0
	punpckldq	%xmm1, %xmm0
	movd	%eax, %xmm4
	movd	%esi, %xmm15
	movdqa	%xmm15, %xmm1
	punpckldq	%xmm4, %xmm1
	punpcklqdq	%xmm1, %xmm0
	movdqa	%xmm0, 112(%rsp)
	leal	(,%rdi,8), %eax
	leal	(%rax,%rax,2), %eax
	movd	%eax, %xmm5
	movl	$4, %eax
	movd	%eax, %xmm1
	movdqa	%xmm1, %xmm6
	punpckldq	%xmm5, %xmm6
	movdqa	%xmm3, %xmm2
	punpckldq	%xmm4, %xmm2
	punpcklqdq	%xmm6, %xmm2
	imull	$120, %edi, %eax
	movd	%eax, %xmm6
	movl	$5, %eax
	movd	%eax, %xmm14
	movdqa	%xmm14, %xmm7
	punpckldq	%xmm6, %xmm7
	movdqa	%xmm3, %xmm4
	punpckldq	%xmm5, %xmm4
	punpcklqdq	%xmm7, %xmm4
	imull	$720, %edi, %eax
	movd	%eax, %xmm7
	movl	$6, %eax
	movd	%eax, %xmm12
	movdqa	%xmm12, %xmm8
	punpckldq	%xmm7, %xmm8
	movdqa	%xmm3, %xmm5
	punpckldq	%xmm6, %xmm5
	punpcklqdq	%xmm8, %xmm5
	imull	$5040, %edi, %eax
	movd	%eax, %xmm8
	movl	$7, %eax
	movd	%eax, %xmm9
	movdqa	%xmm9, %xmm10
	punpckldq	%xmm8, %xmm10
	movdqa	%xmm3, %xmm6
	punpckldq	%xmm7, %xmm6
	punpcklqdq	%xmm10, %xmm6
	imull	$40320, %edi, %eax
	movd	%eax, %xmm11
	movl	$8, %eax
	movdqa	%xmm3, %xmm7
	punpckldq	%xmm8, %xmm7
	movd	%eax, %xmm10
	movdqa	%xmm10, %xmm8
	punpckldq	%xmm11, %xmm8
	punpcklqdq	%xmm8, %xmm7
	imull	$362880, %edi, %eax
	movl	$9, %ecx
	punpckldq	%xmm11, %xmm3
	movd	%ecx, %xmm8
	movd	%eax, %xmm0
	punpckldq	%xmm0, %xmm8
	punpcklqdq	%xmm8, %xmm3
	movl	$47, %ecx
	movd	%ecx, %xmm8
	movdqa	%xmm8, %xmm11
	punpckldq	%xmm0, %xmm11
	psrad	$1, %xmm0
	punpckldq	%xmm0, %xmm13
	punpcklqdq	%xmm13, %xmm11
	movdqa	%xmm8, %xmm13
	punpckldq	%xmm0, %xmm13
	cltq
	imulq	$715827883, %rax, %rcx
	movq	%rcx, %rdx
	shrq	$63, %rdx
	shrq	$32, %rcx
	leal	(%rcx,%rdx), %esi
	movd	%esi, %xmm0
	punpckldq	%xmm0, %xmm15
	punpcklqdq	%xmm15, %xmm13
	movdqa	%xmm8, %xmm15
	punpckldq	%xmm0, %xmm15
	sarl	$2, %ecx
	addl	%edx, %ecx
	movd	%ecx, %xmm0
	punpckldq	%xmm0, %xmm1
	punpcklqdq	%xmm1, %xmm15
	movdqa	%xmm8, %xmm1
	punpckldq	%xmm0, %xmm1
	imulq	$-2004318071, %rax, %rcx
	shrq	$32, %rcx
	addl	%eax, %ecx
	movl	%ecx, %edx
	shrl	$31, %edx
	sarl	$6, %ecx
	addl	%edx, %ecx
	movd	%ecx, %xmm0
	punpckldq	%xmm0, %xmm14
	punpcklqdq	%xmm14, %xmm1
	movdqa	%xmm8, %xmm14
	punpckldq	%xmm0, %xmm14
	imulq	$-1240768329, %rax, %rcx
	shrq	$32, %rcx
	addl	%eax, %ecx
	movl	%ecx, %edx
	shrl	$31, %edx
	sarl	$9, %ecx
	addl	%edx, %ecx
	movd	%ecx, %xmm0
	punpckldq	%xmm0, %xmm12
	punpcklqdq	%xmm12, %xmm14
	movdqa	%xmm8, %xmm12
	punpckldq	%xmm0, %xmm12
	imulq	$-804454191, %rax, %rcx
	shrq	$32, %rcx
	addl	%eax, %ecx
	movl	%ecx, %edx
	sarl	$12, %edx
	movl	%ecx, %esi
	shrl	$31, %esi
	addl	%esi, %edx
	movd	%edx, %xmm0
	punpckldq	%xmm0, %xmm9
	punpcklqdq	%xmm9, %xmm12
	sarl	$15, %ecx
	addl	%esi, %ecx
	movd	%ecx, %xmm9
	punpckldq	%xmm9, %xmm10
	punpckldq	%xmm0, %xmm8
	punpcklqdq	%xmm10, %xmm8
	movaps	128(%rsp), %xmm0
	movups	%xmm0, 168(%rsp)
	movaps	112(%rsp), %xmm0
	movups	%xmm0, 184(%rsp)
	movdqu	%xmm2, 200(%rsp)
	movdqu	%xmm4, 216(%rsp)
	movdqu	%xmm5, 232(%rsp)
	movdqu	%xmm6, 248(%rsp)
	movdqu	%xmm7, 264(%rsp)
	movdqu	%xmm3, 280(%rsp)
	movdqu	%xmm11, 296(%rsp)
	movdqu	%xmm13, 312(%rsp)
	movdqu	%xmm15, 328(%rsp)
	movdqu	%xmm1, 344(%rsp)
	movdqu	%xmm14, 360(%rsp)
	movdqu	%xmm12, 376(%rsp)
	movdqu	%xmm8, 392(%rsp)
	imulq	$775669579, %rax, %rax
	movq	%rax, %rdx
	shrq	$63, %rdx
	sarq	$48, %rax
	addl	%edx, %eax
	movl	$47, 408(%rsp)
	movl	%ecx, 412(%rsp)
	movl	$9, 416(%rsp)
	movl	%eax, 420(%rsp)
	movl	$28, %r15d
	movq	core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt@GOTPCREL(%rip), %r13
	leaq	20(%rsp), %r12
	leaq	.Lanon.02c6613994fbad17c13987d40495760f.3(%rip), %rbx
	leaq	48(%rsp), %r14
	movq	std::io::stdio::_print@GOTPCREL(%rip), %rbp

.LBB0_110:
	movl	140(%rsp,%r15), %eax
	cmpl	$1114112, %eax
	je	.LBB0_113
	movl	152(%rsp,%r15), %ecx
	movl	144(%rsp,%r15), %edx
	movl	148(%rsp,%r15), %esi
	movl	%edx, 8(%rsp)
	movl	%eax, 12(%rsp)
	movl	%esi, 16(%rsp)
	movl	%ecx, 20(%rsp)
	leaq	8(%rsp), %rax
	movq	%rax, 48(%rsp)
	movq	%r13, 56(%rsp)
	leaq	12(%rsp), %rax
	movq	%rax, 64(%rsp)
	movq	<char as core::fmt::Display>::fmt@GOTPCREL(%rip), %rax
	movq	%rax, 72(%rsp)
	leaq	16(%rsp), %rax
	movq	%rax, 80(%rsp)
	movq	%r13, 88(%rsp)
	movq	%r12, 96(%rsp)
	movq	%r13, 104(%rsp)
	movq	%rbx, %rdi
	movq	%r14, %rsi
	callq	*%rbp
	addq	$16, %r15
	cmpq	$284, %r15
	jne	.LBB0_110

.LBB0_113:
	movq	24(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB0_115
	movq	32(%rsp), %rdi
	movl	$1, %edx
	callq	*__rustc::__rust_dealloc@GOTPCREL(%rip)

.LBB0_115:
	addq	$424, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	retq

.LBB0_86:
	movzbl	1(%r9), %ecx
	addl	$-48, %ecx
	movl	$3, %edi
	cmpl	$9, %ecx
	ja	.LBB0_109
	leal	(%rax,%rax,4), %eax
	leal	(%rcx,%rax,2), %eax
	cmpq	$2, %r8
	je	.LBB0_64
	movzbl	2(%r9), %ecx
	addl	$-48, %ecx
	movl	$3, %edi
	cmpl	$9, %ecx
	ja	.LBB0_109
	leal	(%rax,%rax,4), %eax
	leal	(%rcx,%rax,2), %eax
	cmpq	$3, %r8
	je	.LBB0_64
	movzbl	3(%r9), %ecx
	addl	$-48, %ecx
	movl	$3, %edi
	cmpl	$9, %ecx
	ja	.LBB0_109
	leal	(%rax,%rax,4), %eax
	leal	(%rcx,%rax,2), %eax
	cmpq	$4, %r8
	je	.LBB0_64
	movzbl	4(%r9), %ecx
	addl	$-48, %ecx
	movl	$3, %edi
	cmpl	$9, %ecx
	ja	.LBB0_109
	leal	(%rax,%rax,4), %eax
	leal	(%rcx,%rax,2), %eax
	cmpq	$5, %r8
	je	.LBB0_64
	movzbl	5(%r9), %ecx
	addl	$-48, %ecx
	movl	$3, %edi
	cmpl	$9, %ecx
	ja	.LBB0_109
	leal	(%rax,%rax,4), %eax
	leal	(%rcx,%rax,2), %eax
	cmpq	$6, %r8
	je	.LBB0_64
	movzbl	6(%r9), %ecx
	addl	$-48, %ecx
	movl	$3, %edi
	cmpl	$9, %ecx
	ja	.LBB0_109
	leal	(%rax,%rax,4), %eax
	leal	(%rcx,%rax,2), %edi
	jmp	.LBB0_109

.LBB0_99:
	movq	%rdx, 152(%rsp)
	leaq	.Lanon.02c6613994fbad17c13987d40495760f.1(%rip), %rdi
	leaq	.Lanon.02c6613994fbad17c13987d40495760f.5(%rip), %rcx
	leaq	.Lanon.02c6613994fbad17c13987d40495760f.2(%rip), %r8
	leaq	152(%rsp), %rdx
	movl	$19, %esi
	callq	*core::result::unwrap_failed@GOTPCREL(%rip)
	ud2
	jmp	.LBB0_118

.LBB0_118:
	movq	%rax, %rbx
	jmp	.LBB0_120
	movq	%rax, %rbx
	leaq	152(%rsp), %rdi
	callq	core::ptr::drop_in_place<std::io::error::Error>

.LBB0_120:
	movq	24(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB0_122
	movq	32(%rsp), %rdi
	movl	$1, %edx
	callq	*__rustc::__rust_dealloc@GOTPCREL(%rip)

.LBB0_122:
	movq	%rbx, %rdi
	callq	_Unwind_Resume@PLT
	callq	*core::panicking::panic_in_cleanup@GOTPCREL(%rip)

std::rt::lang_start:
	pushq	%rax
	movl	%ecx, %r8d
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movq	%rdi, (%rsp)
	leaq	.Lanon.02c6613994fbad17c13987d40495760f.4(%rip), %rsi
	movq	%rsp, %rdi
	callq	*std::rt::lang_start_internal@GOTPCREL(%rip)
	popq	%rcx
	retq

std::rt::lang_start::{{closure}}:
	pushq	%rax
	movq	(%rdi), %rdi
	callq	std::sys::backtrace::__rust_begin_short_backtrace
	xorl	%eax, %eax
	popq	%rcx
	retq

std::sys::backtrace::__rust_begin_short_backtrace:
	pushq	%rax
	callq	*%rdi
	#APP
	#NO_APP
	popq	%rax
	retq

core::ops::function::FnOnce::call_once{{vtable.shim}}:
	pushq	%rax
	movq	(%rdi), %rdi
	callq	std::sys::backtrace::__rust_begin_short_backtrace
	xorl	%eax, %eax
	popq	%rcx
	retq

core::ptr::drop_in_place<std::io::error::Error>:
	pushq	%r15
	pushq	%r14
	pushq	%r12
	pushq	%rbx
	pushq	%rax
	movq	(%rdi), %rax
	movl	%eax, %ecx
	andl	$3, %ecx
	cmpl	$1, %ecx
	je	.LBB5_1
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r14
	popq	%r15
	retq

.LBB5_1:
	leaq	-1(%rax), %rbx
	movq	-1(%rax), %r14
	movq	7(%rax), %r12
	movq	(%r12), %rax
	testq	%rax, %rax
	je	.LBB5_3
	movq	%r14, %rdi
	callq	*%rax

.LBB5_3:
	movq	8(%r12), %rsi
	testq	%rsi, %rsi
	je	.LBB5_5
	movq	16(%r12), %rdx
	movq	%r14, %rdi
	callq	*__rustc::__rust_dealloc@GOTPCREL(%rip)

.LBB5_5:
	movl	$24, %esi
	movl	$8, %edx
	movq	%rbx, %rdi
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r14
	popq	%r15
	jmpq	*__rustc::__rust_dealloc@GOTPCREL(%rip)
	movq	%rax, %r15
	movq	8(%r12), %rsi
	testq	%rsi, %rsi
	je	.LBB5_8
	movq	16(%r12), %rdx
	movq	%r14, %rdi
	callq	*__rustc::__rust_dealloc@GOTPCREL(%rip)

.LBB5_8:
	movl	$24, %esi
	movl	$8, %edx
	movq	%rbx, %rdi
	callq	*__rustc::__rust_dealloc@GOTPCREL(%rip)
	movq	%r15, %rdi
	callq	_Unwind_Resume@PLT

main:
	pushq	%rax
	movq	%rsi, %rcx
	movslq	%edi, %rdx
	leaq	playground::main(%rip), %rax
	movq	%rax, (%rsp)
	leaq	.Lanon.02c6613994fbad17c13987d40495760f.4(%rip), %rsi
	movq	%rsp, %rdi
	xorl	%r8d, %r8d
	callq	*std::rt::lang_start_internal@GOTPCREL(%rip)
	popq	%rcx
	retq

.Lanon.02c6613994fbad17c13987d40495760f.0:
	.asciz	"src/main.rs"

.Lanon.02c6613994fbad17c13987d40495760f.1:
	.ascii	"Failed to read line"

.Lanon.02c6613994fbad17c13987d40495760f.2:
	.quad	.Lanon.02c6613994fbad17c13987d40495760f.0
	.asciz	"\013\000\000\000\000\000\000\000\026\000\000\000,\000\000"

.Lanon.02c6613994fbad17c13987d40495760f.3:
	.asciz	"\303 \000\000(\n\000\001 \300\001 \300\003 = \300\001\n"

.Lanon.02c6613994fbad17c13987d40495760f.4:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	core::ops::function::FnOnce::call_once{{vtable.shim}}
	.quad	std::rt::lang_start::{{closure}}
	.quad	std::rt::lang_start::{{closure}}

.Lanon.02c6613994fbad17c13987d40495760f.5:
	.quad	core::ptr::drop_in_place<std::io::error::Error>
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	<std::io::error::Error as core::fmt::Debug>::fmt