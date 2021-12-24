use cgmath::Vector3;

// #[derive(Copy, Clone, Debug)]
// #[repr(C)]
// pub struct Transform {
//     pub position: Vector3<f32>,
//     pub rotation: Vector3<f32>,
//     pub scale: Vector3<f32>,
// }

// #[repr(C)]
// #[derive(Clone, Debug, Copy)]
// pub struct CTransform {
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
// }

#[derive(Clone, Debug, Copy)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct CTransform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl Into<CTransform> for Transform {
    fn into(self) -> CTransform {
        CTransform {
            position: self.position,
            rotation: self.rotation,
            scale: self.scale,
        }
    }
}

impl Into<Transform> for CTransform {
    fn into(self) -> Transform {
        Transform {
            position: self.position,
            rotation: self.rotation,
            scale: self.scale,
        }
    }
}

impl Into<CTransform> for &Transform {
    fn into(self) -> CTransform {
        CTransform {
            position: self.position,
            rotation: self.rotation,
            scale: self.scale,
        }
    }
}
