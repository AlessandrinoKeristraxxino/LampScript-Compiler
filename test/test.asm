global main
extern ExitProcess
extern printf

section .data
    format_num db "%llu", 0
    format_nl  db "%llu", 10, 0

section .text
main:
    mov rax, 5

    mov rdx, rax
    lea rcx, [rel format_num]

    sub rsp, 32
    call printf
    add rsp, 32