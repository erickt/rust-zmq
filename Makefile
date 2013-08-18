all:
	rustc lib.rs

test: all
	rustc --test lib.rs
	rustc -L . example.rs
	rustc -L . zguide/hwclient.rs
	rustc -L . zguide/hwserver.rs
	rustc -L . zguide/version.rs
	rustc -L . zguide/wuclient.rs
	rustc -L . zguide/wuserver.rs
	rustc -L . msgsend-zmq.rs

clean:
	rm -rf zmq example msgsend-zmq *.dylib *.dSYM
