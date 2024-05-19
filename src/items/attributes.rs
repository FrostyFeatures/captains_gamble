use bevy::prelude::*;

use crate::tooltip::{TooltipComponent, TooltipSection, TooltipSectionIndex};

pub(super) struct AttributePlugin;

pub const POINTY: &str = "Pointy";

impl Plugin for AttributePlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn Attribute, Pointy>();
    }
}

#[bevy_trait_query::queryable]
pub trait Attribute: TooltipComponent {
    fn name(&self) -> &'static str;
    fn _get_tooltip_section(&self) -> TooltipSection {
        let text = format!("{}", self.name());
        TooltipSection {
            text,
            index: TooltipSectionIndex::Footer,
        }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Pointy;

impl Attribute for Pointy {
    fn name(&self) -> &'static str {
        "Pointy"
    }
}

impl TooltipComponent for Pointy {
    fn get_tooltip_section(&self) -> TooltipSection {
        self._get_tooltip_section()
    }
}
