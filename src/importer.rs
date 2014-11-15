
use libc::{c_int, c_float, c_char, c_uint};
use std::ptr;

use scene::{Scene};
use ffi;
use config::{Config, Pint, Pfloat, Pbool, Pstr, decompose_property};
// use fileio::{AiFileIO};

/// Represents an opaque set of settings to be used during importing.
#[repr(C)]
pub struct PropertyStore {
    sentinel: c_char,
}


pub struct Importer {
    property_store: *mut PropertyStore,
}

impl Importer {

    fn new() -> Importer {
        Importer {
            property_store: unsafe { ffi::aiCreatePropertyStore() },
        }
    }

    fn set_property(&mut self, config: Config) {
        let (s, wrapped_val) = decompose_property(config);

        unsafe {
            match wrapped_val {
                Pfloat(val) =>
                    ffi::aiSetImportPropertyFloat(self.property_store,
                                             s.as_ptr() as *const c_char,
                                             val as c_float),
                Pint(val) =>
                    ffi::aiSetImportPropertyInteger(self.property_store,
                                               s.as_ptr() as *const c_char,
                                               val as c_int),
                Pbool(val) => {
                    let bool_int = match val { true => 1i, false => 0i };
                    ffi::aiSetImportPropertyInteger(self.property_store,
                                               s.as_ptr() as *const c_char,
                                               bool_int as c_int);
                }
                Pstr(val) => 
                    ffi::aiSetImportPropertyString(self.property_store,
                                              s.as_ptr() as *const c_char,
                                              val),
            }
        }
    }

    fn import(&self, file_name: &str, flags: c_uint) -> Option<Scene> {
        unsafe { 
            let raw = file_name.with_c_str(|file|
                ffi::aiImportFileExWithProperties(
                    file,
                    flags,
                    ptr::null_mut(), // no custom file io system
                    self.property_store as *const PropertyStore));
            if raw.is_null() {
                None
            } else {
                Some(Scene::from_raw_scene(raw))
            }
        }
    }
}

impl Drop for Importer {
    fn drop(&mut self) {
        unsafe { ffi::aiReleasePropertyStore(self.property_store); }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use config::*;
    use log;
    #[test]
    fn test_set_property() {
        log::add_log_stream(log::LogStreamStdout);
        log::add_log_stream(log::LogStreamStderr);
        log::add_log_stream(log::LogStreamFile("log.txt"));

        let mut imp = Importer::new();

        imp.set_property(GLOB_MEASURE_TIME(true));
        imp.set_property(PP_DB_THRESHOLD(0.5));
        imp.set_property(IMPORT_MDC_KEYFRAME(0));
        imp.import("cube.dae", 0);
    }
}
