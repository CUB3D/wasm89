all:
	mkdir -p bin
	clang platform.c thunk.c util.c wa.c main.c -std=c89 -g -O3 -lm -o bin/wasm89 -pedantic -v

