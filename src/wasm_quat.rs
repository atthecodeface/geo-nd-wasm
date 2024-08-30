//a Imports

#[macro_export]
macro_rules! wasm_quat {
    ($t:ident, $q:ident, $v:ident, $m3:ident, $m4:ident, $f:ty) => {
        #[wasm_bindgen]
        pub struct $t($q);

        #[wasm_bindgen]
        impl $t {
            #[wasm_bindgen(constructor)]
            pub fn new(i: $f, j: $f, k: $f, r: $f) -> $t {
                Self([i, j, k, r].into())
            }

            #[wasm_bindgen(getter)]
            pub fn array(&self) -> Box<[$f]> {
                let v: [$f; 4] = self.0.into();
                v.into()
            }

            #[wasm_bindgen(getter)]
            pub fn r(&self) -> $f {
                self.0.as_rijk().0
            }

            #[wasm_bindgen(getter)]
            pub fn i(&self) -> $f {
                self.0.as_rijk().1
            }

            #[wasm_bindgen(getter)]
            pub fn j(&self) -> $f {
                self.0.as_rijk().2
            }

            #[wasm_bindgen(getter)]
            pub fn k(&self) -> $f {
                self.0.as_rijk().3
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
            pub fn mul(&self, other: &$t) -> $t {
                (self.0 * other.0).into()
            }
            pub fn div(&self, other: &$t) -> $t {
                (self.0 / other.0).into()
            }
            pub fn mulf(&self, f: $f) -> $t {
                (self.0 * f).into()
            }
            pub fn divf(&self, f: $f) -> $t {
                (self.0 / f).into()
            }

            pub fn unit() -> $t {
                $q::unit().into()
            }
            pub fn of_axis_angle(axis: &$v, angle: $f) -> $t {
                $q::of_axis_angle(&axis.0, angle).into()
            }
            pub fn conjugate(&self) -> $t {
                self.0.conjugate().into()
            }

            #[wasm_bindgen(getter)]
            pub fn length_sq(&self) -> $f {
                self.0.length_sq()
            }

            #[wasm_bindgen(getter)]
            pub fn length(&self) -> $f {
                self.0.length()
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

            pub fn rotate_x(&self, angle: $f) -> $t {
                self.0.rotate_x(angle).into()
            }
            pub fn rotate_y(&self, angle: $f) -> $t {
                self.0.rotate_y(angle).into()
            }
            pub fn rotate_z(&self, angle: $f) -> $t {
                self.0.rotate_z(angle).into()
            }

            pub fn look_at(dirn: &$v, up: &$v) -> $t {
                $q::look_at(&dirn.0, &up.0).into()
            }

            pub fn rotation_of_vec_to_vec(a: &$v, b: &$v) -> $t {
                $q::rotation_of_vec_to_vec(&a.0, &b.0).into()
            }
            pub fn apply3(&self, v: &$v) -> $v {
                self.0.apply3(&v.0).into()
            }

            pub fn set_rotation3(&self, m: &mut $m3) {
                self.0.set_rotation3(m.as_mut())
            }

            pub fn set_rotation4(&self, m: &mut $m4) {
                self.0.set_rotation4(m.as_mut())
            }

            //zz All done
        }

        impl From<$q> for $t {
            fn from(f: $q) -> $t {
                $t(f)
            }
        }
    };
}
