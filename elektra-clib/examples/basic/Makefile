GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)
ELEKTRA_INCLUDES = /home/hoan/Development/libelektra/elektra-install/include/elektra
LIBRARY_PATH = /home/hoan/Development/elektra-rust/elektra-clib/target/release

run: clean build
	LD_LIBRARY_PATH=$(LIBRARY_PATH) ./basic

clean:
	$(CARGO_BIN) clean
	rm -f basic

build:
	$(CARGO_BIN) build --release
	$(GCC_BIN) -o basic main.c -I$(ELEKTRA_INCLUDES)  -L$(LIBRARY_PATH) -lelektra_rust