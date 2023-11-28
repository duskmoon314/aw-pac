#[doc = r"Register block"]
#[repr(C)]
pub struct OHCI_FRAME_COUNTER_PARTITION {
    hc_fm_interval: HC_FM_INTERVAL,
    hc_fm_remaining: HC_FM_REMAINING,
    hc_fm_number: HC_FM_NUMBER,
    hc_periodic_start: HC_PERIODIC_START,
    hc_ls_threshold: HC_LS_THRESHOLD,
}
impl OHCI_FRAME_COUNTER_PARTITION {
    #[doc = "0x00 - OHCI Frame Interval Register"]
    #[inline(always)]
    pub const fn hc_fm_interval(&self) -> &HC_FM_INTERVAL {
        &self.hc_fm_interval
    }
    #[doc = "0x04 - OHCI Frame Remaining Register"]
    #[inline(always)]
    pub const fn hc_fm_remaining(&self) -> &HC_FM_REMAINING {
        &self.hc_fm_remaining
    }
    #[doc = "0x08 - OHCI Frame Number Register"]
    #[inline(always)]
    pub const fn hc_fm_number(&self) -> &HC_FM_NUMBER {
        &self.hc_fm_number
    }
    #[doc = "0x0c - OHCI Periodic Start Register"]
    #[inline(always)]
    pub const fn hc_periodic_start(&self) -> &HC_PERIODIC_START {
        &self.hc_periodic_start
    }
    #[doc = "0x10 - OHCI LS Threshold Register"]
    #[inline(always)]
    pub const fn hc_ls_threshold(&self) -> &HC_LS_THRESHOLD {
        &self.hc_ls_threshold
    }
}
#[doc = "hc_fm_interval (rw) register accessor: OHCI Frame Interval Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_fm_interval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_fm_interval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_fm_interval`] module"]
pub type HC_FM_INTERVAL = crate::Reg<hc_fm_interval::HC_FM_INTERVAL_SPEC>;
#[doc = "OHCI Frame Interval Register"]
pub mod hc_fm_interval;
#[doc = "hc_fm_remaining (rw) register accessor: OHCI Frame Remaining Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_fm_remaining::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_fm_remaining::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_fm_remaining`] module"]
pub type HC_FM_REMAINING = crate::Reg<hc_fm_remaining::HC_FM_REMAINING_SPEC>;
#[doc = "OHCI Frame Remaining Register"]
pub mod hc_fm_remaining;
#[doc = "hc_fm_number (rw) register accessor: OHCI Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_fm_number::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_fm_number::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_fm_number`] module"]
pub type HC_FM_NUMBER = crate::Reg<hc_fm_number::HC_FM_NUMBER_SPEC>;
#[doc = "OHCI Frame Number Register"]
pub mod hc_fm_number;
#[doc = "hc_periodic_start (rw) register accessor: OHCI Periodic Start Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_periodic_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_periodic_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_periodic_start`] module"]
pub type HC_PERIODIC_START = crate::Reg<hc_periodic_start::HC_PERIODIC_START_SPEC>;
#[doc = "OHCI Periodic Start Register"]
pub mod hc_periodic_start;
#[doc = "hc_ls_threshold (rw) register accessor: OHCI LS Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_ls_threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_ls_threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_ls_threshold`] module"]
pub type HC_LS_THRESHOLD = crate::Reg<hc_ls_threshold::HC_LS_THRESHOLD_SPEC>;
#[doc = "OHCI LS Threshold Register"]
pub mod hc_ls_threshold;
