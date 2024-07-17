#include <stdio.h>
#include <stdlib.h>

#include <limits.h>
#include <math.h>
#include <string.h>

#include "util.h"

int log_level = 0;

void wa_set_log_level(int ll) {
    log_level = ll;
}

bool should_trace(void) {
    return log_level > 4;
}

void wa_trace(char* c, ...)
{
if (log_level > 4) {
    va_list ap;
    va_start(ap, c);
    vfprintf(stdout, c, ap);
    va_end(ap);
}
}

void wa_debug(char* c, ...)
{
if (log_level > 3) {
    va_list ap;
    va_start(ap, c);
    vfprintf(stdout, c, ap);
    va_end(ap);
}
}

void wa_info(char* c, ...)
{
if (log_level > 2) {
    va_list ap;
    va_start(ap, c);
    vfprintf(stdout, c, ap);
    va_end(ap);
}
}

void wa_warn(char* c, ...)
{
if (log_level > 1) {
    va_list ap;
    va_start(ap, c);
    vfprintf(stderr, c, ap);
    va_end(ap);
}
}

void wa_error(char* c, ...)
{
if (log_level > 0) {
    va_list ap;
    va_start(ap, c);
    vfprintf(stderr, c, ap);
    va_end(ap);
}
}
