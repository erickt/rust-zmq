all:
	rustpkg install zmq

examples: all
	rustpkg install examples/msgsend
	rustpkg install examples/zguide/helloworld-client
	rustpkg install examples/zguide/helloworld-server
	rustpkg install examples/zguide/version
	rustpkg install examples/zguide/weather-client
	rustpkg install examples/zguide/weather-server

clean:
	rm -rf bin build lib
