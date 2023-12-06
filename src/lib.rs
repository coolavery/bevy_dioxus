mod implementation;

use self::implementation::{tick_dioxus_ui, VirtualDomUnsafe};
use bevy::{
    app::{App, Plugin, Update},
    ecs::{component::Component, entity::Entity},
};

pub use self::implementation::{use_commands, use_res, use_world};
pub use dioxus_core::{Element, Scope};

pub struct DioxusUiPlugin;

impl Plugin for DioxusUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_dioxus_ui);
    }
}

#[derive(Component)]
pub struct DioxusUiRoot {
    virtual_dom: VirtualDomUnsafe,
    root_entity: Option<Entity>,
}

impl DioxusUiRoot {
    pub fn new(root_component: fn(Scope) -> Element) -> Self {
        Self {
            virtual_dom: VirtualDomUnsafe::new(root_component),
            root_entity: None,
        }
    }
}
