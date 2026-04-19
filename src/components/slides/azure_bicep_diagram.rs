use leptos::*;
use crate::i18n::get_translation;

#[component]
pub fn AzureBicepDiagramSlide() -> impl IntoView {
    let t = get_translation;

    view! {
        <div class="slide azure-diagram-slide">

            // ── Header ────────────────────────────────────────────────────────
            <div class="azure-diag-header">
                <div class="azure-diag-icon">"☁"</div>
                <div>
                    <h2 class="azure-diag-title">{move || t("azure.diagram.title")}</h2>
                    <p class="azure-diag-subtitle">{move || t("azure.diagram.subtitle")}</p>
                </div>
                <div class="azure-diag-badges">
                    <span class="azure-badge bicep-badge">"Bicep"</span>
                    <span class="azure-badge az-func-badge">"Azure Functions"</span>
                    <span class="azure-badge cosmos-badge">"Cosmos DB"</span>
                </div>
            </div>

            // ── Body: SVG + steps ─────────────────────────────────────────────
            <div class="azure-diag-body">

                // ── Architecture SVG ──────────────────────────────────────────
                <div class="azure-svg-wrapper">
                    <svg viewBox="0 0 460 520" class="azure-arch-svg" xmlns="http://www.w3.org/2000/svg">

                        // ── Background ────────────────────────────────────────
                        <rect width="460" height="520" fill="#0d1117" rx="10"/>

                        // ── CLIENT box (top) ──────────────────────────────────
                        <rect x="130" y="12" width="200" height="48"
                              rx="24" fill="#1e2a3a" stroke="#3b82f6" stroke-width="1.5"/>
                        <text x="230" y="30" text-anchor="middle"
                              font="bold 11px sans-serif" fill="#93c5fd" font-size="11" font-weight="700">"CLIENT / BROWSER"</text>
                        <text x="230" y="48" text-anchor="middle"
                              font="10px sans-serif" fill="#4a6fa5" font-size="10">"text input · polls every 5 s"</text>

                        // Arrow: client → FA-HTTP
                        <line x1="230" y1="60" x2="230" y2="98"
                              stroke="#3b82f6" stroke-width="1.5" stroke-dasharray="4 3"/>
                        <polygon points="230,100 225,90 235,90" fill="#3b82f6"/>
                        <text x="245" y="82" font-size="9" fill="#4a6fa5">"HTTP POST"</text>
                        <text x="245" y="93" font-size="9" fill="#4a6fa5">"/api/send"</text>

                        // ── FA HTTP TRIGGER box ───────────────────────────────
                        <rect x="40" y="100" width="380" height="78"
                              rx="8" fill="#0f1f35" stroke="#0078d4" stroke-width="2"/>
                        <rect x="40" y="100" width="380" height="4" rx="8" fill="#0078d4"/>
                        // icon placeholder
                        <rect x="52" y="116" width="36" height="36" rx="6"
                              fill="#0078d4" opacity="0.3"/>
                        <text x="70" y="140" text-anchor="middle" font-size="18" fill="#0078d4">"⚡"</text>
                        <text x="104" y="128" font-size="12" font-weight="700" fill="#7ec8f7">"Function App #1"</text>
                        <text x="104" y="144" font-size="10" fill="#4a8fcc">"HTTP Trigger"</text>
                        <text x="104" y="158" font-size="10" fill="#3a6a8a">"POST /api/send  →  writes text as blob"</text>
                        // resource tag
                        <rect x="344" y="108" width="68" height="18" rx="9" fill="#012a4a"/>
                        <text x="378" y="121" text-anchor="middle" font-size="9" fill="#3b82f6">"fa-http-trigger"</text>

                        // Arrow: FA-HTTP → Storage
                        <line x1="230" y1="178" x2="230" y2="218"
                              stroke="#0078d4" stroke-width="1.5" stroke-dasharray="4 3"/>
                        <polygon points="230,220 225,210 235,210" fill="#0078d4"/>
                        <text x="245" y="202" font-size="9" fill="#4a6fa5">"WriteAsync()"</text>
                        <text x="245" y="213" font-size="9" fill="#4a6fa5">"→ blob"</text>

                        // ── STORAGE ACCOUNT box ───────────────────────────────
                        <rect x="40" y="220" width="380" height="78"
                              rx="8" fill="#0f1f2a" stroke="#50e6ff" stroke-width="1.5"/>
                        <rect x="40" y="220" width="380" height="4" rx="8" fill="#50e6ff"/>
                        <rect x="52" y="236" width="36" height="36" rx="6"
                              fill="#50e6ff" opacity="0.2"/>
                        <text x="70" y="260" text-anchor="middle" font-size="18" fill="#50e6ff">"🗄"</text>
                        <text x="104" y="248" font-size="12" font-weight="700" fill="#7eeeff">"Storage Account"</text>
                        <text x="104" y="264" font-size="10" fill="#3abccc">"Blob Container: input-queue"</text>
                        <text x="104" y="278" font-size="10" fill="#2a8a9a">"Each upload fires a BlobCreated event"</text>
                        <rect x="316" y="228" width="96" height="18" rx="9" fill="#012a2a"/>
                        <text x="364" y="241" text-anchor="middle" font-size="9" fill="#50e6ff">"sa{suffix}/input-queue"</text>

                        // Arrow: Storage → FA-Blob
                        <line x1="230" y1="298" x2="230" y2="338"
                              stroke="#50e6ff" stroke-width="1.5" stroke-dasharray="4 3"/>
                        <polygon points="230,340 225,330 235,330" fill="#50e6ff"/>
                        <text x="245" y="320" font-size="9" fill="#4a6fa5">"BlobTrigger"</text>
                        <text x="245" y="331" font-size="9" fill="#4a6fa5">"event"</text>

                        // ── FA BLOB TRIGGER box ────────────────────────────────
                        <rect x="40" y="340" width="380" height="78"
                              rx="8" fill="#0f1f35" stroke="#0078d4" stroke-width="2"/>
                        <rect x="40" y="340" width="380" height="4" rx="8" fill="#0078d4"/>
                        <rect x="52" y="356" width="36" height="36" rx="6"
                              fill="#0078d4" opacity="0.3"/>
                        <text x="70" y="380" text-anchor="middle" font-size="18" fill="#0078d4">"⚡"</text>
                        <text x="104" y="368" font-size="12" font-weight="700" fill="#7ec8f7">"Function App #2"</text>
                        <text x="104" y="384" font-size="10" fill="#4a8fcc">"Blob Trigger"</text>
                        <text x="104" y="398" font-size="10" fill="#3a6a8a">"Reads blob content → persists to Cosmos DB"</text>
                        <rect x="344" y="348" width="68" height="18" rx="9" fill="#012a4a"/>
                        <text x="378" y="361" text-anchor="middle" font-size="9" fill="#3b82f6">"fa-blob-proc"</text>

                        // Arrow: FA-Blob → Cosmos
                        <line x1="230" y1="418" x2="230" y2="456"
                              stroke="#0078d4" stroke-width="1.5" stroke-dasharray="4 3"/>
                        <polygon points="230,458 225,448 235,448" fill="#0078d4"/>
                        <text x="245" y="440" font-size="9" fill="#4a6fa5">"CreateItemAsync()"</text>
                        <text x="245" y="451" font-size="9" fill="#4a6fa5">"→ JSON doc"</text>

                        // ── COSMOS DB box ─────────────────────────────────────
                        <rect x="40" y="458" width="380" height="50"
                              rx="8" fill="#1a0f2e" stroke="#6e40c9" stroke-width="1.5"/>
                        <rect x="40" y="458" width="380" height="4" rx="8" fill="#6e40c9"/>
                        <text x="70" y="488" text-anchor="middle" font-size="18" fill="#a855f7">"🌌"</text>
                        <text x="104" y="479" font-size="12" font-weight="700" fill="#c4b5fd">"Cosmos DB (NoSQL)"</text>
                        <text x="104" y="496" font-size="10" fill="#7c5fb5">"Database: iacdb  ·  Container: items  ·  /id partition"</text>
                        <rect x="344" y="466" width="68" height="18" rx="9" fill="#1a0a2e"/>
                        <text x="378" y="479" text-anchor="middle" font-size="9" fill="#a855f7">"cosmos-{suffix}"</text>

                        // ── RETURN ARROW (right side): Cosmos → Client ─────────
                        <path d="M 420 483 Q 448 483 448 380 Q 448 50 420 36"
                              fill="none" stroke="#6e40c9" stroke-width="1.5" stroke-dasharray="5 3"/>
                        <polygon points="420,36 428,46 415,44" fill="#6e40c9"/>
                        <text x="452" y="270" font-size="9" fill="#6e40c9"
                              writing-mode="tb" text-anchor="middle">"GET /api/items  ·  every 5 s"</text>

                    </svg>
                </div>

                // ── Step titles ───────────────────────────────────────────
                <div class="azure-steps">
                    <h3 class="azure-steps-heading">{move || t("azure.diagram.flow.title")}</h3>
                    <ol class="azure-step-list">
                        <li class="azure-step">
                            <span class="step-num" style="background:#0078d4">"1"</span>
                            <span class="step-title">{move || t("azure.step.1.title")}</span>
                        </li>
                        <li class="azure-step">
                            <span class="step-num" style="background:#50e6ff; color:#000">"2"</span>
                            <span class="step-title">{move || t("azure.step.2.title")}</span>
                        </li>
                        <li class="azure-step">
                            <span class="step-num" style="background:#0078d4">"3"</span>
                            <span class="step-title">{move || t("azure.step.3.title")}</span>
                        </li>
                        <li class="azure-step">
                            <span class="step-num" style="background:#6e40c9">"4"</span>
                            <span class="step-title">{move || t("azure.step.4.title")}</span>
                        </li>
                        <li class="azure-step">
                            <span class="step-num" style="background:#3b82f6">"5"</span>
                            <span class="step-title">{move || t("azure.step.5.title")}</span>
                        </li>
                    </ol>
                </div>
            </div>

        </div>
    }
}
