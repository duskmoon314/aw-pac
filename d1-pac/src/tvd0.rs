#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tvd_en: TVD_EN,
    tvd_mode: TVD_MODE,
    tvd_clamp_agc1: TVD_CLAMP_AGC1,
    tvd_clamp_agc2: TVD_CLAMP_AGC2,
    tvd_hlock1: TVD_HLOCK1,
    tvd_hlock2: TVD_HLOCK2,
    tvd_hlock3: TVD_HLOCK3,
    tvd_hlock4: TVD_HLOCK4,
    tvd_hlock5: TVD_HLOCK5,
    tvd_vlock1: TVD_VLOCK1,
    tvd_vlock2: TVD_VLOCK2,
    _reserved11: [u8; 0x04],
    tvd_clock1: TVD_CLOCK1,
    tvd_clock2: TVD_CLOCK2,
    _reserved13: [u8; 0x08],
    tvd_yc_sep1: TVD_YC_SEP1,
    tvd_yc_sep2: TVD_YC_SEP2,
    _reserved15: [u8; 0x08],
    tvd_enhance1: TVD_ENHANCE1,
    tvd_enhance2: TVD_ENHANCE2,
    tvd_enhance3: TVD_ENHANCE3,
    _reserved18: [u8; 0x04],
    tvd_wb1: TVD_WB1,
    tvd_wb2: TVD_WB2,
    tvd_wb3: TVD_WB3,
    tvd_wb4: TVD_WB4,
    _reserved22: [u8; 0x10],
    tvd_irq_ctl: TVD_IRQ_CTL,
    _reserved23: [u8; 0x0c],
    tvd_irq_status: TVD_IRQ_STATUS,
    _reserved24: [u8; 0x6c],
    tvd_debug1: TVD_DEBUG1,
    _reserved25: [u8; 0x7c],
    tvd_status1: TVD_STATUS1,
    tvd_status2: TVD_STATUS2,
    tvd_status3: TVD_STATUS3,
    tvd_status4: TVD_STATUS4,
    tvd_status5: TVD_STATUS5,
    tvd_status6: TVD_STATUS6,
}
impl RegisterBlock {
    #[doc = "0x00 - TVD MODULE CONTROL Register"]
    #[inline(always)]
    pub const fn tvd_en(&self) -> &TVD_EN {
        &self.tvd_en
    }
    #[doc = "0x04 - TVD MODE CONTROL Register"]
    #[inline(always)]
    pub const fn tvd_mode(&self) -> &TVD_MODE {
        &self.tvd_mode
    }
    #[doc = "0x08 - TVD CLAMP And AGC CONTROL Register1"]
    #[inline(always)]
    pub const fn tvd_clamp_agc1(&self) -> &TVD_CLAMP_AGC1 {
        &self.tvd_clamp_agc1
    }
    #[doc = "0x0c - TVD CLAMP And AGC CONTROL Register2"]
    #[inline(always)]
    pub const fn tvd_clamp_agc2(&self) -> &TVD_CLAMP_AGC2 {
        &self.tvd_clamp_agc2
    }
    #[doc = "0x10 - TVD HLOCK CONTROL Register1"]
    #[inline(always)]
    pub const fn tvd_hlock1(&self) -> &TVD_HLOCK1 {
        &self.tvd_hlock1
    }
    #[doc = "0x14 - TVD HLOCK CONTROL Register2"]
    #[inline(always)]
    pub const fn tvd_hlock2(&self) -> &TVD_HLOCK2 {
        &self.tvd_hlock2
    }
    #[doc = "0x18 - TVD HLOCK CONTROL Register3"]
    #[inline(always)]
    pub const fn tvd_hlock3(&self) -> &TVD_HLOCK3 {
        &self.tvd_hlock3
    }
    #[doc = "0x1c - TVD HLOCK CONTROL Register4"]
    #[inline(always)]
    pub const fn tvd_hlock4(&self) -> &TVD_HLOCK4 {
        &self.tvd_hlock4
    }
    #[doc = "0x20 - TVD HLOCK CONTROL Register5"]
    #[inline(always)]
    pub const fn tvd_hlock5(&self) -> &TVD_HLOCK5 {
        &self.tvd_hlock5
    }
    #[doc = "0x24 - TVD VLOCK CONTROL Register1"]
    #[inline(always)]
    pub const fn tvd_vlock1(&self) -> &TVD_VLOCK1 {
        &self.tvd_vlock1
    }
    #[doc = "0x28 - TVD VLOCK CONTROL Register2"]
    #[inline(always)]
    pub const fn tvd_vlock2(&self) -> &TVD_VLOCK2 {
        &self.tvd_vlock2
    }
    #[doc = "0x30 - TVD CHROMA LOCK CONTROL Register1"]
    #[inline(always)]
    pub const fn tvd_clock1(&self) -> &TVD_CLOCK1 {
        &self.tvd_clock1
    }
    #[doc = "0x34 - TVD CHROMA LOCK CONTROL Register2"]
    #[inline(always)]
    pub const fn tvd_clock2(&self) -> &TVD_CLOCK2 {
        &self.tvd_clock2
    }
    #[doc = "0x40 - TVD YC SEPERATION CONROL Register1"]
    #[inline(always)]
    pub const fn tvd_yc_sep1(&self) -> &TVD_YC_SEP1 {
        &self.tvd_yc_sep1
    }
    #[doc = "0x44 - TVD YC SEPERATION CONROL Register2"]
    #[inline(always)]
    pub const fn tvd_yc_sep2(&self) -> &TVD_YC_SEP2 {
        &self.tvd_yc_sep2
    }
    #[doc = "0x50 - TVD ENHANCEMENT CONTROL Register1"]
    #[inline(always)]
    pub const fn tvd_enhance1(&self) -> &TVD_ENHANCE1 {
        &self.tvd_enhance1
    }
    #[doc = "0x54 - TVD ENHANCEMENT CONTROL Register2"]
    #[inline(always)]
    pub const fn tvd_enhance2(&self) -> &TVD_ENHANCE2 {
        &self.tvd_enhance2
    }
    #[doc = "0x58 - TVD ENHANCEMENT CONTROL Register3"]
    #[inline(always)]
    pub const fn tvd_enhance3(&self) -> &TVD_ENHANCE3 {
        &self.tvd_enhance3
    }
    #[doc = "0x60 - TVD WB DMA CONTROL Register1"]
    #[inline(always)]
    pub const fn tvd_wb1(&self) -> &TVD_WB1 {
        &self.tvd_wb1
    }
    #[doc = "0x64 - TVD WB DMA CONTROL Register2"]
    #[inline(always)]
    pub const fn tvd_wb2(&self) -> &TVD_WB2 {
        &self.tvd_wb2
    }
    #[doc = "0x68 - TVD WB DMA CONTROL Register3"]
    #[inline(always)]
    pub const fn tvd_wb3(&self) -> &TVD_WB3 {
        &self.tvd_wb3
    }
    #[doc = "0x6c - TVD WB DMA CONTROL Register4"]
    #[inline(always)]
    pub const fn tvd_wb4(&self) -> &TVD_WB4 {
        &self.tvd_wb4
    }
    #[doc = "0x80 - TVD DMA Interrupt Control Register"]
    #[inline(always)]
    pub const fn tvd_irq_ctl(&self) -> &TVD_IRQ_CTL {
        &self.tvd_irq_ctl
    }
    #[doc = "0x90 - TVD DMA Interrupt Status Register"]
    #[inline(always)]
    pub const fn tvd_irq_status(&self) -> &TVD_IRQ_STATUS {
        &self.tvd_irq_status
    }
    #[doc = "0x100 - TVD DEBUG CONTROL Register1"]
    #[inline(always)]
    pub const fn tvd_debug1(&self) -> &TVD_DEBUG1 {
        &self.tvd_debug1
    }
    #[doc = "0x180 - TVD DEBUG STATUS Register1"]
    #[inline(always)]
    pub const fn tvd_status1(&self) -> &TVD_STATUS1 {
        &self.tvd_status1
    }
    #[doc = "0x184 - TVD DEBUG STATUS Register2"]
    #[inline(always)]
    pub const fn tvd_status2(&self) -> &TVD_STATUS2 {
        &self.tvd_status2
    }
    #[doc = "0x188 - TVD DEBUG STATUS Register3"]
    #[inline(always)]
    pub const fn tvd_status3(&self) -> &TVD_STATUS3 {
        &self.tvd_status3
    }
    #[doc = "0x18c - TVD DEBUG STATUS Register4"]
    #[inline(always)]
    pub const fn tvd_status4(&self) -> &TVD_STATUS4 {
        &self.tvd_status4
    }
    #[doc = "0x190 - TVD DEBUG STATUS Register5"]
    #[inline(always)]
    pub const fn tvd_status5(&self) -> &TVD_STATUS5 {
        &self.tvd_status5
    }
    #[doc = "0x194 - TVD DEBUG STATUS Register6"]
    #[inline(always)]
    pub const fn tvd_status6(&self) -> &TVD_STATUS6 {
        &self.tvd_status6
    }
}
#[doc = "tvd_en (rw) register accessor: TVD MODULE CONTROL Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_en`] module"]
pub type TVD_EN = crate::Reg<tvd_en::TVD_EN_SPEC>;
#[doc = "TVD MODULE CONTROL Register"]
pub mod tvd_en;
#[doc = "tvd_mode (rw) register accessor: TVD MODE CONTROL Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_mode`] module"]
pub type TVD_MODE = crate::Reg<tvd_mode::TVD_MODE_SPEC>;
#[doc = "TVD MODE CONTROL Register"]
pub mod tvd_mode;
#[doc = "tvd_clamp_agc1 (rw) register accessor: TVD CLAMP And AGC CONTROL Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_clamp_agc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_clamp_agc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_clamp_agc1`] module"]
pub type TVD_CLAMP_AGC1 = crate::Reg<tvd_clamp_agc1::TVD_CLAMP_AGC1_SPEC>;
#[doc = "TVD CLAMP And AGC CONTROL Register1"]
pub mod tvd_clamp_agc1;
#[doc = "tvd_clamp_agc2 (rw) register accessor: TVD CLAMP And AGC CONTROL Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_clamp_agc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_clamp_agc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_clamp_agc2`] module"]
pub type TVD_CLAMP_AGC2 = crate::Reg<tvd_clamp_agc2::TVD_CLAMP_AGC2_SPEC>;
#[doc = "TVD CLAMP And AGC CONTROL Register2"]
pub mod tvd_clamp_agc2;
#[doc = "tvd_hlock1 (rw) register accessor: TVD HLOCK CONTROL Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_hlock1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_hlock1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_hlock1`] module"]
pub type TVD_HLOCK1 = crate::Reg<tvd_hlock1::TVD_HLOCK1_SPEC>;
#[doc = "TVD HLOCK CONTROL Register1"]
pub mod tvd_hlock1;
#[doc = "tvd_hlock2 (rw) register accessor: TVD HLOCK CONTROL Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_hlock2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_hlock2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_hlock2`] module"]
pub type TVD_HLOCK2 = crate::Reg<tvd_hlock2::TVD_HLOCK2_SPEC>;
#[doc = "TVD HLOCK CONTROL Register2"]
pub mod tvd_hlock2;
#[doc = "tvd_hlock3 (rw) register accessor: TVD HLOCK CONTROL Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_hlock3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_hlock3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_hlock3`] module"]
pub type TVD_HLOCK3 = crate::Reg<tvd_hlock3::TVD_HLOCK3_SPEC>;
#[doc = "TVD HLOCK CONTROL Register3"]
pub mod tvd_hlock3;
#[doc = "tvd_hlock4 (rw) register accessor: TVD HLOCK CONTROL Register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_hlock4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_hlock4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_hlock4`] module"]
pub type TVD_HLOCK4 = crate::Reg<tvd_hlock4::TVD_HLOCK4_SPEC>;
#[doc = "TVD HLOCK CONTROL Register4"]
pub mod tvd_hlock4;
#[doc = "tvd_hlock5 (rw) register accessor: TVD HLOCK CONTROL Register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_hlock5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_hlock5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_hlock5`] module"]
pub type TVD_HLOCK5 = crate::Reg<tvd_hlock5::TVD_HLOCK5_SPEC>;
#[doc = "TVD HLOCK CONTROL Register5"]
pub mod tvd_hlock5;
#[doc = "tvd_vlock1 (rw) register accessor: TVD VLOCK CONTROL Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_vlock1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_vlock1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_vlock1`] module"]
pub type TVD_VLOCK1 = crate::Reg<tvd_vlock1::TVD_VLOCK1_SPEC>;
#[doc = "TVD VLOCK CONTROL Register1"]
pub mod tvd_vlock1;
#[doc = "tvd_vlock2 (rw) register accessor: TVD VLOCK CONTROL Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_vlock2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_vlock2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_vlock2`] module"]
pub type TVD_VLOCK2 = crate::Reg<tvd_vlock2::TVD_VLOCK2_SPEC>;
#[doc = "TVD VLOCK CONTROL Register2"]
pub mod tvd_vlock2;
#[doc = "tvd_clock1 (rw) register accessor: TVD CHROMA LOCK CONTROL Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_clock1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_clock1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_clock1`] module"]
pub type TVD_CLOCK1 = crate::Reg<tvd_clock1::TVD_CLOCK1_SPEC>;
#[doc = "TVD CHROMA LOCK CONTROL Register1"]
pub mod tvd_clock1;
#[doc = "tvd_clock2 (rw) register accessor: TVD CHROMA LOCK CONTROL Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_clock2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_clock2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_clock2`] module"]
pub type TVD_CLOCK2 = crate::Reg<tvd_clock2::TVD_CLOCK2_SPEC>;
#[doc = "TVD CHROMA LOCK CONTROL Register2"]
pub mod tvd_clock2;
#[doc = "tvd_yc_sep1 (rw) register accessor: TVD YC SEPERATION CONROL Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_yc_sep1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_yc_sep1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_yc_sep1`] module"]
pub type TVD_YC_SEP1 = crate::Reg<tvd_yc_sep1::TVD_YC_SEP1_SPEC>;
#[doc = "TVD YC SEPERATION CONROL Register1"]
pub mod tvd_yc_sep1;
#[doc = "tvd_yc_sep2 (rw) register accessor: TVD YC SEPERATION CONROL Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_yc_sep2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_yc_sep2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_yc_sep2`] module"]
pub type TVD_YC_SEP2 = crate::Reg<tvd_yc_sep2::TVD_YC_SEP2_SPEC>;
#[doc = "TVD YC SEPERATION CONROL Register2"]
pub mod tvd_yc_sep2;
#[doc = "tvd_enhance1 (rw) register accessor: TVD ENHANCEMENT CONTROL Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_enhance1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_enhance1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_enhance1`] module"]
pub type TVD_ENHANCE1 = crate::Reg<tvd_enhance1::TVD_ENHANCE1_SPEC>;
#[doc = "TVD ENHANCEMENT CONTROL Register1"]
pub mod tvd_enhance1;
#[doc = "tvd_enhance2 (rw) register accessor: TVD ENHANCEMENT CONTROL Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_enhance2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_enhance2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_enhance2`] module"]
pub type TVD_ENHANCE2 = crate::Reg<tvd_enhance2::TVD_ENHANCE2_SPEC>;
#[doc = "TVD ENHANCEMENT CONTROL Register2"]
pub mod tvd_enhance2;
#[doc = "tvd_enhance3 (rw) register accessor: TVD ENHANCEMENT CONTROL Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_enhance3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_enhance3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_enhance3`] module"]
pub type TVD_ENHANCE3 = crate::Reg<tvd_enhance3::TVD_ENHANCE3_SPEC>;
#[doc = "TVD ENHANCEMENT CONTROL Register3"]
pub mod tvd_enhance3;
#[doc = "tvd_wb1 (rw) register accessor: TVD WB DMA CONTROL Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_wb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_wb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_wb1`] module"]
pub type TVD_WB1 = crate::Reg<tvd_wb1::TVD_WB1_SPEC>;
#[doc = "TVD WB DMA CONTROL Register1"]
pub mod tvd_wb1;
#[doc = "tvd_wb2 (rw) register accessor: TVD WB DMA CONTROL Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_wb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_wb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_wb2`] module"]
pub type TVD_WB2 = crate::Reg<tvd_wb2::TVD_WB2_SPEC>;
#[doc = "TVD WB DMA CONTROL Register2"]
pub mod tvd_wb2;
#[doc = "tvd_wb3 (rw) register accessor: TVD WB DMA CONTROL Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_wb3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_wb3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_wb3`] module"]
pub type TVD_WB3 = crate::Reg<tvd_wb3::TVD_WB3_SPEC>;
#[doc = "TVD WB DMA CONTROL Register3"]
pub mod tvd_wb3;
#[doc = "tvd_wb4 (rw) register accessor: TVD WB DMA CONTROL Register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_wb4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_wb4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_wb4`] module"]
pub type TVD_WB4 = crate::Reg<tvd_wb4::TVD_WB4_SPEC>;
#[doc = "TVD WB DMA CONTROL Register4"]
pub mod tvd_wb4;
#[doc = "tvd_irq_ctl (rw) register accessor: TVD DMA Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_irq_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_irq_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_irq_ctl`] module"]
pub type TVD_IRQ_CTL = crate::Reg<tvd_irq_ctl::TVD_IRQ_CTL_SPEC>;
#[doc = "TVD DMA Interrupt Control Register"]
pub mod tvd_irq_ctl;
#[doc = "tvd_irq_status (rw) register accessor: TVD DMA Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_irq_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_irq_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_irq_status`] module"]
pub type TVD_IRQ_STATUS = crate::Reg<tvd_irq_status::TVD_IRQ_STATUS_SPEC>;
#[doc = "TVD DMA Interrupt Status Register"]
pub mod tvd_irq_status;
#[doc = "tvd_debug1 (rw) register accessor: TVD DEBUG CONTROL Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_debug1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_debug1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_debug1`] module"]
pub type TVD_DEBUG1 = crate::Reg<tvd_debug1::TVD_DEBUG1_SPEC>;
#[doc = "TVD DEBUG CONTROL Register1"]
pub mod tvd_debug1;
#[doc = "tvd_status1 (rw) register accessor: TVD DEBUG STATUS Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_status1`] module"]
pub type TVD_STATUS1 = crate::Reg<tvd_status1::TVD_STATUS1_SPEC>;
#[doc = "TVD DEBUG STATUS Register1"]
pub mod tvd_status1;
#[doc = "tvd_status2 (rw) register accessor: TVD DEBUG STATUS Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_status2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_status2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_status2`] module"]
pub type TVD_STATUS2 = crate::Reg<tvd_status2::TVD_STATUS2_SPEC>;
#[doc = "TVD DEBUG STATUS Register2"]
pub mod tvd_status2;
#[doc = "tvd_status3 (rw) register accessor: TVD DEBUG STATUS Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_status3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_status3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_status3`] module"]
pub type TVD_STATUS3 = crate::Reg<tvd_status3::TVD_STATUS3_SPEC>;
#[doc = "TVD DEBUG STATUS Register3"]
pub mod tvd_status3;
#[doc = "tvd_status4 (rw) register accessor: TVD DEBUG STATUS Register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_status4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_status4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_status4`] module"]
pub type TVD_STATUS4 = crate::Reg<tvd_status4::TVD_STATUS4_SPEC>;
#[doc = "TVD DEBUG STATUS Register4"]
pub mod tvd_status4;
#[doc = "tvd_status5 (rw) register accessor: TVD DEBUG STATUS Register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_status5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_status5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_status5`] module"]
pub type TVD_STATUS5 = crate::Reg<tvd_status5::TVD_STATUS5_SPEC>;
#[doc = "TVD DEBUG STATUS Register5"]
pub mod tvd_status5;
#[doc = "tvd_status6 (rw) register accessor: TVD DEBUG STATUS Register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_status6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_status6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_status6`] module"]
pub type TVD_STATUS6 = crate::Reg<tvd_status6::TVD_STATUS6_SPEC>;
#[doc = "TVD DEBUG STATUS Register6"]
pub mod tvd_status6;
