use leptos::prelude::{ClassAttribute, ElementChild, Get, IntoAny, OnAttribute, ReadSignal};
use leptos::*;

use crate::types::{EpisodeNumber, UrlOrVec};
use crate::Entry;

#[component]
pub fn DetailModal(
    entry: ReadSignal<Option<Entry>>,
    on_close: impl Fn() + 'static + Copy,
) -> impl IntoView {
    view! {
        <div class="modal-backdrop" on:click=move |_| on_close()>
            <div class="modal" on:click=|e| e.stop_propagation()>
                <button class="modal-close" on:click=move |_| on_close()>"×"</button>
                {move || entry.get().map(|e| {
                    view! {
                        <div class="detail-content">
                            <h2>{e.title.clone()}</h2>

                            {e.episode.as_ref().map(|ep| {
                                let ep_text = match ep {
                                    EpisodeNumber::Single(n) => format!("Episode {}", n),
                                    EpisodeNumber::Multiple(nums) => format!("Episodes {}",
                                        nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ")),
                                };
                                view! { <div class="episode-badge">{ep_text}</div> }
                            })}

                            {e.claim.as_ref().map(|c| view! {
                                <p><strong>"Claim: "</strong>{c.clone()}</p>
                            })}

                            {e.description.as_ref().map(|d| view! {
                                <p>{d.clone()}</p>
                            })}

                            {(!e.category.is_empty()).then(|| view! {
                                <p><strong>"Categories: "</strong>{e.category.join(", ")}</p>
                            })}

                            {e.url.as_ref().map(|urls| {
                                match urls {
                                    UrlOrVec::Single(url) => view! {
                                        <div>
                                            <a href=url.clone() target="_blank" rel="noopener noreferrer">
                                                "Watch Episode →"
                                            </a>
                                        </div>
                                    }.into_any(),
                                    UrlOrVec::Multiple(urls) => view! {
                                        <div>
                                            <strong>"Episodes:"</strong>
                                            {urls.iter().enumerate().map(|(i, url)| view! {
                                                <p><a href=url.clone() target="_blank" rel="noopener noreferrer">
                                                    {format!("Part {} →", i + 1)}
                                                </a></p>
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }.into_any(),
                                }
                            })}

                            {e.drink.as_ref().filter(|s| !s.is_empty()).map(|s| view! {
                                <p><strong>"Drink: "</strong>{s.clone()}</p>
                            })}

                            {e.sponsor.as_ref().filter(|s| !s.is_empty()).map(|s| view! {
                                <p><strong>"Sponsor: "</strong>{s.clone()}</p>
                            })}
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
