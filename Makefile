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
	rm -rf example msgsend-zmq *.dylib *.dSYM
	rm -rf zguide/hwclient zguide/hwserver zguide/version zguide/wuclient zguide/wuserver zguide/*.dylib zguide/*.dSYM
	rm -rf src/zmq/lib src/zmq/*.dylib src/zmq/*.dSYM
