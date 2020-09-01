//#include <internal_syscall.h>

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





int foo(int a, int b) {
    return a+b;
}

long test(int a, int b) {
    return __internal_syscall(1,0,0,0,0,0,0);
}

int _start() {
    //syscall(1);
    //asm("mv a0, a5");
    int x = foo(1,2);
    foo(x,3);
    

    return test(1,2);
}
