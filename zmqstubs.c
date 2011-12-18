#include <stdlib.h>
#include <zmq.h>

zmq_msg_t* zmqstubs_msg_create() {
  return (zmq_msg_t*)malloc(sizeof(zmq_msg_t));
}

void zmqstubs_msg_destroy(zmq_msg_t* msg) {
  return free(msg);
}
