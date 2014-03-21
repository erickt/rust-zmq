# Rust parameters
ARCH ?= `uname -s`-`uname -r`-`uname -m`
SRC ?= src
BUILD ?= build
RUSTC ?= rustc -W unnecessary-typecast -W unused-result -W non-camel-case-types -W non-uppercase-statics -L $(BUILD)
LIBZMQ_SRC ?= $(SRC)/zmq/lib.rs

all: clean lib examples

examples: msgsend helloworld weather version

clean:
		rm -fr $(BUILD)/* || true

$(BUILD):
		mkdir -p $(BUILD)

lib: build
		$(RUSTC) --out-dir $(BUILD) $(LIBZMQ_SRC) 

msgsend: $(BUILD) lib
		$(RUSTC) src/examples/msgsend/main.rs -o $(BUILD)/msgsend

helloworld: $(BUILD) lib
		$(RUSTC) src/examples/zguide/helloworld-server/main.rs -o $(BUILD)/helloworld-server
		$(RUSTC) src/examples/zguide/helloworld-client/main.rs -o $(BUILD)/helloworld-client

weather: $(BUILD) lib
		$(RUSTC) src/examples/zguide/weather-server/main.rs -o $(BUILD)/weather-server
		$(RUSTC) src/examples/zguide/weather-client/main.rs -o $(BUILD)/weather-client

version: $(BUILD) lib
		$(RUSTC) src/examples/zguide/version/main.rs -o $(BUILD)/version
