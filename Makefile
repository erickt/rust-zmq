all:
	rustc src/zmq/lib.rs

test: all
	rustc --test src/zmq/lib.rs
	rustc -L src/zmq example.rs
	rustc -L src/zmq zguide/hwclient.rs
	rustc -L src/zmq zguide/hwserver.rs
	rustc -L src/zmq zguide/version.rs
	rustc -L src/zmq zguide/wuclient.rs
	rustc -L src/zmq zguide/wuserver.rs
	rustc -L src/zmq msgsend-zmq.rs

clean:
	rm -rf zmq example msgsend-zmq *.dylib *.dSYM
