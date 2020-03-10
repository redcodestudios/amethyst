#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct EntityComponents EntityComponents;

const Transform **get_transform(EntityComponents *components);

void move_up(Transform *t, float amount);

void print_addr(Transform *t);

void rust_log(const char *message);

void set_transform(EntityComponents *components, Transform *t);

float translation_y(Transform *t);
