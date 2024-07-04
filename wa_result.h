#ifndef WA_RESULT_H
#define WA_RESULT_H
#include "fixes.h"
typedef enum {
	S_OK = 0,
	S_ERR = 1
}status_t;

typedef struct {
    status_t status;
    const char* msg;
} result_t;


result_t res_new_ok(void);
result_t res_new_err(const char* msg);

bool res_ok(result_t r);
bool res_err(result_t r);
const char* res_err_msg(result_t r);

#endif
