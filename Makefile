
all: build test

build:
	cargo build
	
test: comments dep echo envvar vars failure

comments: build
	target/debug/fab -f examples/comments.makefile

dep: build
	target/debug/fab -f examples/dep.makefile
	
echo: build
	target/debug/fab -f examples/echo.makefile
	
envvar: build
	target/debug/fab -f examples/envvar.makefile
	
vars: build
	target/debug/fab -f examples/vars.makefile
	
failure: build
	target/debug/fab -f examples/failure.makefile || exit 0