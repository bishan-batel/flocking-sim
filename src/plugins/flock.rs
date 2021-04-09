use crate::utils::*;
use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub enum FlockSystems {
    Update,
    Movement,
}

pub struct Flock {
    pub max_speed: f32,
    pub max_acc: f32,
    pub safe_distance: f32,
    pub flock_radius: f32,
    pub alignment_strength: f32,
    pub cohesion_strength: f32,
    pub seperation_strength: f32,
}

impl Default for Flock {
    fn default() -> Self {
        Flock {
            max_speed: 200.0,
            max_acc: 100.0,
            safe_distance: 30.0,
            flock_radius: 300.0,
            alignment_strength: 3.0,
            cohesion_strength: 2.0,
            seperation_strength: 8.0,
        }
    }
}

impl Flock {
    pub fn movement(time: Res<Time>, mut query: Query<(&mut GlobalTransform, &Velocity)>) {
        let delta = time.delta().as_secs_f32();

        for (mut transform, velocity) in query.iter_mut() {
            // Updates velocity
            transform.translation += velocity.0 * delta;

            // Calculates heading for rotating the sprite
            let mut heading = 0.0;
            if velocity.0.length_squared() != 0.0 {
                let normal_velocity = velocity.0.normalize();
                if normal_velocity.y < 0.0 {
                    heading = -normal_velocity.x.acos();
                } else {
                    heading = normal_velocity.x.acos();
                }

                if heading.is_nan() || heading.is_infinite() {
                    heading = 0.0;
                }
            }

            transform.rotation = Quat::from_rotation_z(heading);
        }
    }

    pub fn update(
        win: Res<Windows>,
        time: Res<Time>,
        mut q_flock: Query<(&Flock, &mut Children)>,
        mut q_boids: Query<(&Boid, &mut GlobalTransform, &mut Velocity)>,
    ) {
        let delta = time.delta().as_secs_f32();

        let uwin = win.get_primary().unwrap();
        let hwin_size = Vec3::new(uwin.width() / 2f32, uwin.height() / 2f32, 0.1);

        // All flocks
        for (flock, children) in q_flock.iter_mut() {
            let mut boids = Vec::new();
            let mut avg_pos = Vec3::ZERO;
            let mut avg_vel = Vec3::ZERO;

            // All children
            for child in children.iter() {
                let (_, mut transform, velocity) =
                    q_boids.get_mut(*child).expect("Boid outside of block");
                // Bounds each child
                transform.translation = boundsv(transform.translation, -hwin_size, hwin_size);

                // Counts averags
                avg_pos += transform.translation;

                avg_vel += velocity.0;

                boids.push((child.id(), transform.translation));
            }

            // Asserts no divide by 0 error
            let boid_count = boids.len();
            if boid_count != 0 {
                avg_vel /= boid_count as f32;
                avg_pos /= boid_count as f32;
            }

            for child in children.iter() {
                let (_, transform, mut velocity) =
                    q_boids.get_mut(*child).expect("Boid outside of block");

                // Calc flocking roces
                let alignment = Self::calculate_alignment(flock.max_speed, avg_vel);
                let cohesion =
                    Self::calculate_cohesion(transform.translation, avg_pos, flock.flock_radius);
                let seperation =
                    Self::calculate_seperation(child.id(), &flock, transform.translation, &boids);

                // Apply acceleration
                let mut acc = flock.max_speed * (alignment + cohesion + seperation);
                if acc.length_squared() > flock.max_acc * flock.max_acc {
                    acc = flock.max_acc * acc.normalize();
                }
                velocity.0 += acc * delta;
                // if velocity.0.length_squared() > flock.max_speed * flock.max_speed {
                velocity.0 = velocity.0.normalize() * flock.max_speed;
                // }
            }
        }
    }

    #[inline]
    fn calculate_seperation(id: u32, flock: &Flock, pos: Vec3, boids: &[(u32, Vec3)]) -> Vec3 {
        let mut seperation = Vec3::ZERO;

        for (oid, opos) in boids.into_iter() {
            if *oid != id {
                let diff = pos - *opos;
                let dist2 = diff.length_squared();

                if dist2 < flock.safe_distance * flock.safe_distance {
                    seperation += diff.normalize() * (flock.safe_distance - dist2.sqrt())
                        / flock.safe_distance;
                }
            }
        }
        if seperation.length_squared() > 1.0 {
            seperation = seperation.normalize();
        }
        seperation
    }

    #[inline]
    fn calculate_cohesion(pos: Vec3, avg_pos: Vec3, flock_radius: f32) -> Vec3 {
        let mut cohesion = avg_pos - pos;

        if cohesion.length_squared() < flock_radius * flock_radius {
            cohesion /= flock_radius;
        } else {
            cohesion = cohesion.normalize();
        }

        cohesion
    }

    #[inline]
    fn calculate_alignment(max_speed: f32, avg_vel: Vec3) -> Vec3 {
        let mut alignment = avg_vel / max_speed;

        if alignment.length_squared() > 1.0 {
            alignment = alignment.normalize();
        }
        alignment
    }
}

pub struct Boid;

pub struct Velocity(pub Vec3);
