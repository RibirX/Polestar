#[cfg(target_os = "windows")]
pub fn app_init_hook() -> bool { return windows_singleton_guard(); }

#[cfg(target_os = "windows")]
pub fn app_run_before_hook() {}

#[cfg(target_os = "windows")]
pub fn permission_prompt() -> bool {
  // mock for windows, wait for windows implementation
  true
}

#[cfg(target_os = "windows")]
pub fn has_permission() -> bool {
  // mock for windows, wait for windows implementation
  true
}

#[cfg(windows)]
fn windows_singleton_guard() -> bool {
  use bytes::BytesMut;
  use ribir::prelude::{App, AppEvent};
  use std::ffi::OsString;
  use std::os::windows::ffi::OsStrExt;
  use winapi::{
    shared::{minwindef::LPVOID, winerror::ERROR_ALREADY_EXISTS},
    um::{
      errhandlingapi::GetLastError,
      fileapi::{CreateFileW, ReadFile, WriteFile, OPEN_EXISTING},
      namedpipeapi::{ConnectNamedPipe, CreateNamedPipeW, DisconnectNamedPipe},
      synchapi::{CreateMutexW, ReleaseMutex},
      winbase::{
        PIPE_ACCESS_DUPLEX, PIPE_READMODE_MESSAGE, PIPE_TYPE_MESSAGE, PIPE_UNLIMITED_INSTANCES,
        PIPE_WAIT,
      },
      winnt::GENERIC_WRITE,
    },
  };

  let mutex_name: Vec<_> = OsString::from("PoleStarChat_Mutex")
    .as_os_str()
    .encode_wide()
    .chain(Some(0))
    .collect();

  let buffer_size = 1024_u32;
  let is_singleton = unsafe {
    let h_mutex = CreateMutexW(std::ptr::null_mut(), 1, mutex_name.as_ptr());
    let err = GetLastError();
    if err == ERROR_ALREADY_EXISTS && !h_mutex.is_null() {
      ReleaseMutex(h_mutex);
    }
    !h_mutex.is_null() && err == 0
  };
  if is_singleton {
    let event_sender = App::event_sender();
    std::thread::spawn(move || {
      let pipe_name: Vec<_> = OsString::from("\\\\.\\pipe\\polestar_chat")
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect();
      let h_pipe = unsafe {
        CreateNamedPipeW(
          pipe_name.as_ptr(),
          PIPE_ACCESS_DUPLEX,
          PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
          PIPE_UNLIMITED_INSTANCES,
          buffer_size,
          buffer_size,
          0,
          std::ptr::null_mut(),
        )
      };

      loop {
        unsafe { ConnectNamedPipe(h_pipe, std::ptr::null_mut()) };
        let len = 1024_u32;
        let mut readed = 0_u32;
        let mut bytes = BytesMut::with_capacity(len as usize);
        if (unsafe {
          ReadFile(
            h_pipe,
            bytes.as_mut_ptr() as LPVOID,
            len,
            &mut readed,
            std::ptr::null_mut(),
          )
        } != 0)
        {
          if readed != 0 {
            unsafe {
              bytes.set_len(readed as usize);
            }
            let msg = String::from_utf8_lossy(&bytes).to_string();
            event_sender.send(AppEvent::OpenUrl(msg.clone()));
          }
        }
        unsafe { DisconnectNamedPipe(h_pipe) };
      }
    });
  }
  let matches = clap::Command::new("PolestarChat")
    .arg(clap::arg!(--"oauth" <URL>).value_parser(clap::value_parser!(String)))
    .get_matches();
  if let Some(oauth_url) = matches.get_one::<String>("oauth") {
    unsafe {
      let pipe_name: Vec<_> = OsString::from("\\\\.\\pipe\\polestar_chat")
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect();
      let h_pipe = CreateFileW(
        pipe_name.as_ptr(),
        GENERIC_WRITE,
        0,
        std::ptr::null_mut(),
        OPEN_EXISTING,
        0,
        std::ptr::null_mut(),
      );
      WriteFile(
        h_pipe,
        oauth_url.as_bytes().as_ptr() as LPVOID,
        oauth_url.len() as u32,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
      );
    };
  }
  println!("is_singleton: {}", is_singleton);
  is_singleton
}
