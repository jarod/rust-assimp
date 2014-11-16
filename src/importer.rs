//! Defines the scene importer

use libc::{c_int, c_float, c_char};
use std::ptr;

use scene::{Scene};
use ffi;
use postprocess::{PostProcessSteps};
use config::*;

pub struct Importer {
    property_store: *mut ffi::PropertyStore,
    flags: u32,
}

impl Importer {
    pub fn new() -> Importer {
        Importer {
            property_store: unsafe { ffi::aiCreatePropertyStore() },
            flags: 0,
        }
    }

    pub fn add_processing_steps(&mut self, set: &[PostProcessSteps]) {
        self.flags = set.iter().fold(self.flags, |x, &y| x | y as u32);
    }

    pub fn rm_processing_steps(&mut self, set: &[PostProcessSteps]) {
        self.flags = set.iter().fold(self.flags, |x, &y| x & !(y as u32));
    }

    pub fn reset_processing_steps(&mut self) {
        self.flags = 0;
    }

    pub fn set_import_property(&mut self, config: ImportProperty) {
        let (s, wrapped_val) = decompose_property(config);

        unsafe {
            match wrapped_val {
                Pfloat(val) => {
                    ffi::aiSetImportPropertyFloat(self.property_store,
                                             s.as_ptr() as *const c_char,
                                             val as c_float);
                }
                Pint(val) => {
                    ffi::aiSetImportPropertyInteger(self.property_store,
                                               s.as_ptr() as *const c_char,
                                               val as c_int);
                }
                Pbool(val) => {
                    let bool_int = match val { true => 1i, false => 0i };
                    println!("bool_int : {}", bool_int);
                    ffi::aiSetImportPropertyInteger(self.property_store,
                                               s.as_ptr() as *const c_char,
                                               bool_int as c_int);
                }
                Pstr(val) => {
                    ffi::aiSetImportPropertyString(self.property_store,
                                              s.as_ptr() as *const c_char,
                                              val);
                }
            }
        }
    }

    pub fn reset_import_properties(&mut self) {
        unsafe {
            ffi::aiReleasePropertyStore(self.property_store);
            self.property_store = ffi::aiCreatePropertyStore();
        }
    }

    pub fn import(&self, file_name: &str) -> Option<Scene> {
        unsafe {
            let raw = file_name.with_c_str(|file|
                ffi::aiImportFileExWithProperties(
                    file,
                    self.flags,
                    ptr::null_mut(), // no custom file io system
                    self.property_store as *const ffi::PropertyStore));
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
    // use std::io::MemWriter;
    // use std::io::BufReader;

    use super::*;
    use config::*;

    // Log to memory
    // let mut writer = MemWriter::new();
    // log::add_log_stream(log::LogStreamCustom(&mut writer));
    // let mut reader = BufReader::new(writer.get_ref());
    //
    #[test]
    fn test_set_property() {
        let mut imp = Importer::new();

        imp.set_import_property(GLOB_MEASURE_TIME(true));
        imp.set_import_property(PP_DB_THRESHOLD(0.5));
        imp.set_import_property(PP_SLM_VERTEX_LIMIT(2500));

        imp.import("cube.dae");
    }
}
