use leptos::*;
use crate::i18n::get_translation;

const CODE: &str = r##"bundle agent ensure_app_config {
  vars:
    "config_path" string => "/etc/myapp/config.conf";

  files:
    "$(config_path)"
      create        => "true",
      perms         => mog("644", "root", "root"),
      content       => "# Managed by CFEngine
app_mode   = production
log_level  = warn
max_conn   = 100";
}

bundle agent main {
  methods:
    "app config" usebundle => ensure_app_config;
}"##;

#[component]
pub fn CfengineSlide() -> impl IntoView {
    let t = get_translation;
    let show_code = create_rw_signal(false);

    view! {
        <div class="slide tool-slide" style="--tool-color: #e07b39">
            <div class="slide-header" style="border-top: 4px solid #e07b39">
                <div class="slide-header-left">
                    <h2 class="slide-title">{move || t("cfengine.title")}</h2>
                    <span class="slide-subtitle">{move || t("cfengine.subtitle")}</span>
                </div>
                <div class="slide-header-right">
                    <span class="slide-year-badge" style="background: #e07b39">"1993"</span>
                    <span class="slide-category-badge">{move || t("slide.config_management")}</span>
                    <span class="slide-creator">{move || t("cfengine.creator")}</span>
                </div>
            </div>

            <div class="slide-tab-bar">
                <button class="slide-tab"
                    class:active=move || !show_code.get()
                    on:click=move |_| show_code.set(false)>
                    "Lore"
                </button>
                <button class="slide-tab"
                    class:active=move || show_code.get()
                    on:click=move |_| show_code.set(true)>
                    "Code"
                </button>
            </div>

            <div class="slide-body" style:display=move || if show_code.get() { "none" } else { "" }>
                <div class="aspects-col">
                    <h3 class="col-heading">{move || t("slide.key_aspects")}</h3>
                    <ul class="aspects-list">
                        <li>{move || t("cfengine.aspect.1")}</li>
                        <li>{move || t("cfengine.aspect.2")}</li>
                        <li>{move || t("cfengine.aspect.3")}</li>
                        <li>{move || t("cfengine.aspect.4")}</li>
                        <li>{move || t("cfengine.aspect.5")}</li>
                        <li>{move || t("cfengine.aspect.6")}</li>
                        <li>{move || t("cfengine.aspect.7")}</li>
                    </ul>
                </div>

                <div class="diagram-col">
                    <h3 class="col-heading">{move || t("cfengine.diagram.title")}</h3>
                    <svg viewBox="0 0 500 310" class="flow-diagram" xmlns="http://www.w3.org/2000/svg">
                        <rect width="500" height="310" fill="#111827" rx="10"/>
                        // CF Policy Hub box
                        <rect x="130" y="20" width="240" height="52" rx="8"
                              fill="#1e2d45" stroke="#e07b39" stroke-width="1.5"/>
                        <text x="250" y="42" text-anchor="middle" class="diag-box-title" fill="#e07b39">
                            "CF Policy Hub"
                        </text>
                        <text x="250" y="60" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "(cf-serverd)"
                        </text>
                        // Arrow down
                        <line x1="250" y1="72" x2="250" y2="115"
                              stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="244,110 256,110 250,120"
                                 fill="#4a5580"/>
                        // Arrow label
                        <text x="260" y="100" class="diag-arrow-label" fill="#6b7cb8">
                            "HTTPS (agent polls)"
                        </text>
                        // cf-agent box
                        <rect x="100" y="120" width="300" height="52" rx="8"
                              fill="#1e2d45" stroke="#e07b39" stroke-width="1.5"/>
                        <text x="250" y="142" text-anchor="middle" class="diag-box-title" fill="#e07b39">
                            "cf-agent"
                        </text>
                        <text x="250" y="160" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "(on every managed host)"
                        </text>
                        // Arrow down
                        <line x1="250" y1="172" x2="250" y2="208"
                              stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="244,203 256,203 250,213"
                                 fill="#4a5580"/>
                        // Convergence box
                        <rect x="60" y="213" width="380" height="76" rx="8"
                              fill="#162030" stroke="#4a5580" stroke-width="1.5"/>
                        <text x="250" y="234" text-anchor="middle" class="diag-box-sub" fill="#a0aec0">
                            "Evaluate local state"
                        </text>
                        <text x="250" y="254" text-anchor="middle" class="diag-box-sub" fill="#a0aec0">
                            "Compare against policy (promises)"
                        </text>
                        <text x="250" y="274" text-anchor="middle" class="diag-box-sub" fill="#68d391">
                            "✓ Remediate drift,apply changes"
                        </text>
                        // Feedback arrow back up (right side)
                        <path d="M 440 250 Q 480 250 480 200 Q 480 96 440 96"
                              fill="none" stroke="#4a5580" stroke-width="1.5" stroke-dasharray="5,3"/>
                        <polygon points="445,100 435,96 440,108" fill="#4a5580"/>
                        <text x="483" y="185" class="diag-arrow-label" fill="#6b7cb8"
                              transform="rotate(90, 483, 185)">"repeat every 5 min"</text>
                    </svg>
                </div>
            </div>

            <div class="code-section code-section-expanded" style:display=move || if !show_code.get() { "none" } else { "" }>
                <h4 class="code-caption">{move || t("cfengine.code.caption")}</h4>
                <pre class="code-block"><code class="language-ruby">{CODE}</code></pre>
            </div>
        </div>
    }
}
