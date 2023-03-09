use egui_macroquad::egui::{
    self, show_tooltip_at_pointer, Context, Id, Response, TextureId, Ui, Vec2, Widget,
};

use bevy_ecs::prelude::*;
use pixels_canvas::prelude::Element;

use super::{State, ToolType};

use crate::{file_dialog::FileDialog, panel, tool_button};

struct ToolButton {
    selected: bool,
    icon: TextureId,
    size: Vec2,
}

#[allow(unused_mut, unused_variables)]
pub fn draw(state: ResMut<State>, mut file_dialog: NonSendMut<FileDialog>) {
    panel!(state, |ctx: &Context,
                   ui: &mut Ui,
                   state: &mut ResMut<State>| {
        #[cfg(target_os = "macos")]
        file_dialog.as_mut().update(&ctx);

        ui.add_space(20.0);
        ui.color_edit_button_rgb(&mut state.color);

        tool_button!(
            ctx,
            ui,
            state,
            ToolType::Mover,
            state.menu_state.move_icon,
            {
                state.selected_tool = ToolType::Mover;
            }
        );

        tool_button!(
            ctx,
            ui,
            state,
            ToolType::Brush,
            state.menu_state.brush_icon,
            {
                state.selected_tool = ToolType::Brush;
            }
        );

        tool_button!(
            ctx,
            ui,
            state,
            ToolType::Picker,
            state.menu_state.picker_icon,
            {
                state.selected_tool = ToolType::Picker;
            }
        );

        tool_button!(
            ctx,
            ui,
            state,
            ToolType::Placer,
            state.menu_state.image_icon,
            {
                state.selected_tool = ToolType::Placer;
                state.image = file_dialog.show().map(Element::new);
                
            }
        );
    });
}

impl ToolButton {
    fn new(selected: bool, icon: TextureId, size: Vec2) -> Self {
        Self {
            selected,
            icon,
            size,
        }
    }
}

impl Widget for ToolButton {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add_space(5.0);
        let button = ui.add(egui::ImageButton::new(self.icon, self.size / 5.0));

        if self.selected {
            button.clone().highlight();
        }

        button
    }
}

#[macro_export]
macro_rules! panel {
    ($state:expr, $body:expr $(,)?) => {
        let mut state = $state;
        egui_macroquad::ui(|ctx| {
            if state.cooldown != 0.0 {
                show_tooltip_at_pointer(ctx, Id::new("cooldown"), |ui| {
                    ui.label(format!("please wait {} secs", state.cooldown.round()));
                });
            }

            let panel = egui::SidePanel::left("settings").show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.set_width(0.0);
                    $body(ctx, ui, &mut state)
                });
            });

            state.focus = ctx.is_pointer_over_area();
            state.menu_state.area = panel.response.rect;
        });

        egui_macroquad::draw();
    };
}

#[macro_export]
macro_rules! tool_button {
    ($ctx:expr, $ui:expr, $state:expr, $tool:expr, $icon:expr, $body:block) => {{
        let button = $ui.add(ToolButton::new(
            $state.selected_tool == $tool,
            $icon.texture_id($ctx),
            $icon.size_vec2(),
        ));

        if button.clicked() {
            $body
        }
    }};
}
