use leptos::prelude::{ClassAttribute, ElementChild, Get, IntoAny, OnAttribute, ReadSignal};
use leptos::*;

use crate::types::{EpisodeNumber, UrlOrVec};
use crate::Entry;

fn extract_youtube_id(url: &str) -> Option<String> {
    // Handle youtu.be format
    if let Some(idx) = url.find("youtu.be/") {
        let start = idx + 9;
        let id = &url[start..];
        let end = id.find('?').unwrap_or(id.len());
        return Some(id[..end].to_string());
    }

    // Handle youtube.com format with v= parameter
    if let Some(idx) = url.find("v=") {
        let start = idx + 2;
        let id = &url[start..];
        let end = id.find('&').unwrap_or(id.len());
        return Some(id[..end].to_string());
    }

    None
}

fn get_youtube_thumbnail(url: &str) -> Option<String> {
    extract_youtube_id(url).map(|id| {
        // Use hqdefault for reliability (all videos have this)
        format!("https://img.youtube.com/vi/{}/hqdefault.jpg", id)
    })
}

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
                    // Build episode-url pairs for proper matching
                    let episode_url_pairs: Vec<(String, String)> = match (&e.episode, &e.url) {
                        (Some(EpisodeNumber::Multiple(eps)), Some(UrlOrVec::Multiple(urls))) => {
                            eps.iter().zip(urls.iter())
                                .map(|(ep_num, url)| (ep_num.to_string(), url.clone()))
                                .collect()
                        },
                        (Some(EpisodeNumber::Single(ep)), Some(UrlOrVec::Single(url))) => {
                            vec![(ep.to_string(), url.clone())]
                        },
                        _ => vec![],
                    };

                    view! {
                        <div class="detail-content">
                            {if !episode_url_pairs.is_empty() {
                                Some(if episode_url_pairs.len() == 1 {
                                    get_youtube_thumbnail(&episode_url_pairs[0].1).map(|thumb_url| view! {
                                        <a href=episode_url_pairs[0].1.clone() target="_blank" rel="noopener noreferrer" class="thumbnail-link">
                                            <div class="modal-thumbnail">
                                                <img src=thumb_url alt="Episode Thumbnail" loading="lazy" />
                                            </div>
                                        </a>
                                    }.into_any())
                                } else {
                                    Some(view! {
                                        <div class="modal-thumbnails-grid">
                                            {episode_url_pairs.iter().map(|(ep_num, url)| {
                                                let thumb = get_youtube_thumbnail(url);
                                                let url = url.clone();
                                                let ep_num = ep_num.clone();
                                                view! {
                                                    <a href=url.clone() target="_blank" rel="noopener noreferrer" class="thumbnail-link">
                                                        <div class="modal-thumbnail-item">
                                                            {thumb.map(|t| view! {
                                                                <img src=t alt=format!("Episode {} Thumbnail", ep_num) loading="lazy" />
                                                            })}
                                                            <div class="thumbnail-label">
                                                                {format!("Episode {}", ep_num)}
                                                            </div>
                                                        </div>
                                                    </a>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }.into_any())
                                })
                            } else {
                                None
                            }}

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

                            {if !episode_url_pairs.is_empty() {
                                Some(if episode_url_pairs.len() == 1 {
                                    view! {
                                        <div>
                                            <a href=episode_url_pairs[0].1.clone() target="_blank" rel="noopener noreferrer">
                                                {format!("Watch Episode {} →", episode_url_pairs[0].0)}
                                            </a>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div>
                                            <strong>"Episodes:"</strong>
                                            {episode_url_pairs.iter().map(|(ep_num, url)| {
                                                let ep_num = ep_num.clone();
                                                let url = url.clone();
                                                view! {
                                                    <p><a href=url target="_blank" rel="noopener noreferrer">
                                                        {format!("Episode {} →", ep_num)}
                                                    </a></p>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }.into_any()
                                })
                            } else {
                                None
                            }}

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
