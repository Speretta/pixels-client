

use egui_macroquad::egui::Pos2;

use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use pixels_canvas::prelude::*;

use crate::state::ImageWorker;

use super::{CanvasContainer, State, ToolType};
use pixels_util::color::Color;

pub fn register_systems(update_schedule: &mut Schedule) {
    update_schedule.add_systems((
        update_zoom.run_if(not(is_panel_focused)),
        update_mouse.run_if(not(is_panel_focused)),
        update_tool_move.run_if(not(is_panel_focused)),
        update_tool_draw.run_if(not(is_panel_focused)),
        update_tool_pick.run_if(not(is_panel_focused)),
        update_tool_place.run_if(not(is_panel_focused)),
    ));
}

fn is_panel_focused(state: Res<State>) -> bool {
    let pos = Pos2::new(mouse_position().0, mouse_position().1);
    state.menu_state.area.contains(pos) || state.focus
}

pub fn update_zoom(mut state: ResMut<State>) {
    state.camera_state.zoom = (state.camera_state.zoom + mouse_wheel().1 / 120.0).clamp(1.0, 10.0);
}

pub fn update_mouse(mut state: ResMut<State>) {
    let pos = super::mouse_world_pos(state.camera_state.instance);

    if is_mouse_button_pressed(MouseButton::Left) {
        state.camera_state.move_origin = pos;
    } else if is_mouse_button_down(MouseButton::Left) && state.selected_tool == ToolType::Mover {
        let origin = state.camera_state.move_origin;
        state.camera_state.position += origin - pos;
    }
}

pub fn update_tool_move(mut state: ResMut<State>) {
    if is_key_down(KeyCode::M) {
        state.selected_tool = ToolType::Mover;
    }
}

pub fn update_tool_draw(mut state: ResMut<State>, mut container: ResMut<CanvasContainer>) {
    if is_key_down(KeyCode::B) {
        state.selected_tool = ToolType::Brush;
    }

    if !is_mouse_button_pressed(MouseButton::Left) {
        return;
    }

    if let ToolType::Brush = state.selected_tool {
        let pos = super::mouse_world_pos(state.camera_state.instance);

        if let Err(e) =
            container
                .canvas
                .set_pixel(pos.x as u32, pos.y as u32, Color::from(state.color))
        {
            match e {
                CanvasError::ClientError => {
                    panic!("couldn't set pixel");
                }
                CanvasError::Cooldown(cooldown) => {
                    println!("please wait cooldown to end: {cooldown}");
                }
            }
        }
    }
}

pub fn update_tool_pick(mut state: ResMut<State>, container: ResMut<CanvasContainer>) {
    if is_key_down(KeyCode::I) {
        state.selected_tool = ToolType::Picker;
    }

    if !is_mouse_button_pressed(MouseButton::Left) {
        return;
    }

    if let ToolType::Picker = state.selected_tool {
        let pos = super::mouse_world_pos(state.camera_state.instance);

        state.color = (container
            .canvas
            .get_pixel(pos.x as u32, pos.y as u32)
            .unwrap_or(Color::default()))
        .try_into()
        .expect("Expected RGB found RGBA")
    }
}

pub fn update_tool_place(mut state: ResMut<State>, mut container: ResMut<CanvasContainer>) {
    if !container.canvas.get_cooldown_object().is_ended(){
        return;
    }
    if let ImageWorker::Image(Some(image)) = &state.image_worker{
        if is_mouse_button_pressed(MouseButton::Left){
            let mouse_pos = super::mouse_world_pos(state.camera_state.instance);
            state.image_worker = ImageWorker::Working(image.get_pixels().into_iter(), (mouse_pos.x as u32, mouse_pos.y as u32));
        }
    }else if let ImageWorker::Working(iterator, (mouse_x, mouse_y)) = &mut state.image_worker{
        if let Some(((pos_x, pos_y), color)) = iterator.next(){
            container.canvas.set_pixel(*mouse_x + pos_x, *mouse_y + pos_y, color).expect("Bir sorun oluştu!");
        }
    }
}
