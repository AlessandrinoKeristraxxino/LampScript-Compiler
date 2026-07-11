global main
extern ExitProcess

section .data
    my_string db "string", 0

section .text
main:
    mov rax, 5
    lea rax, [rel my_string]