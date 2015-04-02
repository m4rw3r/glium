use std::cell::RefCell;
use uniforms::UniformValue;

pub struct UniformsStorage {
    values: RefCell<Vec<Option<UniformValue<'static>>>>,
}

impl UniformsStorage {
    pub fn new() -> UniformsStorage {
        UniformsStorage {
            values: RefCell::new(Vec::with_capacity(0)),
        }
    }

    /// Compares the old value with the new value, replaces the old with the new, and
    /// returns `true` if the values were equal.
    pub fn compare_and_store(&self, uniform_location: u32, value: &UniformValue) -> bool {
        let mut values = self.values.borrow_mut();

        if values.len() <= uniform_location as usize {
            values.reserve(uniform_location as usize + 1);
            for _ in (values.len() .. uniform_location as usize + 1) {
                values.push(None);
            }
        }

        match (value, &mut values[uniform_location as usize]) {
            (&UniformValue::SignedInt(a), &mut Some(UniformValue::SignedInt(b))) if a == b => true,
            (&UniformValue::UnsignedInt(a), &mut Some(UniformValue::UnsignedInt(b))) if a == b => true,
            (&UniformValue::Float(a), &mut Some(UniformValue::Float(b))) if a == b => true,
            (&UniformValue::Mat2(a), &mut Some(UniformValue::Mat2(b))) if a == b => true,
            (&UniformValue::Mat3(a), &mut Some(UniformValue::Mat3(b))) if a == b => true,
            (&UniformValue::Mat4(a), &mut Some(UniformValue::Mat4(b))) if a == b => true,
            (&UniformValue::Vec2(a), &mut Some(UniformValue::Vec2(b))) if a == b => true,
            (&UniformValue::Vec3(a), &mut Some(UniformValue::Vec3(b))) if a == b => true,
            (&UniformValue::Vec4(a), &mut Some(UniformValue::Vec4(b))) if a == b => true,

            (&UniformValue::SignedInt(v), target) => {
                *target = Some(UniformValue::SignedInt(v));
                false
            },

            (&UniformValue::UnsignedInt(v), target) => {
                *target = Some(UniformValue::UnsignedInt(v));
                false
            },
            
            (&UniformValue::Float(v), target) => {
                *target = Some(UniformValue::Float(v));
                false
            },
            
            (&UniformValue::Mat2(v), target) => {
                *target = Some(UniformValue::Mat2(v));
                false
            },
            
            (&UniformValue::Mat3(v), target) => {
                *target = Some(UniformValue::Mat3(v));
                false
            },
            
            (&UniformValue::Mat4(v), target) => {
                *target = Some(UniformValue::Mat4(v));
                false
            },
            
            (&UniformValue::Vec2(v), target) => {
                *target = Some(UniformValue::Vec2(v));
                false
            },
            
            (&UniformValue::Vec3(v), target) => {
                *target = Some(UniformValue::Vec3(v));
                false
            },
            
            (&UniformValue::Vec4(v), target) => {
                *target = Some(UniformValue::Vec4(v));
                false
            },

            _ => false      // we ignore all textures stuff for now
        }
    }
}
