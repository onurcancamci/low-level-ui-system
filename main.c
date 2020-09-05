#include "syscall.h"

void _start() {
    char* ptr = "hahahahha\n";
    //asm("ebreak");
    //asm("ebreak");
    short b = puts(ptr);

    int* arr = (int*)malloc(10);
    arr[0] = 1;
    arr[1] = 2;
    print_number(arr[0]);
    print_number(arr[1]);

    free(arr);
    exit(0);
}
