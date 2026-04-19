use leptos::*;

#[component]
pub fn PreCmSlide() -> impl IntoView {
    view! {
        <div class="slide what-iac-slide">
            <h2 class="what-iac-main-title">"Early Computing"</h2>

            // ── Era banner ────────────────────────────────────────────────
            <div class="what-era-bar">
                <span class="what-era-badge">"1960s to early 1990s"</span>
                <span class="what-era-sep">"|"</span>
                <span class="what-era-text">"Mainframes and early networks"</span>
                <span class="what-era-sep">"|"</span>
                <span class="what-era-text">"Manual administration"</span>
            </div>

            // ── Diagram ───────────────────────────────────────────────────
            <div class="pre-cm-body">

                // ── Flow diagram ──────────────────────────────────────────
                <div class="pre-cm-diagram">
                    <svg viewBox="0 0 460 380" class="flow-diagram" xmlns="http://www.w3.org/2000/svg">
                        <rect width="460" height="380" fill="#111827" rx="10"/>

                        // Ops Engineer
                        <rect x="155" y="16" width="150" height="44" rx="8"
                              fill="#1e2d45" stroke="#6b7cb8" stroke-width="1.5"/>
                        <text x="230" y="35" text-anchor="middle" class="diag-box-title" fill="#a0aec0">
                            "Ops Engineer"
                        </text>
                        <text x="230" y="52" text-anchor="middle" class="diag-box-sub" fill="#6b7cb8">
                            "remembers the steps"
                        </text>

                        // Arrow down
                        <line x1="230" y1="60" x2="230" y2="92"
                              stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="224,87 236,87 230,97" fill="#4a5580"/>
                        <text x="240" y="80" class="diag-arrow-label" fill="#6b7cb8">"SSH"</text>

                        // Server 1
                        <rect x="30" y="97" width="130" height="44" rx="8"
                              fill="#1e2d45" stroke="#e74c3c" stroke-width="1.5"/>
                        <text x="95" y="116" text-anchor="middle" class="diag-box-title" fill="#e74c3c">
                            "Server A"
                        </text>
                        <text x="95" y="133" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "manually configured"
                        </text>

                        // Server 2
                        <rect x="165" y="97" width="130" height="44" rx="8"
                              fill="#1e2d45" stroke="#e74c3c" stroke-width="1.5"/>
                        <text x="230" y="116" text-anchor="middle" class="diag-box-title" fill="#e74c3c">
                            "Server B"
                        </text>
                        <text x="230" y="133" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "slightly different"
                        </text>

                        // Server 3
                        <rect x="300" y="97" width="130" height="44" rx="8"
                              fill="#1e2d45" stroke="#e74c3c" stroke-width="1.5"/>
                        <text x="365" y="116" text-anchor="middle" class="diag-box-title" fill="#e74c3c">
                            "Server C"
                        </text>
                        <text x="365" y="133" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "nobody knows what"
                        </text>

                        // Fan lines from arrow to servers
                        <line x1="230" y1="97" x2="95"  y2="97" stroke="#4a5580" stroke-width="1"/>
                        <line x1="230" y1="97" x2="365" y2="97" stroke="#4a5580" stroke-width="1"/>

                        // Snowflake label
                        <text x="230" y="162" text-anchor="middle" class="diag-box-sub" fill="#fc8181">
                            "Snowflake servers: none are alike"
                        </text>

                        // Divider
                        <line x1="30" y1="178" x2="430" y2="178"
                              stroke="#2d3f6b" stroke-width="1" stroke-dasharray="4,3"/>

                        // Problems box
                        <rect x="30" y="188" width="400" height="84" rx="8"
                              fill="#1a0e0e" stroke="#e74c3c" stroke-width="1"/>
                        <text x="230" y="207" text-anchor="middle" class="diag-box-title" fill="#fc8181">
                            "Consequences"
                        </text>
                        <text x="230" y="226" text-anchor="middle" class="diag-box-sub" fill="#a0aec0">
                            "New environment: days or weeks of manual work"
                        </text>
                        <text x="230" y="244" text-anchor="middle" class="diag-box-sub" fill="#a0aec0">
                            "Config drift: prod and test diverge silently"
                        </text>
                        <text x="230" y="262" text-anchor="middle" class="diag-box-sub" fill="#a0aec0">
                            "\"Works on my machine\" is the rule, not the exception"
                        </text>

                        // Divider
                        <line x1="30" y1="284" x2="430" y2="284"
                              stroke="#2d3f6b" stroke-width="1" stroke-dasharray="4,3"/>

                        // Word doc / wiki
                        <rect x="30" y="294" width="190" height="68" rx="8"
                              fill="#162030" stroke="#4a5580" stroke-width="1"/>
                        <text x="125" y="314" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "Word document / wiki"
                        </text>
                        <text x="125" y="332" text-anchor="middle" class="diag-box-sub" fill="#6b7cb8">
                            "setup instructions"
                        </text>
                        <text x="125" y="350" text-anchor="middle" class="diag-box-sub" fill="#fc8181">
                            "always out of date"
                        </text>

                        // No version control
                        <rect x="240" y="294" width="190" height="68" rx="8"
                              fill="#162030" stroke="#4a5580" stroke-width="1"/>
                        <text x="335" y="314" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "No version control"
                        </text>
                        <text x="335" y="332" text-anchor="middle" class="diag-box-sub" fill="#6b7cb8">
                            "no rollback"
                        </text>
                        <text x="335" y="350" text-anchor="middle" class="diag-box-sub" fill="#fc8181">
                            "no history"
                        </text>
                    </svg>
                </div>
            </div>

        </div>
    }
}
