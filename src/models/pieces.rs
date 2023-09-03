use bevy::prelude::*;
use itertools;

fn spawn_king(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    meshes: (Handle<Mesh>, Handle<Mesh>),
    position: Vec3,
) {
    let (mesh, mesh_cross) = meshes;
    commands
        // Spawn parent entity
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material: material.clone(),
                transform: Transform::from_translation(Vec3::new(-0.2, 0., -1.9))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                ..Default::default()
            });
            parent.spawn(PbrBundle {
                mesh: mesh_cross,
                material,
                transform: Transform::from_translation(Vec3::new(-0.2, 0., -1.9))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                ..Default::default()
            });
        });
}

fn spawn_knight(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    meshes: (Handle<Mesh>, Handle<Mesh>),
    position: Vec3,
) {
    let (mesh_1, mesh_2) = meshes;
    // oddity of knight mesh from GLB is that there are two to complete it.
    commands
        // Spawn parent entity
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: mesh_1,
                material: material.clone(),
                transform: Transform::from_translation(Vec3::new(-0.2, 0., 0.9))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                ..Default::default()
            });
            parent.spawn(PbrBundle {
                mesh: mesh_2,
                material,
                transform: Transform::from_translation(Vec3::new(-0.2, 0., 0.9))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                ..Default::default()
            });
        });
}

fn spawn_queen(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(Vec3::new(-0.2, 0., -0.95))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                ..Default::default()
            });
        });
}

fn spawn_bishop(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(Vec3::new(-0.1, 0., 0.))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                ..Default::default()
            });
        });
}

fn spawn_rook(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(Vec3::new(-0.1, 0., 1.8))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                ..Default::default()
            });
        });
}

fn spawn_pawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(Vec3::new(-0.2, 0., 2.6))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                ..Default::default()
            });
        });
}

#[derive(Clone, Debug)]
struct PieceHandles {
    rook: Handle<Mesh>,
    knight: (Handle<Mesh>, Handle<Mesh>),
    bishop: Handle<Mesh>,
    queen: Handle<Mesh>,
    king: (Handle<Mesh>, Handle<Mesh>),
    pawn: Handle<Mesh>,
}

fn spawn_side(
    commands: &mut Commands,
    pieces: PieceHandles,
    material: &Handle<StandardMaterial>,
    back_row: f32,
) {
    let (pawn_row, first_corner, delta_z): (f32, Vec3, f32) = if back_row == 0. {
        (1., Vec3::new(back_row, 0., 0.), 1.)
    } else {
        (6., Vec3::new(back_row, 0., 7.), -1.)
    };
    // black is rows 6, 7 with pawns on 6.
    let mut pos_iter =
        itertools::iterate(first_corner, |state| *state + Vec3::new(0., 0., delta_z));
    spawn_rook(
        commands,
        material.clone(),
        pieces.rook.clone(),
        pos_iter.next().unwrap(),
    );
    spawn_knight(
        commands,
        material.clone(),
        pieces.knight.clone(),
        pos_iter.next().unwrap(),
    );
    spawn_bishop(
        commands,
        material.clone(),
        pieces.bishop.clone(),
        pos_iter.next().unwrap(),
    );
    if back_row == 0. {
        spawn_queen(
            commands,
            material.clone(),
            pieces.queen,
            pos_iter.next().unwrap(),
        );
        spawn_king(
            commands,
            material.clone(),
            pieces.king,
            pos_iter.next().unwrap(),
        );
    } else {
        spawn_king(
            commands,
            material.clone(),
            pieces.king,
            pos_iter.next().unwrap(),
        );
        spawn_queen(
            commands,
            material.clone(),
            pieces.queen,
            pos_iter.next().unwrap(),
        );
    }
    spawn_bishop(
        commands,
        material.clone(),
        pieces.bishop,
        pos_iter.next().unwrap(),
    );
    spawn_knight(
        commands,
        material.clone(),
        pieces.knight,
        pos_iter.next().unwrap(),
    );
    spawn_rook(
        commands,
        material.clone(),
        pieces.rook,
        pos_iter.next().unwrap(),
    );
    for i in 0i8..8 {
        let i: f32 = f32::from(i);
        spawn_pawn(
            commands,
            material.clone(),
            pieces.pawn.clone(),
            Vec3::new(pawn_row, 0., i),
        );
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn spawn_pieces_on_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load all the meshes
    let king_handle: Handle<Mesh> = asset_server.load("chessKitExport.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> = asset_server.load("chessKitExport.glb#Mesh1/Primitive0");
    let pawn_handle: Handle<Mesh> = asset_server.load("chessKitExport.glb#Mesh2/Primitive0");
    let knight_1_handle: Handle<Mesh> = asset_server.load("chessKitExport.glb#Mesh3/Primitive0");
    let knight_2_handle: Handle<Mesh> = asset_server.load("chessKitExport.glb#Mesh4/Primitive0");
    let rook_handle: Handle<Mesh> = asset_server.load("chessKitExport.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> = asset_server.load("chessKitExport.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> = asset_server.load("chessKitExport.glb#Mesh7/Primitive0");

    // Add some materials
    let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0., 0.2, 0.2).into());
    spawn_side(
        &mut commands,
        PieceHandles {
            rook: rook_handle.clone(),
            knight: (knight_1_handle.clone(), knight_2_handle.clone()),
            bishop: bishop_handle.clone(),
            queen: queen_handle.clone(),
            king: (king_handle.clone(), king_cross_handle.clone()),
            pawn: pawn_handle.clone(),
        },
        &white_material,
        0.,
    );
    spawn_side(
        &mut commands,
        PieceHandles {
            rook: rook_handle,
            knight: (knight_1_handle, knight_2_handle),
            bishop: bishop_handle,
            queen: queen_handle,
            king: (king_handle, king_cross_handle),
            pawn: pawn_handle,
        },
        &black_material,
        7.,
    );
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_vec3_addition() {
        assert_eq!(
            Vec3::new(0., 0., 0.) + Vec3::new(1., 0., 0.),
            Vec3::new(1., 0., 0.)
        );
        assert_eq!(
            Vec3::new(0., 1., 0.) + Vec3::new(1., 1., 0.),
            Vec3::new(1., 2., 0.)
        );
    }

    #[test]
    fn test_vec3_subtraction() {
        assert_eq!(
            Vec3::new(0., 0., 0.) - Vec3::new(1., 0., 0.),
            Vec3::new(-1., 0., 0.)
        );
        assert_eq!(
            Vec3::new(0., 1., 0.) - Vec3::new(1., 1., 0.),
            Vec3::new(-1., 0., 0.)
        );
    }
}
