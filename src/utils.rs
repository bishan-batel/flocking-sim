use bevy::math::Vec3;

pub fn boundsv(v: Vec3, min: Vec3, max: Vec3) -> Vec3 {
    Vec3::new(
        boundsf(v.x, min.x, max.x),
        boundsf(v.y, min.y, max.y),
        boundsf(v.z, min.z, max.z),
    )
}
pub fn boundsf(n: f32, min: f32, max: f32) -> f32 {
    if n >= max {
        return min;
    } else if n <= min {
        return max;
    }
    n
}
