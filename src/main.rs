use std::f32::consts::PI;
use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

#[derive(Component)]
struct Shape;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    // standers
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder::new(0.5, 2.)),
            material: debug_material.clone(),
            transform: Transform::from_xyz(
                0.0,
                1.0,
                0.0,
            )
            .with_rotation(Quat::from_rotation_x(0.)),
            ..default()
        },
        Shape,
    ));

    // cabin
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.5, 2., 1.5)),
            material: debug_material.clone(),
            transform: Transform::from_xyz(
                0.0,
                3.0,
                0.0,
            )
            .with_rotation(Quat::from_rotation_x(0.)),
            ..default()
        },
        Shape,
    ));

    // main boom
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(5., 0.75, 0.75)),
            material: debug_material.clone(),
            transform: Transform::from_xyz(
                2.5,
                4.0,
                0.0,
            )
            .with_rotation(Quat::from_rotation_x(0.)),
            ..default()
        },
        Shape,
    ));

    // knuckle boom
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(2.5, 0.75, 0.75)),
            material: debug_material.clone(),
            transform: Transform::from_xyz(
                4.,
                3.0,
                0.0,
            )
            .with_rotation(Quat::from_rotation_z(PI/4.)),
            ..default()
        },
        Shape,
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::SILVER),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_around(
            Vec3::new(0.0, 1.0, 0.0), 
            Quat::from_rotation_y(time.delta_seconds() / 2.)
        )
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}