#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    prio: [PRIO; 256],
    _reserved1: [u8; 0x0c00],
    ip: [IP; 9],
    _reserved2: [u8; 0x0fdc],
    mie: [MIE; 9],
    _reserved3: [u8; 0x5c],
    sie: [SIE; 9],
    _reserved4: [u8; 0x001f_df58],
    ctrl: CTRL,
    mth: MTH,
    mclaim: MCLAIM,
    _reserved7: [u8; 0x0ff8],
    sth: STH,
    sclaim: SCLAIM,
}
impl RegisterBlock {
    #[doc = "0x00..0x400 - Interrupt Priority Register"]
    #[inline(always)]
    pub const fn prio(&self, n: usize) -> &PRIO {
        &self.prio[n]
    }
    #[doc = "0x1000..0x1024 - Interrupt Pending Register"]
    #[inline(always)]
    pub const fn ip(&self, n: usize) -> &IP {
        &self.ip[n]
    }
    #[doc = "0x2000..0x2024 - Machine Mode Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mie(&self, n: usize) -> &MIE {
        &self.mie[n]
    }
    #[doc = "0x2080..0x20a4 - Supervisor Mode Interrupt Enable Register"]
    #[inline(always)]
    pub const fn sie(&self, n: usize) -> &SIE {
        &self.sie[n]
    }
    #[doc = "0x1ffffc - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x200000 - Machine Mode Priority Threshold Register"]
    #[inline(always)]
    pub const fn mth(&self) -> &MTH {
        &self.mth
    }
    #[doc = "0x200004 - Machine Mode Claim/Complete Register"]
    #[inline(always)]
    pub const fn mclaim(&self) -> &MCLAIM {
        &self.mclaim
    }
    #[doc = "0x201000 - Supervisor Mode Priority Threshold Register"]
    #[inline(always)]
    pub const fn sth(&self) -> &STH {
        &self.sth
    }
    #[doc = "0x201004 - Supervisor Mode Claim/Complete Register"]
    #[inline(always)]
    pub const fn sclaim(&self) -> &SCLAIM {
        &self.sclaim
    }
}
#[doc = "prio (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prio`] module"]
pub type PRIO = crate::Reg<prio::PRIO_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod prio;
#[doc = "ip (rw) register accessor: Interrupt Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ip::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ip::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ip`] module"]
pub type IP = crate::Reg<ip::IP_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ip;
#[doc = "mie (rw) register accessor: Machine Mode Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mie`] module"]
pub type MIE = crate::Reg<mie::MIE_SPEC>;
#[doc = "Machine Mode Interrupt Enable Register"]
pub mod mie;
#[doc = "sie (rw) register accessor: Supervisor Mode Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie`] module"]
pub type SIE = crate::Reg<sie::SIE_SPEC>;
#[doc = "Supervisor Mode Interrupt Enable Register"]
pub mod sie;
#[doc = "ctrl (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "mth (rw) register accessor: Machine Mode Priority Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mth`] module"]
pub type MTH = crate::Reg<mth::MTH_SPEC>;
#[doc = "Machine Mode Priority Threshold Register"]
pub mod mth;
#[doc = "mclaim (rw) register accessor: Machine Mode Claim/Complete Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclaim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclaim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclaim`] module"]
pub type MCLAIM = crate::Reg<mclaim::MCLAIM_SPEC>;
#[doc = "Machine Mode Claim/Complete Register"]
pub mod mclaim;
#[doc = "sth (rw) register accessor: Supervisor Mode Priority Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sth`] module"]
pub type STH = crate::Reg<sth::STH_SPEC>;
#[doc = "Supervisor Mode Priority Threshold Register"]
pub mod sth;
#[doc = "sclaim (rw) register accessor: Supervisor Mode Claim/Complete Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sclaim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sclaim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sclaim`] module"]
pub type SCLAIM = crate::Reg<sclaim::SCLAIM_SPEC>;
#[doc = "Supervisor Mode Claim/Complete Register"]
pub mod sclaim;
