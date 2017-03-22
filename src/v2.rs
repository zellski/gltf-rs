
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde;
use serde_json;
use std;
use ImportError;

/// Index into an array owned by the root glTF object
#[derive(Clone, Copy, Debug)]
pub struct Index<T>(u32, std::marker::PhantomData<T>);

impl<T> Index<T> {
    fn new(value: u32) -> Self {
        Index(value, std::marker::PhantomData)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

/// Generic untyped JSON object
pub type UntypedJsonObject = std::collections::HashMap<String, serde_json::Value>;

/// `extensions` field type
pub type Extensions = Option<UntypedJsonObject>;

/// `extras` field type
pub type Extras = Option<UntypedJsonObject>;

/// [The root object for a glTF asset]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#gltf)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Root {
    #[serde(default)]
    accessors: Vec<Accessor>,
    #[serde(default)]
    animations: Vec<Animation>,
    asset: Asset,
    #[serde(default)]
    buffers: Vec<Buffer>,
    #[serde(default, rename = "bufferViews")]
    buffer_views: Vec<BufferView>,
    #[serde(default, rename = "extensionsUsed")]
    extensions_used: Vec<String>,
    #[serde(default, rename = "extensionsRequired")]
    extensions_required: Vec<String>,
    #[serde(default)]
    cameras: Vec<Camera>,
    #[serde(default)]
    images: Vec<Image>,
    #[serde(default)]
    materials: Vec<Material>,
    #[serde(default)]
    meshes: Vec<Mesh>,
    #[serde(default)]
    nodes: Vec<Node>,
    #[serde(default)]
    samplers: Vec<Sampler>,
    #[serde(default = "root_scene_default")]
    scene: Index<Scene>,
    #[serde(default)]
    scenes: Vec<Scene>,
    #[serde(default)]
    skins: Vec<Skin>,
    #[serde(default)]
    textures: Vec<Texture>,
}

fn root_scene_default() -> Index<Scene> {
    Index(0, std::marker::PhantomData)
}

/// [Defines a method for retrieving data from within a `BufferView`]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#accessors)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Accessor {
    /// The index of the parent `BufferView` this accessor reads from.
    #[serde(rename = "bufferView")]
    pub buffer_view: Index<BufferView>,
    /// The offset relative to the start of the parent `BufferView` in bytes
    #[serde(rename = "byteOffset")]
    pub byte_offset: u32,
    /// The number of elements within the `BufferView` (N.B. not number of bytes)
    pub count: u32,
    /// The data type of each element (renamed from `componentType`)
    #[serde(rename = "componentType")]
    pub data_type: AccessorDataType,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// The multiplicity of each element
    #[serde(rename = "type")]
    pub kind: AccessorKind,
    /// Minimum value of each element in this attribute
    // TODO: Implement me properly
    #[serde(default)]
    pub min: serde_json::Value,
    /// Maximum value of each element in this attribute
    // TODO: Implement me properly
    #[serde(default)]
    pub max: serde_json::Value,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Specifies whether integer data values should be normalized
    #[serde(default)]
    pub normalized: bool,
    /// Sparse storage of attributes that deviate from their initialization value
    pub sparse: Option<AccessorSparseStorage>,
}

// TODO: Complete documentation
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AccessorSparseIndices {
    /// The index of the parent `BufferView` containing the sparse indices
    #[serde(rename = "byteOffset")]
    pub buffer_view: Index<BufferView>,
    /// The offset relative to the start of the parent `BufferView` in bytes
    #[serde(default, rename = "byteOffset")]
    pub byte_offset: u32,
    /// The indices data type (renamed from `componentType`)
    // N.B. Not all values are valid but it would be pedantic to have more than
    // one `DataType` enum and would also create inconsistency with the regular
    // `Accessor` struct.
    pub data_type: AccessorDataType,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
}

/// Sparse storage of attributes that deviate from their initialization value
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessorSparseStorage {
    /// Number of entries stored in the sparse array
    pub count: u32,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    // TODO: Complete documentation
    pub indices: AccessorSparseIndices,
    // TODO: Complete documentation
    pub values: AccessorSparseValues,
}

// TODO: Complete documentation
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AccessorSparseValues {
    /// The index of the parent `BufferView` containing the sparse values
    #[serde(rename = "byteOffset")]
    pub buffer_view: Index<BufferView>,
    /// The offset relative to the start of the parent `BufferView` in bytes
    #[serde(default, rename = "byteOffset")]
    pub byte_offset: u32,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
}

enum_number! {
    AccessorDataType {
        I8 = 5120,
        U8 = 5121,
        I16 = 5122,
        U16 = 5123,
        U32 = 5125,
        F32 = 5126,
    }
}

enum_string! {
    AccessorKind {
        Scalar = "SCALAR",
        Vec2 = "VEC2",
        Vec3 = "VEC3",
        Vec4 = "VEC4",
        Mat2 = "MAT2",
        Mat3 = "MAT3",
        Mat4 = "MAT4",
    }
}

/// [A keyframe animation]
/// (https://github.com/KhronosGroup/glTF/blob/d63b796e6b7f6b084c710b97b048d59d749cb04a/specification/2.0/schema/animation.schema.json)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Animation {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Defines the channels of the animation
    pub channels: Vec<AnimationChannel>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Defines samplers that combine input and output accessors
    pub samplers: Vec<AnimationSampler>,
}

/// Targets an animation's sampler at a node's property
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnimationChannel {
    /// The index of the sampler used to compute the value for the target
    pub sampler: Index<Sampler>,
    /// The index of the node and TRS property to target
    pub target: AnimationChannelTarget,
}

/// The index of the node and TRS property that an animation channel targets
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnimationChannelTarget {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// The index of the node to target
    pub node: Index<Node>,
    /// The name of the node's TRS property to modify
    pub path: AnimationChannelTargetPath,
}

enum_string! {
    AnimationChannelTargetPath {
        Rotation = "rotation",
        Scale = "scale",
        Translation = "translation",
    }
}

/// Defines a keyframe graph but not its target
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnimationSampler {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// The index of the accessor containing keyframe input values (e.g. time)
    pub input: Index<Accessor>,
    /// The interpolation algorithm
    pub interpolation: AnimationSamplerInterpolation,
    /// The index of an accessor containing keyframe output values
    pub output: Index<Accessor>,
}

enum_string! {
    AnimationSamplerInterpolation {
        Linear = "LINEAR",
        Step = "STEP",
    }
}

/// [Contains metadata about the glTF asset]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#asset)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Asset {
    /// A copyright message suitable for display to credit the content creator
    pub copyright: Option<String>,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Tool that generated this glTF model
    pub generator: Option<String>,
    /// glTF version
    #[serde(default = "asset_version_default")]
    pub version: String,
}

fn asset_version_default() -> String {
    "2.0".to_string()
}

/// [The identifier of the `BufferView` this accessor reads from.
/// Describes the location, type, and size of a binary blob included with the asset]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#buffer)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Buffer {
    /// The length of the buffer in bytes
    #[serde(default, rename = "byteLength")]
    pub byte_length: u32,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Uniform resource locator for the buffer data relative to the .gltf file
    // N.B. the spec says this is not required but I think that is incorrect
    pub uri: String,
}

/// [Represents a subset of a `Buffer`]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#buffers-and-buffer-views)  
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BufferView {
    /// The index of the parent `Buffer`
    pub buffer: Index<Buffer>,
    /// The length of the buffer view data in bytes
    #[serde(rename = "byteLength")]
    pub byte_length: u32,
    /// Offset into the parent buffer in bytes
    #[serde(rename = "byteOffset")]
    pub byte_offset: u32,
    /// The stride in bytes between vertex attributes in this buffer view
    #[serde(default)]
    pub byte_stride: u32,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Optional target the buffer should be bound to
    pub target: Option<BufferTarget>,
}

enum_number! {
    BufferTarget {
        ArrayBuffer = 34962,
        ElementArrayBuffer = 34963,
    }
}

// TODO: This implementation is rubbish. Replace with enum instead
// and derive (De)Serialize manually. It would be trivial to do so
// if it were not for the `name`, `extension`, and `extra` fields.
/// A camera's projection
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Camera {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Orthographic camera values
    pub orthographic: Option<CameraOrthographic>,
    /// Perspective camera values
    pub perspective: Option<CameraPerspective>,
    /// `"perspective"` or `"orthographic"`
    #[serde(rename = "type")]
    pub ty: String, 
}

/// Values for an orthographic camera
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CameraOrthographic {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// The horizontal magnification of the view
    #[serde(default, rename = "xmag")]
    pub x_mag: f32,
    /// The vertical magnification of the view
    #[serde(default, rename = "ymag")]
    pub y_mag: f32,
    /// The distance to the far clipping plane
    #[serde(default, rename = "zfar")]
    pub z_far: f32,
    /// The distance to the near clipping plane
    #[serde(default, rename = "znear")]
    pub z_near: f32,
}

/// Values for a perspective camera
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CameraPerspective {
    /// Aspect ratio of the field of view
    #[serde(default, rename = "aspectRatio")]
    pub aspect_ratio: f32,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// The vertical field of view in radians
    #[serde(default, rename = "yfov")]
    pub y_fov: f32,
    /// The distance to the far clipping plane
    #[serde(default, rename = "zfar")]
    pub z_far: f32,
    /// The distance to the near clipping plane
    #[serde(default, rename = "znear")]
    pub z_near: f32,
}

/// Image data used to create a texture
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Image {
    /// The index of the `BufferView` that contains the image
    #[serde(rename = "bufferView")]
    pub buffer_view: Option<Index<BufferView>>,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// The image's MIME type
    // N.B. The spec says this is required but the sample models don't provide it
    // TODO: Remove `Option` as necessary
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// The uniform resource identifier of the image relative to the .gltf file
    pub uri: Option<String>,
}

/// [Describes the material appearance of a primitive]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#material)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Material {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Defines the metallic-roughness material model from Physically-Based Rendering (PBR) methodology
    #[serde(rename = "pbrMetallicRoughness")]
    pub pbr: MaterialPbrMetallicRoughness,
    #[serde(rename = "normalTexture")]
    pub normal_texture: MaterialNormalTexture,
    #[serde(rename = "occlusionTexture")]
    pub occlusion_texture: MaterialOcclusionTexture,
    #[serde(rename = "emissiveTexture")]
    pub emissive_texture: TextureInfo,
    #[serde(rename = "emissiveFactor")]
    #[serde(default)]
    pub emissive_factor: [f32; 3],
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialPbrMetallicRoughness {
    /// The base color factor
    #[serde(default = "material_pbr_metallic_roughness_base_color_factor_default")]
    #[serde(rename = "baseColorFactor")]
    pub base_color_factor: [f32; 4],
    /// The base color texture
    #[serde(rename = "baseColorTexture")]
    pub base_color_texture: TextureInfo,
    /// The metalness of the material
    #[serde(default = "material_pbr_metallic_roughness_metallic_factor_default")]
    #[serde(rename = "metallicFactor")]
    pub metallic_factor: f32,
    /// The roughness of the material
    #[serde(default = "material_pbr_metallic_roughness_roughness_factor_default")]
    #[serde(rename = "roughnessFactor")]
    pub roughness_factor: f32,
    /// The metallic-roughness texture
    #[serde(rename = "metallicRoughnessTexture")]
    pub metallic_roughness_texture: TextureInfo,
}

fn material_pbr_metallic_roughness_base_color_factor_default() -> [f32; 4] {
    [1.0, 1.0, 1.0, 1.0]
}

fn material_pbr_metallic_roughness_metallic_factor_default() -> f32 {
    1.0
}

fn material_pbr_metallic_roughness_roughness_factor_default() -> f32 {
    1.0
}

/// Defines the normal texture of a material
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialNormalTexture {
    /// The index of the texture
    pub index: Index<Texture>,
    /// The scalar multiplier applied to each normal vector of the normal texture
    #[serde(default = "material_normal_texture_scale_default")]
    pub scale: f32,
    /// The set index of the texture's `TEXCOORD` attribute
    #[serde(default, rename = "texCoord")]
    pub tex_coord: u32,
}

fn material_normal_texture_scale_default() -> f32 {
    1.0
}

/// Defines the occlusion texture of a material
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialOcclusionTexture {
    /// The index of the texture
    pub index: Index<Texture>,
    /// The scalar multiplier controlling the amount of occlusion applied
    #[serde(default = "material_occlusion_texture_strength_default")]
    pub strength: f32,
    /// The set index of the texture's `TEXCOORD` attribute
    #[serde(default, rename = "texCoord")]
    pub tex_coord: u32,
}

fn material_occlusion_texture_strength_default() -> f32 {
    1.0
}

/// [A set of primitives to be rendered]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#mesh)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Mesh {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Defines the geometry of this mesh to be renderered with a material
    pub primitives: Vec<MeshPrimitive>,
    /// Defines the weights to be applied to the morph targets
    #[serde(default)]
    pub weights: Vec<f32>,
}

/// [Geometry to be rendered with the given material]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#meshprimitive)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MeshPrimitive {
    /// Maps attribute semantic names to the `Accessor`s containing their data
    #[serde(default)]
    pub attributes: std::collections::HashMap<String, Index<Accessor>>,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Index of the `Accessor` containing mesh indices
    pub indices: Option<Index<Accessor>>,
    /// The index of the material to apply to this primitive when rendering
    pub material: Index<Material>,
    /// The type of primitives to render
    #[serde(default)]
    pub mode: MeshPrimitiveMode,
    #[serde(default)]
    /// Morph targets
    // TODO: Confirm that this the correct implementation and update
    // `Root::indices_are_valid()` as required
    pub targets: Vec<std::collections::HashMap<String, Index<Accessor>>>,
}

enum_number! {
    MeshPrimitiveMode {
        Points = 0,
        Lines = 1,
        LineLoop = 2,
        LineStrip = 3,
        Triangles = 4,
        TriangleStrip = 5,
        TriangleFan = 6,
    }
}

/// [A single member of the glTF scene hierarchy]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#scenes)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Node {
    /// The index of the camera referenced by this node
    // N.B. The spec says this is required but the sample models don't provide it
    // TODO: Remove `Option` as necessary and update
    // `Root::indices_are_valid()` as required
    pub camera: Option<Index<Camera>>,
    /// The indices of this node's children
    #[serde(default)]
    pub children: Vec<Index<Node>>,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// 4x4 column-major transformation matrix
    #[serde(default = "node_matrix_default")]
    pub matrix: [[f32; 4]; 4],
    /// The index of the `Mesh` in this node
    pub mesh: Index<Mesh>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// The node's unit quaternion rotation `[x, y, z, w]`
    #[serde(default = "node_rotation_default")]
    pub rotation: [f32; 4],
    #[serde(default = "node_scale_default")]
    /// The node's non-uniform scale
    pub scale: [f32; 3],
    #[serde(default)]
    /// The node's translation
    pub translation: [f32; 3],
    /// The index of the skin referenced by this node
    // N.B. The spec says this is required but the sample models don't provide it
    // TODO: Remove `Option` as necessary and update
    // `Root::indices_are_valid()` as required
    pub skin: Option<Index<Skin>>,
    /// The weights of the morph target
    // N.B. The spec says this is required but the sample models don't provide it
    // TODO: Remove `Option` as necessary and update
    // `Root::indices_are_valid()` as required
    pub weights: Option<Vec<f32>>,
}

fn node_matrix_default() -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn node_rotation_default() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

fn node_scale_default() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

/// [Defines texture sampler properties for filtering and wrapping modes]
/// (https://github.com/KhronosGroup/glTF/blob/d63b796e6b7f6b084c710b97b048d59d749cb04a/specification/2.0/schema/sampler.schema.json)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Sampler {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Magnification filter
    #[serde(default, rename = "magFilter")]
    pub mag_filter: SamplerMagFilter,
    /// Minification filter
    #[serde(default, rename = "minFilter")]
    pub min_filter: SamplerMinFilter,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// s wrapping mode
    #[serde(default, rename = "wrapS")]
    pub wrap_s: SamplerWrappingMode,
    /// t wrapping mode
    #[serde(default, rename = "wrapT")]
    pub wrap_t: SamplerWrappingMode,
}

enum_number! {
    SamplerMagFilter {
        Nearest = 9728,
        Linear = 9729,
    }
}

enum_number! {
    SamplerMinFilter {
        Nearest = 9728,
        Linear = 9729,
        NearestMipmapNearest = 9984,
        LinearMipmapNearest = 9985,
        NearestMipmapLinear = 9986,
        LinearMipmapLinear = 9987,
    }
}

enum_number! {
    SamplerWrappingMode {
        ClampToEdge = 33071,
        MirroredRepeat = 33648,
        Repeat = 10497,
    }
}

/// [A set of visual objects to render](https://github.com/KhronosGroup/glTF/tree/2.0/specification/2.0#scenes)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// The indices of each root `Node` in this scene
    #[serde(default)]
    pub nodes: Vec<Index<Node>>,
}

/// [Joints and matrices defining a skin](https://github.com/KhronosGroup/glTF/blob/d63b796e6b7f6b084c710b97b048d59d749cb04a/specification/2.0/schema/skin.schema.json)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Skin {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// The index of the accessor containing the 4x4 inverse-bind matrices
    #[serde(rename = "inverseBindMatrices")]
    pub inverse_bind_matrices: Option<Index<Accessor>>,
    /// Indices of skeleton nodes used as joints in this skin
    pub joints: Vec<Index<Node>>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// The index of the node used as a skeleton root
    pub skeleton: Option<Index<Node>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Texture {
    /// Texel data type
    #[serde(default, rename = "type")]
    pub data_type: TextureDataType,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// The texture format
    #[serde(default)]
    pub format: TextureFormat,
    /// The texture internal format
    #[serde(default, rename = "internalFormat")]
    pub internal_format: TextureFormat,
    /// The index of the sampler used by this texture
    pub sampler: Index<Sampler>,
    /// The index of the image used by this texture
    pub source: Index<Image>,
    /// The target the texture should be bound to
    #[serde(default)]
    pub target: TextureTarget,
}

enum_number! {
    TextureDataType {
        U8 = 5121,
        U16_R5_G6_B5 = 33635,
        U16_R4_G4_B4_A4 = 32819,
        U16_R5_G5_B5_A1 = 32820,
    }
}

enum_number! {
    TextureFormat {
        Alpha = 6406,
        Rgb = 6407,
        Rgba = 6408,
        Luminance = 6409,
        LuminanceAlpha = 6410,
    }
}

enum_number! {
    TextureTarget {
        Texture2d = 3553,
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
/// Reference to a `Texture`
pub struct TextureInfo {
    /// The index of the texture
    pub index: Index<Texture>,
    /// The set index of the texture's `TEXCOORD` attribute
    #[serde(default, rename = "texCoord")]
    pub tex_coord: u32,
}

impl Default for MeshPrimitiveMode {
    fn default() -> Self {
        MeshPrimitiveMode::Triangles
    }
}

impl Default for SamplerMagFilter {
    fn default() -> Self {
        SamplerMagFilter::Linear
    }
}

impl Default for SamplerMinFilter {
    fn default() -> Self {
        SamplerMinFilter::NearestMipmapLinear
    }
}

impl Default for SamplerWrappingMode {
    fn default() -> Self {
        SamplerWrappingMode::Repeat
    }
}

impl Default for TextureDataType {
    fn default() -> Self {
        TextureDataType::U8
    }
}

impl Default for TextureFormat {
    fn default() -> Self {
        TextureFormat::Rgba
    }
}

impl Default for TextureTarget {
    fn default() -> Self {
        TextureTarget::Texture2d
    }
}

impl Root {
    /// Loads a glTF version 2.0 asset from raw JSON
    pub fn import_from_str(json: &str) -> Result<Self, ImportError> {
        let root: Root = serde_json::from_str(json)
            .map_err(|err| ImportError::Deserialize(err))?;
        if root.indices_are_valid() {
            Ok(root)
        } else {
            Err(ImportError::Invalid("index out of range".to_string()))
        }
    }

    /// Returns the accessor at the given index
    pub fn accessor(&self, index: Index<Accessor>) -> &Accessor {
    &self.accessors[index.0 as usize]
}

/// Returns all accessors as a slice
pub fn accessors(&self) -> &[Accessor] {
&self.accessors
    }
    
    /// Returns the metadata included with this asset
    pub fn asset(&self) -> &Asset {
        &self.asset
    }

    /// Returns the buffer at the given index
    pub fn buffer(&self, index: Index<Buffer>) -> &Buffer {
        &self.buffers[index.0 as usize]
    }

    /// Returns all buffers as a slice
    pub fn buffers(&self) -> &[Buffer] {
        &self.buffers
    }
    
    /// Returns the buffer view at the given index
    pub fn buffer_view(&self, index: Index<BufferView>) -> &BufferView {
        &self.buffer_views[index.0 as usize]
    }

    /// Returns all buffer views as a slice
    pub fn buffer_views(&self) -> &[BufferView] {
        &self.buffer_views
    }

    /// Returns the camera at the given index
    pub fn camera(&self, index: Index<Camera>) -> &Camera {
        &self.cameras[index.0 as usize]
    }

    /// Returns all cameras as a slice
    pub fn cameras(&self) -> &[Camera] {
        &self.cameras
    }

    /// Returns the extensions referenced in this .gltf file
    pub fn extensions_used(&self) -> &[String] {
        &self.extensions_used
    }

    /// Returns the extensions required to load and render this asset
    pub fn extensions_required(&self) -> &[String] {
        &self.extensions_required
    }

    /// Returns the image at the given index
    pub fn image(&self, index: Index<Image>) -> &Image {
        &self.images[index.0 as usize]
    }

    /// Returns all images as a slice
    pub fn images(&self) -> &[Image] {
        &self.images
    }

    /// Returns the material at the given index
    pub fn material(&self, index: Index<Material>) -> &Material {
        &self.materials[index.0 as usize]
    }

    /// Returns all materials as a slice
    pub fn materials(&self) -> &[Material] {
        &self.materials
    }

    /// Returns the mesh at the given index
    pub fn mesh(&self, index: Index<Mesh>) -> &Mesh {
        &self.meshes[index.0 as usize]
    }

    /// Returns all meshes as a slice
    pub fn meshes(&self) -> &[Mesh] {
        &self.meshes
    }
    
    /// Returns the node at the given index
    pub fn node(&self, index: Index<Node>) -> &Node {
        &self.nodes[index.0 as usize]
    }

    /// Returns all nodes as a slice
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    /// Returns the sampler at the given index
    pub fn sampler(&self, index: Index<Sampler>) -> &Sampler {
        &self.samplers[index.0 as usize]
    }

    /// Returns all samplers as a slice
    pub fn samplers(&self) -> &[Sampler] {
        &self.samplers
    }
    
    /// Returns the scene at the given index
    pub fn scene(&self, index: Index<Scene>) -> &Scene {
        &self.scenes[index.0 as usize]
    }

    /// Returns all scenes as a slice
    pub fn scenes(&self) -> &[Scene] {
        &self.scenes
    }

    /// Returns the skin at the given index
    pub fn skin(&self, index: Index<Skin>) -> &Skin {
        &self.skins[index.0 as usize]
    }

    /// Returns all skins as a slice
    pub fn skins(&self) -> &[Skin] {
        &self.skins
    }

    /// Returns the texture at the given index
    pub fn texture(&self, index: Index<Texture>) -> &Texture {
        &self.textures[index.0 as usize]
    }

    /// Returns all textures as a slice
    pub fn textures(&self) -> &[Texture] {
        &self.textures
    }

    /// Performs a search for any indices that are out of range of the array
    /// they reference. Returns true if all indices are within range.
    fn indices_are_valid(&self) -> bool {
        // TODO: Implement me
        true
    }
}

impl<T> serde::Serialize for Index<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ::serde::Serializer
    {
        serializer.serialize_u64(self.value() as u64)
    }
}

impl<T> serde::Deserialize for Index<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer
    {
        struct Visitor<T>(std::marker::PhantomData<T>);
        impl<T> serde::de::Visitor for Visitor<T> {
            type Value = Index<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter)
                         -> std::fmt::Result
            {
                formatter.write_str("GLenum")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                where E: serde::de::Error
            {
                Ok(Index::new(value as u32))
            }
        }
        deserializer.deserialize_u64(Visitor::<T>(std::marker::PhantomData))
    }
}
