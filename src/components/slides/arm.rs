use leptos::*;
use crate::i18n::get_translation;

const CODE: &str = r#"{
  "$schema": "https://schema.management.azure.com/schemas/2019-04-01/deploymentTemplate.json#",
  "contentVersion": "1.0.0.0",
  "parameters": {
    "storageAccountName": {
      "type": "string",
      "metadata": { "description": "Name of the storage account" }
    },
    "sku": {
      "type": "string",
      "defaultValue": "Standard_LRS",
      "allowedValues": ["Standard_LRS", "Standard_GRS", "Premium_LRS"]
    }
  },
  "resources": [
    {
      "type":       "Microsoft.Storage/storageAccounts",
      "apiVersion": "2023-01-01",
      "name":       "[parameters('storageAccountName')]",
      "location":   "[resourceGroup().location]",
      "sku":        { "name": "[parameters('sku')]" },
      "kind":       "StorageV2",
      "properties": { "supportsHttpsTrafficOnly": true }
    }
  ],
  "outputs": {
    "storageId": {
      "type":  "string",
      "value": "[resourceId('Microsoft.Storage/storageAccounts', parameters('storageAccountName'))]"
    }
  }
}"#;

#[component]
pub fn ArmSlide() -> impl IntoView {
    let t = get_translation;
    let show_code = create_rw_signal(false);

    view! {
        <div class="slide tool-slide" style="--tool-color: #0078d4">
            <div class="slide-header" style="border-top: 4px solid #0078d4">
                <div class="slide-header-left">
                    <h2 class="slide-title">{move || t("arm.title")}</h2>
                    <span class="slide-subtitle">{move || t("arm.subtitle")}</span>
                </div>
                <div class="slide-header-right">
                    <span class="slide-year-badge" style="background: #0078d4">"2014"</span>
                    <span class="slide-category-badge">{move || t("slide.cloud_provisioning")}</span>
                    <span class="slide-creator">{move || t("arm.creator")}</span>
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
                        <li>{move || t("arm.aspect.1")}</li>
                        <li>{move || t("arm.aspect.2")}</li>
                        <li>{move || t("arm.aspect.3")}</li>
                        <li>{move || t("arm.aspect.4")}</li>
                        <li>{move || t("arm.aspect.5")}</li>
                        <li>{move || t("arm.aspect.6")}</li>
                    </ul>
                </div>

                <div class="diagram-col">
                    <h3 class="col-heading">{move || t("arm.diagram.title")}</h3>
                    <svg viewBox="0 0 500 310" class="flow-diagram" xmlns="http://www.w3.org/2000/svg">
                        <rect width="500" height="310" fill="#111827" rx="10"/>

                        // ARM Template box
                        <rect x="140" y="15" width="220" height="46" rx="8"
                              fill="#1e2d45" stroke="#0078d4" stroke-width="1.5"/>
                        <text x="250" y="35" text-anchor="middle" class="diag-box-title" fill="#0078d4">
                            "azuredeploy.json"
                        </text>
                        <text x="250" y="53" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "ARM JSON template"
                        </text>

                        // Arrow
                        <line x1="250" y1="61" x2="250" y2="98"
                              stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="244,93 256,93 250,103" fill="#4a5580"/>
                        <text x="260" y="86" class="diag-arrow-label" fill="#6b7cb8">
                            "az deployment group create"
                        </text>

                        // Azure Resource Manager
                        <rect x="70" y="103" width="360" height="70" rx="8"
                              fill="#1e2d45" stroke="#0078d4" stroke-width="1.5"/>
                        <text x="250" y="123" text-anchor="middle" class="diag-box-title" fill="#0078d4">
                            "Azure Resource Manager"
                        </text>
                        <text x="250" y="141" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "Authenticate (Entra ID)"
                        </text>
                        <text x="250" y="158" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "Validate schema → resolve dependsOn"
                        </text>

                        // Fan arrows to resources
                        <line x1="250" y1="173" x2="250" y2="195"
                              stroke="#4a5580" stroke-width="1.5"/>

                        <line x1="250" y1="195" x2="80"  y2="210" stroke="#4a5580" stroke-width="1"/>
                        <line x1="250" y1="195" x2="250" y2="210" stroke="#4a5580" stroke-width="1"/>
                        <line x1="250" y1="195" x2="420" y2="210" stroke="#4a5580" stroke-width="1"/>

                        // Resource boxes
                        <rect x="15"  y="210" width="130" height="40" rx="6"
                              fill="#162030" stroke="#0078d4" stroke-width="1"/>
                        <text x="80"  y="235" text-anchor="middle" class="diag-box-sub" fill="#68d391">
                            "VirtualNetworks"
                        </text>

                        <rect x="185" y="210" width="130" height="40" rx="6"
                              fill="#162030" stroke="#0078d4" stroke-width="1"/>
                        <text x="250" y="235" text-anchor="middle" class="diag-box-sub" fill="#68d391">
                            "VirtualMachines"
                        </text>

                        <rect x="355" y="210" width="130" height="40" rx="6"
                              fill="#162030" stroke="#0078d4" stroke-width="1"/>
                        <text x="420" y="235" text-anchor="middle" class="diag-box-sub" fill="#68d391">
                            "StorageAccounts"
                        </text>

                        // State note
                        <rect x="60" y="268" width="380" height="32" rx="6"
                              fill="#0d1117" stroke="#2d3f6b" stroke-width="1"/>
                        <text x="250" y="289" text-anchor="middle" class="diag-box-sub" fill="#6b7cb8">
                            "No state file:ARM tracks deployment state in Azure"
                        </text>
                    </svg>
                </div>
            </div>

            <div class="code-section code-section-expanded" style:display=move || if !show_code.get() { "none" } else { "" }>
                <h4 class="code-caption">{move || t("arm.code.caption")}</h4>
                <pre class="code-block"><code class="language-json">{CODE}</code></pre>
            </div>
        </div>
    }
}
