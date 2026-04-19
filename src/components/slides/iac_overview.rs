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
                    <li>"Your entire cloud environment is a text file. Read it, diff it, check it in."</li>
                    <li>"Spin up a complete environment from scratch in minutes, not weeks of asking around."</li>
                    <li>"Changes go through pull requests. Not tickets. Not clicking things in a portal."</li>
                    <li>"Rollback is git revert. If yours is not, you have a problem."</li>
                </ul>
            </div>

            // ── Tool list ─────────────────────────────────────────────────
            <div class="chapter-tool-list">
                <div class="chapter-tool-row" style="border-left: 4px solid #e67e22">
                    <span class="chapter-tool-year" style="color:#e67e22">"2011"</span>
                    <span class="chapter-tool-name" style="color:#e67e22">"CloudFormation"</span>
                    <span class="chapter-tool-desc">"AWS-native. Atomic operations. No state file. Works without extra tooling."</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #0078d4">
                    <span class="chapter-tool-year" style="color:#0078d4">"2014"</span>
                    <span class="chapter-tool-name" style="color:#0078d4">"ARM Templates"</span>
                    <span class="chapter-tool-desc">"Azure's control plane in raw JSON. Writing it was unpleasant enough to spawn Bicep."</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #7b42bc">
                    <span class="chapter-tool-year" style="color:#7b42bc">"2014"</span>
                    <span class="chapter-tool-name" style="color:#7b42bc">"Terraform"</span>
                    <span class="chapter-tool-desc">"HCL, multi-cloud, state file. Guard the state file or start over."</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #50e6ff">
                    <span class="chapter-tool-year" style="color:#50e6ff">"2020"</span>
                    <span class="chapter-tool-name" style="color:#50e6ff">"Bicep"</span>
                    <span class="chapter-tool-desc">"Azure-native DSL, compiles to ARM, no state file. Correct decision."</span>
                </div>
            </div>
        </div>
    }
}
