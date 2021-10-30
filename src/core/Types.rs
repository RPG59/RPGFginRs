use cgmath::{Matrix3, Matrix4, Quaternion, Vector3, Vector4};

pub type float4x4 = Matrix4<f32>;
pub type float3x3 = Matrix3<f32>;
pub type float4 = Vector4<f32>;
pub type float3 = Vector3<f32>;
pub type qut = Quaternion<f32>;

pub fn to_slice(mat: &float4x4) -> [f32; 16] {
    [
        mat.x.x, mat.x.y, mat.x.z, mat.x.w, mat.y.x, mat.y.y, mat.y.z, mat.y.w, mat.z.x, mat.z.y,
        mat.z.z, mat.z.w, mat.w.x, mat.w.y, mat.w.z, mat.w.w,
    ]
}
