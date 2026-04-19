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
                    <li>"Describe the end state. The tool figures out the steps."</li>
                    <li>"Running it twice does the same thing as running it once. This matters."</li>
                    <li>"One policy file, a thousand servers. That's the pitch."</li>
                    <li>"It's in git. Review it, revert it, blame it. Same workflow as your code."</li>
                </ul>
            </div>

            // ── Tool list ─────────────────────────────────────────────────
            <div class="chapter-tool-list">
                <div class="chapter-tool-row" style="border-left: 4px solid #e07b39">
                    <span class="chapter-tool-year" style="color:#e07b39">"1993"</span>
                    <span class="chapter-tool-name" style="color:#e07b39">"CFEngine"</span>
                    <span class="chapter-tool-desc">"The original. Pull model, formal semantics, built at UiO."</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #f5a623">
                    <span class="chapter-tool-year" style="color:#f5a623">"2005"</span>
                    <span class="chapter-tool-name" style="color:#f5a623">"Puppet"</span>
                    <span class="chapter-tool-desc">"Custom DSL that felt clever in 2005. Agent/master, idempotent."</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #e74c3c">
                    <span class="chapter-tool-year" style="color:#e74c3c">"2009"</span>
                    <span class="chapter-tool-name" style="color:#e74c3c">"Chef"</span>
                    <span class="chapter-tool-desc">"Ruby all the way down. Testing story was genuinely good."</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #c0392b">
                    <span class="chapter-tool-year" style="color:#c0392b">"2012"</span>
                    <span class="chapter-tool-name" style="color:#c0392b">"Ansible"</span>
                    <span class="chapter-tool-desc">"No agent, no setup. SSH and run. Won by being simpler than everything else."</span>
                </div>
            </div>
        </div>
    }
}
