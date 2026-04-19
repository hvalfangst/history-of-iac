use leptos::*;

#[component]
pub fn TitleSlide() -> impl IntoView {
    view! {
        <div class="slide title-slide">
            <h1 class="title-heading">"IaC with Bicep"</h1>
            <img src="arnold.jpg" alt="Arnold" class="title-image"/>
        </div>
    }
}
