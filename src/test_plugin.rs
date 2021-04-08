use bevy::prelude::*;

pub struct TestPlugin;
impl Plugin for TestPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet.system());
    }
}

struct Person;
struct Name(String);

struct GreetTimer(Timer);

fn greet(query: Query<&Name, With<Person>>, time: Res<Time>, mut timer: ResMut<GreetTimer>) {
    if !timer.0.tick(time.delta()).finished() {
        ()
    }

    for name in query.iter() {
        println!("rofl {}", name.0);
    }
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Elaina Proctor".to_string()));

    commands
        .spawn()
        .insert(Person)
        .insert(Name("Rofl bobn".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Elaina Rofl".to_string()));
}
