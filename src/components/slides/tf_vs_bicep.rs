use leptos::*;
use crate::i18n::get_translation;

// (dimension_key, tf_value_key, bi_value_key)
const DIMENSIONS: &[(&str, &str, &str)] = &[
    ("comparison.dimension.scope",       "comparison.tf.scope",       "comparison.bi.scope"),
    ("comparison.dimension.state",       "comparison.tf.state",       "comparison.bi.state"),
    ("comparison.dimension.language",    "comparison.tf.language",    "comparison.bi.language"),
    ("comparison.dimension.plan",        "comparison.tf.plan",        "comparison.bi.plan"),
    ("comparison.dimension.idempotency", "comparison.tf.idempotency", "comparison.bi.idempotency"),
    ("comparison.dimension.secrets",     "comparison.tf.secrets",     "comparison.bi.secrets"),
    ("comparison.dimension.drift",       "comparison.tf.drift",       "comparison.bi.drift"),
    ("comparison.dimension.rollback",    "comparison.tf.rollback",    "comparison.bi.rollback"),
];

const TF_PROS: &[&str] = &[
    "comparison.tf.pros.1",
    "comparison.tf.pros.2",
    "comparison.tf.pros.3",
    "comparison.tf.pros.4",
];
const TF_CONS: &[&str] = &[
    "comparison.tf.cons.1",
    "comparison.tf.cons.2",
    "comparison.tf.cons.3",
    "comparison.tf.cons.4",
];
const BI_PROS: &[&str] = &[
    "comparison.bi.pros.1",
    "comparison.bi.pros.2",
    "comparison.bi.pros.3",
    "comparison.bi.pros.4",
];
const BI_CONS: &[&str] = &[
    "comparison.bi.cons.1",
    "comparison.bi.cons.2",
    "comparison.bi.cons.3",
    "comparison.bi.cons.4",
];

#[component]
pub fn TfVsBicepSlide() -> impl IntoView {
    let t = get_translation;
    let focused: RwSignal<Option<u8>> = create_rw_signal(None);

    // Comparison table rows
    let rows = DIMENSIONS.iter().enumerate().map(|(i, (dim, tf_val, bi_val))| {
        let row_class = if i % 2 == 0 { "table-row even" } else { "table-row odd" };
        view! {
            <tr class=row_class>
                <td class="dim-cell">{move || t(dim)}</td>
                <td class="tf-cell">{move || t(tf_val)}</td>
                <td class="bi-cell">{move || t(bi_val)}</td>
            </tr>
        }
    }).collect::<Vec<_>>();

    let tf_pros = TF_PROS.iter().map(|k| view! {
        <li class="pro-item">{move || t(k)}</li>
    }).collect::<Vec<_>>();

    let tf_cons = TF_CONS.iter().map(|k| view! {
        <li class="con-item">{move || t(k)}</li>
    }).collect::<Vec<_>>();

    let bi_pros = BI_PROS.iter().map(|k| view! {
        <li class="pro-item">{move || t(k)}</li>
    }).collect::<Vec<_>>();

    let bi_cons = BI_CONS.iter().map(|k| view! {
        <li class="con-item">{move || t(k)}</li>
    }).collect::<Vec<_>>();

    view! {
        <div class="slide comparison-slide">
            <div class="comparison-header">
                <div class="comparison-tool tf-header">
                    <span class="comp-tool-name" style="color: #7b42bc">"Terraform"</span>
                    <span class="comp-badge tf-badge">{move || t("comparison.terraform.badge")}</span>
                </div>
                <h2 class="comparison-title">{move || t("comparison.title")}</h2>
                <div class="comparison-tool bi-header">
                    <span class="comp-tool-name" style="color: #50e6ff">"Bicep"</span>
                    <span class="comp-badge bi-badge">{move || t("comparison.bicep.badge")}</span>
                </div>
            </div>

            // ── Comparison table ─────────────────────────────────────────────────
            <div class="table-wrapper">
                <table class="comparison-table">
                    <thead>
                        <tr>
                            <th class="dim-header">"Dimension"</th>
                            <th class="tf-header-cell">
                                <span style="color: #a78bfa">"Terraform"</span>
                            </th>
                            <th class="bi-header-cell">
                                <span style="color: #50e6ff">"Bicep"</span>
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {rows}
                    </tbody>
                </table>
            </div>

            // ── Pros / Cons ──────────────────────────────────────────────────────
            <div class="pros-cons-grid"
                 class:has-focus=move || focused.get().is_some()
                 on:click=move |_| focused.set(None)>

                // Terraform pros (index 0)
                <div class="pros-col pc-box"
                     class:box-focused=move || focused.get() == Some(0)
                     class:box-dimmed=move || focused.get().is_some() && focused.get() != Some(0)
                     on:click=move |e: ev::MouseEvent| {
                         e.stop_propagation();
                         focused.update(|f| *f = if *f == Some(0) { None } else { Some(0) });
                     }>
                    <h4 class="pc-heading tf-heading">
                        <span class="pc-icon">"✓"</span>
                        "Terraform: "{move || t("comparison.pros")}
                    </h4>
                    <ul class="pc-list">{tf_pros}</ul>
                </div>

                // Terraform cons (index 1)
                <div class="cons-col pc-box"
                     class:box-focused=move || focused.get() == Some(1)
                     class:box-dimmed=move || focused.get().is_some() && focused.get() != Some(1)
                     on:click=move |e: ev::MouseEvent| {
                         e.stop_propagation();
                         focused.update(|f| *f = if *f == Some(1) { None } else { Some(1) });
                     }>
                    <h4 class="pc-heading tf-cons-heading">
                        <span class="pc-icon">"✗"</span>
                        "Terraform: "{move || t("comparison.cons")}
                    </h4>
                    <ul class="pc-list">{tf_cons}</ul>
                </div>

                // Bicep pros (index 2)
                <div class="pros-col pc-box"
                     class:box-focused=move || focused.get() == Some(2)
                     class:box-dimmed=move || focused.get().is_some() && focused.get() != Some(2)
                     on:click=move |e: ev::MouseEvent| {
                         e.stop_propagation();
                         focused.update(|f| *f = if *f == Some(2) { None } else { Some(2) });
                     }>
                    <h4 class="pc-heading bi-heading">
                        <span class="pc-icon">"✓"</span>
                        "Bicep: "{move || t("comparison.pros")}
                    </h4>
                    <ul class="pc-list">{bi_pros}</ul>
                </div>

                // Bicep cons (index 3)
                <div class="cons-col pc-box"
                     class:box-focused=move || focused.get() == Some(3)
                     class:box-dimmed=move || focused.get().is_some() && focused.get() != Some(3)
                     on:click=move |e: ev::MouseEvent| {
                         e.stop_propagation();
                         focused.update(|f| *f = if *f == Some(3) { None } else { Some(3) });
                     }>
                    <h4 class="pc-heading bi-cons-heading">
                        <span class="pc-icon">"✗"</span>
                        "Bicep: "{move || t("comparison.cons")}
                    </h4>
                    <ul class="pc-list">{bi_cons}</ul>
                </div>

            </div>

        </div>
    }
}
