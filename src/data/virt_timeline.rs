pub struct VirtTech {
    pub key: &'static str,
    pub display_name: &'static str,
    pub year: u16,
    pub color: &'static str,
    /// i18n key for the correlation-band callout annotation
    pub corr_key: &'static str,
    /// Vertical row: 0 = on baseline, -1 = above, +1 = below
    pub row: i32,
    pub x_nudge: f32,
}

pub const VIRT_TECHS: &[VirtTech] = &[
    VirtTech {
        key: "vmware_ws",
        display_name: "VMware WS",
        year: 1999,
        color: "#78909c",
        corr_key: "timeline.corr.pre2013",
        row: 0,
        x_nudge: -20.0,
    },
    VirtTech {
        key: "vmware_esx",
        display_name: "VMware ESX",
        year: 2001,
        color: "#546e7a",
        corr_key: "timeline.corr.pre2013",
        row: 0,
        x_nudge: 20.0,
    },
    VirtTech {
        key: "docker",
        display_name: "Docker",
        year: 2013,
        color: "#2496ed",
        corr_key: "timeline.corr.docker",
        row: 0,
        x_nudge: 78.0,
    },
    VirtTech {
        key: "kubernetes",
        display_name: "Kubernetes",
        year: 2014,
        color: "#326ce5",
        corr_key: "timeline.corr.k8s",
        row: 0,
        x_nudge: 102.0,
    },
];
