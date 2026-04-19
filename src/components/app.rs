use leptos::*;
use leptos::ev::KeyboardEvent;

use crate::components::slide_nav::SlideNav;
use crate::components::slides::{SlideDispatch, MAX_SLIDE};

#[component]
pub fn App() -> impl IntoView {
    let slide: RwSignal<usize> = create_rw_signal(0_usize);

    provide_context(slide);

    // Keyboard navigation: left/right arrows change slide
    window_event_listener(ev::keydown, move |e: KeyboardEvent| {
        match e.key().as_str() {
            "ArrowLeft" | "ArrowUp" => slide.update(|s| {
                if *s > 0 {
                    *s -= 1;
                }
            }),
            "ArrowRight" | "ArrowDown" => slide.update(|s| {
                if *s < MAX_SLIDE {
                    *s += 1;
                }
            }),
            _ => {}
        }
    });

    view! {
        <div class="app">
            <SlideDispatch />
            <SlideNav />
        </div>
    }
}
