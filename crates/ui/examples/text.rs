use ambient_app::AppBuilder;
use ambient_cameras::UICamera;
use ambient_core::camera::active_camera;
use ambient_ecs::World;
use ambient_element::{ElementComponentExt, Group};
use ambient_renderer::color;
use ambient_ui::{font_size, padding, space_between_items, Borders, FlowColumn, Separator, StylesExt, Text};
use glam::vec4;

fn init(world: &mut World) {
    Group(vec![
        UICamera.el().set(active_camera(), 0.),
        FlowColumn(vec![
            Text::el("Header").header_style(),
            Text::el("Section").section_style(),
            Text::el("Default text \u{f1e2} \u{fb8f}"),
            Text::el("Small").small_style(),
            Separator { vertical: false }.el(),
            Text::el("Custom size").set(font_size(), 40.),
            Text::el("Custom color").set(color(), vec4(1., 0., 0., 1.)),
            Text::el("Multi\n\nLine"),
        ])
        .el()
        .set(padding(), Borders::even(10.))
        .set(space_between_items(), 10.),
    ])
    .el()
    .spawn_interactive(world);
}

fn main() {
    env_logger::init();
    AppBuilder::simple_ui().run_world(init);
}
