all: libzmq.dylib test

rustzmq.o: rustzmq.c
	gcc -c -fPIC -I /opt/local/include -o rustzmq.o rustzmq.c

libzmq.dylib: zmq.rc zmq.rs rustzmq.o
	../rust/run-rustc --lib --link-args "-L /opt/local/lib -lzmq rustzmq.o" zmq.rc

test: test.rs
	../rust/run-rustc -L . test.rs

clean:
	rm -rf *.dylib *.o *.dSYM
