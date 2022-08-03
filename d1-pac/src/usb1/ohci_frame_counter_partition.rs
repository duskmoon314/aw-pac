#[doc = r"Register block"]
#[repr(C)]
pub struct OHCI_FRAME_COUNTER_PARTITION {
    #[doc = "0x00 - OHCI Frame Interval Register"]
    pub hc_fm_interval: HC_FM_INTERVAL,
    #[doc = "0x04 - OHCI Frame Remaining Register"]
    pub hc_fm_remaining: HC_FM_REMAINING,
    #[doc = "0x08 - OHCI Frame Number Register"]
    pub hc_fm_number: HC_FM_NUMBER,
    #[doc = "0x0c - OHCI Periodic Start Register"]
    pub hc_periodic_start: HC_PERIODIC_START,
    #[doc = "0x10 - OHCI LS Threshold Register"]
    pub hc_ls_threshold: HC_LS_THRESHOLD,
}
#[doc = "hc_fm_interval (rw) register accessor: an alias for `Reg<HC_FM_INTERVAL_SPEC>`"]
pub type HC_FM_INTERVAL = crate::Reg<hc_fm_interval::HC_FM_INTERVAL_SPEC>;
#[doc = "OHCI Frame Interval Register"]
pub mod hc_fm_interval;
#[doc = "hc_fm_remaining (rw) register accessor: an alias for `Reg<HC_FM_REMAINING_SPEC>`"]
pub type HC_FM_REMAINING = crate::Reg<hc_fm_remaining::HC_FM_REMAINING_SPEC>;
#[doc = "OHCI Frame Remaining Register"]
pub mod hc_fm_remaining;
#[doc = "hc_fm_number (rw) register accessor: an alias for `Reg<HC_FM_NUMBER_SPEC>`"]
pub type HC_FM_NUMBER = crate::Reg<hc_fm_number::HC_FM_NUMBER_SPEC>;
#[doc = "OHCI Frame Number Register"]
pub mod hc_fm_number;
#[doc = "hc_periodic_start (rw) register accessor: an alias for `Reg<HC_PERIODIC_START_SPEC>`"]
pub type HC_PERIODIC_START = crate::Reg<hc_periodic_start::HC_PERIODIC_START_SPEC>;
#[doc = "OHCI Periodic Start Register"]
pub mod hc_periodic_start;
#[doc = "hc_ls_threshold (rw) register accessor: an alias for `Reg<HC_LS_THRESHOLD_SPEC>`"]
pub type HC_LS_THRESHOLD = crate::Reg<hc_ls_threshold::HC_LS_THRESHOLD_SPEC>;
#[doc = "OHCI LS Threshold Register"]
pub mod hc_ls_threshold;
