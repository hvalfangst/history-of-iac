#[derive(Clone, Copy, PartialEq)]
pub enum IacCategory {
    ConfigManagement,
    CloudProvisioning,
}

pub struct IacTool {
    /// i18n key prefix, e.g. "cfengine"
    pub key: &'static str,
    pub display_name: &'static str,
    pub year: u16,
    /// Which slide index this tool's slide is on (0-indexed)
    pub slide_index: usize,
    /// Brand hex colour
    pub color: &'static str,
    pub category: IacCategory,
    /// Vertical row offset: 0 = on baseline, -1 = above, +1 = below
    /// Used to resolve overlapping nodes on the same (or near) year
    pub row: i32,
    /// Horizontal pixel nudge to separate same-year nodes
    pub x_nudge: f32,
}

pub const IAC_TOOLS: &[IacTool] = &[
    IacTool {
        key: "cfengine",
        display_name: "CFEngine",
        year: 1993,
        slide_index: 4,
        color: "#e07b39",
        category: IacCategory::ConfigManagement,
        row: 0,
        x_nudge: 0.0,
    },
    IacTool {
        key: "puppet",
        display_name: "Puppet",
        year: 2005,
        slide_index: 5,
        color: "#f5a623",
        category: IacCategory::ConfigManagement,
        row: 0,
        x_nudge: 0.0,
    },
    IacTool {
        key: "chef",
        display_name: "Chef",
        year: 2009,
        slide_index: 6,
        color: "#e74c3c",
        category: IacCategory::ConfigManagement,
        row: 0,
        x_nudge: 0.0,
    },
    IacTool {
        key: "cloudformation",
        display_name: "CloudFormation",
        year: 2011,
        slide_index: 9,
        color: "#e67e22",
        category: IacCategory::CloudProvisioning,
        row: 0,
        x_nudge: 20.0,
    },
    IacTool {
        key: "ansible",
        display_name: "Ansible",
        year: 2012,
        slide_index: 7,
        color: "#c0392b",
        category: IacCategory::ConfigManagement,
        row: 0,
        x_nudge: 75.0,
    },
    IacTool {
        key: "arm",
        display_name: "ARM",
        year: 2014,
        slide_index: 10,
        color: "#0078d4",
        category: IacCategory::CloudProvisioning,
        row: 0,
        x_nudge: 80.0,
    },
    IacTool {
        key: "terraform",
        display_name: "Terraform",
        year: 2014,
        slide_index: 11,
        color: "#7b42bc",
        category: IacCategory::CloudProvisioning,
        row: 0,
        x_nudge: 160.0,
    },
    IacTool {
        key: "bicep",
        display_name: "Bicep",
        year: 2020,
        slide_index: 12,
        color: "#50e6ff",
        category: IacCategory::CloudProvisioning,
        row: 0,
        x_nudge: 0.0,
    },
];

/// Maps a calendar year to an SVG x-coordinate within a 1560-wide canvas.
/// The time axis spans 1990–2026 with 100 px left padding and 80 px right padding.
pub fn year_to_x(year: u16) -> f32 {
    const LEFT_PAD: f32 = 100.0;
    const AVAIL_W: f32 = 1380.0; // 1560 - 100 - 80
    const YEAR_SPAN: f32 = 36.0; // 2026 - 1990
    LEFT_PAD + (year as f32 - 1990.0) / YEAR_SPAN * AVAIL_W
}
