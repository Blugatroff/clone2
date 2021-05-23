use cgmath::{InnerSpace, Matrix3, Rad, Vector3};

pub fn rotation_matrix_from_direction(d: Vector3<f32>) -> Matrix3<f32> {
    let d = d.normalize();
    Matrix3::from_angle_y(Rad(-d.z.atan2(d.x)))
        * Matrix3::from_angle_z(Rad(d.y.atan2((d.x * d.x + d.z * d.z).sqrt())))
}
pub fn mix(start: Vector3<f32>, end: Vector3<f32>, part: f32) -> Vector3<f32> {
    start + (end - start) * part
}
