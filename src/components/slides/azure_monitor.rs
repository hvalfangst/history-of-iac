use leptos::*;
use crate::i18n::get_translation;

// ── Placeholder log lines for each resource ────────────────────────────────

const FA_HTTP_LOGS: &[(&str, &str, &str)] = &[
    ("INFO",  "2024-03-04 10:00:01", "Function 'HttpSend' triggered,method: POST"),
    ("INFO",  "2024-03-04 10:00:01", "Request body parsed: { \"text\": \"Hello IaC world\" }"),
    ("INFO",  "2024-03-04 10:00:01", "Writing blob to container 'input-queue' ..."),
    ("INFO",  "2024-03-04 10:00:02", "Blob written: input-queue/2024-03-04T100001Z_a3f9.txt (18 bytes)"),
    ("INFO",  "2024-03-04 10:00:02", "HTTP 200 OK returned to caller"),
    ("INFO",  "2024-03-04 10:00:14", "Function 'HttpSend' triggered,method: POST"),
    ("INFO",  "2024-03-04 10:00:14", "Request body parsed: { \"text\": \"Bicep is stateless\" }"),
    ("INFO",  "2024-03-04 10:00:15", "Blob written: input-queue/2024-03-04T100014Z_b72c.txt (19 bytes)"),
    ("INFO",  "2024-03-04 10:00:15", "HTTP 200 OK returned to caller"),
    ("WARN",  "2024-03-04 10:00:28", "Function 'HttpSend' triggered,method: GET (unexpected)"),
    ("WARN",  "2024-03-04 10:00:28", "Returning 405 Method Not Allowed"),
    ("INFO",  "2024-03-04 10:00:44", "Function 'HttpSend' triggered,method: POST"),
    ("INFO",  "2024-03-04 10:00:44", "Request body parsed: { \"text\": \"ARM compiles from Bicep\" }"),
    ("INFO",  "2024-03-04 10:00:45", "Blob written: input-queue/2024-03-04T100044Z_d18e.txt (22 bytes)"),
];

const STORAGE_BLOBS: &[(&str, &str, &str)] = &[
    ("input-queue/2024-03-04T100001Z_a3f9.txt", "18 B",  "2024-03-04 10:00:02"),
    ("input-queue/2024-03-04T100014Z_b72c.txt", "19 B",  "2024-03-04 10:00:15"),
    ("input-queue/2024-03-04T100044Z_d18e.txt", "22 B",  "2024-03-04 10:00:45"),
    ("input-queue/2024-03-04T100113Z_f04a.txt", "15 B",  "2024-03-04 10:01:14"),
    ("input-queue/2024-03-04T100230Z_c99b.txt", "28 B",  "2024-03-04 10:02:31"),
];

const FA_BLOB_LOGS: &[(&str, &str, &str)] = &[
    ("INFO",  "2024-03-04 10:00:02", "BlobTrigger fired for: input-queue/2024-03-04T100001Z_a3f9.txt"),
    ("INFO",  "2024-03-04 10:00:03", "Read blob content: \"Hello IaC world\""),
    ("INFO",  "2024-03-04 10:00:03", "Upserting document to Cosmos DB container 'items' ..."),
    ("INFO",  "2024-03-04 10:00:04", "Document created,id: a3f9, partition: /id"),
    ("INFO",  "2024-03-04 10:00:15", "BlobTrigger fired for: input-queue/2024-03-04T100014Z_b72c.txt"),
    ("INFO",  "2024-03-04 10:00:16", "Read blob content: \"Bicep is stateless\""),
    ("INFO",  "2024-03-04 10:00:16", "Document created,id: b72c, partition: /id"),
    ("INFO",  "2024-03-04 10:00:45", "BlobTrigger fired for: input-queue/2024-03-04T100044Z_d18e.txt"),
    ("INFO",  "2024-03-04 10:00:46", "Read blob content: \"ARM compiles from Bicep\""),
    ("INFO",  "2024-03-04 10:00:46", "Document created,id: d18e, partition: /id"),
    ("ERROR", "2024-03-04 10:01:02", "BlobTrigger: transient Cosmos throttle (429),retrying in 1 s"),
    ("INFO",  "2024-03-04 10:01:03", "Retry succeeded,document created,id: f04a"),
];

const COSMOS_ITEMS: &[(&str, &str, &str, &str)] = &[
    ("a3f9", "Hello IaC world",        "2024-03-04T10:00:04Z", "18"),
    ("b72c", "Bicep is stateless",     "2024-03-04T10:00:16Z", "19"),
    ("d18e", "ARM compiles from Bicep","2024-03-04T10:00:46Z", "22"),
    ("f04a", "IaC enables repeatability","2024-03-04T10:01:03Z","15"),
    ("c99b", "Terraform uses state files","2024-03-04T10:02:31Z","28"),
];

// ── Tab enum ───────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
enum MonitorTab { HttpTrigger, Storage, BlobTrigger, CosmosDb }

// ── Component ──────────────────────────────────────────────────────────────

#[component]
pub fn AzureMonitorSlide() -> impl IntoView {
    let t = get_translation;

    let active_tab: RwSignal<MonitorTab> = create_rw_signal(MonitorTab::HttpTrigger);

    // Simulate input value (not wired to backend,placeholder UI)
    let input_val: RwSignal<String> = create_rw_signal(String::new());

    // Fake "last polled" counter driven by a reactive memo of the tab
    let cosmos_note = move || {
        if active_tab.get() == MonitorTab::CosmosDb {
            "Last polled: just now,5 items"
        } else {
            ""
        }
    };

    view! {
        <div class="slide azure-monitor-slide">

            // ── Header ────────────────────────────────────────────────────────
            <div class="monitor-header">
                <div class="monitor-header-left">
                    <span class="monitor-icon">"📡"</span>
                    <div>
                        <h2 class="monitor-title">{move || t("azure.monitor.title")}</h2>
                        <p class="monitor-subtitle">{move || t("azure.monitor.subtitle")}</p>
                    </div>
                </div>
                <div class="monitor-status-badges">
                    <span class="status-badge status-ok">"● FA HTTP  OK"</span>
                    <span class="status-badge status-ok">"● Storage  OK"</span>
                    <span class="status-badge status-ok">"● FA Blob  OK"</span>
                    <span class="status-badge status-ok">"● Cosmos   OK"</span>
                </div>
            </div>

            // ── Simulated input row ───────────────────────────────────────────
            <div class="monitor-input-row">
                <label class="monitor-input-label">{move || t("azure.monitor.input.label")}</label>
                <input
                    class="monitor-text-input"
                    type="text"
                    placeholder="e.g. \"Bicep deploys via ARM\""
                    prop:value=move || input_val.get()
                    on:input=move |ev| input_val.set(event_target_value(&ev))
                />
                <button class="monitor-send-btn" on:click=move |_| {
                    // placeholder: no real backend call
                    input_val.set(String::new());
                }>
                    {move || t("azure.monitor.send.btn")}
                </button>
                <span class="monitor-input-note">{move || t("azure.monitor.input.note")}</span>
            </div>

            // ── Tab bar ───────────────────────────────────────────────────────
            <div class="monitor-tab-bar">
                <button
                    class=move || if active_tab.get() == MonitorTab::HttpTrigger
                        { "monitor-tab monitor-tab-active" } else { "monitor-tab" }
                    on:click=move |_| active_tab.set(MonitorTab::HttpTrigger)>
                    <span class="tab-dot" style="background:#0078d4"/>
                    "⚡ HTTP Trigger"
                </button>
                <button
                    class=move || if active_tab.get() == MonitorTab::Storage
                        { "monitor-tab monitor-tab-active" } else { "monitor-tab" }
                    on:click=move |_| active_tab.set(MonitorTab::Storage)>
                    <span class="tab-dot" style="background:#50e6ff"/>
                    "🗄 Blob Storage"
                </button>
                <button
                    class=move || if active_tab.get() == MonitorTab::BlobTrigger
                        { "monitor-tab monitor-tab-active" } else { "monitor-tab" }
                    on:click=move |_| active_tab.set(MonitorTab::BlobTrigger)>
                    <span class="tab-dot" style="background:#0078d4"/>
                    "⚡ Blob Trigger"
                </button>
                <button
                    class=move || if active_tab.get() == MonitorTab::CosmosDb
                        { "monitor-tab monitor-tab-active" } else { "monitor-tab" }
                    on:click=move |_| active_tab.set(MonitorTab::CosmosDb)>
                    <span class="tab-dot" style="background:#a855f7"/>
                    "🌌 Cosmos DB"
                </button>
            </div>

            // ── Tab panels ────────────────────────────────────────────────────
            <div class="monitor-panel">

                // ── FA HTTP TRIGGER panel ─────────────────────────────────────
                {move || if active_tab.get() == MonitorTab::HttpTrigger {
                    view! {
                        <div class="log-panel">
                            <div class="log-panel-header">
                                <span class="log-panel-title">"fa-http-trigger  ·  Function: HttpSend"</span>
                                <span class="log-panel-note">"Application Insights · Live tail (placeholder)"</span>
                            </div>
                            <div class="log-scroll">
                                {FA_HTTP_LOGS.iter().map(|(level, ts, msg)| {
                                    let level_class = match *level {
                                        "WARN"  => "log-line log-warn",
                                        "ERROR" => "log-line log-error",
                                        _       => "log-line log-info",
                                    };
                                    view! {
                                        <div class=level_class>
                                            <span class="log-level">{*level}</span>
                                            <span class="log-ts">{*ts}</span>
                                            <span class="log-msg">{*msg}</span>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                            <div class="log-footer">
                                <span class="log-footer-note">
                                    "→ Deployment: "
                                    <code>"az functionapp deployment source config-zip"</code>
                                    " or GitHub Actions"
                                </span>
                            </div>
                        </div>
                    }.into_view()
                } else { view! { <></> }.into_view() }}

                // ── STORAGE panel ─────────────────────────────────────────────
                {move || if active_tab.get() == MonitorTab::Storage {
                    view! {
                        <div class="log-panel">
                            <div class="log-panel-header">
                                <span class="log-panel-title">"sa{suffix}  ·  Blob Container: input-queue"</span>
                                <span class="log-panel-note">"Storage Explorer view (placeholder)"</span>
                            </div>
                            <div class="blob-table-wrapper">
                                <table class="blob-table">
                                    <thead>
                                        <tr>
                                            <th>"Blob Name"</th>
                                            <th>"Size"</th>
                                            <th>"Last Modified"</th>
                                            <th>"Status"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {STORAGE_BLOBS.iter().map(|(name, size, ts)| {
                                            view! {
                                                <tr>
                                                    <td class="blob-name"><code>{*name}</code></td>
                                                    <td class="blob-size">{*size}</td>
                                                    <td class="blob-ts">{*ts}</td>
                                                    <td><span class="blob-status">"✓ Processed"</span></td>
                                                </tr>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tbody>
                                </table>
                            </div>
                            <div class="log-footer">
                                <span class="log-footer-note">
                                    "→ CLI: "
                                    <code>"az storage blob list --container-name input-queue --account-name ..."</code>
                                </span>
                            </div>
                        </div>
                    }.into_view()
                } else { view! { <></> }.into_view() }}

                // ── FA BLOB TRIGGER panel ─────────────────────────────────────
                {move || if active_tab.get() == MonitorTab::BlobTrigger {
                    view! {
                        <div class="log-panel">
                            <div class="log-panel-header">
                                <span class="log-panel-title">"fa-blob-proc  ·  Function: BlobProcessor"</span>
                                <span class="log-panel-note">"Application Insights · Live tail (placeholder)"</span>
                            </div>
                            <div class="log-scroll">
                                {FA_BLOB_LOGS.iter().map(|(level, ts, msg)| {
                                    let level_class = match *level {
                                        "WARN"  => "log-line log-warn",
                                        "ERROR" => "log-line log-error",
                                        _       => "log-line log-info",
                                    };
                                    view! {
                                        <div class=level_class>
                                            <span class="log-level">{*level}</span>
                                            <span class="log-ts">{*ts}</span>
                                            <span class="log-msg">{*msg}</span>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                            <div class="log-footer">
                                <span class="log-footer-note">
                                    "→ Trigger binding: "
                                    <code>"[BlobTrigger(\"input-queue/{name}\", Connection=\"AzureWebJobsStorage\")]"</code>
                                </span>
                            </div>
                        </div>
                    }.into_view()
                } else { view! { <></> }.into_view() }}

                // ── COSMOS DB panel ────────────────────────────────────────────
                {move || if active_tab.get() == MonitorTab::CosmosDb {
                    view! {
                        <div class="log-panel">
                            <div class="log-panel-header">
                                <span class="log-panel-title">"cosmos-{suffix}  ·  DB: iacdb  ·  Container: items"</span>
                                <span class="log-panel-note">{cosmos_note()}</span>
                            </div>
                            <div class="blob-table-wrapper">
                                <table class="blob-table cosmos-table">
                                    <thead>
                                        <tr>
                                            <th>"id"</th>
                                            <th>"text"</th>
                                            <th>"_ts (UTC)"</th>
                                            <th>"bytes"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {COSMOS_ITEMS.iter().map(|(id, text, ts, size)| {
                                            view! {
                                                <tr>
                                                    <td><code class="cosmos-id">{*id}</code></td>
                                                    <td class="cosmos-text">"\""  {*text}  "\""</td>
                                                    <td class="blob-ts">{*ts}</td>
                                                    <td class="blob-size">{*size}</td>
                                                </tr>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tbody>
                                </table>
                            </div>
                            <div class="cosmos-query-row">
                                <span class="cosmos-query-label">"Query used:"</span>
                                <code class="cosmos-query">"SELECT * FROM c ORDER BY c._ts DESC OFFSET 0 LIMIT 20"</code>
                            </div>
                            <div class="log-footer">
                                <span class="log-footer-note">
                                    "→ SDK: "
                                    <code>"CosmosClient.GetContainer(\"iacdb\",\"items\").GetItemQueryIterator&lt;T&gt;(query)"</code>
                                </span>
                            </div>
                        </div>
                    }.into_view()
                } else { view! { <></> }.into_view() }}

            </div>


            // ── Guide callout ─────────────────────────────────────────────────
            <div class="monitor-guide">
                <h3 class="guide-heading">{move || t("azure.monitor.guide.title")}</h3>
                <ol class="guide-list">
                    <li>"Deploy Bicep: "<code>"az deployment group create --template-file main.bicep"</code></li>
                    <li>"Deploy FA #1 (HttpSend)"</li>
                    <li>"Deploy FA #2 (BlobProcessor)"</li>
                    <li>"Replace placeholder URL with FA #1's invoke URL"</li>
                    <li>"Connect Cosmos poll to GET /api/items"</li>
                    <li>"Enable Application Insights on both Function Apps"</li>
                </ol>
            </div>

        </div>
    }
}
