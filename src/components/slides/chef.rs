use leptos::*;
use crate::i18n::get_translation;

const CODE: &str = r#"package 'apache2' do
  action :install
end

service 'apache2' do
  action [:enable, :start]
end

template '/etc/apache2/sites-available/myapp.conf' do
  source   'myapp.conf.erb'
  owner    'root'
  group    'root'
  mode     '0644'
  variables(
    server_name: node['myapp']['server_name'],
    doc_root:    node['myapp']['doc_root']
  )
  notifies :restart, 'service[apache2]', :delayed
end"#;

#[component]
pub fn ChefSlide() -> impl IntoView {
    let t = get_translation;
    let show_code = create_rw_signal(false);

    view! {
        <div class="slide tool-slide" style="--tool-color: #e74c3c">
            <div class="slide-header" style="border-top: 4px solid #e74c3c">
                <div class="slide-header-left">
                    <h2 class="slide-title">{move || t("chef.title")}</h2>
                    <span class="slide-subtitle">{move || t("chef.subtitle")}</span>
                </div>
                <div class="slide-header-right">
                    <span class="slide-year-badge" style="background: #e74c3c">"2009"</span>
                    <span class="slide-category-badge">{move || t("slide.config_management")}</span>
                    <span class="slide-creator">{move || t("chef.creator")}</span>
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
                        <li>{move || t("chef.aspect.1")}</li>
                        <li>{move || t("chef.aspect.2")}</li>
                        <li>{move || t("chef.aspect.3")}</li>
                        <li>{move || t("chef.aspect.4")}</li>
                        <li>{move || t("chef.aspect.5")}</li>
                        <li>{move || t("chef.aspect.6")}</li>
                    </ul>
                </div>

                <div class="diagram-col">
                    <h3 class="col-heading">{move || t("chef.diagram.title")}</h3>
                    <svg viewBox="0 0 500 340" class="flow-diagram" xmlns="http://www.w3.org/2000/svg">
                        <rect width="500" height="340" fill="#111827" rx="10"/>

                        // Workstation
                        <rect x="20" y="18" width="155" height="58" rx="8"
                              fill="#1e2d45" stroke="#e74c3c" stroke-width="1.5"/>
                        <text x="97" y="38" text-anchor="middle" class="diag-box-title" fill="#e74c3c">
                            "Workstation"
                        </text>
                        <text x="97" y="54" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "write recipe (Ruby)"
                        </text>
                        <text x="97" y="68" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "knife upload cookbook"
                        </text>

                        // Arrow
                        <line x1="175" y1="47" x2="218" y2="47"
                              stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="213,42 225,47 213,52" fill="#4a5580"/>

                        // Chef Server
                        <rect x="225" y="10" width="250" height="74" rx="8"
                              fill="#1e2d45" stroke="#e74c3c" stroke-width="1.5"/>
                        <text x="350" y="30" text-anchor="middle" class="diag-box-title" fill="#e74c3c">
                            "Chef Server"
                        </text>
                        <text x="350" y="48" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "cookbooks, roles, environments"
                        </text>
                        <text x="350" y="64" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "data bags, node objects"
                        </text>
                        <text x="350" y="78" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "node run-lists"
                        </text>

                        // Arrow down
                        <line x1="350" y1="84" x2="350" y2="128"
                              stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="344,123 356,123 350,133" fill="#4a5580"/>
                        <text x="360" y="112" class="diag-arrow-label" fill="#6b7cb8">
                            "HTTPS"
                        </text>

                        // chef-client
                        <rect x="225" y="133" width="250" height="58" rx="8"
                              fill="#1e2d45" stroke="#e74c3c" stroke-width="1.5"/>
                        <text x="350" y="153" text-anchor="middle" class="diag-box-title" fill="#e74c3c">
                            "chef-client"
                        </text>
                        <text x="350" y="171" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "fetch run-list → compile resources"
                        </text>
                        <text x="350" y="183" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "(on managed node)"
                        </text>

                        // Arrow down
                        <line x1="350" y1="191" x2="350" y2="228"
                              stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="344,223 356,223 350,233" fill="#4a5580"/>

                        // Converge phase
                        <rect x="225" y="233" width="250" height="58" rx="8"
                              fill="#162030" stroke="#4a5580" stroke-width="1.5"/>
                        <text x="350" y="253" text-anchor="middle" class="diag-box-sub" fill="#68d391">
                            "Converge phase:run resource providers"
                        </text>
                        <text x="350" y="270" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "(install pkg, write file, start svc…)"
                        </text>
                        <text x="350" y="285" text-anchor="middle" class="diag-box-sub" fill="#6b7cb8">
                            "Report result back to Chef Server"
                        </text>
                    </svg>
                </div>
            </div>

            <div class="code-section code-section-expanded" style:display=move || if !show_code.get() { "none" } else { "" }>
                <h4 class="code-caption">{move || t("chef.code.caption")}</h4>
                <pre class="code-block"><code class="language-ruby">{CODE}</code></pre>
            </div>
        </div>
    }
}
