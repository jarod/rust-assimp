//! Data structures for handling animation

use types::{Vector3D, Quaternion, AiString};
use libc::{c_double, c_uint};

/// A time-value pair specifying a certain 3D vector for the given time.
#[repr(C)]
pub struct VectorKey {
    /// The time of this key
    pub time: c_double,

    /// The value of this key
    pub value: Vector3D,
}

/** A time-value pair specifying a rotation for the given time.
    *  Rotations are expressed with quaternions. */
#[repr(C)]
pub struct QuatKey {
    /// The time of this key */
    pub time: c_double,

    /// The value of this key */
    pub value: Quaternion,

}

/** Binds a anim mesh to a specific point in time. */
#[repr(C)]
pub struct MeshKey {
    /// The time of this key */
    pub time: c_double,

    /// Index into the aiMesh::mAnimMeshes array of the
     // *  mesh coresponding to the #aiMeshAnim hosting this
     // *  key frame. The referenced anim mesh is evaluated
     // *  according to the rules defined in the docs for #aiAnimMesh.*/
    pub value: c_uint,

}

/** Defines how an animation channel behaves outside the defined time
    *  range. This corresponds to aiNodeAnim::mPreState and
    *  aiNodeAnim::mPostState.*/
#[repr(C, u32)]
pub enum AnimBehaviour {
    /// The value from the default node transformation is taken*/
    AnimBehaviour_DEFAULT  = 0x0,

    /// The nearest key value is used without interpolation */
    AnimBehaviour_CONSTANT = 0x1,

    /// The value of the nearest two keys is linearly
    /// *  extrapolated for the current time value.*/
    AnimBehaviour_LINEAR   = 0x2,

    /// The animation is repeated.
    /// *
    /// *  If the animation key go from n to m and the current
    /// *  time is t, use the value at (t-n) % (|m-n|).*/
    AnimBehaviour_REPEAT   = 0x3,
}

/** Describes the animation of a single node. The name specifies the
    *  bone/node which is affected by this animation channel. The keyframes
    *  are given in three separate series of values, one each for position,
    *  rotation and scaling. The transformation matrix computed from these
    *  values replaces the node's original transformation matrix at a
    *  specific time.
    *  This means all keys are absolute and not relative to the bone default pose.
    *  The order in which the transformations are applied is
    *  - as usual - scaling, rotation, translation.
    *
    *  @note All keys are returned in their correct, chronological order.
    *  Duplicate keys don't pass the validation step. Most likely there
    *  will be no negative time values, but they are not forbidden also ( so
    *  implementations need to cope with them! ) */
#[repr(C)]
pub struct NodeAnim {
    /// The name of the node affected by this animation. The node
    /// *  must exist and it must be unique.*/
    pub name: AiString,

    /// The number of position keys */
    mNumPositionKeys: c_uint,

    /// The position keys of this animation channel. Positions are
    /// * specified as 3D vector. The array is mNumPositionKeys in size.
    /// *
    /// * If there are position keys, there will also be at least one
    /// * scaling and one rotation key.*/
    mPositionKeys: *mut VectorKey,

    /// The number of rotation keys */
    mNumRotationKeys: c_uint,

    /// The rotation keys of this animation channel. Rotations are
    /// *  given as quaternions,  which are 4D vectors. The array is
    /// *  mNumRotationKeys in size.
    /// *
    /// * If there are rotation keys, there will also be at least one
    /// * scaling and one position key. */
    mRotationKeys: *mut QuatKey,


    /// The number of scaling keys */
    mNumScalingKeys: c_uint,

    /// The scaling keys of this animation channel. Scalings are
    /// *  specified as 3D vector. The array is mNumScalingKeys in size.
    /// *
    /// * If there are scaling keys, there will also be at least one
    /// * position and one rotation key.*/
    mScalingKeys: *mut VectorKey,


    /// Defines how the animation behaves before the first
    /// *  key is encountered.
    /// *
    /// *  The default value is aiAnimBehaviour_DEFAULT (the original
    /// *  transformation matrix of the affected node is used).*/
    mPreState: AnimBehaviour ,

    /// Defines how the animation behaves after the last
    /// *  key was processed.
    /// *
    /// *  The default value is aiAnimBehaviour_DEFAULT (the original
    /// *  transformation matrix of the affected node is taken).*/
    mPostState: AnimBehaviour ,
}

impl NodeAnim {
    pub fn get_name(&self) -> Option<&str> {
        self.mNodeName.as_str()
    }
}

/** Describes vertex-based animations for a single mesh or a group of
    ///meshes. Meshes carry the animation data for each frame in their
    ///aiMesh::mAnimMeshes array. The purpose of aiMeshAnim is to
    ///define keyframes linking each mesh attachment to a particular
    ///point in time. */
#[repr(C)]
pub struct MeshAnim {
    /// Name of the mesh to be animated. An empty string is not allowed,
    /// *  animated meshes need to be named (not necessarily uniquely,
    /// *  the name can basically serve as wildcard to select a group
    /// *  of meshes with similar animation setup)*/
    name: AiString,

    /// Size of the #mKeys array. Must be 1, at least. */
    num_keys: c_uint,

    /// Key frames of the animation. May not be NULL. */
    keys: *mut MeshKey,
}

/** An animation consists of keyframe data for a number of nodes. For
    *  each node affected by the animation a separate series of data is given.*/
#[repr(C)]
pub struct Animation {
    /// The name of the animation. If the modeling package this data was
    /// *  exported from does support only a single animation channel, this
    /// *  name is usually empty (length is zero). */
    pub name: AiString,

    /// Duration of the animation in ticks.  */
    pub duration: c_double,

    /// Ticks per second. 0 if not specified in the imported file */
    pub ticks_per_sec: c_double,

    /// The number of bone animation channels. Each channel affects
    /// *  a single node. */
    pub num_channels: c_uint ,

    /// The node animation channels. Each channel affects a single node.
    /// *  The array is mNumChannels in size. */
    channels: *mut*mut NodeAnim,


    /// The number of mesh animation channels. Each channel affects
    /// *  a single mesh and defines vertex-based animation. */
    pub num_mesh_channels: c_uint ,

    /// The mesh animation channels. Each channel affects a single mesh.
    /// *  The array is mNumMeshChannels in size. */
    pub mesh_channels: *mut*mut MeshAnim,
}
