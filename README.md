## Wasm89 - Wasm for bad compilers

This is a port of [wac-esp](https://github.com/grassel/wac-esp) with the following changes to support bad/outdated c89/c90 compilers, for when you have to deal with outdated vendor toolchains.

### Changes:
- Removed dependency on `stdint.h`
- Doesn't need `snprintf`
- Comments style changed from `//` to `/*`
- Removed ESP specific code
- Replaced `clz`/`popcnt`/etc builtins with slow c versions
- Replaced c89 missing float ops
- Moved inline variable definitions to start of function
- Added `extern "C"` guards for use from c++

### Compiler requirements:
This currently requires the following non-standard features, pull requests to change this are welcome:
- A 64 bit integer type `long long` / `_int64`
- Parser support for variadic macros (or remove all the logs)

### Compiler support:
- clang
- gcc
- MSVC 2008 or newer (Tested with 15.0.21022.08)


### Architecture support:
- ARMv4T
- ARM64
- x86
- x86_64
