RUSTPKG ?= rustpkg
RUST_FLAGS ?= -Z debug-info -O

all:
	$(RUSTPKG) $(RUST_FLAGS) install zmq

examples: all
	$(RUSTPKG) $(RUST_FLAGS) install examples/msgsend
	$(RUSTPKG) $(RUST_FLAGS) install examples/zguide/helloworld-client
	$(RUSTPKG) $(RUST_FLAGS) install examples/zguide/helloworld-server
	$(RUSTPKG) $(RUST_FLAGS) install examples/zguide/version
	$(RUSTPKG) $(RUST_FLAGS) install examples/zguide/weather-client
	$(RUSTPKG) $(RUST_FLAGS) install examples/zguide/weather-server

clean:
	rm -rf bin build lib
