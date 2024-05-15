use bevy::prelude::*;

use crate::tooltip::{TooltipComponent, TooltipSection};

pub(super) struct AttributePlugin;

impl Plugin for AttributePlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn TooltipComponent, Pointy>();
    }
}

trait Attribute: TooltipComponent {
    fn name() -> &'static str;
    fn _get_tooltip_section(&self) -> TooltipSection {
        let text = format!("{}", Self::name());
        TooltipSection { text, index: 2 }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Pointy;

impl Attribute for Pointy {
    fn name() -> &'static str {
        "Pointy"
    }
}

impl TooltipComponent for Pointy {
    fn get_tooltip_section(&self) -> TooltipSection {
        self._get_tooltip_section()
    }
}
