//! Defines the scene importer

use libc::{c_int, c_char};
use std::ptr;

use scene::Scene;
use ffi;
use postprocess::PostProcessSteps;
use properties::*;
use types::AiString;

/// A scene importer.
pub struct Importer {
    property_store: *mut ffi::PropertyStore,
    flags: u32,
}

impl<'a> Importer {
    /// Create a new scene importer
    pub fn new() -> Importer {
        Importer {
            property_store: unsafe { ffi::aiCreatePropertyStore() },
            flags: 0,
        }
    }

    /// Add post processing steps to this scene importer.
    ///
    /// See `PostProcessSteps` for a list of possible values.
    pub fn add_processing_steps(&mut self, set: &[PostProcessSteps]) {
        self.flags = set.iter().fold(self.flags, |x, &y| x | y as u32);
    }

    /// Removes the given post processing steps from this importer.
    ///
    /// See `PostProcessSteps` for a list of possible values.
    pub fn rm_processing_steps(&mut self, set: &[PostProcessSteps]) {
        self.flags = set.iter().fold(self.flags, |x, &y| x & !(y as u32));
    }

    /// Perform no post processing steps
    pub fn no_post_processing(&mut self) {
        self.flags = 0;
    }

    /// Set an import property.
    ///
    /// These propereties affect how the import will load models.
    /// See `ImportProperty` for a list of possible values.
    pub fn set_import_property(&mut self, config: ImportProperty) {
        let (s, wrapped_val) = decompose_property(config);

        unsafe {
            match wrapped_val {
                Pfloat(val) => {
                    ffi::aiSetImportPropertyFloat(self.property_store,
                                             s.as_ptr() as *const c_char,
                                             val);
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

    /// Unset all import properties.
    pub fn reset_import_properties(&mut self) {
        unsafe {
            ffi::aiReleasePropertyStore(self.property_store);
            self.property_store = ffi::aiCreatePropertyStore();
        }
    }

    /// Create a `Scene` from the given file.
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

enum PropertyType<'a> {
    Pfloat(f32),
    Pint(int),
    Pbool(bool),
    Pstr(&'a AiString),
}

fn decompose_property(config: ImportProperty) -> (&'static str, PropertyType) {
    match config {
        GLOB_MEASURE_TIME(a) => ( "GLOB_MEASURE_TIME", Pbool(a) ),
        PP_SBBC_MAX_BONES(a) => ( "PP_SBBC_MAX_BONES", Pint(a) ),
        PP_CT_MAX_SMOOTHING_ANGLE(a) => ( "PP_CT_MAX_SMOOTHING_ANGLE", Pfloat(a) ),
        PP_CT_TEXTURE_CHANNEL_INDEX(a) => ( "PP_CT_TEXTURE_CHANNEL_INDEX", Pint(a) ),
        PP_GSN_MAX_SMOOTHING_ANGLE(a) => ( "PP_GSN_MAX_SMOOTHING_ANGLE", Pfloat(a) ),
        IMPORT_MDL_COLORMAP(a) => ( "IMPORT_MDL_COLORMAP", Pstr(a) ),
        PP_RRM_EXCLUDE_LIST(a) => ( "PP_RRM_EXCLUDE_LIST", Pstr(a) ),
        PP_PTV_KEEP_HIERARCHY(a) => ( "PP_PTV_KEEP_HIERARCHY", Pbool(a) ),
        PP_PTV_NORMALIZE(a)  => ( "PP_PTV_NORMALIZE", Pfloat(a) ),
        PP_FD_REMOVE(a) => ( "PP_FD_REMOVE", Pbool(a) ),
        PP_OG_EXCLUDE_LIST(a)    => ( "PP_OG_EXCLUDE_LIST", Pstr(a) ),
        PP_SLM_TRIANGLE_LIMIT(a) => ( "PP_SLM_TRIANGLE_LIMIT", Pint(a) ),
        PP_SLM_VERTEX_LIMIT(a) => ( "PP_SLM_VERTEX_LIMIT", Pint(a) ),
        PP_LBW_MAX_WEIGHTS(a) => ( "PP_LBW_MAX_WEIGHTS", Pint(a) ),
        PP_DB_THRESHOLD(a) => ( "PP_DB_THRESHOLD", Pfloat(a) ),
        PP_DB_ALL_OR_NONE(a) => ( "PP_DB_ALL_OR_NONE", Pbool(a) ),
        PP_ICL_PTCACHE_SIZE(a) => ( "PP_ICL_PTCACHE_SIZE", Pint(a) ),

        PP_RVC_FLAGS(list) => {
            let result = list.iter().fold(0, |sum, &y| sum | y as u32);
            ( "PP_RVC_FLAGS", Pint(result as int) )
        }
        PP_SBP_REMOVE(list) => {
            let result = list.iter().fold(0, |sum, &y| sum | y as u32);
            ( "PP_SBP_REMOVE", Pint(result as int) )
        }
        PP_TUV_EVALUATE(list) => {
            let result = list.iter().fold(0, |sum, &y| sum | y as u32);
            ( "PP_TUV_EVALUATE", Pint(result as int) )
        }

        PP_FID_ANIM_ACCURACY(a) => ( "PP_FID_ANIM_ACCURACY", Pfloat(a) ),
        FAVOUR_SPEED(a) => ( "FAVOUR_SPEED", Pbool(a) ),
        IMPORT_GLOBAL_KEYFRAME(a) => ( "IMPORT_GLOBAL_KEYFRAME", Pint(a) ),
        IMPORT_MD2_KEYFRAME(a) => ( "IMPORT_MD2_KEYFRAME", Pint(a) ),
        IMPORT_MD3_KEYFRAME(a) => ( "IMPORT_MD3_KEYFRAME", Pint(a) ),
        IMPORT_MDC_KEYFRAME(a) => ( "IMPORT_MDC_KEYFRAME", Pint(a) ),
        IMPORT_MDL_KEYFRAME(a) => ( "IMPORT_MDL_KEYFRAME", Pint(a) ),
        IMPORT_SMD_KEYFRAME(a) => ( "IMPORT_SMD_KEYFRAME", Pint(a) ),
        IMPORT_UNREAL_KEYFRAME(a) => ( "IMPORT_UNREAL_KEYFRAME", Pint(a) ),
        IMPORT_AC_SEPARATE_BFCULL(a) => ( "IMPORT_AC_SEPARATE_BFCULL", Pbool(a) ),
        IMPORT_AC_EVAL_SUBDIVISION(a) => ( "IMPORT_AC_EVAL_SUBDIVISION", Pbool(a) ),
        IMPORT_UNREAL_HANDLE_FLAGS(a) => ( "UNREAL_HANDLE_FLAGS", Pbool(a) ),
        IMPORT_TER_MAKE_UVS(a) => ( "IMPORT_TER_MAKE_UVS", Pbool(a) ),
        IMPORT_ASE_RECONSTRUCT_NORMALS(a) => ( "IMPORT_ASE_RECONSTRUCT_NORMALS", Pbool(a) ),
        IMPORT_MD3_HANDLE_MULTIPART(a) => ( "IMPORT_MD3_HANDLE_MULTIPART", Pbool(a) ),
        IMPORT_MD3_SKIN_NAME(a) => ( "IMPORT_MD3_SKIN_NAME", Pstr(a) ),
        IMPORT_MD3_SHADER_SRC(a) => ( "IMPORT_MD3_SHADER_SRC", Pstr(a) ),
        IMPORT_MD5_NO_ANIM_AUTOLOAD(a) => ( "IMPORT_MD5_NO_ANIM_AUTOLOAD", Pbool(a) ),
        IMPORT_LWO_ONE_LAYER_ONLY(a) => ( "IMPORT_LWO_ONE_LAYER_ONLY", Pint(a) ),
        IMPORT_LWS_ANIM_START(a) => ( "IMPORT_LWS_ANIM_START", Pint(a) ),
        IMPORT_LWS_ANIM_END(a) => ( "IMPORT_LWS_ANIM_END", Pint(a) ),
        IMPORT_IRR_ANIM_FPS(a) => ( "IMPORT_IRR_ANIM_FPS", Pint(a) ),
        IMPORT_OGRE_MATERIAL_FILE(a) => ( "IMPORT_OGRE_MATERIAL_FILE", Pstr(a) ),
        IMPORT_OGRE_TEXTURETYPE_FROM_FILENAME(a) => ( "IMPORT_OGRE_TEXTURETYPE_FROM_FILENAME", Pbool(a) ),
        IMPORT_IFC_SKIP_SPACE_REPRESENTATIONS(a) => ( "IMPORT_IFC_SKIP_SPACE_REPRESENTATIONS", Pbool(a) ),
        IMPORT_IFC_SKIP_CURVE_REPRESENTATIONS(a) => ( "IMPORT_IFC_SKIP_CURVE_REPRESENTATIONS", Pbool(a) ),
        IMPORT_IFC_CUSTOM_TRIANGULATION(a) => ( "IMPORT_IFC_CUSTOM_TRIANGULATION", Pbool(a) ),
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

        imp.set_import_property(PP_TUV_EVALUATE([]));

        imp.import("cube.dae");
    }
}
