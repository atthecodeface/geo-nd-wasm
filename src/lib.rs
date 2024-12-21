use wasm_bindgen::prelude::*;

use geo_nd::{FArray, FArray2, QArray, Quaternion, SqMatrix, SqMatrix4, Vector, Vector3};

pub type Vec2f32 = FArray<f32, 2>;
pub type Vec3f32 = FArray<f32, 3>;
pub type Vec4f32 = FArray<f32, 4>;
pub type Mat3f32 = FArray2<f32, 3, 9>;
pub type Mat4f32 = FArray2<f32, 4, 16>;
pub type Quatf32 = QArray<f32, Vec3f32, Vec4f32>;

pub type Vec2f64 = FArray<f64, 2>;
pub type Vec3f64 = FArray<f64, 3>;
pub type Vec4f64 = FArray<f64, 4>;
pub type Mat3f64 = FArray2<f64, 3, 9>;
pub type Mat4f64 = FArray2<f64, 4, 16>;
pub type Quatf64 = QArray<f64, Vec3f64, Vec4f64>;

mod wasm_vec;
wasm_vec!(WasmVec2f32, Vec2f32, 2, f32);
wasm_vec2!(WasmVec2f32, Vec2f32, f32);
wasm_vec!(WasmVec2f64, Vec2f64, 2, f64);
wasm_vec2!(WasmVec2f64, Vec2f64, f64);

wasm_vec!(WasmVec3f32, Vec3f32, 3, f32);
wasm_vec3!(WasmVec3f32, Vec3f32, f32);
wasm_vec!(WasmVec3f64, Vec3f64, 3, f64);
wasm_vec3!(WasmVec3f64, Vec3f64, f64);

wasm_vec!(WasmVec4f32, Vec4f32, 4, f32);
wasm_vec4!(WasmVec4f32, Vec4f32, f32);
wasm_vec!(WasmVec4f64, Vec4f64, 4, f64);
wasm_vec4!(WasmVec4f64, Vec4f64, f64);

mod wasm_mat;
wasm_mat!(WasmMat3f32, Mat3f32, WasmVec3f32, f32);
wasm_mat3!(WasmMat3f32, Mat3f32, WasmVec3f32, f32);
wasm_mat!(WasmMat3f64, Mat3f64, WasmVec3f64, f64);
wasm_mat3!(WasmMat3f64, Mat3f64, WasmVec3f64, f64);

wasm_mat!(WasmMat4f32, Mat4f32, WasmVec4f32, f32);
wasm_mat4!(WasmMat4f32, Mat4f32, WasmVec3f32, WasmVec4f32, f32);
wasm_mat!(WasmMat4f64, Mat4f64, WasmVec4f64, f64);
wasm_mat4!(WasmMat4f64, Mat4f64, WasmVec3f64, WasmVec4f64, f64);

mod wasm_quat;
wasm_quat!(
    WasmQuatf32,
    Quatf32,
    WasmVec3f32,
    WasmMat3f32,
    WasmMat4f32,
    f32
);
wasm_quat!(
    WasmQuatf64,
    Quatf64,
    WasmVec3f64,
    WasmMat3f64,
    WasmMat4f64,
    f64
);
