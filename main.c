#include "syscall.h"

void _start() {
    char* ptr = "hahahahha\n";
    //asm("ebreak");
    //asm("ebreak");
    short b = puts(ptr);
    print_number(b);
    exit(0);
}
