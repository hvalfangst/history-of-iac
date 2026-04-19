use leptos::*;

#[component]
pub fn WhatIsCmSlide() -> impl IntoView {
    view! {
        <div class="slide what-iac-slide">
            <h2 class="what-iac-main-title">"Configuration Management"</h2>

            // ── Era banner ────────────────────────────────────────────────
            <div class="what-era-bar">
                <span class="what-era-badge">"~1993 to the 2010s"</span>
                <span class="what-era-sep">"|"</span>
                <span class="what-era-text">"Physical servers and VMs"</span>
                <span class="what-era-sep">"|"</span>
                <span class="what-era-text">"Pre-cloud"</span>
                <span class="what-era-sep">"|"</span>
                <span class="what-era-text">"VMware Workstation (1999) · VMware ESX (2001)"</span>
            </div>

            // ── CM card ──────────────────────────────────────────────────
            <div class="what-card what-card-wide">
                <div class="what-card-header" style="border-left: 4px solid #f5a623">
                    <span class="what-card-acronym" style="color:#f5a623">"CM"</span>
                    <span class="what-card-full">"Configuration Management"</span>
                </div>
                <div class="what-card-cols">
                    <ul class="what-card-list">
                        <li>
                            <span class="what-bullet" style="color:#f5a623">"▸"</span>
                            <span>"Declarative: describe state, not steps"</span>
                        </li>
                        <li>
                            <span class="what-bullet" style="color:#f5a623">"▸"</span>
                            <span>"Idempotent: safe to run many times"</span>
                        </li>
                    </ul>
                    <ul class="what-card-list">
                        <li>
                            <span class="what-bullet" style="color:#f5a623">"▸"</span>
                            <span>"One policy → hundreds of servers"</span>
                        </li>
                        <li>
                            <span class="what-bullet" style="color:#f5a623">"▸"</span>
                            <span>"Version control and code review"</span>
                        </li>
                    </ul>
                </div>
            </div>

        </div>
    }
}
