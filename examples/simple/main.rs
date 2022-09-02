use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_double_res::prelude::*;

#[derive(Clone)]
struct MyColors(Color, Color, Color);

#[derive(Component)]
struct FirstMarker;
#[derive(Component)]
struct SecondMarker;
#[derive(Component)]
struct ThirdMarker;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::splat(60.)).into()).into(),
            material: colors.add(Color::RED.into()),
            transform: Transform::from_xyz(-100., 0., 0.),
            ..default()
        })
        .insert(FirstMarker);
    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::splat(60.)).into()).into(),
            material: colors.add(Color::BLUE.into()),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        })
        .insert(SecondMarker);
    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::splat(60.)).into()).into(),
            material: colors.add(Color::GREEN.into()),
            transform: Transform::from_xyz(0., 100., 0.),
            ..default()
        })
        .insert(ThirdMarker);

    let my_colors = MyColors(Color::RED, Color::BLUE, Color::GREEN).into_double_buf();

    commands.insert_resource(my_colors);
}

fn circular_dependent_system(mut colors: DoubleResMut<MyColors>) {
    colors.apply(|current, next| {
        next.0 = current.1;
        next.1 = current.2;
        next.2 = current.0;
    });
    colors.swap();
}

fn display_system(
    first: Query<&Handle<ColorMaterial>, With<FirstMarker>>,
    second: Query<&Handle<ColorMaterial>, With<SecondMarker>>,
    third: Query<&Handle<ColorMaterial>, With<ThirdMarker>>,
    mut assets: ResMut<Assets<ColorMaterial>>,
    colors: DoubleRes<MyColors>,
) {
    assets
        .get_mut(first.single())
        .map(|x| x.color = colors.current().0);
    assets
        .get_mut(second.single())
        .map(|x| x.color = colors.current().1);
    assets
        .get_mut(third.single())
        .map(|x| x.color = colors.current().2);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(1.0))
                .with_system(circular_dependent_system)
                .before(display_system),
        )
        .add_system(display_system)
        .run();
}
