all:
	gcc -dynamiclib -I /opt/local/include -L /opt/local/lib -lzmq -o librustzmq.dylib rustzmq.c
