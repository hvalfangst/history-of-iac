use leptos::*;
use crate::i18n::get_translation;

const CODE: &str = r#"@description('Storage account name (3-24 lowercase alphanumeric chars)')
@minLength(3)
@maxLength(24)
param storageAccountName string

@description('Azure region for deployment')
param location string = resourceGroup().location

@description('Storage SKU')
@allowed(['Standard_LRS', 'Standard_GRS', 'Standard_ZRS', 'Premium_LRS'])
param sku string = 'Standard_LRS'

resource storageAccount 'Microsoft.Storage/storageAccounts@2023-01-01' = {
  name:     storageAccountName
  location: location
  sku: {
    name: sku
  }
  kind: 'StorageV2'
  properties: {
    supportsHttpsTrafficOnly: true
    minimumTlsVersion:        'TLS1_2'
    allowBlobPublicAccess:    false
  }
}

output storageAccountId   string = storageAccount.id
output storageAccountName string = storageAccount.name"#;

#[component]
pub fn BicepSlide() -> impl IntoView {
    let t = get_translation;
    let show_code = create_rw_signal(false);

    view! {
        <div class="slide tool-slide" style="--tool-color: #50e6ff">
            <div class="slide-header" style="border-top: 4px solid #50e6ff">
                <div class="slide-header-left">
                    <h2 class="slide-title">{move || t("bicep.title")}</h2>
                    <span class="slide-subtitle">{move || t("bicep.subtitle")}</span>
                </div>
                <div class="slide-header-right">
                    <span class="slide-year-badge" style="background: #0078d4">"2020"</span>
                    <span class="slide-category-badge">{move || t("slide.cloud_provisioning")}</span>
                    <span class="slide-creator">{move || t("bicep.creator")}</span>
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
                        <li>{move || t("bicep.aspect.1")}</li>
                        <li>{move || t("bicep.aspect.2")}</li>
                        <li>{move || t("bicep.aspect.3")}</li>
                        <li>{move || t("bicep.aspect.4")}</li>
                        <li>{move || t("bicep.aspect.5")}</li>
                        <li>{move || t("bicep.aspect.6")}</li>
                    </ul>
                </div>

                <div class="diagram-col">
                    <h3 class="col-heading">{move || t("bicep.diagram.title")}</h3>
                    <svg viewBox="0 0 500 320" class="flow-diagram" xmlns="http://www.w3.org/2000/svg">
                        <rect width="500" height="320" fill="#111827" rx="10"/>

                        // Bicep source
                        <rect x="145" y="12" width="210" height="42" rx="8"
                              fill="#1e2d45" stroke="#50e6ff" stroke-width="1.5"/>
                        <text x="250" y="30" text-anchor="middle" class="diag-box-title" fill="#50e6ff">
                            "main.bicep"
                        </text>
                        <text x="250" y="47" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "Bicep DSL source file"
                        </text>

                        // Arrow: bicep build
                        <line x1="250" y1="54" x2="250" y2="90"
                              stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="244,85 256,85 250,95" fill="#4a5580"/>
                        <text x="260" y="78" class="diag-arrow-label" fill="#6b7cb8">
                            "bicep build (or az deployment…)"
                        </text>

                        // ARM JSON
                        <rect x="130" y="95" width="240" height="42" rx="8"
                              fill="#1e2a3a" stroke="#0078d4" stroke-width="1.5"/>
                        <text x="250" y="113" text-anchor="middle" class="diag-box-title" fill="#0078d4">
                            "azuredeploy.json"
                        </text>
                        <text x="250" y="130" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "ARM JSON (1-to-1 transpile)"
                        </text>

                        // Arrow: deploy
                        <line x1="250" y1="137" x2="250" y2="172"
                              stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="244,167 256,167 250,177" fill="#4a5580"/>
                        <text x="260" y="160" class="diag-arrow-label" fill="#6b7cb8">
                            "same ARM deployment flow"
                        </text>

                        // Azure Resource Manager
                        <rect x="70" y="177" width="360" height="58" rx="8"
                              fill="#1e2d45" stroke="#0078d4" stroke-width="1.5"/>
                        <text x="250" y="197" text-anchor="middle" class="diag-box-title" fill="#0078d4">
                            "Azure Resource Manager"
                        </text>
                        <text x="250" y="215" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "Authenticate → validate schema"
                        </text>
                        <text x="250" y="228" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "resolve dependencies → provision"
                        </text>

                        // Resources
                        <line x1="250" y1="235" x2="250" y2="258" stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="244,253 256,253 250,263" fill="#4a5580"/>

                        <rect x="60" y="263" width="380" height="36" rx="6"
                              fill="#162030" stroke="#50e6ff" stroke-width="1"/>
                        <text x="250" y="281" text-anchor="middle" class="diag-box-sub" fill="#68d391">
                            "Azure Resources provisioned / reconciled"
                        </text>
                        <text x="250" y="294" text-anchor="middle" class="diag-box-sub" fill="#6b7cb8">
                            "No state file:stateless from developer perspective"
                        </text>
                    </svg>
                </div>
            </div>

            <div class="code-section code-section-expanded" style:display=move || if !show_code.get() { "none" } else { "" }>
                <h4 class="code-caption">{move || t("bicep.code.caption")}</h4>
                <pre class="code-block"><code class="language-typescript">{CODE}</code></pre>
            </div>
        </div>
    }
}
