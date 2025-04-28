//! `extern` functions for FFI-support.
//!
//! This module contains functions that are callable from C/C++ code.
//! To be more type-safe, one can create a dummy struct in C/C++ and use it as a pointer.
//!
//! ```c
//! typedef struct libp2p_chat_t;
//! typedef struct libp2p_chat_handle libp2p_chat_handle_t;
//! ```
//!
//! Each function in this module is prefixed with `libp2p_chat_` to avoid name clashes.
//! They also have their declarations within their docstrings.
use debug_print::debug_eprintln;
use std::thread::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::ChatClient;

/// Creates a new chat client.
///
/// To be declared in C/C++ as:
/// ```c
/// extern libp2p_chat_t* libp2p_chat_new(void);
/// ```
///
/// Must be freed with [`libp2p_chat_free()`], otherwise will cause a **memory leak**.
#[unsafe(no_mangle)]
pub extern "C" fn libp2p_chat_new() -> *mut ChatClient {
    let (client, _ /* we could return a pointer to channel too! */) =
        ChatClient::new(CancellationToken::new()).expect("could not create LibP2P");
    Box::into_raw(Box::new(client))
}

/// Gracefully shutdown the chat client.
///
/// This first calls `cancel()` on the client, and then waits for the handle to finish.
/// It is expected to finish due to the internal cancellation token.
///
/// To be declared in C/C++ as:
/// ```c
/// extern int libp2p_chat_stop(libp2p_chat_t* ptr, libp2p_chat_handle_t* handle_ptr);
/// ```
#[unsafe(no_mangle)]
pub extern "C" fn libp2p_chat_stop(
    client_ptr: *mut ChatClient,
    handle_ptr: *mut JoinHandle<()>,
) -> i32 {
    let client = unsafe {
        assert!(!client_ptr.is_null(), "libp2p_ptr is null");
        &mut *client_ptr
    };

    let handle = unsafe {
        assert!(!handle_ptr.is_null(), "handle_ptr is null");
        Box::from_raw(handle_ptr)
    };

    client.cancel();
    match handle.join() {
        Ok(_) => 0,
        Err(e) => {
            debug_eprintln!("Error while stopping the client: {:?}", e);
            -1
        }
    }
}

/// Frees the memory allocated for the `LibP2P` instance.
///
/// To be declared in C/C++ as:
/// ```c
/// extern void libp2p_chat_free(libp2p_chat_t* ptr);
/// ```
///
/// Does no action if the pointer is `NULL`.
#[unsafe(no_mangle)]
pub extern "C" fn libp2p_chat_free(chat_ptr: *mut ChatClient) {
    if chat_ptr.is_null() {
        return;
    }

    // since the object was allocated by Rust, it must be freed by Rust as well;
    // so we use `Box::from_raw` to convert the raw pointer back into a `Box` and then drop it (explicitly).
    unsafe {
        drop(Box::from_raw(chat_ptr));
    }
}

/// Starts the chat client in a new thread, and returns a join handle.
///
/// To be declared in C/C++ as:
/// ```c
/// extern libp2p_chat_handle_t* libp2p_chat_start(libp2p_chat_t* ptr, const char* addr);
/// ```
///
/// The returned handle should be passed to [`libp2p_chat_stop()`] to stop the daemon gracefully.
#[unsafe(no_mangle)]
pub extern "C" fn libp2p_chat_start(client_ptr: *mut ChatClient, port: u16) -> *mut JoinHandle<()> {
    let client = unsafe {
        assert!(!client_ptr.is_null());
        &mut *client_ptr
    };

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("could not create runtime");

    let handle = std::thread::spawn(move || {
        debug_eprintln!("Starting the node!");
        rt.block_on(async { client.run(port).await.expect("could not run the client") });
    });

    Box::into_raw(Box::new(handle))
}

/// Sends raw bytes to all connected peers in the network.
///
/// To be declared in C/C++ as:
/// ```c
/// extern int libp2p_chat_publish(libp2p_chat_t* ptr, const char* data, size_t data_len);
/// ```
///
/// Returns non-zero on error, such as when there are no peers to send a message to.
#[unsafe(no_mangle)]
pub fn libp2p_chat_publish(
    client_ptr: *mut ChatClient,
    data_ptr: *const u8,
    data_len: usize,
) -> i32 {
    let client = unsafe {
        assert!(!client_ptr.is_null());
        &mut *client_ptr
    };

    let data = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };

    match client.publish(data) {
        Ok(_) => 0,
        Err(e) => {
            debug_eprintln!("Error while publishing: {:?}", e);
            -1
        }
    }
}

/// Pops a message from the chat client.
///
/// To be declared in C/C++ as:
/// ```c
/// extern int libp2p_chat_receive(libp2p_chat_t *ptr, void *buf, size_t buf_size);
/// ```
///
/// Returns the number of bytes received on success; othewrwise, returns -1.
#[unsafe(no_mangle)]
pub fn libp2p_chat_receive(client_ptr: *mut ChatClient, buf: *const u8, buf_size: usize) -> i32 {
    let client = unsafe {
        assert!(!client_ptr.is_null());
        &mut *client_ptr
    };

    if let Some((_, msg)) = client.received.pop_front() {
        let msg_len: usize = msg.len();

        if msg_len == 0 {
            // if the message is empty, we cannot copy the data
            // but the message is consumed nevertheless
            0
        } else if buf_size < msg_len {
            // if the buffer is too small, we cannot copy the data
            // but the message is consumed nevertheless
            -1
        } else {
            unsafe {
                std::ptr::copy_nonoverlapping(msg.as_ptr(), buf as *mut u8, msg_len);
            }
            msg_len as i32
        }
    } else {
        0
    }
}
