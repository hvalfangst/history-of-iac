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
                    <li>"Describe what you want. Let the tool figure out the steps."</li>
                    <li>"Safe to run repeatedly. Not a one-shot script that breaks on the second run."</li>
                    <li>"One policy file, a thousand servers. You stop SSHing into things individually."</li>
                    <li>"It lives in git. You can review it, revert it, and find out exactly who broke it."</li>
                </ul>
            </div>

            // ── Tool list ─────────────────────────────────────────────────
            <div class="chapter-tool-list">
                <div class="chapter-tool-row" style="border-left: 4px solid #e07b39">
                    <span class="chapter-tool-year" style="color:#e07b39">"1993"</span>
                    <span class="chapter-tool-name" style="color:#e07b39">"CFEngine"</span>
                    <span class="chapter-tool-desc">"The original. 1993. Still running in production somewhere right now."</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #f5a623">
                    <span class="chapter-tool-year" style="color:#f5a623">"2005"</span>
                    <span class="chapter-tool-name" style="color:#f5a623">"Puppet"</span>
                    <span class="chapter-tool-desc">"Custom DSL, agent/master, idempotent. Worked. The DSL did not age well."</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #e74c3c">
                    <span class="chapter-tool-year" style="color:#e74c3c">"2009"</span>
                    <span class="chapter-tool-name" style="color:#e74c3c">"Chef"</span>
                    <span class="chapter-tool-desc">"Everything is Ruby. Good testing story. Too many moving parts."</span>
                </div>
                <div class="chapter-tool-row" style="border-left: 4px solid #c0392b">
                    <span class="chapter-tool-year" style="color:#c0392b">"2012"</span>
                    <span class="chapter-tool-name" style="color:#c0392b">"Ansible"</span>
                    <span class="chapter-tool-desc">"No agent, no daemon. SSH in and run a playbook. That is the entire pitch."</span>
                </div>
            </div>
        </div>
    }
}
