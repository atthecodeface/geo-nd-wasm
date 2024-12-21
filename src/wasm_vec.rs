//a Imports

#[macro_export]
macro_rules! wasm_vec {
    ($t:ident, $v:ident, $n:expr, $f:ty) => {
        #[wasm_bindgen]
        pub struct $t($v);

        #[wasm_bindgen]
        impl $t {
            #[wasm_bindgen(getter)]
            pub fn is_zero(&self) -> bool {
                self.0.is_zero()
            }
            pub fn reduce_sum(&self) -> $f {
                self.0.reduce_sum()
            }
            #[wasm_bindgen(getter)]
            pub fn length_sq(&self) -> $f {
                self.0.length_sq()
            }

            #[wasm_bindgen(getter)]
            pub fn length(&self) -> $f {
                self.0.length()
            }

            pub fn neg(&self) -> $t {
                (-self.0).into()
            }
            pub fn add(&self, other: &$t) -> $t {
                (self.0 + other.0).into()
            }
            pub fn sub(&self, other: &$t) -> $t {
                (self.0 - other.0).into()
            }
            pub fn mulf(&self, f: $f) -> $t {
                (self.0 * f).into()
            }
            pub fn divf(&self, f: $f) -> $t {
                (self.0 / f).into()
            }

            pub fn normalize(&self) -> $t {
                self.0.normalize().into()
            }

            pub fn dot(&self, other: &$t) -> $f {
                self.0.dot(&other.0)
            }
            pub fn distance_sq(&self, other: &$t) -> $f {
                self.0.distance_sq(&other.0)
            }
            pub fn distance(&self, other: &$t) -> $f {
                self.0.distance(&other.0)
            }

            pub fn mix(&self, other: &$t, t: $f) -> $t {
                self.0.mix(&other.0, t).into()
            }

            pub fn set(&mut self, array: &[$f]) {
                if array.len() < $n {
                    return;
                }
                AsMut::<[$f; $n]>::as_mut(&mut self.0).copy_from_slice(array);
            }

            #[wasm_bindgen(getter)]
            pub fn array(&self) -> Box<[$f]> {
                let v: [$f; $n] = self.0.into();
                v.into()
            }

            //zz All done
        }

        impl From<$v> for $t {
            fn from(f: $v) -> $t {
                $t(f)
            }
        }

        impl From<$t> for $v {
            fn from(f: $t) -> $v {
                f.0
            }
        }
        impl From<&$t> for $v {
            fn from(f: &$t) -> $v {
                f.0
            }
        }
        impl From<[$f; $n]> for $t {
            fn from(f: [$f; $n]) -> $t {
                $t(f.into())
            }
        }
    };
}

#[macro_export]
macro_rules! wasm_vec2 {
    ($t:ident, $v:ident, $f:ty) => {
        #[wasm_bindgen]
        impl $t {
            //cp new
            /// Create a new
            #[wasm_bindgen(constructor)]
            pub fn new(x: $f, y: $f) -> $t {
                Self([x, y].into())
            }

            //zz All done
        }
    };
}

#[macro_export]
macro_rules! wasm_vec3 {
    ($t:ident, $v:ident, $f:ty) => {
        #[wasm_bindgen]
        impl $t {
            //cp new
            /// Create a new
            #[wasm_bindgen(constructor)]
            pub fn new(x: $f, y: $f, z: $f) -> $t {
                Self([x, y, z].into())
            }

            pub fn cross_product(&self, other: &$t) -> $t {
                self.0.cross_product(&other.0).into()
            }
            //zz All done
        }
    };
}

#[macro_export]
macro_rules! wasm_vec4 {
    ($t:ident, $v:ident, $f:ty) => {
        #[wasm_bindgen]
        impl $t {
            //cp new
            /// Create a new
            #[wasm_bindgen(constructor)]
            pub fn new(x: $f, y: $f, z: $f, w: $f) -> $t {
                Self([x, y, z, w].into())
            }

            //zz All done
        }
    };
}
