use leptos::*;
use crate::i18n::get_translation;

const CODE: &str = r#"terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
  }
  backend "azurerm" {
    resource_group_name  = "tfstate-rg"
    storage_account_name = "tfstateacct"
    container_name       = "tfstate"
    key                  = "prod.terraform.tfstate"
  }
}

resource "azurerm_resource_group" "main" {
  name     = var.resource_group_name
  location = var.location
}

resource "azurerm_storage_account" "main" {
  name                     = var.storage_account_name
  resource_group_name      = azurerm_resource_group.main.name
  location                 = azurerm_resource_group.main.location
  account_tier             = "Standard"
  account_replication_type = "LRS"

  tags = {
    environment = var.environment
    managed_by  = "terraform"
  }
}"#;

#[component]
pub fn TerraformSlide() -> impl IntoView {
    let t = get_translation;
    let show_code = create_rw_signal(false);

    view! {
        <div class="slide tool-slide" style="--tool-color: #7b42bc">
            <div class="slide-header" style="border-top: 4px solid #7b42bc">
                <div class="slide-header-left">
                    <h2 class="slide-title">{move || t("terraform.title")}</h2>
                    <span class="slide-subtitle">{move || t("terraform.subtitle")}</span>
                </div>
                <div class="slide-header-right">
                    <span class="slide-year-badge" style="background: #7b42bc">"2014"</span>
                    <span class="slide-category-badge">{move || t("slide.cloud_provisioning")}</span>
                    <span class="slide-creator">{move || t("terraform.creator")}</span>
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
                        <li>{move || t("terraform.aspect.1")}</li>
                        <li>{move || t("terraform.aspect.2")}</li>
                        <li>{move || t("terraform.aspect.3")}</li>
                        <li>{move || t("terraform.aspect.4")}</li>
                        <li>{move || t("terraform.aspect.5")}</li>
                        <li>{move || t("terraform.aspect.6")}</li>
                    </ul>
                </div>

                <div class="diagram-col">
                    <h3 class="col-heading">{move || t("terraform.diagram.title")}</h3>
                    <svg viewBox="0 0 500 360" class="flow-diagram" xmlns="http://www.w3.org/2000/svg">
                        <rect width="500" height="360" fill="#111827" rx="10"/>

                        // .tf files
                        <rect x="150" y="12" width="200" height="40" rx="8"
                              fill="#1e2d45" stroke="#7b42bc" stroke-width="1.5"/>
                        <text x="250" y="30" text-anchor="middle" class="diag-box-title" fill="#7b42bc">
                            "*.tf  (HCL)"
                        </text>
                        <text x="250" y="46" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "resource, variable, output, module"
                        </text>

                        // terraform init
                        <line x1="250" y1="52" x2="250" y2="82" stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="244,77 256,77 250,87" fill="#4a5580"/>
                        <rect x="130" y="87" width="240" height="36" rx="6"
                              fill="#1a1540" stroke="#7b42bc" stroke-width="1"/>
                        <text x="250" y="107" text-anchor="middle" class="diag-box-sub" fill="#a78bfa">
                            "terraform init"
                        </text>
                        <text x="250" y="118" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "download providers + modules"
                        </text>

                        // terraform plan
                        <line x1="250" y1="123" x2="250" y2="152" stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="244,147 256,147 250,157" fill="#4a5580"/>
                        <rect x="80" y="157" width="340" height="54" rx="6"
                              fill="#1a1540" stroke="#7b42bc" stroke-width="1.5"/>
                        <text x="250" y="177" text-anchor="middle" class="diag-box-title" fill="#a78bfa">
                            "terraform plan"
                        </text>
                        <text x="250" y="194" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "read state file + call provider APIs"
                        </text>
                        <text x="250" y="205" text-anchor="middle" class="diag-box-sub" fill="#68d391">
                            "diff: +add  ~change  -destroy"
                        </text>

                        // terraform apply
                        <line x1="250" y1="211" x2="250" y2="242" stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="244,237 256,237 250,247" fill="#4a5580"/>
                        <rect x="100" y="247" width="300" height="46" rx="6"
                              fill="#1a1540" stroke="#7b42bc" stroke-width="1.5"/>
                        <text x="250" y="267" text-anchor="middle" class="diag-box-title" fill="#a78bfa">
                            "terraform apply"
                        </text>
                        <text x="250" y="285" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "call provider APIs (CRUD)"
                        </text>

                        // State file update
                        <line x1="250" y1="293" x2="250" y2="318" stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="244,313 256,313 250,323" fill="#4a5580"/>
                        <rect x="110" y="323" width="280" height="30" rx="6"
                              fill="#0d1117" stroke="#4a5580" stroke-width="1"/>
                        <text x="250" y="343" text-anchor="middle" class="diag-box-sub" fill="#6b7cb8">
                            "terraform.tfstate updated"
                        </text>

                        // Feedback arrow from state back to plan
                        <path d="M 100 338 Q 50 338 50 181 Q 50 165 80 165"
                              fill="none" stroke="#4a5580" stroke-width="1.5"
                              stroke-dasharray="5,3"/>
                        <polygon points="85,159 75,165 82,173" fill="#4a5580"/>
                        <text x="35" y="260" class="diag-arrow-label" fill="#6b7cb8"
                              transform="rotate(-90, 35, 260)">"next plan reads state"</text>
                    </svg>
                </div>
            </div>

            <div class="code-section code-section-expanded" style:display=move || if !show_code.get() { "none" } else { "" }>
                <h4 class="code-caption">{move || t("terraform.code.caption")}</h4>
                <pre class="code-block"><code class="language-typescript">{CODE}</code></pre>
            </div>
        </div>
    }
}
