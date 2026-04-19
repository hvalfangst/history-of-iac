use leptos::*;
use crate::i18n::get_translation;

const CODE: &str = r#"class profile::nginx {
  package { 'nginx':
    ensure => installed,
  }

  service { 'nginx':
    ensure  => running,
    enable  => true,
    require => Package['nginx'],
  }

  file { '/etc/nginx/conf.d/app.conf':
    ensure  => file,
    owner   => 'root',
    group   => 'root',
    mode    => '0644',
    content => template('nginx/app.conf.erb'),
    notify  => Service['nginx'],
    require => Package['nginx'],
  }
}"#;

#[component]
pub fn PuppetSlide() -> impl IntoView {
    let t = get_translation;
    let show_code = create_rw_signal(false);

    view! {
        <div class="slide tool-slide" style="--tool-color: #f5a623">
            <div class="slide-header" style="border-top: 4px solid #f5a623">
                <div class="slide-header-left">
                    <h2 class="slide-title">{move || t("puppet.title")}</h2>
                    <span class="slide-subtitle">{move || t("puppet.subtitle")}</span>
                </div>
                <div class="slide-header-right">
                    <span class="slide-year-badge" style="background: #f5a623">"2005"</span>
                    <span class="slide-category-badge">{move || t("slide.config_management")}</span>
                    <span class="slide-creator">{move || t("puppet.creator")}</span>
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
                        <li>{move || t("puppet.aspect.1")}</li>
                        <li>{move || t("puppet.aspect.2")}</li>
                        <li>{move || t("puppet.aspect.3")}</li>
                        <li>{move || t("puppet.aspect.4")}</li>
                        <li>{move || t("puppet.aspect.5")}</li>
                        <li>{move || t("puppet.aspect.6")}</li>
                    </ul>
                </div>

                <div class="diagram-col">
                    <h3 class="col-heading">{move || t("puppet.diagram.title")}</h3>
                    <svg viewBox="0 0 500 330" class="flow-diagram" xmlns="http://www.w3.org/2000/svg">
                        <rect width="500" height="330" fill="#111827" rx="10"/>

                        // Workstation
                        <rect x="20" y="20" width="150" height="46" rx="8"
                              fill="#1e2d45" stroke="#f5a623" stroke-width="1.5"/>
                        <text x="95" y="40" text-anchor="middle" class="diag-box-title" fill="#f5a623">
                            "Workstation"
                        </text>
                        <text x="95" y="58" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "puppet code + git"
                        </text>

                        // Arrow right to Puppet Server
                        <line x1="170" y1="43" x2="215" y2="43"
                              stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="210,38 222,43 210,48" fill="#4a5580"/>

                        // Puppet Server
                        <rect x="222" y="10" width="246" height="68" rx="8"
                              fill="#1e2d45" stroke="#f5a623" stroke-width="1.5"/>
                        <text x="345" y="30" text-anchor="middle" class="diag-box-title" fill="#f5a623">
                            "Puppet Server"
                        </text>
                        <text x="345" y="48" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "classify node (ENC / Hiera)"
                        </text>
                        <text x="345" y="66" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "compile Catalog"
                        </text>

                        // Arrow down from server
                        <line x1="345" y1="78" x2="345" y2="120"
                              stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="339,115 351,115 345,125" fill="#4a5580"/>
                        <text x="355" y="105" class="diag-arrow-label" fill="#6b7cb8">
                            "HTTPS / 30 min"
                        </text>

                        // puppet agent
                        <rect x="222" y="125" width="246" height="46" rx="8"
                              fill="#1e2d45" stroke="#f5a623" stroke-width="1.5"/>
                        <text x="345" y="145" text-anchor="middle" class="diag-box-title" fill="#f5a623">
                            "puppet agent"
                        </text>
                        <text x="345" y="163" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "(on managed node)"
                        </text>

                        // Arrow down
                        <line x1="345" y1="171" x2="345" y2="208"
                              stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="339,203 351,203 345,213" fill="#4a5580"/>

                        // Apply resources box
                        <rect x="222" y="213" width="246" height="46" rx="8"
                              fill="#162030" stroke="#4a5580" stroke-width="1.5"/>
                        <text x="345" y="233" text-anchor="middle" class="diag-box-sub" fill="#68d391">
                            "Apply resources from Catalog"
                        </text>
                        <text x="345" y="251" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "(package, file, service, user…)"
                        </text>

                        // Report arrow back to server
                        <path d="M 222 236 Q 100 236 100 148 Q 100 60 180 44"
                              fill="none" stroke="#4a5580" stroke-width="1.5"
                              stroke-dasharray="5,3"/>
                        <polygon points="185,38 175,44 182,52" fill="#4a5580"/>
                        <text x="50" y="165" class="diag-arrow-label" fill="#6b7cb8"
                              transform="rotate(-90, 85, 165)">"send report"</text>
                    </svg>
                </div>
            </div>

            <div class="code-section code-section-expanded" style:display=move || if !show_code.get() { "none" } else { "" }>
                <h4 class="code-caption">{move || t("puppet.code.caption")}</h4>
                <pre class="code-block"><code class="language-ruby">{CODE}</code></pre>
            </div>
        </div>
    }
}
