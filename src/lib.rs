use wasm_bindgen::prelude::*;

use geo_nd::{FArray, QArray, Quaternion, Vector, Vector3};

pub type Vec3f32 = FArray<f32, 3>;
pub type Vec4f32 = FArray<f32, 4>;
pub type Quatf32 = QArray<f32, Vec3f32, Vec4f32>;

pub type Vec3f64 = FArray<f64, 3>;
pub type Vec4f64 = FArray<f64, 4>;
pub type Quatf64 = QArray<f64, Vec3f64, Vec4f64>;

mod wasm_vec;
wasm_vec!(WasmVec3f32, Vec3f32, f32);
wasm_vec!(WasmVec3f64, Vec3f64, f64);

mod wasm_quat;
wasm_quat!(WasmQuatf32, Quatf32, WasmVec3f32, f32);
wasm_quat!(WasmQuatf64, Quatf64, WasmVec3f64, f64);
