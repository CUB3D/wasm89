#include "wa_result.h"
#include <string.h>
#include <stdlib.h>

result_t res_new_ok(void) {
	result_t x;
	x.status= S_OK;
	x.msg = 0;
	return x;
}

result_t res_new_err(char* msg) {
	result_t x;
	x.status= S_ERR;
	x.msg = msg;
	return x;
}

result_t res_new_nest(result_t parent, char *msg) {
    result_t x;
    x.status = S_ERR_NEST;
    x.msg = msg;
    x.parent = res_clone(&parent);
    return x;
}

bool res_ok(result_t r) {
	return r.status == S_OK;
}
bool res_err(result_t r) {
	return r.status == S_ERR || r.status == S_ERR_NEST;
}
const char* res_err_msg(result_t r) {
	return r.msg;
}

result_t* res_clone(result_t* res) {
    result_t* nres = (result_t*)calloc(1, sizeof(result_t));
    memcpy(nres, res, sizeof(result_t));
    return nres;
}
