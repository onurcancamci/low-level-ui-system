#include "syscall.h"


int a = 0;

void _start() {
    char* ptr = "hahahahha\n";
    //asm("ebreak");
    //asm("ebreak");
    short b = puts(ptr);

    //int* arr = (int*)malloc(2 * sizeof(int));
    //arr[0] = 1;
    //arr[1] = 2;
    //print_number(arr[0]);
    //print_number(arr[1]);

    //free(arr);
    
    a = 5;

    exit(a);
}
