#ifndef UTIL_H
#define UTIL_H

#include <stdio.h>

#include "fixes.h"
#include <stdarg.h>


/*
#define DEBUG 0
#define INFO 0
#define WARN 0
#define TRACE 0
*/


#define DEBUG 1
#define INFO 1
#define WARN 1
#define TRACE 1


#define FATAL(...) { \
    fprintf(stderr, "Error(%s:%d): ", __FILE__, __LINE__); \
    fprintf(stderr, __VA_ARGS__); exit(1); \
}

#ifndef ASSERT
#define ASSERT(exp, ...) { \
    if (! (exp)) { \
        fprintf(stderr, "Assert Failed (%s:%d): ", __FILE__, __LINE__); \
        fprintf(stderr, __VA_ARGS__); exit(1); \
    } \
}
#endif


void wa_debug(char* c, ...);
void wa_info(char* c, ...);
void wa_warn(char* c, ...);
void wa_error(char* c, ...);



uint64_t read_LEB(uint8_t *bytes, uint32_t *pos, uint32_t maxbits);
uint64_t read_LEB_signed(uint8_t *bytes, uint32_t *pos, uint32_t maxbits);

uint32_t read_uint32(uint8_t *bytes, uint32_t *pos);

char *read_string(uint8_t *bytes, uint32_t *pos, uint32_t *result_len);

/* Math*/
void sext_8_32(uint32_t *val);
void sext_16_32(uint32_t *val);
void sext_8_64(uint64_t *val);
void sext_16_64(uint64_t *val);
void sext_32_64(uint64_t *val);
uint32_t rotl32 (uint32_t n, unsigned int c);
uint32_t rotr32 (uint32_t n, unsigned int c);
uint64_t rotl64(uint64_t n, unsigned int c);
uint64_t rotr64(uint64_t n, unsigned int c);
double wa_fmax(double a, double b);
double wa_fmin(double a, double b);

#endif
