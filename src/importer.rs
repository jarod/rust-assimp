//! Defines the scene importer

use libc::{c_int, c_char};
use std::ptr;

use scene::Scene;
use ffi;
use postprocess::Process;
use property::Property;
use types::AiString;

use importer::PropertyType::{PBool, PInt, PFloat, PStr};


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
    /// See `Process` for a list of possible values.
    pub fn add_processing_steps(&mut self, set: &[Process]) {
        self.flags = set.iter().fold(self.flags, |x, &y| x | y as u32);
    }

    /// Removes the given post processing steps from this importer.
    ///
    /// See `Process` for a list of possible values.
    pub fn rm_processing_steps(&mut self, set: &[Process]) {
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
    pub fn set_import_property(&mut self, property: Property) {
        let (s, wrapped_val) = decompose_property(property);

        unsafe {
            match wrapped_val {
                PFloat(val) => {
                    ffi::aiSetImportPropertyFloat(self.property_store,
                                             s.as_ptr() as *const c_char,
                                             val);
                }
                PInt(val) => {
                    ffi::aiSetImportPropertyInteger(self.property_store,
                                               s.as_ptr() as *const c_char,
                                               val as c_int);
                }
                PBool(val) => {
                    let bool_int = match val { true => 1i, false => 0i };
                    println!("bool_int : {}", bool_int);
                    ffi::aiSetImportPropertyInteger(self.property_store,
                                               s.as_ptr() as *const c_char,
                                               bool_int as c_int);
                }
                PStr(val) => {
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
    pub fn import_from_file(&self, file_name: &str) -> Option<Scene> {
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
    PFloat(f32),
    PInt(int),
    PBool(bool),
    PStr(&'a AiString),
}

fn decompose_property(property: Property) -> (&'static str, PropertyType) {
    match property {
        Property::GLOB_MEASURE_TIME(a) =>
            ( "GLOB_MEASURE_TIME", PBool(a) ),
        Property::PP_SBBC_MAX_BONES(a) =>
            ( "PP_SBBC_MAX_BONES", PInt(a) ),
        Property::PP_CT_MAX_SMOOTHING_ANGLE(a) =>
            ( "PP_CT_MAX_SMOOTHING_ANGLE", PFloat(a) ),
        Property::PP_CT_TEXTURE_CHANNEL_INDEX(a) =>
            ( "PP_CT_TEXTURE_CHANNEL_INDEX", PInt(a) ),
        Property::PP_GSN_MAX_SMOOTHING_ANGLE(a) =>
            ( "PP_GSN_MAX_SMOOTHING_ANGLE", PFloat(a) ),
        Property::IMPORT_MDL_COLORMAP(a) =>
            ( "IMPORT_MDL_COLORMAP", PStr(a) ),
        Property::PP_RRM_EXCLUDE_LIST(a) =>
            ( "PP_RRM_EXCLUDE_LIST", PStr(a) ),
        Property::PP_PTV_KEEP_HIERARCHY(a) =>
            ( "PP_PTV_KEEP_HIERARCHY", PBool(a) ),
        Property::PP_PTV_NORMALIZE(a)  =>
            ( "PP_PTV_NORMALIZE", PFloat(a) ),
        Property::PP_FD_REMOVE(a) =>
            ( "PP_FD_REMOVE", PBool(a) ),
        Property::PP_OG_EXCLUDE_LIST(a)    =>
            ( "PP_OG_EXCLUDE_LIST", PStr(a) ),
        Property::PP_SLM_TRIANGLE_LIMIT(a) =>
            ( "PP_SLM_TRIANGLE_LIMIT", PInt(a) ),
        Property::PP_SLM_VERTEX_LIMIT(a) =>
            ( "PP_SLM_VERTEX_LIMIT", PInt(a) ),
        Property::PP_LBW_MAX_WEIGHTS(a) =>
            ( "PP_LBW_MAX_WEIGHTS", PInt(a) ),
        Property::PP_DB_THRESHOLD(a) =>
            ( "PP_DB_THRESHOLD", PFloat(a) ),
        Property::PP_DB_ALL_OR_NONE(a) =>
            ( "PP_DB_ALL_OR_NONE", PBool(a) ),
        Property::PP_ICL_PTCACHE_SIZE(a) =>
            ( "PP_ICL_PTCACHE_SIZE", PInt(a) ),
        // This properties take a list as an arguement
        Property::PP_RVC_FLAGS(list) => {
            let result = list.iter().fold(0, |sum, &y| sum | y as u32);
            ( "PP_RVC_FLAGS", PInt(result as int) )
        }
        Property::PP_SBP_REMOVE(list) => {
            let result = list.iter().fold(0, |sum, &y| sum | y as u32);
            ( "PP_SBP_REMOVE", PInt(result as int) )
        }
        Property::PP_TUV_EVALUATE(list) => {
            let result = list.iter().fold(0, |sum, &y| sum | y as u32);
            ( "PP_TUV_EVALUATE", PInt(result as int) )
        }
        Property::PP_FID_ANIM_ACCURACY(a) =>
            ( "PP_FID_ANIM_ACCURACY", PFloat(a) ),
        Property::FAVOUR_SPEED(a) =>
            ( "FAVOUR_SPEED", PBool(a) ),
        Property::IMPORT_GLOBAL_KEYFRAME(a) =>
            ( "IMPORT_GLOBAL_KEYFRAME", PInt(a) ),
        Property::IMPORT_MD2_KEYFRAME(a) =>
            ( "IMPORT_MD2_KEYFRAME", PInt(a) ),
        Property::IMPORT_MD3_KEYFRAME(a) =>
            ( "IMPORT_MD3_KEYFRAME", PInt(a) ),
        Property::IMPORT_MDC_KEYFRAME(a) =>
            ( "IMPORT_MDC_KEYFRAME", PInt(a) ),
        Property::IMPORT_MDL_KEYFRAME(a) =>
            ( "IMPORT_MDL_KEYFRAME", PInt(a) ),
        Property::IMPORT_SMD_KEYFRAME(a) =>
            ( "IMPORT_SMD_KEYFRAME", PInt(a) ),
        Property::IMPORT_UNREAL_KEYFRAME(a) =>
            ( "IMPORT_UNREAL_KEYFRAME", PInt(a) ),
        Property::IMPORT_AC_SEPARATE_BFCULL(a) =>
            ( "IMPORT_AC_SEPARATE_BFCULL", PBool(a) ),
        Property::IMPORT_AC_EVAL_SUBDIVISION(a) =>
            ( "IMPORT_AC_EVAL_SUBDIVISION", PBool(a) ),
        Property::IMPORT_UNREAL_HANDLE_FLAGS(a) =>
            ( "UNREAL_HANDLE_FLAGS", PBool(a) ),
        Property::IMPORT_TER_MAKE_UVS(a) =>
            ( "IMPORT_TER_MAKE_UVS", PBool(a) ),
        Property::IMPORT_ASE_RECONSTRUCT_NORMALS(a) =>
            ( "IMPORT_ASE_RECONSTRUCT_NORMALS", PBool(a) ),
        Property::IMPORT_MD3_HANDLE_MULTIPART(a) =>
            ( "IMPORT_MD3_HANDLE_MULTIPART", PBool(a) ),
        Property::IMPORT_MD3_SKIN_NAME(a) =>
            ( "IMPORT_MD3_SKIN_NAME", PStr(a) ),
        Property::IMPORT_MD3_SHADER_SRC(a) =>
            ( "IMPORT_MD3_SHADER_SRC", PStr(a) ),
        Property::IMPORT_MD5_NO_ANIM_AUTOLOAD(a) =>
            ( "IMPORT_MD5_NO_ANIM_AUTOLOAD", PBool(a) ),
        Property::IMPORT_LWO_ONE_LAYER_ONLY(a) =>
            ( "IMPORT_LWO_ONE_LAYER_ONLY", PInt(a) ),
        Property::IMPORT_LWS_ANIM_START(a) =>
            ( "IMPORT_LWS_ANIM_START", PInt(a) ),
        Property::IMPORT_LWS_ANIM_END(a) =>
            ( "IMPORT_LWS_ANIM_END", PInt(a) ),
        Property::IMPORT_IRR_ANIM_FPS(a) =>
            ( "IMPORT_IRR_ANIM_FPS", PInt(a) ),
        Property::IMPORT_OGRE_MATERIAL_FILE(a) =>
            ( "IMPORT_OGRE_MATERIAL_FILE", PStr(a) ),
        Property::IMPORT_OGRE_TEXTURETYPE_FROM_FILENAME(a) =>
            ( "IMPORT_OGRE_TEXTURETYPE_FROM_FILENAME", PBool(a) ),
        Property::IMPORT_IFC_SKIP_SPACE_REPRESENTATIONS(a) =>
            ( "IMPORT_IFC_SKIP_SPACE_REPRESENTATIONS", PBool(a) ),
        Property::IMPORT_IFC_SKIP_CURVE_REPRESENTATIONS(a) =>
            ( "IMPORT_IFC_SKIP_CURVE_REPRESENTATIONS", PBool(a) ),
        Property::IMPORT_IFC_CUSTOM_TRIANGULATION(a) =>
            ( "IMPORT_IFC_CUSTOM_TRIANGULATION", PBool(a) ),
    }
}

#[cfg(test)]
mod test {
    // use std::io::MemWriter;
    // use std::io::BufReader;

    use super::Importer;
    use property::Property;

    // Log to memory
    // let mut writer = MemWriter::new();
    // log::add_log_stream(log::LogStreamCustom(&mut writer));
    // let mut reader = BufReader::new(writer.get_ref());
    //
    #[test]
    fn test_set_property() {
        let mut imp = Importer::new();

        imp.set_import_property(Property::GLOB_MEASURE_TIME(true));
        imp.set_import_property(Property::PP_DB_THRESHOLD(0.5));
        imp.set_import_property(Property::PP_SLM_VERTEX_LIMIT(2500));

        // imp.set_import_property(Property::PP_TUV_EVALUATE([]));

        let _ = imp.import_from_file("examples/assets/cube.dae");
    }
}

// vim: et tw=78 sw=4:
