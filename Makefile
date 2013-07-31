all:
	rustc zmq.rs

test:
	rustc --test zmq.rs

example: all
	rustc -L . example.rs

msgsend-zmq: all
	rustc -L . msgsend-zmq.rs

clean:
	rm -rf zmq example msgsend-zmq *.dylib *.dSYM
