use std::collections::BTreeSet;

use bevy_egui::egui;
use stream_town_domain::{ContentCatalog, StableId, TechnologyGraphLayout};

const NODE_SIZE: egui::Vec2 = egui::vec2(220.0, 76.0);
const GROUP_HEADER_HEIGHT: f32 = 42.0;
const MIN_GROUP_SIZE: egui::Vec2 = egui::vec2(280.0, 180.0);
const MIN_ZOOM: f32 = 0.01;
const MAX_ZOOM: f32 = 2.5;

#[derive(Clone, Debug)]
pub(crate) struct TechnologyGraphViewState {
    pan: egui::Vec2,
    zoom: f32,
    fit_requested: bool,
    focus_requested: Option<StableId>,
    pub show_minimap: bool,
}

impl Default for TechnologyGraphViewState {
    fn default() -> Self {
        Self {
            pan: egui::Vec2::ZERO,
            zoom: 1.0,
            fit_requested: true,
            focus_requested: None,
            show_minimap: true,
        }
    }
}

impl TechnologyGraphViewState {
    pub fn request_fit(&mut self) {
        self.fit_requested = true;
    }

    pub fn request_focus(&mut self, id: StableId) {
        self.focus_requested = Some(id);
    }

    #[cfg(test)]
    pub fn zoom(&self) -> f32 {
        self.zoom
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct TechnologyGraphCanvasOutput {
    pub selected_node: Option<StableId>,
    pub selected_group: Option<StableId>,
    pub layout_edit_started: bool,
}

pub(crate) fn show(
    ui: &mut egui::Ui,
    catalog: &ContentCatalog,
    layout: &mut TechnologyGraphLayout,
    view: &mut TechnologyGraphViewState,
    selected_node: Option<&StableId>,
    selected_group: Option<&StableId>,
    search: &str,
) -> TechnologyGraphCanvasOutput {
    let desired = egui::vec2(ui.available_width().max(320.0), 510.0);
    let (rect, background_response) =
        ui.allocate_exact_size(desired, egui::Sense::click_and_drag());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 6.0, egui::Color32::from_rgb(15, 20, 26));

    let bounds = content_bounds(layout);
    if view.fit_requested {
        fit_bounds(view, bounds, rect);
        view.fit_requested = false;
    }
    if let Some(id) = view.focus_requested.take()
        && let Some(node) = layout.nodes.get(&id)
    {
        let center = egui::pos2(
            node.position.x + NODE_SIZE.x * 0.5,
            node.position.y + NODE_SIZE.y * 0.5,
        );
        center_world(view, center);
        view.zoom = view.zoom.max(0.65);
        center_world(view, center);
    }

    handle_navigation(ui, rect, &background_response, view);
    draw_grid(&painter, rect, view);

    let mut output = TechnologyGraphCanvasOutput::default();
    let pointer_delta = ui.input(|input| input.pointer.delta()) / view.zoom;

    // Group bodies sit behind every connection and node.
    for (id, group) in &catalog.technology.groups {
        let Some(group_layout) = layout.groups.get(id).copied() else {
            continue;
        };
        let group_rect = world_rect_to_screen(
            rect,
            view,
            egui::Rect::from_min_size(
                egui::pos2(group_layout.position.x, group_layout.position.y),
                egui::vec2(group_layout.size.width, group_layout.size.height),
            ),
        );
        let selected = selected_group == Some(id);
        painter.rect_filled(
            group_rect,
            9.0,
            if selected {
                egui::Color32::from_rgba_unmultiplied(44, 75, 92, 105)
            } else {
                egui::Color32::from_rgba_unmultiplied(34, 45, 55, 86)
            },
        );
        painter.rect_stroke(
            group_rect,
            9.0,
            egui::Stroke::new(1.5, egui::Color32::from_rgb(72, 97, 113)),
            egui::StrokeKind::Inside,
        );
        if view.zoom >= 0.08 {
            painter.text(
                group_rect.left_top() + egui::vec2(12.0, 8.0),
                egui::Align2::LEFT_TOP,
                format!("{}  ·  {} nodes", group.display_name, group.nodes.len()),
                egui::FontId::proportional((15.0 * view.zoom).clamp(10.0, 16.0)),
                egui::Color32::from_rgb(171, 195, 207),
            );
        }
    }

    // Connections are rendered before nodes so node cards mask their ends.
    for (id, node) in &catalog.technology.nodes {
        let Some(target_layout) = layout.nodes.get(id) else {
            continue;
        };
        let target = world_to_screen(
            rect,
            view,
            egui::pos2(
                target_layout.position.x,
                target_layout.position.y + NODE_SIZE.y * 0.5,
            ),
        );
        for prerequisite in &node.prerequisites {
            let Some(source_layout) = layout.nodes.get(prerequisite) else {
                continue;
            };
            let source = world_to_screen(
                rect,
                view,
                egui::pos2(
                    source_layout.position.x + NODE_SIZE.x,
                    source_layout.position.y + NODE_SIZE.y * 0.5,
                ),
            );
            draw_connection(&painter, source, target, view.zoom);
        }
    }

    // Group headers move the group and all of its members. The lower-right
    // handle persists explicit group sizing.
    for (id, group) in &catalog.technology.groups {
        let Some(group_layout) = layout.groups.get(id).copied() else {
            continue;
        };
        let world_rect = egui::Rect::from_min_size(
            egui::pos2(group_layout.position.x, group_layout.position.y),
            egui::vec2(group_layout.size.width, group_layout.size.height),
        );
        let group_rect = world_rect_to_screen(rect, view, world_rect);
        let header_height = (GROUP_HEADER_HEIGHT * view.zoom).max(8.0);
        let header_rect = egui::Rect::from_min_max(
            group_rect.min,
            egui::pos2(
                group_rect.right(),
                (group_rect.top() + header_height).min(group_rect.bottom()),
            ),
        );
        let header_response = ui.interact(
            header_rect,
            ui.id().with(("technology_group_header", id.as_str())),
            egui::Sense::click_and_drag(),
        );
        if header_response.clicked() {
            output.selected_group = Some(id.clone());
        }
        if header_response.drag_started() {
            output.layout_edit_started = true;
        }
        if header_response.dragged() && pointer_delta != egui::Vec2::ZERO {
            if let Some(value) = layout.groups.get_mut(id) {
                value.position.x += pointer_delta.x;
                value.position.y += pointer_delta.y;
            }
            for node_id in &group.nodes {
                if let Some(value) = layout.nodes.get_mut(node_id) {
                    value.position.x += pointer_delta.x;
                    value.position.y += pointer_delta.y;
                }
            }
        }

        let handle_size = 14.0;
        let handle_rect = egui::Rect::from_min_max(
            group_rect.max - egui::vec2(handle_size, handle_size),
            group_rect.max,
        );
        let resize_response = ui.interact(
            handle_rect,
            ui.id().with(("technology_group_resize", id.as_str())),
            egui::Sense::drag(),
        );
        painter.rect_filled(
            handle_rect.shrink(3.0),
            2.0,
            egui::Color32::from_rgb(91, 133, 151),
        );
        if resize_response.drag_started() {
            output.layout_edit_started = true;
        }
        if resize_response.dragged()
            && pointer_delta != egui::Vec2::ZERO
            && let Some(value) = layout.groups.get_mut(id)
        {
            value.size.width = (value.size.width + pointer_delta.x).max(MIN_GROUP_SIZE.x);
            value.size.height = (value.size.height + pointer_delta.y).max(MIN_GROUP_SIZE.y);
        }
    }

    let search = search.trim().to_ascii_lowercase();
    let search_matches: BTreeSet<_> = catalog
        .technology
        .nodes
        .iter()
        .filter(|(id, node)| {
            !search.is_empty()
                && (id.as_str().to_ascii_lowercase().contains(&search)
                    || node.display_name.to_ascii_lowercase().contains(&search))
        })
        .map(|(id, _)| id.clone())
        .collect();

    for (id, node) in &catalog.technology.nodes {
        let Some(node_layout) = layout.nodes.get(id).copied() else {
            continue;
        };
        let node_rect = world_rect_to_screen(
            rect,
            view,
            egui::Rect::from_min_size(
                egui::pos2(node_layout.position.x, node_layout.position.y),
                NODE_SIZE,
            ),
        );
        if !rect.intersects(node_rect) {
            continue;
        }
        let response = ui.interact(
            node_rect,
            ui.id().with(("technology_node", id.as_str())),
            egui::Sense::click_and_drag(),
        );
        if response.clicked() {
            output.selected_node = Some(id.clone());
            output.selected_group.clone_from(&node.group);
        }
        if response.drag_started() {
            output.layout_edit_started = true;
        }
        if response.dragged()
            && pointer_delta != egui::Vec2::ZERO
            && let Some(value) = layout.nodes.get_mut(id)
        {
            value.position.x += pointer_delta.x;
            value.position.y += pointer_delta.y;
        }

        let selected = selected_node == Some(id);
        let search_match = search_matches.contains(id);
        let fill = if node.unavailable {
            egui::Color32::from_rgb(75, 53, 58)
        } else if node.initially_unlocked {
            egui::Color32::from_rgb(43, 92, 68)
        } else {
            egui::Color32::from_rgb(42, 64, 77)
        };
        painter.rect_filled(node_rect, (8.0 * view.zoom).clamp(2.0, 8.0), fill);
        let stroke = if selected {
            egui::Stroke::new(3.0, egui::Color32::YELLOW)
        } else if search_match {
            egui::Stroke::new(2.5, egui::Color32::from_rgb(83, 210, 255))
        } else {
            egui::Stroke::new(1.2, egui::Color32::from_rgb(101, 139, 158))
        };
        painter.rect_stroke(
            node_rect,
            (8.0 * view.zoom).clamp(2.0, 8.0),
            stroke,
            egui::StrokeKind::Inside,
        );
        if view.zoom >= 0.09 {
            painter.text(
                node_rect.left_top() + egui::vec2(9.0, 8.0),
                egui::Align2::LEFT_TOP,
                &node.display_name,
                egui::FontId::proportional((14.0 * view.zoom).clamp(9.0, 15.0)),
                egui::Color32::from_rgb(229, 236, 239),
            );
            painter.text(
                node_rect.left_bottom() + egui::vec2(9.0, -7.0),
                egui::Align2::LEFT_BOTTOM,
                format!("{} · tier {}", node.age, node.tier),
                egui::FontId::monospace((10.0 * view.zoom).clamp(8.0, 11.0)),
                egui::Color32::from_rgb(157, 178, 188),
            );
        }
    }

    if view.show_minimap {
        draw_minimap(ui, &painter, rect, bounds, catalog, layout, view);
    }
    painter.text(
        rect.left_bottom() + egui::vec2(10.0, -8.0),
        egui::Align2::LEFT_BOTTOM,
        format!(
            "{} nodes · {} edges · {:.0}% · wheel zoom · middle/Space-drag pan",
            catalog.technology.nodes.len(),
            catalog
                .technology
                .nodes
                .values()
                .map(|node| node.prerequisites.len())
                .sum::<usize>(),
            view.zoom * 100.0
        ),
        egui::FontId::monospace(10.0),
        egui::Color32::from_rgb(133, 153, 164),
    );
    output
}

fn handle_navigation(
    ui: &egui::Ui,
    rect: egui::Rect,
    response: &egui::Response,
    view: &mut TechnologyGraphViewState,
) {
    let pointer = ui.input(|input| input.pointer.hover_pos());
    if pointer.is_some_and(|position| rect.contains(position)) {
        let scroll = ui.input(|input| input.smooth_scroll_delta.y);
        if scroll.abs() > f32::EPSILON {
            let pointer = pointer.unwrap_or(rect.center());
            let world = screen_to_world(rect, view, pointer);
            view.zoom = (view.zoom * (scroll * 0.002).exp()).clamp(MIN_ZOOM, MAX_ZOOM);
            view.pan = pointer - rect.center() - world.to_vec2() * view.zoom;
        }
    }
    let space_down = ui.input(|input| input.key_down(egui::Key::Space));
    if response.dragged_by(egui::PointerButton::Middle)
        || (space_down && response.dragged_by(egui::PointerButton::Primary))
    {
        view.pan += ui.input(|input| input.pointer.delta());
    }
}

fn draw_grid(painter: &egui::Painter, rect: egui::Rect, view: &TechnologyGraphViewState) {
    let spacing = 200.0 * view.zoom;
    if spacing < 16.0 {
        return;
    }
    let origin = rect.center() + view.pan;
    let mut x = rect.left() + (origin.x - rect.left()).rem_euclid(spacing);
    while x < rect.right() {
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
            egui::Stroke::new(0.5, egui::Color32::from_rgb(28, 38, 46)),
        );
        x += spacing;
    }
    let mut y = rect.top() + (origin.y - rect.top()).rem_euclid(spacing);
    while y < rect.bottom() {
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            egui::Stroke::new(0.5, egui::Color32::from_rgb(28, 38, 46)),
        );
        y += spacing;
    }
}

fn draw_connection(painter: &egui::Painter, source: egui::Pos2, target: egui::Pos2, zoom: f32) {
    let bend = ((target.x - source.x).abs() * 0.45).max(25.0 * zoom);
    let control_a = source + egui::vec2(bend, 0.0);
    let control_b = target - egui::vec2(bend, 0.0);
    let points: Vec<_> = (0_u8..=18)
        .map(|step| {
            let t = f32::from(step) / 18.0;
            cubic_bezier(source, control_a, control_b, target, t)
        })
        .collect();
    painter.add(egui::Shape::line(
        points,
        egui::Stroke::new(
            (1.5 * zoom).clamp(0.65, 2.0),
            egui::Color32::from_rgb(94, 128, 143),
        ),
    ));
    if zoom >= 0.08 {
        let direction = (target - control_b).normalized();
        painter.line_segment(
            [target, target - direction.rot90() * 5.0 - direction * 9.0],
            egui::Stroke::new(1.4, egui::Color32::from_rgb(94, 128, 143)),
        );
        painter.line_segment(
            [target, target + direction.rot90() * 5.0 - direction * 9.0],
            egui::Stroke::new(1.4, egui::Color32::from_rgb(94, 128, 143)),
        );
    }
}

fn cubic_bezier(
    start: egui::Pos2,
    control_start: egui::Pos2,
    control_end: egui::Pos2,
    end: egui::Pos2,
    progress: f32,
) -> egui::Pos2 {
    let one_minus_t = 1.0 - progress;
    let value = start.to_vec2() * one_minus_t.powi(3)
        + control_start.to_vec2() * 3.0 * one_minus_t.powi(2) * progress
        + control_end.to_vec2() * 3.0 * one_minus_t * progress.powi(2)
        + end.to_vec2() * progress.powi(3);
    egui::pos2(value.x, value.y)
}

fn draw_minimap(
    ui: &egui::Ui,
    painter: &egui::Painter,
    canvas_rect: egui::Rect,
    bounds: egui::Rect,
    catalog: &ContentCatalog,
    layout: &TechnologyGraphLayout,
    view: &mut TechnologyGraphViewState,
) {
    let minimap_rect = egui::Rect::from_min_size(
        canvas_rect.right_bottom() - egui::vec2(238.0, 174.0),
        egui::vec2(224.0, 146.0),
    );
    painter.rect_filled(
        minimap_rect,
        5.0,
        egui::Color32::from_rgba_unmultiplied(8, 12, 16, 225),
    );
    painter.rect_stroke(
        minimap_rect,
        5.0,
        egui::Stroke::new(1.0, egui::Color32::from_rgb(88, 111, 122)),
        egui::StrokeKind::Inside,
    );
    let inner = minimap_rect.shrink(8.0);
    let scale =
        (inner.width() / bounds.width().max(1.0)).min(inner.height() / bounds.height().max(1.0));
    let offset = inner.center() - bounds.center().to_vec2() * scale;
    let map_point =
        |point: egui::Pos2| egui::pos2(offset.x + point.x * scale, offset.y + point.y * scale);
    for group in layout.groups.values() {
        let group_rect = egui::Rect::from_min_size(
            egui::pos2(group.position.x, group.position.y),
            egui::vec2(group.size.width, group.size.height),
        );
        painter.rect_stroke(
            egui::Rect::from_min_max(map_point(group_rect.min), map_point(group_rect.max)),
            1.0,
            egui::Stroke::new(0.6, egui::Color32::from_rgb(55, 76, 88)),
            egui::StrokeKind::Inside,
        );
    }
    for (id, node) in &catalog.technology.nodes {
        let Some(position) = layout.nodes.get(id).map(|value| value.position) else {
            continue;
        };
        let point = map_point(egui::pos2(
            position.x + NODE_SIZE.x * 0.5,
            position.y + NODE_SIZE.y * 0.5,
        ));
        painter.circle_filled(
            point,
            1.7,
            if node.unavailable {
                egui::Color32::from_rgb(156, 83, 88)
            } else {
                egui::Color32::from_rgb(77, 174, 128)
            },
        );
    }
    let visible_world = egui::Rect::from_min_max(
        screen_to_world(canvas_rect, view, canvas_rect.min),
        screen_to_world(canvas_rect, view, canvas_rect.max),
    );
    let mapped_viewport =
        egui::Rect::from_min_max(map_point(visible_world.min), map_point(visible_world.max));
    if mapped_viewport.intersects(inner) {
        painter.rect_stroke(
            mapped_viewport.intersect(inner),
            1.0,
            egui::Stroke::new(1.2, egui::Color32::YELLOW),
            egui::StrokeKind::Inside,
        );
    }

    let response = ui.interact(
        minimap_rect,
        ui.id().with("technology_graph_minimap"),
        egui::Sense::click_and_drag(),
    );
    if (response.clicked() || response.dragged())
        && let Some(pointer) = response.interact_pointer_pos()
    {
        let world = egui::pos2(
            (pointer.x - offset.x) / scale,
            (pointer.y - offset.y) / scale,
        );
        center_world(view, world);
    }
}

fn fit_bounds(view: &mut TechnologyGraphViewState, bounds: egui::Rect, rect: egui::Rect) {
    let width_scale = (rect.width() - 60.0) / bounds.width().max(1.0);
    let height_scale = (rect.height() - 60.0) / bounds.height().max(1.0);
    view.zoom = width_scale.min(height_scale).clamp(MIN_ZOOM, 1.25);
    center_world(view, bounds.center());
}

fn center_world(view: &mut TechnologyGraphViewState, point: egui::Pos2) {
    view.pan = -point.to_vec2() * view.zoom;
}

fn content_bounds(layout: &TechnologyGraphLayout) -> egui::Rect {
    let mut min = egui::pos2(f32::INFINITY, f32::INFINITY);
    let mut max = egui::pos2(f32::NEG_INFINITY, f32::NEG_INFINITY);
    for group in layout.groups.values() {
        min.x = min.x.min(group.position.x);
        min.y = min.y.min(group.position.y);
        max.x = max.x.max(group.position.x + group.size.width);
        max.y = max.y.max(group.position.y + group.size.height);
    }
    for node in layout.nodes.values() {
        min.x = min.x.min(node.position.x);
        min.y = min.y.min(node.position.y);
        max.x = max.x.max(node.position.x + NODE_SIZE.x);
        max.y = max.y.max(node.position.y + NODE_SIZE.y);
    }
    if !min.x.is_finite() {
        return egui::Rect::from_center_size(egui::Pos2::ZERO, egui::vec2(100.0, 100.0));
    }
    egui::Rect::from_min_max(min, max).expand(80.0)
}

fn world_to_screen(
    rect: egui::Rect,
    view: &TechnologyGraphViewState,
    point: egui::Pos2,
) -> egui::Pos2 {
    rect.center() + view.pan + point.to_vec2() * view.zoom
}

fn screen_to_world(
    rect: egui::Rect,
    view: &TechnologyGraphViewState,
    point: egui::Pos2,
) -> egui::Pos2 {
    let value = (point - rect.center() - view.pan) / view.zoom;
    egui::pos2(value.x, value.y)
}

fn world_rect_to_screen(
    rect: egui::Rect,
    view: &TechnologyGraphViewState,
    world: egui::Rect,
) -> egui::Rect {
    egui::Rect::from_min_max(
        world_to_screen(rect, view, world.min),
        world_to_screen(rect, view, world.max),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fit_handles_large_unity_coordinate_ranges() {
        let mut view = TechnologyGraphViewState::default();
        let bounds = egui::Rect::from_min_max(
            egui::pos2(-4_500.0, -17_000.0),
            egui::pos2(6_000.0, 24_000.0),
        );
        fit_bounds(
            &mut view,
            bounds,
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 510.0)),
        );
        assert!((MIN_ZOOM..=MAX_ZOOM).contains(&view.zoom()));
        assert!(view.zoom() < 0.1);
    }

    #[test]
    fn screen_world_transform_round_trips() {
        let rect = egui::Rect::from_min_size(egui::pos2(30.0, 40.0), egui::vec2(900.0, 500.0));
        let view = TechnologyGraphViewState {
            pan: egui::vec2(17.0, -38.0),
            zoom: 0.72,
            ..TechnologyGraphViewState::default()
        };
        let world = egui::pos2(-200.0, 640.0);
        let screen = world_to_screen(rect, &view, world);
        let restored = screen_to_world(rect, &view, screen);
        assert!((restored - world).length() < 0.001);
    }
}
