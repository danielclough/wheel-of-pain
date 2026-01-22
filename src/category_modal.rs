use leptos::prelude::{
    ClassAttribute, ElementChild, Get, Memo, OnAttribute, ReadSignal, Set, StyleAttribute, Update,
    WriteSignal,
};
use leptos::*;
use std::collections::HashMap;

#[component]
pub fn CategoryModal(
    categories: Memo<HashMap<String, usize>>,
    selected: ReadSignal<Vec<String>>,
    set_selected: WriteSignal<Vec<String>>,
    on_close: impl Fn() + 'static + Copy,
) -> impl IntoView {
    let toggle_category = move |cat: String| {
        set_selected.update(|sel| {
            if sel.contains(&cat) {
                sel.retain(|c| c != &cat);
            } else {
                sel.push(cat);
            }
        });
    };

    let clear_all = move |_| {
        set_selected.set(vec![]);
    };

    view! {
        <div class="modal-backdrop" on:click=move |_| on_close()>
            <div class="modal category-modal" on:click=|e| e.stop_propagation()>
                <button class="modal-close" on:click=move |_| on_close()>"×"</button>
                <div style="padding: 2rem 2rem 0 2rem;">
                    <h2 style="padding: 0; margin: 0 0 0.5rem 0;">"Select Categories"</h2>
                    <p style="color: #aaa; margin: 0;">
                        "Click categories to filter. Size indicates frequency."
                    </p>
                </div>
                <div class="word-cloud">
                    {move || {
                        let mut cats: Vec<_> = categories.get().into_iter().collect();
                        cats.sort_by(|a, b| b.1.cmp(&a.1));

                        cats.into_iter().map(|(cat, count)| {
                            let size = 0.8 + (count as f64 * 0.2).min(1.2);
                            let weight = if count > 3 { "bold" } else { "normal" };
                            let cat_for_check = cat.clone();
                            let is_selected = move || selected.get().contains(&cat_for_check);
                            let cat_clone = cat.clone();

                            view! {
                                <div
                                    class="word-cloud-item"
                                    class:selected=is_selected
                                    style=format!("font-size: {}rem; font-weight: {}", size, weight)
                                    on:click=move |_| toggle_category(cat_clone.clone())
                                >
                                    {cat.clone()} " (" {count} ")"
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
                <div style="display: flex; gap: 1rem; padding: 1.5rem 2rem 2rem 2rem;">
                    <button on:click=clear_all style="flex: 1;">"Clear All"</button>
                    <button on:click=move |_| on_close() style="flex: 1;">"Apply"</button>
                </div>
            </div>
        </div>
    }
}
