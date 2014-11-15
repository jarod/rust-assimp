
use libc::{c_int, c_float, c_char};

use scene::Scene;
use cimport::*;
use config::{Config, Pint, Pfloat, Pbool, Pstr, decompose_property};

pub struct Importer {
    pstore: *mut PropertyStore,
}

impl Importer {

    fn new() -> Importer {
        Importer {
            pstore: unsafe { aiCreatePropertyStore() },
        }
    }

    // fn import

    fn set_property(&mut self, config: Config) {
        let (s, wrapped_val) = decompose_property(config);

        unsafe {
            match wrapped_val {
                Pfloat(val) => aiSetImportPropertyFloat( self.pstore,
                                                         s.as_ptr() as *const c_char,
                                                         val as c_float),
                Pint(val) => aiSetImportPropertyInteger( self.pstore,
                                                         s.as_ptr() as *const c_char,
                                                         val as c_int),
                Pbool(val) => {
                    let bool_int = match val { true => 1i, false => 0i };
                    aiSetImportPropertyInteger( self.pstore,
                                                s.as_ptr() as *const c_char,
                                                bool_int as c_int);
                }
                Pstr(val) => aiSetImportPropertyString( self.pstore,
                                                        s.as_ptr() as *const c_char,
                                                        val),
            }
        }
    }

    fn import(&self, file_name: &str) -> () {
    }

    fn import_from_buf(&self) -> () {
    }
}

impl Drop for Importer {
    fn drop(&mut self) {
        unsafe { aiReleasePropertyStore(self.pstore); }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use config::*;
    #[test]
    fn test_set_property() {
        let mut imp = Importer::new();
        imp.set_property(GLOB_MEASURE_TIME(true));
    }
}
