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
void wa_trace(char* c, ...);

/* type readers */


static uint64_t read_LEB_(uint8_t *bytes, uint32_t *pos, uint32_t maxbits, bool sign) {
    uint64_t result = 0;
    uint32_t shift = 0;
    uint32_t bcnt = 0;
    uint32_t startpos = *pos;
    uint64_t  byte;

    while (true) {
        byte = bytes[*pos];
        *pos += 1;
        result |= ((byte & 0x7f)<<shift);
        shift += 7;
        if ((byte & 0x80) == 0) {
            break;
        }
        bcnt += 1;
        if (bcnt > (maxbits + 7 - 1) / 7) {
            FATAL("Unsigned LEB at byte %d overflow", startpos);
        }
    }
    if (sign && (shift < maxbits) && (byte & 0x40)) {
        /* Sign extend */
        result |= - (1 << shift);
    }
    return result;
}

static uint64_t read_LEB(uint8_t *bytes, uint32_t *pos, uint32_t maxbits) {
    return read_LEB_(bytes, pos, maxbits, false);
}

static uint64_t read_LEB_signed(uint8_t *bytes, uint32_t *pos, uint32_t maxbits) {
    return read_LEB_(bytes, pos, maxbits, true);
}

static uint32_t read_uint32(uint8_t *bytes, uint32_t *pos) {
    *pos += 4;
    return ((uint32_t *) (bytes+*pos-4))[0];
}

/* Reads a string from the bytes array at pos that starts with a LEB length
 if result_len is not NULL, then it will be set to the string length*/
static char *read_string(uint8_t *bytes, uint32_t *pos, uint32_t *result_len) {
    uint32_t str_len = read_LEB(bytes, pos, 32);
    char * str = (char*)calloc(str_len+1, 1);
    memcpy(str, bytes+*pos, str_len);
    str[str_len] = '\0';
    *pos += str_len;
    if (result_len) { *result_len = str_len; }
    return str;
}


/* Maths */

/* Inplace sign extend */
static void sext_8_32(uint32_t *val) {
    if (*val & 0x80) { *val = *val | 0xffffff00; }
}
static void sext_16_32(uint32_t *val) {
    if (*val & 0x8000) { *val = *val | 0xffff0000; }
}
static void sext_8_64(uint64_t *val) {
    if (*val & 0x80) { *val = *val | 0xffffffffffffff00; }
}
static void sext_16_64(uint64_t *val) {
    if (*val & 0x8000) { *val = *val | 0xffffffffffff0000; }
}
static void sext_32_64(uint64_t *val) {
    if (*val & 0x80000000) { *val = *val | 0xffffffff00000000; }
}

/* Based on: http://stackoverflow.com/a/776523/471795 */
static uint32_t rotl32(uint32_t n, unsigned int c) {
  const unsigned int mask = (CHAR_BIT*sizeof(n)-1);
  c = c % 32;
  c &= mask;
  return (n<<c) | (n>>( (-c)&mask ));
}

static uint32_t rotr32(uint32_t n, unsigned int c) {
  const unsigned int mask = (CHAR_BIT*sizeof(n)-1);
  c = c % 32;
  c &= mask;
  return (n>>c) | (n<<( (-c)&mask ));
}

static uint64_t rotl64(uint64_t n, unsigned int c) {
  const unsigned int mask = (CHAR_BIT*sizeof(n)-1);
  c = c % 64;
  c &= mask;
  return (n<<c) | (n>>( (-c)&mask ));
}

static uint64_t rotr64(uint64_t n, unsigned int c) {
  const unsigned int mask = (CHAR_BIT*sizeof(n)-1);
  c = c % 64;
  c &= mask;
  return (n>>c) | (n<<( (-c)&mask ));
}

static int _signbit(double x) {
    union {
        double d;
        uint64_t u64;
    } u;
    u.d =  x;
    return u.u64 >> 63;
}

static uint32_t u32_max(uint32_t a, uint32_t b) {
    return a>b?a:b;
}

static uint32_t u32_min(uint32_t a, uint32_t b) {
    return a<b?a:b;
}

#ifndef isnan
static int isnan(double x) { return (x != x); }
#endif


static double wa_fmax(double a, double b) {
    if (isnan(a)) return a;
    if (isnan(b)) return b;
    if (_signbit(a) != _signbit(b)) return _signbit(a) ? b : a;
    return a > b ? a : b;
}
static double wa_fmin(double a, double b) {
    if (isnan(a)) return a;
    if (isnan(b)) return b;
    if (_signbit(a) != _signbit(b)) return _signbit(a) ? a : b;

    return a < b ? a : b;
}
bool should_trace(void);

#endif
