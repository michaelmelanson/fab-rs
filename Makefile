
all: build test

build:
	cargo build
	
test: comments dep echo envvar vars failure

comments: build
	target/debug/make -f examples/comments.makefile

dep: build
	target/debug/make -f examples/dep.makefile
	
echo: build
	target/debug/make -f examples/echo.makefile
	
envvar: build
	target/debug/make -f examples/envvar.makefile
	
vars: build
	target/debug/make -f examples/vars.makefile
	
failure: build
	target/debug/make -f examples/failure.makefile || exit 0