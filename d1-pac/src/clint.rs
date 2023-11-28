#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    msip: MSIP,
    _reserved1: [u8; 0x3ffc],
    mtimecmpl: MTIMECMPL,
    mtimecmph: MTIMECMPH,
    _reserved3: [u8; 0x7ff0],
    mtime: MTIME,
    ssip: SSIP,
    _reserved5: [u8; 0x0ffc],
    stimecmpl: STIMECMPL,
    stimecmph: STIMECMPH,
}
impl RegisterBlock {
    #[doc = "0x00 - MSIP Register for hart 0"]
    #[inline(always)]
    pub const fn msip(&self) -> &MSIP {
        &self.msip
    }
    #[doc = "0x4000 - MTIMECMPL Register for hart 0"]
    #[inline(always)]
    pub const fn mtimecmpl(&self) -> &MTIMECMPL {
        &self.mtimecmpl
    }
    #[doc = "0x4004 - MTIMECMPH Register for hart 0"]
    #[inline(always)]
    pub const fn mtimecmph(&self) -> &MTIMECMPH {
        &self.mtimecmph
    }
    #[doc = "0xbff8..0xc000 - MTIME\n\nREF: opensbi"]
    #[inline(always)]
    pub const fn mtime(&self) -> &MTIME {
        &self.mtime
    }
    #[doc = "0xc000 - SSIP Register for hart 0"]
    #[inline(always)]
    pub const fn ssip(&self) -> &SSIP {
        &self.ssip
    }
    #[doc = "0xd000 - STIMECMPL Register for hart 0"]
    #[inline(always)]
    pub const fn stimecmpl(&self) -> &STIMECMPL {
        &self.stimecmpl
    }
    #[doc = "0xd004 - STIMECMPH Register for hart 0"]
    #[inline(always)]
    pub const fn stimecmph(&self) -> &STIMECMPH {
        &self.stimecmph
    }
}
#[doc = "msip (rw) register accessor: MSIP Register for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msip`] module"]
pub type MSIP = crate::Reg<msip::MSIP_SPEC>;
#[doc = "MSIP Register for hart 0"]
pub mod msip;
#[doc = "mtimecmpl (rw) register accessor: MTIMECMPL Register for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmpl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmpl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmpl`] module"]
pub type MTIMECMPL = crate::Reg<mtimecmpl::MTIMECMPL_SPEC>;
#[doc = "MTIMECMPL Register for hart 0"]
pub mod mtimecmpl;
#[doc = "mtimecmph (rw) register accessor: MTIMECMPH Register for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmph::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmph::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmph`] module"]
pub type MTIMECMPH = crate::Reg<mtimecmph::MTIMECMPH_SPEC>;
#[doc = "MTIMECMPH Register for hart 0"]
pub mod mtimecmph;
#[doc = "mtime (r) register accessor: MTIME\n\nREF: opensbi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime`] module"]
pub type MTIME = crate::Reg<mtime::MTIME_SPEC>;
#[doc = "MTIME\n\nREF: opensbi"]
pub mod mtime;
#[doc = "ssip (rw) register accessor: SSIP Register for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssip::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssip::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssip`] module"]
pub type SSIP = crate::Reg<ssip::SSIP_SPEC>;
#[doc = "SSIP Register for hart 0"]
pub mod ssip;
#[doc = "stimecmpl (rw) register accessor: STIMECMPL Register for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stimecmpl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stimecmpl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stimecmpl`] module"]
pub type STIMECMPL = crate::Reg<stimecmpl::STIMECMPL_SPEC>;
#[doc = "STIMECMPL Register for hart 0"]
pub mod stimecmpl;
#[doc = "stimecmph (rw) register accessor: STIMECMPH Register for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stimecmph::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stimecmph::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stimecmph`] module"]
pub type STIMECMPH = crate::Reg<stimecmph::STIMECMPH_SPEC>;
#[doc = "STIMECMPH Register for hart 0"]
pub mod stimecmph;
