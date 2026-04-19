use leptos::*;
use crate::i18n::get_translation;

const CODE: &str = r#"---
- name: Configure web servers
  hosts: webservers
  become: true

  tasks:
    - name: Install nginx
      ansible.builtin.package:
        name: nginx
        state: present

    - name: Deploy configuration
      ansible.builtin.template:
        src:   nginx.conf.j2
        dest:  /etc/nginx/nginx.conf
        owner: root
        mode:  '0644'
      notify: Restart nginx

    - name: Ensure nginx is started and enabled
      ansible.builtin.service:
        name:    nginx
        state:   started
        enabled: true

  handlers:
    - name: Restart nginx
      ansible.builtin.service:
        name:  nginx
        state: restarted"#;

#[component]
pub fn AnsibleSlide() -> impl IntoView {
    let t = get_translation;
    let show_code = create_rw_signal(false);

    view! {
        <div class="slide tool-slide" style="--tool-color: #c0392b">
            <div class="slide-header" style="border-top: 4px solid #c0392b">
                <div class="slide-header-left">
                    <h2 class="slide-title">{move || t("ansible.title")}</h2>
                    <span class="slide-subtitle">{move || t("ansible.subtitle")}</span>
                </div>
                <div class="slide-header-right">
                    <span class="slide-year-badge" style="background: #c0392b">"2012"</span>
                    <span class="slide-category-badge">{move || t("slide.config_management")}</span>
                    <span class="slide-creator">{move || t("ansible.creator")}</span>
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
                        <li>{move || t("ansible.aspect.1")}</li>
                        <li>{move || t("ansible.aspect.2")}</li>
                        <li>{move || t("ansible.aspect.3")}</li>
                        <li>{move || t("ansible.aspect.4")}</li>
                        <li>{move || t("ansible.aspect.5")}</li>
                        <li>{move || t("ansible.aspect.6")}</li>
                    </ul>
                </div>

                <div class="diagram-col">
                    <h3 class="col-heading">{move || t("ansible.diagram.title")}</h3>
                    <svg viewBox="0 0 500 320" class="flow-diagram" xmlns="http://www.w3.org/2000/svg">
                        <rect width="500" height="320" fill="#111827" rx="10"/>

                        // Control Node
                        <rect x="20" y="18" width="170" height="70" rx="8"
                              fill="#1e2d45" stroke="#c0392b" stroke-width="1.5"/>
                        <text x="105" y="38" text-anchor="middle" class="diag-box-title" fill="#c0392b">
                            "Control Node"
                        </text>
                        <text x="105" y="54" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "ansible-playbook site.yml"
                        </text>
                        <text x="105" y="70" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "reads inventory + playbook"
                        </text>
                        <text x="105" y="82" text-anchor="middle" class="diag-box-sub" fill="#6b7cb8">
                            "(no agent required)"
                        </text>

                        // SSH arrows to nodes (fan out)
                        // Arrow to Node 1
                        <line x1="190" y1="40" x2="295" y2="40"
                              stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="290,35 302,40 290,45" fill="#4a5580"/>
                        // Arrow to Node 2
                        <line x1="190" y1="53" x2="295" y2="130"
                              stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="292,126 303,132 295,140" fill="#4a5580"/>
                        // Arrow to Node 3
                        <line x1="190" y1="65" x2="295" y2="220"
                              stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="292,216 303,222 295,230" fill="#4a5580"/>

                        // SSH label
                        <text x="240" y="32" class="diag-arrow-label" fill="#6b7cb8">
                            "SSH"
                        </text>

                        // Managed Node 1
                        <rect x="302" y="18" width="176" height="46" rx="8"
                              fill="#162030" stroke="#c0392b" stroke-width="1.5"/>
                        <text x="390" y="38" text-anchor="middle" class="diag-box-title" fill="#c0392b">
                            "Node 1"
                        </text>
                        <text x="390" y="56" text-anchor="middle" class="diag-box-sub" fill="#68d391">
                            "✓ tasks applied"
                        </text>

                        // Managed Node 2
                        <rect x="302" y="108" width="176" height="46" rx="8"
                              fill="#162030" stroke="#c0392b" stroke-width="1.5"/>
                        <text x="390" y="128" text-anchor="middle" class="diag-box-title" fill="#c0392b">
                            "Node 2"
                        </text>
                        <text x="390" y="146" text-anchor="middle" class="diag-box-sub" fill="#68d391">
                            "✓ tasks applied"
                        </text>

                        // Managed Node 3
                        <rect x="302" y="198" width="176" height="46" rx="8"
                              fill="#162030" stroke="#c0392b" stroke-width="1.5"/>
                        <text x="390" y="218" text-anchor="middle" class="diag-box-title" fill="#c0392b">
                            "Node N"
                        </text>
                        <text x="390" y="236" text-anchor="middle" class="diag-box-sub" fill="#68d391">
                            "✓ tasks applied"
                        </text>

                        // Python module note at bottom
                        <rect x="20" y="270" width="460" height="36" rx="6"
                              fill="#0d1117" stroke="#2d3f6b" stroke-width="1"/>
                        <text x="250" y="286" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "Python interpreter invoked per task on target,no persistent agent"
                        </text>
                        <text x="250" y="300" text-anchor="middle" class="diag-box-sub" fill="#6b7cb8">
                            "Results: ok / changed / failed,reported back to control node"
                        </text>
                    </svg>
                </div>
            </div>

            <div class="code-section code-section-expanded" style:display=move || if !show_code.get() { "none" } else { "" }>
                <h4 class="code-caption">{move || t("ansible.code.caption")}</h4>
                <pre class="code-block"><code class="language-yaml">{CODE}</code></pre>
            </div>
        </div>
    }
}
