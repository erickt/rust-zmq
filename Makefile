all:
	rustc zmq.rc

test:
	rustc --test zmq.rc

example: all
	rustc -L . example.rs

msgsend-zmq: all
	rustc -L . msgsend-zmq.rs

clean:
	rm -rf zmq example msgsend-zmq *.dylib *.dSYM
