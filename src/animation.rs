//! Data structures for handling animation

use libc::{c_double, c_uint};

use util::{ptr_ptr_to_slice, ptr_to_slice};
use types::{Vector3D, Quaternion, AiString};

/// A time-value pair specifying a certain 3D vector for the given time.
#[deriving(Show)]
#[repr(C)]
pub struct VectorKey {
    /// The time of this key
    pub time: c_double,

    /// The value of this key
    pub value: Vector3D,
}

/// A time-value pair specifying a rotation for the given time.
///
/// Rotations are expressed with quaternions.
#[deriving(Show)]
#[repr(C)]
pub struct QuatKey {
    /// The time of this key
    pub time: c_double,

    /// The value of this key
    pub value: Quaternion,

}

/// Binds a anim mesh to a specific point in time.
#[deriving(Show)]
#[repr(C)]
pub struct MeshKey {
    /// The time of this key
    pub time: c_double,

    /// Index into the Mesh::anim_meshes array of the
    /// mesh coresponding to the MeshAnim hosting this
    /// key frame. The referenced anim mesh is evaluated
    /// according to the rules defined in the docs for AnimMesh.
    pub value: c_uint,
}

/// Defines how an animation channel behaves outside the defined time range.
///
/// This corresponds to NodeAnim::pre_state and NodeAnim::post_state.
#[deriving(Show)]
#[repr(C)]
pub enum AnimBehaviour {
    /// The value from the default node transformation is taken
    AnimBehaviour_DEFAULT  = 0x0,

    /// The nearest key value is used without interpolation
    AnimBehaviour_CONSTANT = 0x1,

    /// The value of the nearest two keys is linearly extrapolated 
    /// for the current time value.
    AnimBehaviour_LINEAR   = 0x2,

    /// The animation is repeated.
    ///
    /// If the animation key go from n to m and the current
    /// time is t, use the value at (t-n) % (|m-n|).
    AnimBehaviour_REPEAT   = 0x3,
}

/// Describes the animation of a single node.
///
/// The name specifies the bone/node which is affected
/// by this animation channel. The keyframes are given in three
/// separate series of values, one each for position, rotation and
/// scaling. The transformation matrix computed from these
/// values replaces the node's original transformation matrix at a
/// specific time.
/// This means all keys are absolute and not relative to the bone default pose.
/// The order in which the transformations are applied is
/// - as usual - scaling, rotation, translation.
///
/// Note: All keys are returned in their correct, chronological order.
/// Duplicate keys don't pass the validation step. Most likely there
/// will be no negative time values, but they are not forbidden also (so
/// implementations need to cope with them! )
#[deriving(Show)]
#[repr(C)]
pub struct NodeAnim {
    /// The name of the node affected by this animation. The node
    /// must exist and it must be unique.
    pub name: AiString,

    /// The number of position keys
    pub num_position_keys: c_uint,

    /// The position keys of this animation channel. Positions are
    /// specified as 3D vector. The array is num_position_keys in size.
    ///
    /// If there are position keys, there will also be at least one
    /// scaling and one rotation key.
    position_keys: *mut VectorKey,

    /// The number of rotation keys
    pub num_rotation_keys: c_uint,

    /// The rotation keys of this animation channel. Rotations are
    /// given as quaternions,  which are 4D vectors. The array is
    /// num_rotation_keys in size.
    ///
    /// If there are rotation keys, there will also be at least one
    /// scaling and one position key.
    rotation_keys: *mut QuatKey,

    /// The number of scaling keys
    pub num_scaling_keys: c_uint,

    /// The scaling keys of this animation channel. Scalings are
    /// specified as 3D vector. The array is num_scaling_keys in size.
    ///
    /// If there are scaling keys, there will also be at least one
    /// position and one rotation key.
    scaling_keys: *mut VectorKey,

    /// Defines how the animation behaves before the first
    /// key is encountered.
    ///
    /// The default value is AnimBehaviour_DEFAULT (the original
    /// transformation matrix of the affected node is used).
    pub pre_state: AnimBehaviour,

    /// Defines how the animation behaves after the last
    /// key was processed.
    ///
    /// The default value is AnimBehaviour_DEFAULT (the original
    /// transformation matrix of the affected node is taken).
    pub post_state: AnimBehaviour,
}

impl NodeAnim {
    pub fn get_rotation_keys(&self) -> &[QuatKey] {
        unsafe { ptr_to_slice(self.rotation_keys, self.num_rotation_keys as uint) }
    }

    pub fn get_position_keys(&self) -> &[VectorKey] {
        unsafe { ptr_to_slice(self.position_keys, self.num_position_keys as uint) }
    }

    pub fn get_scaling_keys(&self) -> &[VectorKey] {
        unsafe { ptr_to_slice(self.scaling_keys, self.num_scaling_keys as uint) }
    }
}

/// Describes vertex-based animations for a single mesh or a group of
/// meshes. Meshes carry the animation data for each frame in their
/// Mesh::anim_mesh array. The purpose of MeshAnim is to
/// define keyframes linking each mesh attachment to a particular
/// point in time.
#[deriving(Show)]
#[repr(C)]
pub struct MeshAnim {
    /// Name of the mesh to be animated. An empty string is not allowed,
    /// animated meshes need to be named (not necessarily uniquely,
    /// the name can basically serve as wildcard to select a group
    /// of meshes with similar animation setup)
    pub name: AiString,

    /// Size of the keys array. Must be 1, at least.
    pub num_keys: c_uint,

    /// Key frames of the animation. May not be NULL.
    keys: *mut MeshKey,
}

impl MeshAnim {
    pub fn get_keys(&self) -> &[MeshKey] {
        unsafe { ptr_to_slice(self.keys, self.num_keys as uint) }
    }
}

/// An animation consists of keyframe data for a number of nodes. For
/// each node affected by the animation a separate series of data is given.
#[deriving(Show)]
#[repr(C)]
pub struct Animation {
    /// The name of the animation. If the modeling package this data was
    /// exported from does support only a single animation channel, this
    /// name is usually empty (length is zero).
    pub name: AiString,

    /// Duration of the animation in ticks.
    pub duration: c_double,

    /// Ticks per second. 0 if not specified in the imported file
    pub ticks_per_sec: c_double,

    /// The number of bone animation channels. Each channel affects
    /// a single node.
    pub num_channels: c_uint ,

    /// The node animation channels. Each channel affects a single node.
    /// The array is num_channels in size.
    channels: *mut*mut NodeAnim,

    /// The number of mesh animation channels. Each channel affects
    /// a single mesh and defines vertex-based animation.
    pub num_mesh_channels: c_uint ,

    /// The mesh animation channels. Each channel affects a single mesh.
    /// The array is num_mesh_channels in size.
    mesh_channels: *mut*mut MeshAnim,
}

impl Animation {
    pub fn get_channels(&self) -> &[&NodeAnim] {
        unsafe { ptr_ptr_to_slice(self.channels, self.num_channels as uint) }
    }

    pub fn get_mesh_channels(&self) -> &[&MeshAnim] {
        unsafe { ptr_ptr_to_slice(self.mesh_channels,
                                  self.num_mesh_channels as uint) }
    }
}
