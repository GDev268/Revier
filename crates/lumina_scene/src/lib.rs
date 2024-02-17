pub mod scene;
pub mod query;

use ash::vk;
use lumina_render::camera::Camera;

const MAX_LIGHTS:i32  = 10;

struct PointLight{
    position:glam::Vec4,
    color:glam::Vec4
}

#[repr(C,align(16))]
pub struct GlobalUBO{
    pub projection:[[f32;4];4],
}

pub struct FrameInfo<'a>{
    pub frame_time:f64,
    pub command_buffer:vk::CommandBuffer,
    pub camera:&'a Camera,
}