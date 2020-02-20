#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  const char *name;
} Test;

typedef struct {
  int32_t x;
  int32_t y;
  Test t;
} Position;

void rust_log(const char *message);

void translate(Position *pos);

void tst(Test *t);
