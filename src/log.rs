//! Defines logging functions

// use libc::{c_char};
// use std::c_str::CString;
use std::ptr;

use types::AiBool;
use ffi;
use ffi::{DefaultLogStream_FILE, DefaultLogStream_STDOUT,
          DefaultLogStream_STDERR, DefaultLogStream_DEBUGGER };

pub enum LogStream<'a> {
    LogStreamStdout,
    LogStreamStderr,
    LogStreamDebugger,
    LogStreamFile(&'a str),
    LogStreamCustom(&'a mut Writer+'a)
}

// TODO
// pub struct Logger {
//     log: Vec<LogStream>
// }

// pub fn get_error_string() -> Option<String> {
//     unsafe {
//         let pstr = ffi::aiGetErrorString();
//         if pstr.is_null() {
//             return None;
//         }
//         let cstr = CString::new(pstr as *const i8, false);
//         match cstr {
//             Some(ss) => Some(ss.into_string()),
//             None => None,
//         }
//     }
// }

// extern fn stream_call_back(msg: *const c_char, data: *const u8) {
//     unsafe {
//         // this code should work, but gives an Internal Compiler Error
//         // let writer = data as *mut Writer;
//         let cstr = CString::new(msg, false);
//         (*stream).write(cstr.as_bytes()).unwrap();
//     }
// }

pub fn enable_verbose_logging(choice: bool) {
    unsafe {
        ffi::aiEnableVerboseLogging(AiBool::new(choice))
    }
}

pub fn add_log_stream(log_type: LogStream) {
    unsafe {
        let null = ptr::null();
        let log = match log_type {
            LogStreamFile(fname) => fname.with_c_str(|s|
                ffi::aiGetPredefinedLogStream(DefaultLogStream_FILE, s) ),
            LogStreamStdout =>
                ffi::aiGetPredefinedLogStream(DefaultLogStream_STDOUT, null),
            LogStreamStderr =>
                ffi::aiGetPredefinedLogStream(DefaultLogStream_STDERR, null),
            LogStreamDebugger =>
                ffi::aiGetPredefinedLogStream(DefaultLogStream_DEBUGGER, null),
            LogStreamCustom(_writer) => {
                // writer.write_be_u32(0u32);
                // ffi::LogStream {
                //     callback: stream_call_back,
                //     // user data will be used to reference our writer
                //     user: mem::transmute(writer),
                // }
                unimplemented!();
            }
        };
        ffi::aiAttachLogStream(&log);
    }
}

pub fn detach_all_log_streams() {
    unsafe {
        ffi::aiDetachAllLogStreams();
    }
}
