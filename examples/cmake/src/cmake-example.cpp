#include <stdio.h>
#include <stdint.h>

extern "C" {
  int mini_entry(void);
  int32_t entry_point(void);
}

int main() {
  mini_entry();
  int32_t entry = entry_point();
  printf("%d", (char)entry);
}
