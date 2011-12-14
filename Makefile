CFLAGS ?=
LDFLAGS ?= -lzmq
RUSTC ?= rustc

all: libzmq.dylib test

rustzmq.o: rustzmq.c
	gcc -c -fPIC ${CFLAGS} -o rustzmq.o rustzmq.c

libzmq.dylib: zmq.rc zmq.rs rustzmq.o
	${RUSTC} --lib --link-args "${LDFLAGS} rustzmq.o" zmq.rc

test: test.rs
	${RUSTC} -L . test.rs

clean:
	rm -rf *.dylib *.o *.dSYM test
