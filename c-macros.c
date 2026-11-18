#include <stdio.h>

#define SQUARE(x) x * x 
// A fix:
// #define SQUARE(x) (x) * (x)

int main() {
  int result = SQUARE(2 + 3);
  // int result = 2 + 3 * 2 + 3;
  printf("result: %d\n", result);
}