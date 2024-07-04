#include "wa_result.h"

result_t res_new_ok(void) {
	result_t x;
	x.status= S_OK;
	x.msg = 0;
	return x; 
}

result_t res_new_err(const char* msg) {
	result_t x;
	x.status= S_ERR;
	x.msg = msg;
	return x; 
}

bool res_ok(result_t r) {
	return r.status == S_OK; 
}
bool res_err(result_t r) {
	return r.status == S_ERR; 
}
const char* res_err_msg(result_t r) {
	return r.msg; 
}
