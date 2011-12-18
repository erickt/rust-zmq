CFLAGS ?=
LDFLAGS ?= -lzmq
RUSTC ?= rustc

all: libzmq.dylib test

zmqstubs.o: zmqstubs.c
	gcc -c -fPIC ${CFLAGS} -o $@ $<

libzmqstubs.a: zmqstubs.o
	ar -r $@ $<

libzmq.dylib: zmq.rc zmq.rs libzmqstubs.a
	${RUSTC} --lib zmq.rc

test: test.rs
	${RUSTC} -L . test.rs

clean:
	rm -rf *.a *.so *.dylib *.o *.dSYM test
