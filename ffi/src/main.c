#include <signal.h>
#include <stdio.h>
#include <unistd.h>

#include "libp2p_chat.h"

static int is_running = 1;
static char buf[256];

static inline void signal_handler(int signal) {
  (void)signal; // unused
  is_running = 0;
}

int main(void) {
  signal(SIGINT, &signal_handler);
  libp2p_chat_t *libp2p = libp2p_chat_new();
  if (!libp2p) {
    fprintf(stderr, "Failed to create libp2p instance\n");
    return 1;
  }
  libp2p_chat_handle_t *libp2p_chat_handle = libp2p_chat_start(libp2p, 0);

  // start listening
  while (is_running) {
    int bytes = libp2p_chat_receive(libp2p, buf, sizeof(buf));
    if (bytes < 0) {
      fprintf(stderr, "Failed to receive messages\n");
      break;
    } else if (bytes == 0) {
      usleep(1000 * 250); // sleep 250ms
      continue;
    } else {
      buf[bytes] = '\0'; // ensure null-termination
      printf("%s\n", buf);
    }
  }

  libp2p_chat_stop(libp2p, libp2p_chat_handle);
  libp2p_chat_free(libp2p);
  signal(SIGINT, SIG_DFL);
  return 0;
}