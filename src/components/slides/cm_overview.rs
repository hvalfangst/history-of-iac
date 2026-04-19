use leptos::*;

#[component]
pub fn CmOverviewSlide() -> impl IntoView {
    view! {
        <div class="slide chapter-overview-slide">
            <div class="chapter-overview-header">
                <span class="chapter-tag">"Chapter 1"</span>
                <h2 class="chapter-overview-title">"Configuration Management"</h2>
                <p class="chapter-overview-sub">"1993–2012 · Physical machines and VMs, no cloud yet"</p>
            </div>

            // ── Definition card ───────────────────────────────────────────
            <div class="what-card what-card-wide">
                <ul class="overview-bullets">
                    <li>"Declare what you want, not the steps to get there"</li>
                    <li>"Re-running the same config is safe — only actual drift gets fixed"</li>
                    <li>"One file applied uniformly across the whole fleet"</li>
                    <li>"Config in git: reviewed, versioned, and rollback-able like any code"</li>
                </ul>
            </div>

            // ── Tool list ─────────────────────────────────────────────────
            <div class="chapter-tool-list">
                <div class="chapter-tool-row" style="border-left: 4px solid #e07b39">
                    <span class="chapter-tool-year" style="color:#e07b39">"1993"</span>
                    <span class="chapter-tool-name" style="color:#e07b39">"CFEngine"</span>
                    <span class="chapter-tool-desc">"First CM tool — pull model, Promise Theory, created at UiO"</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #f5a623">
                    <span class="chapter-tool-year" style="color:#f5a623">"2005"</span>
                    <span class="chapter-tool-name" style="color:#f5a623">"Puppet"</span>
                    <span class="chapter-tool-desc">"Custom DSL, agent/master architecture, idempotent by design"</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #e74c3c">
                    <span class="chapter-tool-year" style="color:#e74c3c">"2009"</span>
                    <span class="chapter-tool-name" style="color:#e74c3c">"Chef"</span>
                    <span class="chapter-tool-desc">"Ruby DSL, cookbooks, strong focus on testing and compliance"</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #c0392b">
                    <span class="chapter-tool-year" style="color:#c0392b">"2012"</span>
                    <span class="chapter-tool-name" style="color:#c0392b">"Ansible"</span>
                    <span class="chapter-tool-desc">"Agentless push via SSH, YAML playbooks, no setup required"</span>
                </div>
            </div>
        </div>
    }
}
