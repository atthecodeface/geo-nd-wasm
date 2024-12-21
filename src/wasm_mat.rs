//a Imports

#[macro_export]
macro_rules! wasm_mat {
    ($t:ident, $m:ident, $v:ident, $f:ty) => {
        #[wasm_bindgen]
        pub struct $t($m);

        #[wasm_bindgen]
        impl $t {
            pub fn identity() -> $t {
                $m::identity().into()
            }

            pub fn from_array(f: &[$f]) -> $t {
                let mut m = $m::default();
                for (i, f) in f.iter().enumerate() {
                    m[i] = *f;
                }
                m.into()
            }

            #[wasm_bindgen(getter)]
            pub fn is_zero(&self) -> bool {
                self.0.is_zero()
            }
            pub fn determinant(&self) -> $f {
                self.0.determinant()
            }

            pub fn add(&self, other: &$t) -> $t {
                (self.0 + other.0).into()
            }
            pub fn sub(&self, other: &$t) -> $t {
                (self.0 - other.0).into()
            }
            pub fn mul(&self, other: &$t) -> $t {
                (self.0 * other.0).into()
            }
            pub fn mulf(&self, f: $f) -> $t {
                (self.0 * f).into()
            }
            pub fn divf(&self, f: $f) -> $t {
                (self.0 / f).into()
            }

            pub fn transpose(&self) -> $t {
                self.0.transpose().into()
            }
            pub fn inverse(&self) -> $t {
                self.0.inverse().into()
            }

            pub fn transform(&self, v: &$v) -> $v {
                self.0.transform(&v.0).into()
            }

            //zz All done
        }

        impl From<$m> for $t {
            fn from(f: $m) -> $t {
                $t(f)
            }
        }

        impl From<$t> for $m {
            fn from(f: $t) -> $m {
                f.0
            }
        }
        impl From<&$t> for $m {
            fn from(f: &$t) -> $m {
                f.0
            }
        }
        impl AsMut<$m> for $t {
            fn as_mut(&mut self) -> &mut $m {
                &mut self.0
            }
        }
    };
}

#[macro_export]
macro_rules! wasm_mat3 {
    ($t:ident, $m:ident, $v:ident, $f:ty) => {
        #[wasm_bindgen]
        impl $t {
            #[wasm_bindgen(getter)]
            pub fn array(&self) -> Box<[$f]> {
                let v: [$f; 9] = self.0.into_array();
                v.into()
            }

            //zz All done
        }

        impl From<[$f; 9]> for $t {
            fn from(f: [$f; 9]) -> $t {
                $t($m::from_array(f))
            }
        }
    };
}

#[macro_export]
macro_rules! wasm_mat4 {
    ($t:ident, $m:ident, $v3:ident, $v4:ident, $f:ty) => {
        #[wasm_bindgen]
        impl $t {
            pub fn perspective(fov: $f, aspect: $f, near: $f, far: $f) -> $t {
                let m = $m::perspective(fov, aspect, near, far);
                m.into()
            }

            pub fn look_at(eye: &$v3, center: &$v3, up: &$v3) -> $t {
                let m = $m::look_at(&eye.0, &center.0, &up.0);
                m.into()
            }

            #[wasm_bindgen(getter)]
            pub fn array(&self) -> Box<[$f]> {
                let v: [$f; 16] = $m::into_array(self.0);
                v.into()
            }

            pub fn translate3(&mut self, by: &$v3) {
                self.0.translate3(&by.0);
            }

            pub fn translate4(&mut self, by: &$v4) {
                self.0.translate4(&by.0);
            }

            //zz All done
        }

        impl From<[$f; 16]> for $t {
            fn from(f: [$f; 16]) -> $t {
                $t(f.into())
            }
        }
    };
}
