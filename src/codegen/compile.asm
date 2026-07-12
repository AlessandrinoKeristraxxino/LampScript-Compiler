global main
extern ExitProcess
extern printf

section .data
    format_num db "%llu", 0
    format_nl  db "%llu", 10, 0

section .text
main:
    push rbp
    mov rbp, rsp
    sub rsp, 256

    ; <--- here the compiled code 

    mov rcx, 0
    call ExitProcess