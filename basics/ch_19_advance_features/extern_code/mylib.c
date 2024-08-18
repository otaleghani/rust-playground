// mylib.c
// this is a simple lib that we will define with the header file (the .h one)
#include "mylib.h"

int add(int a, int b) {
    return a + b;
}

// the steps to create a lib package are as follows
// here we are creating the .o file from the .c source code
// gcc -c -fPIC mylib.c -o mylib.o
//
// here we are creating the final .so file
// gcc -shared -o libmylib.so mylib.o
