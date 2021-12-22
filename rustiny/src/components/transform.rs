use cgmath::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Transform {
    pub position: Vector3<f32>,
}

impl Into<CTransform> for Transform {
    fn into(self) -> CTransform {
        CTransform {
            x: self.position.x,
            y: self.position.y,
            z: self.position.z,
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct CTransform {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
