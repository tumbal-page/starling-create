//! A native mod's world-space text and textured quads, through the normal renderer.
use super::native_block_entity_draw::BlockEntityDraw;
use glam::{DVec3, Mat4, Vec3};
use std::ops::{Deref, DerefMut};

/// One world frame, independently of block entities or source blocks. Its pose
/// starts camera-relative and shares `BlockEntityDraw`'s push/pop, translate,
/// rotate and scale operations, partial tick, game time and packed light.
/// `keep` does not cache a world canvas. API 431.
pub struct NativeWorldDraw {
    pub camera: [f64; 3],
    draw: BlockEntityDraw,
    patches: Vec<serde_json::Value>,
    profiles: Vec<serde_json::Value>,
}
impl Deref for NativeWorldDraw {
    type Target = BlockEntityDraw;
    fn deref(&self) -> &Self::Target {
        &self.draw
    }
}
impl DerefMut for NativeWorldDraw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.draw
    }
}
impl NativeWorldDraw {
    fn new(camera: [f64; 3], partial: f32, time: i64) -> Self {
        Self {
            camera,
            draw: BlockEntityDraw::posed(
                partial,
                time,
                240 | 240 << 8,
                [0; 3],
                Mat4::IDENTITY,
                (Vec3::Y, Vec3::Y),
                Vec::new(),
            ),
            patches: Vec::new(),
            profiles: Vec::new(),
        }
    }
    /// Translate to a world position, subtracting the camera in f64 first.
    /// Returns false without changing the pose for a non-finite offset.
    pub fn translate_world(&mut self, position: [f64; 3]) -> bool {
        let offset = (DVec3::from_array(position) - DVec3::from_array(self.camera)).as_vec3();
        if !offset.is_finite() {
            return false;
        }
        self.draw.translate(offset.to_array());
        true
    }
}
