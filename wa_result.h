#ifndef WA_RESULT_H
#define WA_RESULT_H
#include "fixes.h"

typedef enum { S_OK = 0, S_ERR = 1, S_ERR_NEST = 2 } status_t;

typedef struct {
    status_t status;
    char* msg;
    void* parent;
} result_t;

result_t res_new_ok(void);
result_t res_new_err(char* msg);
result_t res_new_nest(result_t parent, char* msg);

bool res_ok(result_t r);
bool res_err(result_t r);
const char* res_err_msg(result_t r);

result_t* res_clone(result_t* res);

#endif
