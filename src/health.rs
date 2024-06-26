use bevy::prelude::*;
use crate::{
    states::GameState,

};

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
        app.add_systems(PostUpdate, (rotate_healthbars, update_health).run_if(in_state(GameState::InGame)));
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
            // println!("Rotation: {}", y);
            let new_translation = rotation.inverse().mul_vec3(Vec3::new(0., 2., 5.));
            
            bar_transform.translation = new_translation;
            bar_transform.rotation = rotation.inverse();
            // bar_transform.rotate_y(y);
        }
    }

fn update_health(
    health_containers: Query<&Parent, With<Healthbar>>,
    mut healthbars: Query<(&Parent, &mut Transform), With<HealthbarVal>>,
    health: Query<&Health>,
) {
    for (parent, mut healthbar) in healthbars.iter_mut() {
        if let Ok(health_container) = health_containers.get(parent.get()) {
            if let Ok(hp) = health.get(health_container.get()) {
                // println!("{} {}", hp.value, hp.max);
                healthbar.scale = Vec3::new(hp.value/hp.max, 1., 1.);
            }
        }
    }
}