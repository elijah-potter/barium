use std::f32::consts::PI;

use glam::Vec2;

pub trait Vec2Ext {
    /// Rotate the [Vec2] around (0, 0).
    fn rotate(self, radians: f32) -> Self;
}

impl Vec2Ext for Vec2 {
    fn rotate(self, radians: f32) -> Self {
        let l = self.length();
        let mut a = (self.y / self.x).atan() + radians;

        if self.x < 0.0 {
            a += PI;
        }

        Vec2::new(a.cos(), a.sin()) * l
    }
}
