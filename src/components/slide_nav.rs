use leptos::*;
use crate::i18n::get_translation;
use crate::components::slides::MAX_SLIDE;

#[component]
pub fn SlideNav() -> impl IntoView {
    let slide = use_context::<RwSignal<usize>>().expect("slide signal");
    let t = get_translation;

    let dots = (0..=MAX_SLIDE).map(|i| {
        view! {
            <button
                class=move || {
                    if slide.get() == i { "slide-dot active" } else { "slide-dot" }
                }
                on:click=move |_| slide.set(i)
                aria-label=format!("Go to slide {}", i + 1)
            />
        }
    }).collect::<Vec<_>>();

    view! {
        <nav class="slide-nav">
            <button
                class="slide-nav-btn"
                disabled=move || slide.get() == 0
                on:click=move |_| slide.update(|s| { if *s > 0 { *s -= 1; } })
            >
                {move || t("nav.prev")}
            </button>

            <div class="slide-dots">
                {dots}
            </div>

            <span class="slide-counter">
                {move || format!("{} / {}", slide.get() + 1, MAX_SLIDE + 1)}
            </span>

            <button
                class="slide-nav-btn"
                disabled=move || slide.get() == MAX_SLIDE
                on:click=move |_| slide.update(|s| { if *s < MAX_SLIDE { *s += 1; } })
            >
                {move || t("nav.next")}
            </button>
        </nav>
    }
}
