use leptos::*;
use crate::i18n::get_translation;
use crate::data::iac_timeline::{IAC_TOOLS, year_to_x};
use crate::data::virt_timeline::VIRT_TECHS;

// ── SVG layout constants ──────────────────────────────────────────────────────
const SVG_W: f32 = 1560.0;

// IaC timeline band
const IAC_BASE_Y: f32 = 180.0;  // main baseline
const IAC_ROW_STEP: f32 = 50.0; // px per row offset
const IAC_NODE_R: f32 = 14.0;

// Virt timeline band
const VIRT_SECTION_TOP: f32 = 365.0;
const VIRT_BASE_Y: f32 = 490.0;
const VIRT_ROW_STEP: f32 = 50.0;
const VIRT_NODE_R: f32 = 13.0;


fn node_y(base: f32, row: i32, step: f32) -> f32 {
    base + (row as f32) * step
}

#[component]
pub fn TimelinesSlide() -> impl IntoView {
    let t = get_translation;

    // ── IaC tool nodes ─────────────────────────────────────────────────────
    let iac_nodes = IAC_TOOLS.iter().map(|tool| {
        let x = year_to_x(tool.year) + tool.x_nudge;
        let ny = node_y(IAC_BASE_Y, tool.row, IAC_ROW_STEP);
        let color = tool.color;
        let label = tool.display_name;

        // Label placement: above or below depending on row
        let label_y = if tool.row <= 0 {
            ny - IAC_NODE_R - 8.0
        } else {
            ny + IAC_NODE_R + 16.0
        };
        let year_str = tool.year.to_string();
        let year_label_y = if tool.row <= 0 {
            ny - IAC_NODE_R - 22.0
        } else {
            ny + IAC_NODE_R + 30.0
        };

        // Connector line from baseline to offset node
        let connector = if tool.row != 0 {
            let cx = year_to_x(tool.year);
            Some(view! {
                <line x1=cx y1=IAC_BASE_Y x2=x y2=ny
                      stroke=color stroke-width="1.5" stroke-dasharray="4,3" opacity="0.6"/>
            })
        } else {
            None
        };

        view! {
            <g class="iac-node">
                {connector}
                <circle cx=x cy=ny r={IAC_NODE_R + 4.0}
                        fill=color opacity="0.15"/>
                <circle cx=x cy=ny r=IAC_NODE_R fill=color/>
                <text x=x y=label_y text-anchor="middle"
                      class="node-name">{label}</text>
                <text x=x y=year_label_y text-anchor="middle"
                      class="node-year-label">{year_str}</text>
            </g>
        }
    }).collect::<Vec<_>>();

    // ── Virtualisation tech nodes ──────────────────────────────────────────
    let virt_nodes = VIRT_TECHS.iter().map(|tech| {
        let x = year_to_x(tech.year) + tech.x_nudge;
        let ny = node_y(VIRT_BASE_Y, tech.row, VIRT_ROW_STEP);
        let color = tech.color;
        let label = tech.display_name;
        let year_str = tech.year.to_string();

        let label_y = if tech.row <= 0 {
            ny - VIRT_NODE_R - 8.0
        } else {
            ny + VIRT_NODE_R + 16.0
        };
        let year_label_y = if tech.row <= 0 {
            ny - VIRT_NODE_R - 22.0
        } else {
            ny + VIRT_NODE_R + 30.0
        };

        let connector = if tech.row != 0 {
            let cx = year_to_x(tech.year);
            Some(view! {
                <line x1=cx y1=VIRT_BASE_Y x2=x y2=ny
                      stroke=color stroke-width="1.5" stroke-dasharray="4,3" opacity="0.6"/>
            })
        } else {
            None
        };

        // Square nodes for virt techs (diamond shape via transform rotate)
        let half = VIRT_NODE_R;
        let pts = format!(
            "{},{} {},{} {},{} {},{}",
            x, ny - half * 1.4,
            x + half * 1.4, ny,
            x, ny + half * 1.4,
            x - half * 1.4, ny
        );

        // Outer glow ring: slightly larger diamond
        let half_outer = half * 1.55;
        let pts_outer = format!(
            "{},{} {},{} {},{} {},{}",
            x, ny - half_outer,
            x + half_outer, ny,
            x, ny + half_outer,
            x - half_outer, ny
        );

        view! {
            <g class="virt-node">
                {connector}
                <polygon points=pts_outer fill=color opacity="0.12"/>
                <polygon points=pts fill=color opacity="0.9"/>
                <text x=x y=label_y text-anchor="middle"
                      class="node-name virt-label">{label}</text>
                <text x=x y=year_label_y text-anchor="middle"
                      class="node-year-label virt-year">{year_str}</text>
            </g>
        }
    }).collect::<Vec<_>>();

    // ── Category band x extents ────────────────────────────────────────────
    // Config-mgmt era: CFEngine(1993) → Ansible(2012)
    let x_config_start = year_to_x(1993) - 20.0;
    let x_config_end   = year_to_x(2011) - 5.0;
    // Cloud IaC era: CloudFormation(2011) → 2026
    let x_cloud_start  = year_to_x(2011) - 5.0;
    let x_cloud_end    = year_to_x(2026);

    let band_top = 60.0_f32;
    let band_h   = (IAC_BASE_Y + IAC_ROW_STEP + IAC_NODE_R + 35.0) - band_top;


    view! {
        <div class="slide timelines-slide">
            <h2 class="slide-main-title">{move || t("timeline.title")}</h2>

            <svg viewBox="0 0 1560 620" class="timeline-svg" xmlns="http://www.w3.org/2000/svg">

                // ── Background ────────────────────────────────────────────
                <rect x="0" y="0" width=SVG_W height="620" fill="#0d1117"/>

                // ── IaC section background ────────────────────────────────
                <rect x="0" y="0" width=SVG_W height="335" fill="#111827" rx="0"/>

                // IaC section title
                <text x="780" y="28" text-anchor="middle" class="section-title iac-section-title">
                    {move || t("timeline.iac.label")}
                </text>

                // Category bands
                // Config management (amber tint)
                <rect
                    x=x_config_start y=band_top
                    width={x_config_end - x_config_start} height=band_h
                    fill="rgba(224,123,57,0.10)" rx="6"/>
                <text
                    x={(x_config_start + x_config_end) / 2.0 - 40.0}
                    y={band_top + 14.0}
                    class="band-label config-label">
                    {move || t("timeline.category.config")}
                </text>

                // Cloud IaC (blue tint)
                <rect
                    x=x_cloud_start y=band_top
                    width={x_cloud_end - x_cloud_start} height=band_h
                    fill="rgba(0,120,212,0.10)" rx="6"/>
                <text
                    x={(x_cloud_start + x_cloud_end) / 2.0 + 20.0}
                    y={band_top + 14.0}
                    class="band-label cloud-label">
                    {move || t("timeline.category.cloud")}
                </text>

                // IaC baseline
                <line x1="80" y1=IAC_BASE_Y x2="1480" y2=IAC_BASE_Y
                      stroke="#2d3f6b" stroke-width="2"/>

                // IaC tool nodes
                {iac_nodes}

                // ── Virt section ──────────────────────────────────────────
                <rect x="0" y=VIRT_SECTION_TOP width=SVG_W height={620.0 - VIRT_SECTION_TOP}
                      fill="#0d1117"/>

                // Virt section title
                <text x="780" y={VIRT_SECTION_TOP + 42.0} text-anchor="middle"
                      class="section-title virt-section-title">
                    {move || t("timeline.virt.label")}
                </text>

                // Virt baseline
                <line x1="80" y1=VIRT_BASE_Y x2="1480" y2=VIRT_BASE_Y
                      stroke="#2d3f6b" stroke-width="2"/>

                // Virt nodes
                {virt_nodes}


            </svg>

        </div>
    }
}
