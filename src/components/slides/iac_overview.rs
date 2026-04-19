use leptos::*;

#[component]
pub fn IacOverviewSlide() -> impl IntoView {
    view! {
        <div class="slide chapter-overview-slide">
            <div class="chapter-overview-header">
                <span class="chapter-tag">"Chapter 2"</span>
                <h2 class="chapter-overview-title">"Infrastructure as Code"</h2>
                <p class="chapter-overview-sub">"2011–2020 · Cloud APIs replace the server rack"</p>
            </div>

            // ── Definition card ───────────────────────────────────────────
            <div class="what-card what-card-wide">
                <ul class="overview-bullets">
                    <li>"Your whole cloud stack is a file you can read, diff, and commit"</li>
                    <li>"Spin up a full environment from scratch in minutes"</li>
                    <li>"Changes go through pull requests, not portal tickets"</li>
                    <li>"Rollback is a git revert, not a manual cleanup job"</li>
                </ul>
            </div>

            // ── Tool list ─────────────────────────────────────────────────
            <div class="chapter-tool-list">
                <div class="chapter-tool-row" style="border-left: 4px solid #e67e22">
                    <span class="chapter-tool-year" style="color:#e67e22">"2011"</span>
                    <span class="chapter-tool-name" style="color:#e67e22">"CloudFormation"</span>
                    <span class="chapter-tool-desc">"AWS-native, Stacks, atomic operations, no state file"</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #0078d4">
                    <span class="chapter-tool-year" style="color:#0078d4">"2014"</span>
                    <span class="chapter-tool-name" style="color:#0078d4">"ARM Templates"</span>
                    <span class="chapter-tool-desc">"Azure's control plane in JSON — predecessor to Bicep"</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #7b42bc">
                    <span class="chapter-tool-year" style="color:#7b42bc">"2014"</span>
                    <span class="chapter-tool-name" style="color:#7b42bc">"Terraform"</span>
                    <span class="chapter-tool-desc">"Multi-cloud HCL, state file, init → plan → apply"</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #50e6ff">
                    <span class="chapter-tool-year" style="color:#50e6ff">"2020"</span>
                    <span class="chapter-tool-name" style="color:#50e6ff">"Bicep"</span>
                    <span class="chapter-tool-desc">"Azure-native DSL, compiles to ARM, no state file"</span>
                </div>
            </div>
        </div>
    }
}
