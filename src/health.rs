use bevy::{prelude::*, render::primitives::Aabb};
use bevy::transform::TransformSystem;
use bevy_xpbd_3d::prelude::*;
use crate::{
    states::GameState,

};

pub const HEALTHBAR_TRANSLATION: Vec3 = Vec3::new(0., 5., 5.);
pub const HEALTHBAR_SIZE: Vec3 = Vec3::new(10., 0., 1.);
pub const ASTEROID_HEALTHBAR_TRANSLATION: Vec3 = Vec3::new(0., 5., 3.);
pub const ASTEROID_HEALTHBAR_SIZE: Vec3 = Vec3::new(6., 0., 1.);

#[derive(Component, Debug)]
pub struct Health{
    pub value: f32,
    pub max: f32,
}

#[derive(Component, Debug)]
pub struct Healthbar;

#[derive(Component, Debug)]
pub struct HealthbarVal;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (rotate_healthbars, update_health).run_if(in_state(GameState::InGame)).after(PhysicsSet::Sync).before(TransformSystem::TransformPropagate));
    }
}

fn rotate_healthbars(
    mut healthbars: Query<(&Parent, &mut Transform), With<Healthbar>>,
    parent_transform: Query<&GlobalTransform, With<Health>>,
) {
    for (parent, mut bar_transform) in healthbars.iter_mut() {
        if let Ok(p) = parent_transform.get(parent.get()) {
            let (scale, rotation, translation) = p.to_scale_rotation_translation();
            // let y = rotation.to_euler(EulerRot::YXZ).0;
            let bar_rotation = bar_transform.rotation;
            // println!("x: {}, y: {}, z: {}", bar_transform.translation.x, bar_transform.translation.y, bar_transform.translation.z);
            let new_translation = rotation.inverse().mul_vec3(bar_rotation.inverse().mul_vec3(bar_transform.translation));
            
            bar_transform.translation = new_translation;
            bar_transform.rotation = rotation.inverse();
            // bar_transform.rotate_y(y);
        }
    }
}

fn update_health(
    health_containers: Query<(&Parent, &Aabb), With<Healthbar>>,
    mut healthbars: Query<(&Parent, &mut Transform), With<HealthbarVal>>,
    health: Query<&Health>,
) {
    // println!("{} {}\n", health_containers.iter().len(), healthbars.iter().len());
    for (parent, mut healthbar) in healthbars.iter_mut() {
        if let Ok((health_container, health_bar_shape)) = health_containers.get(parent.get()) {
            if let Ok(hp) = health.get(health_container.get()) {
                // if hp.value <= 0. {
                //     println!("{} {}", hp.value, hp.max);
                // }
                let half_width = health_bar_shape.half_extents.x;
                // println!("{}\n", half_width);
                healthbar.scale = Vec3::new(hp.value/hp.max, 1., 1.);
                healthbar.translation = Vec3::new((1.-hp.value/hp.max)*half_width, 0.1, 0.); //0.1 for y so the healthbar can be seen above the background
            }
        }
    }
}