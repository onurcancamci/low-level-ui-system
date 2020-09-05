#ifndef _syscall_h
#define _syscall_h

static inline long
__internal_syscall(long n, long _a0, long _a1, long _a2, long _a3, long _a4, long _a5);

#define EXIT 500
#define PRINT_NUM 501
#define PUT_CHAR 502

void exit(int code) {
    __internal_syscall(EXIT, code, 0, 0, 0, 0, 0);
}

void print_number(int num) {
    __internal_syscall(PRINT_NUM, num, 0, 0, 0, 0, 0);
}

int put_char(char ch) {
    return __internal_syscall(PUT_CHAR, ch, 0, 0, 0, 0, 0);
}

int puts(char* ptr) {
    int ct = 0;
    for(int k = 0; ptr[k]; k++) {
        ct += __internal_syscall(PUT_CHAR, ptr[k], 0, 0, 0, 0, 0);
    }
    return ct;
}









































static inline long
__internal_syscall(long n, long _a0, long _a1, long _a2, long _a3, long _a4, long _a5)
{
  register long a0 asm("a0") = _a0;
  register long a1 asm("a1") = _a1;
  register long a2 asm("a2") = _a2;
  register long a3 asm("a3") = _a3;
  register long a4 asm("a4") = _a4;
  register long a5 asm("a5") = _a5;

  register long syscall_id asm("a7") = n;

  asm volatile ("ecall"
		: "+r"(a0) : "r"(a1), "r"(a2), "r"(a3), "r"(a4), "r"(a5), "r"(syscall_id));

  return a0;
}

#endif
