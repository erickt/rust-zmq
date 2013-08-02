all:
	rustc lib.rs

test:
	rustc --test lib.rs

example: all
	rustc -L . example.rs

msgsend-zmq: all
	rustc -L . msgsend-zmq.rs

clean:
	rm -rf zmq example msgsend-zmq *.dylib *.dSYM
