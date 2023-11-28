#[doc = r"Register block"]
#[repr(C)]
pub struct OHCI_MEMORY_POINTER_PARTITION {
    hc_hcca: HC_HCCA,
    hc_period_current_ed: HC_PERIOD_CURRENT_ED,
    hc_control_head_ed: HC_CONTROL_HEAD_ED,
    hc_control_current_ed: HC_CONTROL_CURRENT_ED,
    hc_bulk_head_ed: HC_BULK_HEAD_ED,
    hc_bulk_current_ed: HC_BULK_CURRENT_ED,
    hc_done_head: HC_DONE_HEAD,
}
impl OHCI_MEMORY_POINTER_PARTITION {
    #[doc = "0x00 - OHCI HCCA Base"]
    #[inline(always)]
    pub const fn hc_hcca(&self) -> &HC_HCCA {
        &self.hc_hcca
    }
    #[doc = "0x04 - OHCI Period Current ED Base"]
    #[inline(always)]
    pub const fn hc_period_current_ed(&self) -> &HC_PERIOD_CURRENT_ED {
        &self.hc_period_current_ed
    }
    #[doc = "0x08 - OHCI Control Head ED Base"]
    #[inline(always)]
    pub const fn hc_control_head_ed(&self) -> &HC_CONTROL_HEAD_ED {
        &self.hc_control_head_ed
    }
    #[doc = "0x0c - OHCI Control Current ED Base"]
    #[inline(always)]
    pub const fn hc_control_current_ed(&self) -> &HC_CONTROL_CURRENT_ED {
        &self.hc_control_current_ed
    }
    #[doc = "0x10 - OHCI Bulk Head ED Base"]
    #[inline(always)]
    pub const fn hc_bulk_head_ed(&self) -> &HC_BULK_HEAD_ED {
        &self.hc_bulk_head_ed
    }
    #[doc = "0x14 - OHCI Bulk Current ED Base"]
    #[inline(always)]
    pub const fn hc_bulk_current_ed(&self) -> &HC_BULK_CURRENT_ED {
        &self.hc_bulk_current_ed
    }
    #[doc = "0x18 - OHCI Done Head Base"]
    #[inline(always)]
    pub const fn hc_done_head(&self) -> &HC_DONE_HEAD {
        &self.hc_done_head
    }
}
#[doc = "hc_hcca (rw) register accessor: OHCI HCCA Base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_hcca::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_hcca::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_hcca`] module"]
pub type HC_HCCA = crate::Reg<hc_hcca::HC_HCCA_SPEC>;
#[doc = "OHCI HCCA Base"]
pub mod hc_hcca;
#[doc = "hc_period_current_ed (rw) register accessor: OHCI Period Current ED Base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_period_current_ed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_period_current_ed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_period_current_ed`] module"]
pub type HC_PERIOD_CURRENT_ED = crate::Reg<hc_period_current_ed::HC_PERIOD_CURRENT_ED_SPEC>;
#[doc = "OHCI Period Current ED Base"]
pub mod hc_period_current_ed;
#[doc = "hc_control_head_ed (rw) register accessor: OHCI Control Head ED Base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_control_head_ed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_control_head_ed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_control_head_ed`] module"]
pub type HC_CONTROL_HEAD_ED = crate::Reg<hc_control_head_ed::HC_CONTROL_HEAD_ED_SPEC>;
#[doc = "OHCI Control Head ED Base"]
pub mod hc_control_head_ed;
#[doc = "hc_control_current_ed (rw) register accessor: OHCI Control Current ED Base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_control_current_ed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_control_current_ed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_control_current_ed`] module"]
pub type HC_CONTROL_CURRENT_ED = crate::Reg<hc_control_current_ed::HC_CONTROL_CURRENT_ED_SPEC>;
#[doc = "OHCI Control Current ED Base"]
pub mod hc_control_current_ed;
#[doc = "hc_bulk_head_ed (rw) register accessor: OHCI Bulk Head ED Base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_bulk_head_ed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_bulk_head_ed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_bulk_head_ed`] module"]
pub type HC_BULK_HEAD_ED = crate::Reg<hc_bulk_head_ed::HC_BULK_HEAD_ED_SPEC>;
#[doc = "OHCI Bulk Head ED Base"]
pub mod hc_bulk_head_ed;
#[doc = "hc_bulk_current_ed (rw) register accessor: OHCI Bulk Current ED Base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_bulk_current_ed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_bulk_current_ed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_bulk_current_ed`] module"]
pub type HC_BULK_CURRENT_ED = crate::Reg<hc_bulk_current_ed::HC_BULK_CURRENT_ED_SPEC>;
#[doc = "OHCI Bulk Current ED Base"]
pub mod hc_bulk_current_ed;
#[doc = "hc_done_head (rw) register accessor: OHCI Done Head Base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_done_head::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_done_head::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_done_head`] module"]
pub type HC_DONE_HEAD = crate::Reg<hc_done_head::HC_DONE_HEAD_SPEC>;
#[doc = "OHCI Done Head Base"]
pub mod hc_done_head;
