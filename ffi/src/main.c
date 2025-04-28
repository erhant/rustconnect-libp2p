#include <signal.h>
#include <stdbool.h>
#include <stdio.h>
#include <unistd.h>

#include "libp2p_chat.h"

static bool is_running;
static char buf[256];

/// Signal handler to stop the program gracefully,
/// will simply reset `is_running`.
static inline void signal_handler(int signal) {
  (void)signal; // unused
  is_running = false;
}

int main(void) {
  signal(SIGINT, &signal_handler);

  // enables logging, respects `RUST_LOG` environment variable
  libp2p_chat_enable_logs();

  // create a new libp2p instance
  libp2p_chat_t *libp2p_chat = libp2p_chat_new();
  if (!libp2p_chat) {
    fprintf(stderr, "Failed to create libp2p chat instance\n");
    return 1;
  }
  libp2p_chat_handle_t *libp2p_chat_handle = libp2p_chat_start(libp2p_chat, 0);

  // start listening
  is_running = true;
  while (is_running) {
    int bytes = libp2p_chat_receive(libp2p_chat, buf, sizeof(buf));
    if (bytes < 0) {
      // something went wrong
      fprintf(stderr, "Failed to receive messages\n");
      break;
    } else if (bytes == 0) {
      // no messages received, wait a bit and then poll again
      usleep(1000 * 250); // (ms)
      continue;
    } else {
      // message received!
      buf[bytes] = '\0'; // ensure null-termination
      printf("%s\n", buf);
    }
  }

  // gracefully stop the libp2p instance
  libp2p_chat_stop(libp2p_chat, libp2p_chat_handle);

  // free the memory
  libp2p_chat_free(libp2p_chat);
  signal(SIGINT, SIG_DFL);
  return 0;
}