// #include <jni.h>
#include <string>
#include <stdint.h>

extern int32_t entry_point(void);
extern int mini_entry(void);

int main() {
  mini_entry();
  entry_point();
}
