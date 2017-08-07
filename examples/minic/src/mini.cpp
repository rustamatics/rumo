#include <stdio.h>

extern "C" int mini_entry(void) {
  printf("\n mini_entry says hello!\n");
  return 33;
}
