use leptos::prelude::{
    signal, ClassAttribute, CustomAttribute, ElementChild, Get, NodeRef, NodeRefAttribute,
    OnAttribute, ReadSignal, Set, StyleAttribute, Update, WriteSignal,
};
use leptos::*;

use crate::Entry;

#[component]
pub fn WheelComponent(
    entries: ReadSignal<Vec<Entry>>,
    rotation: ReadSignal<f64>,
    set_rotation: WriteSignal<f64>,
    set_velocity: WriteSignal<f64>,
    is_spinning: ReadSignal<bool>,
    set_is_spinning: WriteSignal<bool>,
    velocity: ReadSignal<f64>,
) -> impl IntoView {
    let (is_dragging, set_is_dragging) = signal(false);
    let (last_angle, set_last_angle) = signal(0.0);
    let (drag_velocity, set_drag_velocity) = signal(0.0);
    let node_ref: NodeRef<svg::Svg> = NodeRef::new();

    let get_angle_from_event = move |client_x: i32, client_y: i32| -> f64 {
        if let Some(svg) = node_ref.get() {
            let rect = svg.get_bounding_client_rect();
            let center_x = rect.left() + rect.width() / 2.0;
            let center_y = rect.top() + rect.height() / 2.0;
            let dx = client_x as f64 - center_x;
            let dy = client_y as f64 - center_y;
            dy.atan2(dx).to_degrees()
        } else {
            0.0
        }
    };

    let on_mouse_down = move |e: web_sys::MouseEvent| {
        if !is_spinning.get() {
            e.prevent_default();
            set_is_dragging.set(true);
            let angle = get_angle_from_event(e.client_x(), e.client_y());
            set_last_angle.set(angle);
            set_velocity.set(0.0);
        }
    };

    let on_mouse_move = move |e: web_sys::MouseEvent| {
        if is_dragging.get() && !is_spinning.get() {
            e.prevent_default();
            let angle = get_angle_from_event(e.client_x(), e.client_y());
            let delta = angle - last_angle.get();
            let normalized_delta = if delta > 180.0 {
                delta - 360.0
            } else if delta < -180.0 {
                delta + 360.0
            } else {
                delta
            };
            set_rotation.update(|r| *r += normalized_delta);
            set_drag_velocity.set(normalized_delta);
            set_last_angle.set(angle);
        }
    };

    let on_mouse_up = move |_| {
        let current_drag_velocity = drag_velocity.get().abs();
        set_is_dragging.set(false);

        // If released with sufficient drag velocity, throw the wheel
        if current_drag_velocity > 0.5 {
            set_is_spinning.set(true);
            // Amplify the drag velocity for a satisfying throw (multiply by ~15-25)
            let throw_velocity = current_drag_velocity * (18.0 + (rand::random::<f64>() * 7.0));
            set_velocity.set(throw_velocity.min(50.0)); // Cap at reasonable max
        }

        set_drag_velocity.set(0.0);
    };

    let colors = [
        "#ff6b35", "#f7931e", "#fdc82f", "#8ac926", "#1982c4", "#6a4c93",
    ];

    view! {
        <svg
            node_ref=node_ref
            class=move || if is_spinning.get() { "wheel-svg spinning" } else { "wheel-svg" }
            viewBox="-250 -250 500 500"
            on:mousedown=on_mouse_down
            on:mousemove=on_mouse_move
            on:mouseup=on_mouse_up
            on:mouseleave=on_mouse_up
            style=move || {
                let v = velocity.get();
                // Simplified transform for better Firefox performance
                // Only apply effects when spinning fast enough to be noticeable
                if v > 10.0 {
                    let scale = 1.0 + (v / 250.0).min(0.05);
                    format!(
                        "transform: scale({}); \
                         filter: drop-shadow(0 8px 16px rgba(255, 107, 53, 0.4));",
                        scale
                    )
                } else {
                    "".to_string()
                }
            }
        >
            <g
                transform=move || format!("rotate({})", rotation.get())
            >
                {move || {
                    let e = entries.get();
                    let count = e.len().max(1);
                    let angle = 360.0 / count as f64;

                    e.iter().enumerate().map(|(i, entry)| {
                        let start_angle = i as f64 * angle;
                        let color = colors[i % colors.len()];

                        view! {
                            <WheelSegment
                                title=entry.title.clone()
                                start_angle=start_angle
                                angle=angle
                                color=color.to_string()
                            />
                        }
                    }).collect::<Vec<_>>()
                }}
            </g>
            <circle cx="0" cy="0" r="30" fill="#2a2a2a" stroke="#ff6b35" stroke-width="3"/>
        </svg>
    }
}

#[component]
fn WheelSegment(title: String, start_angle: f64, angle: f64, color: String) -> impl IntoView {
    let start_rad = start_angle.to_radians();
    let end_rad = (start_angle + angle).to_radians();
    let mid_rad = (start_angle + angle / 2.0).to_radians();

    let radius = 220.0;
    let x1 = radius * start_rad.cos();
    let y1 = radius * start_rad.sin();
    let x2 = radius * end_rad.cos();
    let y2 = radius * end_rad.sin();

    let large_arc = if angle > 180.0 { 1 } else { 0 };
    let path = format!(
        "M 0 0 L {} {} A {} {} 0 {} 1 {} {} Z",
        x1, y1, radius, radius, large_arc, x2, y2
    );

    let text_radius = 150.0;
    let text_x = text_radius * mid_rad.cos();
    let text_y = text_radius * mid_rad.sin();
    let text_rotation = start_angle + angle / 2.0;

    let truncated_title = if title.len() > 20 {
        format!("{}...", title.chars().take(17).collect::<String>())
    } else {
        title.clone()
    };

    view! {
        <g>
            <path d=path fill=color stroke="#2a2a2a" stroke-width="2"/>
            <text
                x=text_x
                y=text_y
                text-anchor="middle"
                dominant-baseline="middle"
                transform=format!("rotate({} {} {})", text_rotation, text_x, text_y)
                fill="#fff"
                font-size="11"
                font-weight="bold"
                style="pointer-events: none; user-select: none;"
            >
                {truncated_title}
            </text>
        </g>
    }
}
