use std::ops::{Add, AddAssign, Sub, SubAssign};

use glam::Vec2;

#[derive(Default, Clone, Copy, Debug)]
pub struct Transform {
    /// Position offset in parent space.
    pub translate: Vec2,
    /// Rotation of the element. In radians.
    pub rotation: f32,
    /// Size of the object relative to parent.
    pub scale: Vec2,
}

impl Transform {
    pub fn new(translate: Vec2, rotation: f32, scale: Vec2) -> Self {
        Self {
            translate,
            rotation,
            scale,
        }
    }

    /// Produces a transform that does nothing.
    pub fn one() -> Self{
        Self::new(Vec2::ZERO, 0.0, Vec2::ONE)
    }

    /// Gets a [Transform] with the original's translation, but rotation and scale are set to 0, and [one](Vec2), respectively.
    pub fn extract_translate(&self) -> Self {
        Self {
            translate: self.translate,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }

    /// Gets a [Transform] with the original's rotation, but translate and scale are set to [zero](Vec2) and [one](Vec2), respectively.
    pub fn extract_rotation(&self) -> Self {
        Self {
            translate: Vec2::ZERO,
            rotation: self.rotation,
            scale: Vec2::ONE,
        }
    }

    /// Gets a [Transform] with the original's scale, but rotation and translate are set to 0, and [one](Vec2), respectively.
    pub fn extract_scale(&self) -> Self {
        Self {
            translate: Vec2::ZERO,
            rotation: 0.0,
            scale: self.scale,
        }
    }

    pub fn identity() -> Self {
        Self::default()
    }
}

impl Add<Transform> for Transform {
    type Output = Transform;

    fn add(self, rhs: Transform) -> Self::Output {
        Self {
            translate: self.translate + rhs.translate,
            rotation: self.rotation + rhs.rotation,
            scale: self.scale + rhs.scale,
        }
    }
}

impl AddAssign<Transform> for Transform {
    fn add_assign(&mut self, rhs: Transform) {
        self.translate += rhs.translate;
        self.rotation += rhs.rotation;
        self.scale += rhs.rotation;
    }
}

impl Sub<Transform> for Transform {
    type Output = Transform;

    fn sub(self, rhs: Transform) -> Self::Output {
        Self {
            translate: self.translate - rhs.translate,
            rotation: self.rotation - rhs.rotation,
            scale: self.scale - rhs.scale,
        }
    }
}

impl SubAssign<Transform> for Transform {
    fn sub_assign(&mut self, rhs: Transform) {
        self.translate += rhs.translate;
        self.rotation += rhs.rotation;
        self.scale += rhs.rotation;
    }
}

#[cfg(feature = "tiny_skia_renderer")]
impl From<Transform> for tiny_skia::Transform {
    fn from(transform: Transform) -> Self {
        let output = Self::from_scale(transform.scale.x, transform.scale.y);

        // Get rotation transform
        let a = transform.rotation.cos();
        let b = transform.rotation.sin();
        let output = output.post_concat(tiny_skia::Transform::from_row(a, b, -b, a, 0.0, 0.0));

        output.post_translate(transform.translate.x, transform.translate.y)
    }
}

#[cfg(feature = "tiny_skia_renderer")]
impl From<&Transform> for tiny_skia::Transform {
    fn from(transform: &Transform) -> Self {
        let output = Self::from_scale(transform.scale.x, transform.scale.y);

        // Get rotation transform
        let a = transform.rotation.cos();
        let b = transform.rotation.sin();
        let output = output.post_concat(tiny_skia::Transform::from_row(a, b, -b, a, 0.0, 0.0));

        output.post_translate(transform.translate.x, transform.translate.y)
    }
}
