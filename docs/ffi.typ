#import "@preview/touying:0.6.1": pause

= FFI

== Why?

You love rust-libp2p, but you want to use it in a different language? You can do that with the FFI (Foreign Function Interface) bindings. Simply add the following to your Cargo.toml, and it will output shared library:

```toml
[lib]
crate-type = [
  "cdylib", # allows C/C++ to use this library
  "rlib",   # allows Rust to use this library
]
```

#pause

We will use the following types for our pointer type that lives in C.

```c
typedef struct libp2p_chat libp2p_chat_t;
typedef struct libp2p_chat_handle libp2p_chat_handle_t;
```

== Allocations

#text(size: 0.80em)[
  ```rust
  // extern libp2p_chat_t *libp2p_chat_new(void);
  #[unsafe(no_mangle)]
  pub extern "C" fn libp2p_chat_new() -> *mut ChatClient {
      let client =
          ChatClient::new(CancellationToken::new()).unwrap();
      Box::into_raw(Box::new(client))
  }
  // extern void libp2p_chat_free(libp2p_chat_t *ptr);
  #[unsafe(no_mangle)]
  pub extern "C" fn libp2p_chat_free(chat_ptr: *mut ChatClient) {
      if chat_ptr.is_null() { return; }
      unsafe {
          drop(Box::from_raw(chat_ptr));
      }
  }
  ```
]

== Starting

#text(size: 0.7em)[
  ```rust
  // extern libp2p_chat_handle_t* libp2p_chat_start(libp2p_chat_t* ptr, unsigned short port);
  #[unsafe(no_mangle)]
  pub extern "C" fn libp2p_chat_start(client_ptr: *mut ChatClient, port: u16) -> *mut JoinHandle<()> {
      let client = unsafe {
          assert!(!client_ptr.is_null());
          &mut *client_ptr
      };

      let rt = tokio::runtime::Builder::new_multi_thread()
          .enable_all().build().unwrap();
      let handle = std::thread::spawn(move || {
          rt.block_on(async { client.run(port).await.expect("could not run the client") });
      });
      Box::into_raw(Box::new(handle))
  }
  ```
]

== Stopping

#text(size: 0.7em)[
  ```rust
  // extern void libp2p_chat_stop(libp2p_chat_t *ptr, libp2p_chat_handle_t *handle_ptr);
  #[unsafe(no_mangle)]
  pub extern "C" fn libp2p_chat_stop(
      client_ptr: *mut ChatClient,
      handle_ptr: *mut JoinHandle<()>,
  ) -> i32 {
      let client = unsafe {
          assert!(!client_ptr.is_null());
          &mut *client_ptr
      };
      let handle = unsafe {
          assert!(!handle_ptr.is_null());
          Box::from_raw(handle_ptr)
      };
      client.cancel();
      match handle.join() {
          Ok(_) => 0,
          Err(err) => -1
      }
  }
  ```
]

== Receive

#text(size: 0.7em)[
  ```rust
  // extern int libp2p_chat_receive(libp2p_chat_t *ptr, void *buf, size_t buf_size);
  #[unsafe(no_mangle)]
  pub fn libp2p_chat_receive(client_ptr: *mut ChatClient, buf: *const u8, buf_size: usize) -> i32 {
      let client = unsafe {
          assert!(!client_ptr.is_null());
          &mut *client_ptr
      };
      if let Some((_, msg)) = client.received.pop_front() {
          let msg_len: usize = msg.len();
          if msg_len == 0 { 0 }
          else if buf_size < msg_len { -1 }
          else {
              unsafe {
                  std::ptr::copy_nonoverlapping(msg.as_ptr(), buf as *mut u8, msg_len);
              }
              msg_len as i32
          }
      } else { 0 }
  }
  ```
]
