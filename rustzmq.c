#include <stdlib.h>
#include <zmq.h>

zmq_msg_t* rustzmq_msg_create() {
  return (zmq_msg_t*)malloc(sizeof(zmq_msg_t));
}

void rustzmq_msg_destroy(zmq_msg_t* msg) {
  return free(msg);
}
