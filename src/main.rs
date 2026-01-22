use leptos::prelude::{
    event_target_value, request_animation_frame, signal, ClassAttribute, Effect, ElementChild, Get,
    GetValue, Memo, OnAttribute, Set, Show, StoredValue, StyleAttribute, Update,
};
use leptos::*;
use std::collections::HashMap;

use crate::category_modal::CategoryModal;
use crate::types::Entry;
use crate::wheel::WheelComponent;

mod category_modal;
mod detail_modal;
mod types;
mod wheel;

#[component]
fn App() -> impl IntoView {
    let yaml_data = include_str!("../data.yaml");
    let entries_vec: Vec<Entry> = serde_yaml::from_str(yaml_data).unwrap_or_default();
    let entries = StoredValue::new(entries_vec.clone());

    let (filtered_entries, set_filtered_entries) = signal(entries_vec);
    let (show_category_modal, set_show_category_modal) = signal(false);
    let (show_detail_modal, set_show_detail_modal) = signal(false);
    let (selected_entry, set_selected_entry) = signal::<Option<Entry>>(None);
    let (selected_categories, set_selected_categories) = signal::<Vec<String>>(vec![]);
    let (episode_filter, set_episode_filter) = signal("all".to_string());
    let (rotation, set_rotation) = signal(0.0);
    let (is_spinning, set_is_spinning) = signal(false);
    let (velocity, set_velocity) = signal(0.0);

    let all_categories = Memo::new(move |_| {
        let mut category_count: HashMap<String, usize> = HashMap::new();
        for entry in entries.get_value().iter() {
            for cat in &entry.category {
                *category_count.entry(cat.clone()).or_insert(0) += 1;
            }
        }
        category_count
    });

    let apply_filters = move || {
        let cats = selected_categories.get();
        let ep_filter = episode_filter.get();

        let filtered: Vec<Entry> = entries
            .get_value()
            .iter()
            .filter(|e| {
                let cat_match = cats.is_empty() || e.category.iter().any(|c| cats.contains(c));

                let ep_match = match ep_filter.as_str() {
                    "existing" => e.episode.is_some(),
                    "non-existing" => e.episode.is_none(),
                    _ => true,
                };

                cat_match && ep_match
            })
            .cloned()
            .collect();

        set_filtered_entries.set(filtered);
    };

    // Animation loop effect with smooth easing
    Effect::new(move |_| {
        let current_velocity = velocity.get();
        let current_spinning = is_spinning.get();

        if current_velocity > 0.05 {
            // Update rotation
            set_rotation.update(|r| *r += current_velocity);

            // Improved easing with exponential decay and smooth landing
            let new_velocity = if current_velocity > 20.0 {
                // Fast spin phase: minimal friction
                current_velocity * 0.985
            } else if current_velocity > 8.0 {
                // Medium spin: increased friction
                current_velocity * 0.97
            } else if current_velocity > 2.0 {
                // Slowing down: more friction
                current_velocity * 0.94
            } else if current_velocity > 0.5 {
                // Final approach: smooth landing
                current_velocity * 0.88
            } else {
                // Very slow: rapid stop
                current_velocity * 0.75
            };

            // Schedule next frame
            request_animation_frame(move || {
                set_velocity.set(new_velocity);
            });
        } else if current_spinning && current_velocity <= 0.05 {
            set_is_spinning.set(false);
            set_velocity.set(0.0);

            // Select entry - arrow points at top (270 degrees)
            let final_angle = rotation.get() % 360.0;
            let entries = filtered_entries.get();
            if !entries.is_empty() {
                let segment_angle = 360.0 / entries.len() as f64;
                // Arrow is at top (270 degrees). Find which segment is there.
                // After rotating by final_angle, the segment originally at (270 - final_angle) is now at top
                let arrow_position = 270.0;
                let target_angle = ((arrow_position - final_angle) % 360.0 + 360.0) % 360.0;
                let selected_idx = (target_angle / segment_angle) as usize % entries.len();

                if let Some(entry) = entries.get(selected_idx) {
                    set_selected_entry.set(Some(entry.clone()));
                    set_show_detail_modal.set(true);
                }
            }
        }
    });

    let spin_wheel = move |_| {
        if is_spinning.get() {
            return;
        }

        let entries = filtered_entries.get();
        if entries.is_empty() {
            return;
        }

        set_is_spinning.set(true);

        // Enhanced random spin with more dramatic variation
        // 6-12 full rotations for excitement
        let spins = 6.0 + (rand::random::<f64>() * 6.0);
        // Higher initial velocity for more dramatic effect
        // Adjusted for the new multi-stage easing system
        let initial_velocity = spins * 360.0 / 50.0;
        set_velocity.set(initial_velocity);
    };

    let confirm_selection = move |_| {
        if is_spinning.get() {
            return;
        }

        // Calculate which entry is under the arrow
        let final_angle = rotation.get() % 360.0;
        let entries = filtered_entries.get();
        if !entries.is_empty() {
            let segment_angle = 360.0 / entries.len() as f64;
            // Arrow is at top (270 degrees). Find which segment is there.
            let arrow_position = 270.0;
            let target_angle = ((arrow_position - final_angle) % 360.0 + 360.0) % 360.0;
            let selected_idx = (target_angle / segment_angle) as usize % entries.len();

            if let Some(entry) = entries.get(selected_idx) {
                set_selected_entry.set(Some(entry.clone()));
                set_show_detail_modal.set(true);
            }
        }
    };

    view! {
        <div>
            <div class="controls">
                <button on:click=move |_| set_show_category_modal.set(true)>
                    "Categories"
                </button>
                <select
                    on:change=move |e| {
                        set_episode_filter.set(event_target_value(&e));
                        apply_filters();
                    }
                    style="padding: 0.75rem; background: #3a3a3a; color: #fff; border: none; border-radius: 6px; cursor: pointer; width: 100%; font-size: 1rem;"
                >
                    <option value="all">"All Entries"</option>
                    <option value="existing">"Existing Episodes"</option>
                    <option value="non-existing">"Future Topics"</option>
                </select>
                <button on:click=spin_wheel disabled=move || is_spinning.get()>
                    {move || if is_spinning.get() { "Spinning..." } else { "Spin!" }}
                </button>
                <button on:click=confirm_selection disabled=move || is_spinning.get()>
                    "Confirm Selection"
                </button>
            </div>

            <div class="wheel-container">
                <div class=move || if is_spinning.get() { "arrow spinning" } else { "arrow" }></div>
                <WheelComponent
                    entries=filtered_entries
                    rotation=rotation
                    set_rotation=set_rotation
                    set_velocity=set_velocity
                    is_spinning=is_spinning
                    set_is_spinning=set_is_spinning
                    velocity=velocity
                />
            </div>

            <Show when=move || show_category_modal.get()>
                <CategoryModal
                    categories=all_categories
                    selected=selected_categories
                    set_selected=set_selected_categories
                    on_close=move || {
                        set_show_category_modal.set(false);
                        apply_filters();
                    }
                />
            </Show>

            <Show when=move || show_detail_modal.get()>
                <detail_modal::DetailModal
                    entry=selected_entry
                    on_close=move || set_show_detail_modal.set(false)
                />
            </Show>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
