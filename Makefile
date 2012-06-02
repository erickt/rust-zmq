all:
	rustc zmq.rc

test:
	rustc --test zmq.rc

example: all
	rustc -L . example.rs

clean:
	rm -rf zmq example *.dylib *.dSYM
