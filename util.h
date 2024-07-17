#ifndef UTIL_H
#define UTIL_H

#include <stdio.h>

#include "fixes.h"
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

#include <limits.h>
#include <math.h>
#include <string.h>

#define FATAL(...)                                                             \
  {                                                                            \
    fprintf(stderr, "Error(%s:%d): ", __FILE__, __LINE__);                     \
    fprintf(stderr, __VA_ARGS__);                                              \
    exit(1);                                                                   \
  }

#ifndef ASSERT
#define ASSERT(exp, ...)                                                       \
  {                                                                            \
    if (!(exp)) {                                                              \
      fprintf(stderr, "Assert Failed (%s:%d): ", __FILE__, __LINE__);          \
      fprintf(stderr, __VA_ARGS__);                                            \
      exit(1);                                                                 \
    }                                                                          \
  }
#endif

void wa_debug(char *c, ...);
void wa_info(char *c, ...);
void wa_warn(char *c, ...);
void wa_error(char *c, ...);
void wa_trace(char *c, ...);

bool should_trace(void);

#endif
