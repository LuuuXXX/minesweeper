fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(spawn_layout)
        .add_system(print_moust_button_input_system)
        .run()
    print!("{:?}", WinitSettings::desktop_app());
}
