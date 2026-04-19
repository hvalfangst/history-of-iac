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
                    <li>"Your entire cloud environment is text. Read it, diff it, commit it."</li>
                    <li>"Reproduce a full environment in minutes instead of weeks of tribal knowledge"</li>
                    <li>"Changes go through code review, not portal tickets and Jira queues"</li>
                    <li>"Rollback is git revert. Or it should be."</li>
                </ul>
            </div>

            // ── Tool list ─────────────────────────────────────────────────
            <div class="chapter-tool-list">
                <div class="chapter-tool-row" style="border-left: 4px solid #e67e22">
                    <span class="chapter-tool-year" style="color:#e67e22">"2011"</span>
                    <span class="chapter-tool-name" style="color:#e67e22">"CloudFormation"</span>
                    <span class="chapter-tool-desc">"AWS-native, atomic stack operations, no state file to babysit."</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #0078d4">
                    <span class="chapter-tool-year" style="color:#0078d4">"2014"</span>
                    <span class="chapter-tool-name" style="color:#0078d4">"ARM Templates"</span>
                    <span class="chapter-tool-desc">"Azure's control plane as raw JSON. Painful. Led directly to Bicep."</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #7b42bc">
                    <span class="chapter-tool-year" style="color:#7b42bc">"2014"</span>
                    <span class="chapter-tool-name" style="color:#7b42bc">"Terraform"</span>
                    <span class="chapter-tool-desc">"HCL, state file, multi-cloud. The state file is both its strength and its liability."</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #50e6ff">
                    <span class="chapter-tool-year" style="color:#50e6ff">"2020"</span>
                    <span class="chapter-tool-name" style="color:#50e6ff">"Bicep"</span>
                    <span class="chapter-tool-desc">"Azure-native DSL, compiles to ARM, no state file. The right abstraction."</span>
                </div>
            </div>
        </div>
    }
}
