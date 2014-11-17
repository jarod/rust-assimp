//! Functions for querying the version of assimp being used

use libc::{c_char, c_uint};

use types::{AiBool, AiString};

#[link(name = "assimp")]
extern {
    pub fn aiGetLegalString() -> *const c_char;

    pub fn aiGetVersionMinor() -> c_uint;

    pub fn aiGetVersionMajor() -> c_uint;

    pub fn aiGetVersionRevision() -> c_uint;

    pub fn aiGetCompileFlags() -> c_uint;

    pub fn aiGetExtensionList(out: *mut AiString);

    pub fn aiIsExtensionSupported(ext: *const c_char) -> AiBool;
}
