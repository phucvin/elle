.data
.balign 8
.3:
	.ascii "true"
	.byte 0
/* end data */

.data
.balign 8
.4:
	.ascii "false"
	.byte 0
/* end data */

.data
.balign 8
.786:
	.ascii "*"
	.byte 0
/* end data */

.data
.balign 8
.792:
	.ascii "%p"
	.byte 0
/* end data */

.data
.balign 8
.801:
	.ascii "string"
	.byte 0
/* end data */

.data
.balign 8
.808:
	.ascii "\""
	.byte 0
/* end data */

.data
.balign 8
.809:
	.ascii "\""
	.byte 0
/* end data */

.data
.balign 8
.811:
	.ascii "\"\\\"\""
	.byte 0
/* end data */

.data
.balign 8
.812:
	.ascii "arg"
	.byte 0
/* end data */

.data
.balign 8
.813:
	.ascii "\"\\\"\""
	.byte 0
/* end data */

.data
.balign 8
.823:
	.ascii "string"
	.byte 0
/* end data */

.data
.balign 8
.824:
	.ascii "string"
	.byte 0
/* end data */

.data
.balign 8
.825:
	.ascii "string"
	.byte 0
/* end data */

.data
.balign 8
.836:
	.ascii "__to_string"
	.byte 0
/* end data */

.data
.balign 8
.838:
	.ascii "string"
	.byte 0
/* end data */

.data
.balign 8
.849:
	.ascii "i32"
	.byte 0
/* end data */

.data
.balign 8
.855:
	.ascii "%d"
	.byte 0
/* end data */

.data
.balign 8
.864:
	.ascii "i64"
	.byte 0
/* end data */

.data
.balign 8
.870:
	.ascii "%ld"
	.byte 0
/* end data */

.data
.balign 8
.881:
	.ascii "f32"
	.byte 0
/* end data */

.data
.balign 8
.884:
	.ascii "f64"
	.byte 0
/* end data */

.data
.balign 8
.890:
	.ascii "%f"
	.byte 0
/* end data */

.data
.balign 8
.892:
	.ascii "f32"
	.byte 0
/* end data */

.data
.balign 8
.907:
	.ascii "char"
	.byte 0
/* end data */

.data
.balign 8
.914:
	.ascii "'%c'"
	.byte 0
/* end data */

.data
.balign 8
.916:
	.ascii "%c"
	.byte 0
/* end data */

.data
.balign 8
.922:
	.ascii "bool"
	.byte 0
/* end data */

.data
.balign 8
.930:
	.ascii "%s"
	.byte 0
/* end data */

.data
.balign 8
.943:
	.ascii "<%s at %p>"
	.byte 0
/* end data */

.data
.balign 8
.1186:
	.ascii "[%s:%d:%d] %s %s = %s\n"
	.byte 0
/* end data */

.data
.balign 8
.1194:
	.ascii "%s"
	.byte 0
/* end data */

.data
.balign 8
.1477:
	.ascii "Hello, world!"
	.byte 0
/* end data */

.data
.balign 8
.1479:
	.ascii "Hello"
	.byte 0
/* end data */

.data
.balign 8
.1487:
	.ascii "string"
	.byte 0
/* end data */

.data
.balign 8
.1496:
	.ascii "main"
	.byte 0
/* end data */

.data
.balign 8
.1498:
	.ascii "hello.l"
	.byte 0
/* end data */

.text
.balign 16
bool.to_string:
	endbr64
	cmpl $0, %edi
	jnz .Lbb2
	leaq .4(%rip), %rax
	jmp .Lbb3
.Lbb2:
	leaq .3(%rip), %rax
.Lbb3:
	ret
.type bool.to_string, @function
.size bool.to_string, .-bool.to_string
/* end function bool.to_string */

.text
.balign 16
nil:
	endbr64
	movl $0, %eax
	ret
.type nil, @function
.size nil, .-nil
/* end function nil */

.text
.balign 16
.globl string.len
string.len:
	endbr64
	pushq %rbp
	movq %rsp, %rbp
	callq strlen
	leave
	ret
.type string.len, @function
.size string.len, .-string.len
/* end function string.len */

.text
.balign 16
.globl string.contains
string.contains:
	endbr64
	pushq %rbp
	movq %rsp, %rbp
	subq $8, %rsp
	pushq %rbx
	callq strstr
	movq %rax, %rbx
	callq nil
	cmpq %rax, %rbx
	setnz %al
	movzbl %al, %eax
	popq %rbx
	leave
	ret
.type string.contains, @function
.size string.contains, .-string.contains
/* end function string.contains */

.text
.balign 16
.globl string.starts_with
string.starts_with:
	endbr64
	pushq %rbp
	movq %rsp, %rbp
	pushq %rbx
	pushq %r12
	movq %rdi, %r12
	movq %rsi, %rdi
	movq %rdi, %rbx
	callq string.len
	movq %r12, %rsi
	movq %rbx, %rdi
	movq %rax, %rdx
	callq strncmp
	cmpl $0, %eax
	setz %al
	movzbl %al, %eax
	popq %r12
	popq %rbx
	leave
	ret
.type string.starts_with, @function
.size string.starts_with, .-string.starts_with
/* end function string.starts_with */

.text
.balign 16
.globl string.concat
string.concat:
	endbr64
	pushq %rbp
	movq %rsp, %rbp
	subq $200, %rsp
	movq %rdi, -176(%rbp)
	movq %rsi, -168(%rbp)
	movq %rdx, -160(%rbp)
	movq %rcx, -152(%rbp)
	movq %r8, -144(%rbp)
	movq %r9, -136(%rbp)
	movaps %xmm0, -128(%rbp)
	movaps %xmm1, -112(%rbp)
	movaps %xmm2, -96(%rbp)
	movaps %xmm3, -80(%rbp)
	movaps %xmm4, -64(%rbp)
	movaps %xmm5, -48(%rbp)
	movaps %xmm6, -32(%rbp)
	movaps %xmm7, -16(%rbp)
	pushq %rbx
	pushq %r12
	pushq %r13
	pushq %r14
	pushq %r15
	movl 32(%rbp), %eax
	movslq %eax, %rcx
	addq $15, %rcx
	andq $-16, %rcx
	subq %rcx, %rsp
	movq %rsp, %r13
	movl $0, (%r13)
	movl $48, 4(%r13)
	movq %rbp, %rcx
	addq $64, %rcx
	movq %rcx, 8(%r13)
	movq %rbp, %rcx
	addq $-176, %rcx
	movq %rcx, 16(%r13)
	movslq %eax, %rcx
	imulq $8, %rcx, %rcx
	addq $15, %rcx
	andq $-16, %rcx
	subq %rcx, %rsp
	movq %rsp, %r12
	movslq %eax, %rax
	imulq $4, %rax, %rax
	addq $15, %rax
	andq $-16, %rax
	subq %rax, %rsp
	movq %rsp, %rbx
	movl $0, %r15d
	movl $0, %r14d
.Lbb14:
	movl 32(%rbp), %eax
	cmpl %eax, %r14d
	jge .Lbb20
	movslq %r14d, %rax
	imulq $8, %rax, %rax
	movq %r12, %rcx
	addq %rax, %rcx
	movslq 0(%r13), %rdx
	cmpl $48, %edx
	jb .Lbb17
	movq 8(%r13), %rax
	movq %rax, %rdx
	addq $8, %rdx
	movq %rdx, 8(%r13)
	jmp .Lbb18
.Lbb17:
	movq 16(%r13), %rax
	addq %rdx, %rax
	addl $8, %edx
	movl %edx, 0(%r13)
.Lbb18:
	movq (%rax), %rax
	movq %rax, (%rcx)
	movslq %r14d, %rcx
	movq %rcx, -192(%rbp)
	movslq %r14d, %rax
	movq (%r12, %rax, 8), %rdi
	callq string.len
	movq -192(%rbp), %rcx
	movl %eax, (%rbx, %rcx, 4)
	movslq %r14d, %rax
	movl (%rbx, %rax, 4), %eax
	movslq %eax, %rax
	addq %rax, %r15
	addl $1, %r14d
	jmp .Lbb14
.Lbb20:
	movq %r15, %rax
	addq $1, %rax
	imulq $1, %rax, %rdi
	callq malloc
	subq $16, %rsp
	movq %rsp, %rcx
	movq %rax, (%rcx)
	cmpl $0, %eax
	jz .Lbb32
	subq $16, %rsp
	movq %rsp, %r9
	movl $0, (%r9)
	movq %rax, %r8
	movl $0, %edx
	movl $0, %ecx
.Lbb23:
	movl 32(%rbp), %eax
	cmpl %eax, %edx
	jge .Lbb30
	movslq %edx, %rax
	movq (%r12, %rax, 8), %rsi
	subq $16, %rsp
	movq %rsp, %rax
	movq %rsi, (%rax)
	subq $16, %rsp
	movq %rsp, %rdi
	movl $0, (%rdi)
	movl $0, %eax
.Lbb26:
	movslq %edx, %r10
	movl (%rbx, %r10, 4), %r10d
	cmpl %r10d, %eax
	jge .Lbb29
	movslq %ecx, %r11
	movslq %eax, %r10
	movsbl (%rsi, %r10, 1), %r10d
	movb %r10b, (%r8, %r11, 1)
	addl $1, %ecx
	movl %ecx, (%r9)
	addl $1, %eax
	movl %eax, (%rdi)
	jmp .Lbb26
.Lbb29:
	addl $1, %edx
	jmp .Lbb23
.Lbb30:
	movq %r8, %rax
	movslq %ecx, %rcx
	movb $0, (%rax, %rcx, 1)
	jmp .Lbb33
.Lbb32:
	callq nil
.Lbb33:
	movq %rbp, %rsp
	subq $240, %rsp
	popq %r15
	popq %r14
	popq %r13
	popq %r12
	popq %rbx
	leave
	ret
.type string.concat, @function
.size string.concat, .-string.concat
/* end function string.concat */

.text
.balign 16
.globl __to_string
__to_string:
	endbr64
	pushq %rbp
	movq %rsp, %rbp
	subq $24, %rsp
	pushq %rbx
	pushq %r12
	pushq %r13
	pushq %r14
	pushq %r15
	movl %edx, %r12d
	movq %rsi, %rbx
	movq %rdi, %r14
	callq nil
	movq %rax, %r13
	callq nil
	movq %rbx, %rsi
	movq %rsi, %rbx
	leaq .786(%rip), %rsi
	movq %r14, %rdi
	callq string.contains
	movq %rbx, %rsi
	cmpl $0, %eax
	jnz .Lbb93
	movq %rsi, %rbx
	leaq .801(%rip), %rsi
	movq %r14, %rdi
	callq string.starts_with
	movq %rbx, %rsi
	cmpl $0, %eax
	jnz .Lbb86
	movq %rsi, %rbx
	leaq .849(%rip), %rsi
	movq %r14, %rdi
	callq string.starts_with
	movl %eax, %ecx
	movq %r13, %rax
	cmpl $0, %ecx
	jnz .Lbb38
	movq %rbx, %rsi
	movl $0, %r15d
	jmp .Lbb42
.Lbb38:
	movslq 0(%rbx), %rcx
	cmpl $48, %ecx
	jb .Lbb40
	movq 8(%rbx), %rax
	movq %rax, %rcx
	addq $8, %rcx
	movq %rcx, 8(%rbx)
	jmp .Lbb41
.Lbb40:
	movq 16(%rbx), %rax
	addq %rcx, %rax
	addl $8, %ecx
	movl %ecx, 0(%rbx)
.Lbb41:
	movl (%rax), %r13d
	subq $16, %rsp
	movq %rsp, %rax
	movl %r13d, (%rax)
	callq nil
	movl %r13d, %edx
	movq %rbx, %rsi
	movq %rax, %rdi
	movl %edx, %ecx
	movl %edx, %r13d
	leaq .855(%rip), %rdx
	movq %rsi, %rbx
	movl $0, %esi
	movl $0, %eax
	callq snprintf
	movl %eax, %r15d
	movslq %r15d, %rdi
	callq malloc
	movl %r13d, %edx
	movq %rbx, %rsi
	movq %rsi, %r13
	leaq .855(%rip), %rsi
	movq %rax, %rdi
	movq %rax, %rbx
	movl $0, %eax
	callq sprintf
	movq %r13, %rsi
	movq %rbx, %rax
.Lbb42:
	movq %rax, %r13
	movq %rsi, %rbx
	leaq .864(%rip), %rsi
	movq %r14, %rdi
	callq string.starts_with
	movl %eax, %ecx
	movq %r13, %rax
	cmpl $0, %ecx
	jnz .Lbb44
	movq %rbx, %rsi
	jmp .Lbb48
.Lbb44:
	movslq 0(%rbx), %rcx
	cmpl $48, %ecx
	jb .Lbb46
	movq 8(%rbx), %rax
	movq %rax, %rcx
	addq $8, %rcx
	movq %rcx, 8(%rbx)
	jmp .Lbb47
.Lbb46:
	movq 16(%rbx), %rax
	addq %rcx, %rax
	addl $8, %ecx
	movl %ecx, 0(%rbx)
.Lbb47:
	movq (%rax), %r13
	subq $16, %rsp
	movq %rsp, %rax
	movq %r13, (%rax)
	callq nil
	movq %r13, %rdx
	movq %rbx, %rsi
	movq %rax, %rdi
	movq %rdx, %rcx
	movq %rdx, %r13
	leaq .870(%rip), %rdx
	movq %rsi, %rbx
	movl $0, %esi
	movl $0, %eax
	callq snprintf
	movl %eax, %r15d
	movslq %r15d, %rdi
	callq malloc
	movq %r13, %rdx
	movq %rbx, %rsi
	movq %rsi, %r13
	leaq .870(%rip), %rsi
	movq %rax, %rdi
	movq %rax, %rbx
	movl $0, %eax
	callq sprintf
	movq %r13, %rsi
	movq %rbx, %rax
.Lbb48:
	movq %rax, %rbx
	movq %rsi, %r13
	leaq .881(%rip), %rsi
	movq %r14, %rdi
	callq string.starts_with
	movq %r13, %rsi
	movl %eax, %ecx
	cmpl $0, %ecx
	jz .Lbb51
	movq %rbx, %rax
	jmp .Lbb53
.Lbb51:
	movq %rsi, %r13
	leaq .884(%rip), %rsi
	movq %r14, %rdi
	callq string.starts_with
	movq %r13, %rsi
	movl %eax, %ecx
	movq %rbx, %rax
	cmpl $0, %ecx
	jnz .Lbb53
	movl $0, %ecx
.Lbb53:
	cmpl $0, %ecx
	jz .Lbb66
	subq $16, %rsp
	movq %rsp, %r13
	movq $0, (%r13)
	movq %rsi, %rbx
	leaq .892(%rip), %rsi
	movq %r14, %rdi
	callq string.starts_with
	cmpl $0, %eax
	jnz .Lbb60
	movslq 4(%rbx), %rcx
	cmpl $176, %ecx
	jb .Lbb57
	movq 8(%rbx), %rax
	movq %rax, %rcx
	addq $8, %rcx
	movq %rcx, 8(%rbx)
	jmp .Lbb58
.Lbb57:
	movq 16(%rbx), %rax
	addq %rcx, %rax
	addl $16, %ecx
	movl %ecx, 4(%rbx)
.Lbb58:
	movsd (%rax), %xmm0
	movsd %xmm0, (%r13)
	movsd %xmm0, -16(%rbp)
	jmp .Lbb65
.Lbb60:
	movslq 4(%rbx), %rcx
	cmpl $176, %ecx
	jb .Lbb62
	movq 8(%rbx), %rax
	movq %rax, %rcx
	addq $8, %rcx
	movq %rcx, 8(%rbx)
	jmp .Lbb63
.Lbb62:
	movq 16(%rbx), %rax
	addq %rcx, %rax
	addl $16, %ecx
	movl %ecx, 4(%rbx)
.Lbb63:
	movss (%rax), %xmm0
	cvtss2sd %xmm0, %xmm0
	movsd %xmm0, (%r13)
	movsd %xmm0, -16(%rbp)
.Lbb65:
	callq nil
	movl %r12d, %edx
	movq %rbx, %rsi
	movq %rax, %rdi
	movsd -16(%rbp), %xmm0
	movl %edx, %r12d
	leaq .890(%rip), %rdx
	movq %rsi, %rbx
	movl $0, %esi
	movl $1, %eax
	callq snprintf
	movl %eax, %r15d
	movslq %r15d, %rdi
	callq malloc
	movq %rbx, %rsi
	movsd -16(%rbp), %xmm0
	movq %rsi, %r13
	leaq .890(%rip), %rsi
	movq %rax, %rdi
	movq %rax, %rbx
	movl $1, %eax
	callq sprintf
	movq %r13, %rsi
	movq %rbx, %rax
.Lbb66:
	movq %rax, %rbx
	movq %rsi, %r13
	leaq .907(%rip), %rsi
	movq %r14, %rdi
	callq string.starts_with
	movl %r12d, %edx
	movl %eax, %ecx
	movq %rbx, %rax
	cmpl $0, %ecx
	jnz .Lbb68
	movq %r13, %rsi
	movq %r14, %rbx
	jmp .Lbb75
.Lbb68:
	movslq 0(%r13), %rcx
	cmpl $48, %ecx
	jb .Lbb70
	movq 8(%r13), %rax
	movq %rax, %rcx
	addq $8, %rcx
	movq %rcx, 8(%r13)
	jmp .Lbb71
.Lbb70:
	movq 16(%r13), %rax
	addq %rcx, %rax
	addl $8, %ecx
	movl %ecx, 0(%r13)
.Lbb71:
	movl (%rax), %r12d
	subq $16, %rsp
	movq %rsp, %rax
	movb %r12b, (%rax)
	cmpl $0, %edx
	jnz .Lbb73
	leaq .916(%rip), %rax
	jmp .Lbb74
.Lbb73:
	leaq .914(%rip), %rax
.Lbb74:
	movq %rax, %rbx
	movslq %r15d, %rdi
	callq malloc
	movq %r14, %rdx
	movq %rbx, %rsi
	movq %rdx, %rbx
	movsbl %r12b, %edx
	movq %rax, %rdi
	movq %rax, %r12
	movl $0, %eax
	callq sprintf
	movq %r13, %rsi
	movq %r12, %rax
.Lbb75:
	movq %rax, %r13
	movq %rsi, %r12
	leaq .922(%rip), %rsi
	movq %rbx, %rdi
	callq string.starts_with
	movl %eax, %ecx
	movq %r13, %rax
	cmpl $0, %ecx
	jnz .Lbb77
	movq %r12, %r13
	jmp .Lbb81
.Lbb77:
	movslq 0(%r12), %rcx
	cmpl $48, %ecx
	jb .Lbb79
	movq 8(%r12), %rax
	movq %rax, %rcx
	addq $8, %rcx
	movq %rcx, 8(%r12)
	jmp .Lbb80
.Lbb79:
	movq 16(%r12), %rax
	addq %rcx, %rax
	addl $8, %ecx
	movl %ecx, 0(%r12)
.Lbb80:
	movl (%rax), %edi
	callq bool.to_string
	movq %rax, %r13
	subq $16, %rsp
	movq %rsp, %rax
	movq %r13, (%rax)
	movq %r13, %rdi
	callq string.len
	movq %rax, %rdi
	addq $1, %rdi
	callq malloc
	movq %r13, %rdx
	movq %r12, %rsi
	movq %rsi, %r13
	leaq .930(%rip), %rsi
	movq %rax, %rdi
	movq %rax, %r12
	movl $0, %eax
	callq sprintf
	movq %r12, %rax
.Lbb81:
	movq %rax, %r12
	callq nil
	movq %r13, %rsi
	movq %rax, %rcx
	movq %r12, %rax
	cmpq %rcx, %rax
	jnz .Lbb97
	movslq 0(%rsi), %rcx
	cmpl $48, %ecx
	jb .Lbb84
	movq 8(%rsi), %rax
	movq %rax, %rcx
	addq $8, %rcx
	movq %rcx, 8(%rsi)
	jmp .Lbb85
.Lbb84:
	movq 16(%rsi), %rax
	addq %rcx, %rax
	addl $8, %ecx
	movl %ecx, 0(%rsi)
.Lbb85:
	movq (%rax), %r12
	subq $16, %rsp
	movq %rsp, %rax
	movq %r12, (%rax)
	callq nil
	movq %r12, %rcx
	movq %rbx, %rdx
	movq %rax, %rdi
	movq %rcx, %r8
	movq %rcx, %r12
	movq %rdx, %rcx
	movq %rdx, %rbx
	leaq .943(%rip), %rdx
	movl $0, %esi
	movl $0, %eax
	callq snprintf
	movslq %eax, %rdi
	callq malloc
	movq %r12, %rcx
	movq %rbx, %rdx
	leaq .943(%rip), %rsi
	movq %rax, %rdi
	movq %rax, %rbx
	movl $0, %eax
	callq sprintf
	movq %rbx, %rax
	jmp .Lbb97
.Lbb86:
	movl %r12d, %edx
	movslq 0(%rsi), %rcx
	cmpl $48, %ecx
	jb .Lbb89
	movq 8(%rsi), %rax
	movq %rax, %rcx
	addq $8, %rcx
	movq %rcx, 8(%rsi)
	jmp .Lbb90
.Lbb89:
	movq 16(%rsi), %rax
	addq %rcx, %rax
	addl $8, %ecx
	movl %ecx, 0(%rsi)
.Lbb90:
	movq (%rax), %rsi
	subq $16, %rsp
	movq %rsp, %rbx
	movq %rsi, (%rbx)
	cmpl $0, %edx
	jz .Lbb92
	subq $48, %rsp
	movq %rsp, %rax
	subq $32, %rsp
	movq %rsp, %rdi
	movl $3, (%rdi)
	movq %rdi, %rcx
	addq $4, %rcx
	leaq .811(%rip), %rdx
	movq %rdx, 4(%rdi)
	leaq .812(%rip), %rdx
	movq %rdx, 12(%rdi)
	leaq .813(%rip), %rdx
	movq %rdx, 20(%rdi)
	movq %rcx, (%rax)
	subq $32, %rsp
	movq %rsp, %rdi
	movl $3, (%rdi)
	movq %rdi, %rcx
	addq $4, %rcx
	leaq .823(%rip), %rdx
	movq %rdx, 4(%rdi)
	leaq .824(%rip), %rdx
	movq %rdx, 12(%rdi)
	leaq .825(%rip), %rdx
	movq %rdx, 20(%rdi)
	movq %rcx, 8(%rax)
	movl $3, 16(%rax)
	leaq .836(%rip), %rcx
	movq %rcx, 20(%rax)
	leaq .838(%rip), %rcx
	movq %rcx, 28(%rax)
	movl $350, 36(%rax)
	movl $34, 40(%rax)
	subq $48, %rsp
	movq %rsp, %rcx
	movq 0(%rax), %rdx
	movq %rdx, 0(%rcx)
	movq 8(%rax), %rdx
	movq %rdx, 8(%rcx)
	movq 16(%rax), %rdx
	movq %rdx, 16(%rcx)
	movq 24(%rax), %rdx
	movq %rdx, 24(%rcx)
	movq 32(%rax), %rdx
	movq %rdx, 32(%rcx)
	movq 40(%rax), %rax
	movq %rax, 40(%rcx)
	leaq .809(%rip), %rdx
	leaq .808(%rip), %rdi
	movl $0, %eax
	callq string.concat
	movq %rax, %rsi
	subq $-48, %rsp
	movq %rsi, (%rbx)
.Lbb92:
	movq %rsi, %rbx
	movq %rbx, %rdi
	callq string.len
	movq %rax, %rdi
	addq $1, %rdi
	callq malloc
	movq %rbx, %rsi
	movq %rax, %rbx
	movq %rbx, %rdi
	callq strcpy
	movq %rbx, %rax
	jmp .Lbb97
.Lbb93:
	movslq 0(%rsi), %rcx
	cmpl $48, %ecx
	jb .Lbb95
	movq 8(%rsi), %rax
	movq %rax, %rcx
	addq $8, %rcx
	movq %rcx, 8(%rsi)
	jmp .Lbb96
.Lbb95:
	movq 16(%rsi), %rax
	addq %rcx, %rax
	addl $8, %ecx
	movl %ecx, 0(%rsi)
.Lbb96:
	movq (%rax), %rbx
	subq $16, %rsp
	movq %rsp, %rax
	movq %rbx, (%rax)
	callq nil
	movq %rbx, %rdx
	movq %rax, %rdi
	movq %rdx, %rcx
	movq %rdx, %rbx
	leaq .792(%rip), %rdx
	movl $0, %esi
	movl $0, %eax
	callq snprintf
	movslq %eax, %rdi
	callq malloc
	movq %rbx, %rdx
	leaq .792(%rip), %rsi
	movq %rax, %rdi
	movq %rax, %rbx
	movl $0, %eax
	callq sprintf
	movq %rbx, %rax
.Lbb97:
	movq %rbp, %rsp
	subq $64, %rsp
	popq %r15
	popq %r14
	popq %r13
	popq %r12
	popq %rbx
	leave
	ret
.type __to_string, @function
.size __to_string, .-__to_string
/* end function __to_string */

.text
.balign 16
__print:
	endbr64
	pushq %rbp
	movq %rsp, %rbp
	subq $24, %rsp
	pushq %rbx
	pushq %r12
	pushq %r13
	pushq %r14
	pushq %r15
	movl %ecx, -16(%rbp)
	movl %edx, -12(%rbp)
	movl $0, %ebx
.Lbb100:
	movl 32(%rbp), %eax
	cmpl %eax, %ebx
	jge .Lbb108
	movq 24(%rbp), %rax
	movslq %ebx, %rcx
	movq (%rax, %rcx, 8), %r14
	subq $16, %rsp
	movq %rsp, %rax
	movq %r14, (%rax)
	movq 16(%rbp), %rax
	movslq %ebx, %rcx
	movq (%rax, %rcx, 8), %r15
	subq $16, %rsp
	movq %rsp, %rax
	movq %r15, (%rax)
	movl %esi, %edx
	movl %esi, %r13d
	movq %rdi, %rsi
	movq %rdi, %r12
	movq %r14, %rdi
	callq __to_string
	movq %r15, %r9
	movq %r14, %r8
	movl %r13d, %esi
	movq %rax, %rdi
	movl -12(%rbp), %r15d
	subq $16, %rsp
	movq %rsp, %rax
	movq %rdi, (%rax)
	cmpl $0, %r15d
	jnz .Lbb104
	movl %esi, %r14d
	movq %rdi, %rsi
	movq %rdi, %r13
	leaq .1194(%rip), %rdi
	movl $0, %eax
	callq printf
	movl %r15d, %edx
	movl %r14d, %esi
	movq %r13, %rdi
	movl %edx, %r14d
	movl %esi, %r13d
	jmp .Lbb106
.Lbb104:
	movl %r15d, %edx
	movl %esi, %r13d
	movq 44(%rbp), %rsi
	movl %edx, %r14d
	movl 52(%rbp), %edx
	movl 56(%rbp), %ecx
	subq $16, %rsp
	movq %rsp, %rax
	movq %rdi, 0(%rax)
	movq %rdi, %r15
	leaq .1186(%rip), %rdi
	movl $0, %eax
	callq printf
	movq %r15, %rdi
	subq $-16, %rsp
.Lbb106:
	callq free
	movl %r14d, %edx
	movl %r13d, %esi
	movq %r12, %rdi
	addl $1, %ebx
	jmp .Lbb100
.Lbb108:
	movl -16(%rbp), %ecx
	cmpl $0, %ecx
	jz .Lbb111
	movl $10, %edi
	callq putchar
.Lbb111:
	movl $0, %eax
	movq %rbp, %rsp
	subq $64, %rsp
	popq %r15
	popq %r14
	popq %r13
	popq %r12
	popq %rbx
	leave
	ret
.type __print, @function
.size __print, .-__print
/* end function __print */

.text
.balign 16
.globl io.println
io.println:
	endbr64
	pushq %rbp
	movq %rsp, %rbp
	subq $176, %rsp
	movq %rdi, -176(%rbp)
	movq %rsi, -168(%rbp)
	movq %rdx, -160(%rbp)
	movq %rcx, -152(%rbp)
	movq %r8, -144(%rbp)
	movq %r9, -136(%rbp)
	movaps %xmm0, -128(%rbp)
	movaps %xmm1, -112(%rbp)
	movaps %xmm2, -96(%rbp)
	movaps %xmm3, -80(%rbp)
	movaps %xmm4, -64(%rbp)
	movaps %xmm5, -48(%rbp)
	movaps %xmm6, -32(%rbp)
	movaps %xmm7, -16(%rbp)
	movl 32(%rbp), %eax
	movslq %eax, %rax
	addq $15, %rax
	andq $-16, %rax
	subq %rax, %rsp
	movq %rsp, %rdi
	movl $0, (%rdi)
	movl $48, 4(%rdi)
	movq %rbp, %rax
	addq $64, %rax
	movq %rax, 8(%rdi)
	movq %rbp, %rax
	addq $-176, %rax
	movq %rax, 16(%rdi)
	subq $48, %rsp
	movq %rsp, %rcx
	movq 16(%rbp), %rax
	movq %rax, 0(%rcx)
	movq 24(%rbp), %rax
	movq %rax, 8(%rcx)
	movq 32(%rbp), %rax
	movq %rax, 16(%rcx)
	movq 40(%rbp), %rax
	movq %rax, 24(%rcx)
	movq 48(%rbp), %rax
	movq %rax, 32(%rcx)
	movq 56(%rbp), %rax
	movq %rax, 40(%rcx)
	movl $1, %ecx
	movl $0, %edx
	movl $0, %esi
	callq __print
	subq $-48, %rsp
	movl $0, %eax
	movq %rbp, %rsp
	subq $176, %rsp
	leave
	ret
.type io.println, @function
.size io.println, .-io.println
/* end function io.println */

.text
.balign 16
.globl main
main:
	endbr64
	pushq %rbp
	movq %rsp, %rbp
	subq $80, %rsp
	movl $1, -32(%rbp)
	leaq -32(%rbp), %rax
	addq $4, %rax
	leaq .1479(%rip), %rcx
	movq %rcx, -28(%rbp)
	movq %rax, -80(%rbp)
	movl $1, -16(%rbp)
	leaq -16(%rbp), %rax
	addq $4, %rax
	leaq .1487(%rip), %rcx
	movq %rcx, -12(%rbp)
	movq %rax, -72(%rbp)
	movl $1, -64(%rbp)
	leaq .1496(%rip), %rax
	movq %rax, -60(%rbp)
	leaq .1498(%rip), %rax
	movq %rax, -52(%rbp)
	movl $4, -44(%rbp)
	movl $17, -40(%rbp)
	subq $48, %rsp
	movq %rsp, %rcx
	movq -80(%rbp), %rax
	movq %rax, 0(%rcx)
	movq -72(%rbp), %rax
	movq %rax, 8(%rcx)
	movq -64(%rbp), %rax
	movq %rax, 16(%rcx)
	movq -56(%rbp), %rax
	movq %rax, 24(%rcx)
	movq -48(%rbp), %rax
	movq %rax, 32(%rcx)
	movq -40(%rbp), %rax
	movq %rax, 40(%rcx)
	leaq .1477(%rip), %rdi
	movl $0, %eax
	callq io.println
	subq $-48, %rsp
	movl $0, %eax
	leave
	ret
.type main, @function
.size main, .-main
/* end function main */

.section .note.GNU-stack,"",@progbits
