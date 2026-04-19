use leptos::*;

#[component]
pub fn WhatIsIacSlide() -> impl IntoView {
    view! {
        <div class="slide what-iac-slide">
            <h2 class="what-iac-main-title">"Infrastructure as Code"</h2>

            // ── Era banner ────────────────────────────────────────────────
            <div class="what-era-bar">
                <span class="what-era-badge">"2006+"</span>
                <span class="what-era-sep">"|"</span>
                <span class="what-era-text">"AWS (2006) · Azure (2010) made cloud the standard"</span>
                <span class="what-era-sep">"|"</span>
                <span class="what-era-text">"Docker (2013) · Kubernetes (2014) changed configuration"</span>
            </div>

            // ── IaC card ──────────────────────────────────────────────────
            <div class="what-card what-card-wide">
                <div class="what-card-header" style="border-left: 4px solid #60a5fa">
                    <span class="what-card-acronym" style="color:#60a5fa">"IaC"</span>
                    <span class="what-card-full">"Infrastructure as Code"</span>
                </div>
                <div class="what-card-cols">
                    <ul class="what-card-list">
                        <li>
                            <span class="what-bullet" style="color:#60a5fa">"▸"</span>
                            <span>"Infrastructure in Git alongside the code"</span>
                        </li>
                        <li>
                            <span class="what-bullet" style="color:#60a5fa">"▸"</span>
                            <span>"New environment in minutes, not days"</span>
                        </li>
                    </ul>
                    <ul class="what-card-list">
                        <li>
                            <span class="what-bullet" style="color:#60a5fa">"▸"</span>
                            <span>"PRs and code review for infrastructure"</span>
                        </li>
                        <li>
                            <span class="what-bullet" style="color:#60a5fa">"▸"</span>
                            <span>"Rollback = revert a commit"</span>
                        </li>
                    </ul>
                </div>
            </div>

        </div>
    }
}
