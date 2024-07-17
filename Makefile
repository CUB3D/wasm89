all:
	mkdir -p bin
	clang platform.c thunk.c util.c wa.c main.c wa_result.c -Wall -std=c89 -g -O2 -lm -o bin/wasm89 -pedantic -Wno-long-long -Wno-variadic-macros

	clang platform.c thunk.c util.c wa.c wa_result.c -Wall -std=c89 -g -O2 -lm -shared -fPIC -o bin/libwasm89_asan.so -pedantic -fno-omit-frame-pointer -Wno-long-long -Wno-variadic-macros -fsanitize=address -shared-libasan
	clang platform.c thunk.c util.c wa.c wa_result.c -Wall -std=c89 -g -O2 -lm -shared -fPIC -o bin/libwasm89_ubsan.so -pedantic -fno-omit-frame-pointer -Wno-long-long -Wno-variadic-macros -fsanitize=undefined,local-bounds,float-divide-by-zero -shared-libasan -fsanitize-trap=all
	clang platform.c thunk.c util.c wa.c wa_result.c -Wall -std=c89 -g -O2 -lm -shared -fPIC -o bin/libwasm89.so -pedantic -fno-omit-frame-pointer -Wno-long-long -Wno-variadic-macros
