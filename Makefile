all:
	mkdir -p bin
	clang platform.c thunk.c util.c wa.c main.c wa_result.c -std=c89 -g -O2 -lm -o bin/wasm89 -pedantic -Wno-long-long -Wno-variadic-macros
	clang platform.c thunk.c util.c wa.c wa_result.c -std=c89 -g -O2 -lm -shared -fPIC -o bin/libwasm89.so -pedantic -Wno-long-long -Wno-variadic-macros
check:
	clang-check --analyze platform.c thunk.c util.c wa.c  wa_result.c --extra-arg="-std=c89"

tb:
	clang platform.c thunk.c util.c wa.c wa_result.c -std=c89 -g -O2 -lm -shared -fPIC -o bin/libwasm89.so
