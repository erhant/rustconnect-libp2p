#ifndef LIBP2P_CHAT_H
#define LIBP2P_CHAT_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef struct libp2p_chat libp2p_chat_t;
typedef struct libp2p_chat_handle libp2p_chat_handle_t;

/**
 * @brief Create a new libp2p instance
 * @return libp2p_chat_t* pointer to the libp2p instance
 */
extern libp2p_chat_t *libp2p_chat_new(void);

/**
 * @brief Free the libp2p instance
 * @param ptr pointer to the libp2p instance
 */
extern void libp2p_chat_free(libp2p_chat_t *ptr);

/**
 * @brief Start listening on the given address
 * @param ptr pointer to the libp2p instance
 * @param port port to listen on
 * @return libp2p_chat_handle_t* handle for the thread that runs `libp2p`
 */
extern libp2p_chat_handle_t *libp2p_chat_start(libp2p_chat_t *ptr,
                                               unsigned short port);

/**
 * @brief Stop the libp2p instance
 * @param ptr pointer to the libp2p instance
 * @param handle_ptr handle for the thread that runs `libp2p`
 */
extern void libp2p_chat_stop(libp2p_chat_t *ptr,
                             libp2p_chat_handle_t *handle_ptr);

/**
 * @brief Publishes a message to all connected peers.
 * Will fail if there are no peers connected.
 *
 * @param ptr pointer to the libp2p instance
 * @param data raw bytes
 * @param data_len length of `data`
 * @return non-zero if error
 */
extern int libp2p_chat_publish(libp2p_chat_t *ptr, const void *data,
                               size_t data_len);

/**
 * @brief Receives a message that has been sent to this peer.
 * @param ptr pointer to the libp2p instance
 * @param buf buffer to store the message
 * @param buf_size size of the buffer
 * @return int number of bytes received, or -1 on error
 */
extern int libp2p_chat_receive(libp2p_chat_t *ptr, void *buf, size_t buf_size);

#endif // LIBP2P_CHAT_H