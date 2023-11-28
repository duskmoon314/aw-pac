#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pier: PIER,
    pisr: PISR,
    _reserved2: [u8; 0x08],
    cier: CIER,
    cisr: CISR,
    _reserved4: [u8; 0x08],
    pccr01: PCCR01,
    pccr23: PCCR23,
    pccr45: PCCR45,
    pccr67: PCCR67,
    _reserved8: [u8; 0x10],
    pcgr: PCGR,
    _reserved9: [u8; 0x1c],
    pdzcr01: PDZCR01,
    pdzcr23: PDZCR23,
    pdzcr45: PDZCR45,
    pdzcr67: PDZCR67,
    _reserved13: [u8; 0x10],
    per: PER,
    _reserved14: [u8; 0x0c],
    pgr: [PGR; 4],
    _reserved15: [u8; 0x20],
    cer: CER,
    _reserved16: [u8; 0x3c],
    pcr: (),
    _reserved17: [u8; 0x04],
    ppr: (),
    _reserved18: [u8; 0x04],
    pcntr: (),
    _reserved19: [u8; 0x04],
    ppcntr: (),
    _reserved20: [u8; 0x04],
    ccr: (),
    _reserved21: [u8; 0x04],
    crlr: (),
    _reserved22: [u8; 0x04],
    cflr: (),
}
impl RegisterBlock {
    #[doc = "0x00 - PWM IRQ Enable Register"]
    #[inline(always)]
    pub const fn pier(&self) -> &PIER {
        &self.pier
    }
    #[doc = "0x04 - PWM IRQ Status Register"]
    #[inline(always)]
    pub const fn pisr(&self) -> &PISR {
        &self.pisr
    }
    #[doc = "0x10 - Capture IRQ Enable Register"]
    #[inline(always)]
    pub const fn cier(&self) -> &CIER {
        &self.cier
    }
    #[doc = "0x14 - Capture IRQ Status Register"]
    #[inline(always)]
    pub const fn cisr(&self) -> &CISR {
        &self.cisr
    }
    #[doc = "0x20 - PWM01 Clock Configuration Register"]
    #[inline(always)]
    pub const fn pccr01(&self) -> &PCCR01 {
        &self.pccr01
    }
    #[doc = "0x24 - PWM23 Clock Configuration Register"]
    #[inline(always)]
    pub const fn pccr23(&self) -> &PCCR23 {
        &self.pccr23
    }
    #[doc = "0x28 - PWM45 Clock Configuration Register"]
    #[inline(always)]
    pub const fn pccr45(&self) -> &PCCR45 {
        &self.pccr45
    }
    #[doc = "0x2c - PWM67 Clock Configuration Register"]
    #[inline(always)]
    pub const fn pccr67(&self) -> &PCCR67 {
        &self.pccr67
    }
    #[doc = "0x40 - PWM Clock Gating Register"]
    #[inline(always)]
    pub const fn pcgr(&self) -> &PCGR {
        &self.pcgr
    }
    #[doc = "0x60 - PWM01 Dead Zone Control Register"]
    #[inline(always)]
    pub const fn pdzcr01(&self) -> &PDZCR01 {
        &self.pdzcr01
    }
    #[doc = "0x64 - PWM23 Dead Zone Control Register"]
    #[inline(always)]
    pub const fn pdzcr23(&self) -> &PDZCR23 {
        &self.pdzcr23
    }
    #[doc = "0x68 - PWM45 Dead Zone Control Register"]
    #[inline(always)]
    pub const fn pdzcr45(&self) -> &PDZCR45 {
        &self.pdzcr45
    }
    #[doc = "0x6c - PWM67 Dead Zone Control Register"]
    #[inline(always)]
    pub const fn pdzcr67(&self) -> &PDZCR67 {
        &self.pdzcr67
    }
    #[doc = "0x80 - PWM Enable Register"]
    #[inline(always)]
    pub const fn per(&self) -> &PER {
        &self.per
    }
    #[doc = "0x90..0xa0 - PWM Group\\[g\\] Register"]
    #[inline(always)]
    pub const fn pgr(&self, n: usize) -> &PGR {
        &self.pgr[n]
    }
    #[doc = "0xc0 - Capture Enable Register"]
    #[inline(always)]
    pub const fn cer(&self) -> &CER {
        &self.cer
    }
    #[doc = "0x100..0x120 - PWM Control Register"]
    #[inline(always)]
    pub const fn pcr(&self, n: usize) -> &PCR {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "0x104..0x124 - PWM Period Register"]
    #[inline(always)]
    pub const fn ppr(&self, n: usize) -> &PPR {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(260)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "0x108..0x128 - PWM Count Register"]
    #[inline(always)]
    pub const fn pcntr(&self, n: usize) -> &PCNTR {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(264)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "0x10c..0x12c - PWM Pulse Counter Register"]
    #[inline(always)]
    pub const fn ppcntr(&self, n: usize) -> &PPCNTR {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(268)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "0x110..0x130 - Capture Control Register"]
    #[inline(always)]
    pub const fn ccr(&self, n: usize) -> &CCR {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(272)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "0x114..0x134 - Capture Rise Lock Register"]
    #[inline(always)]
    pub const fn crlr(&self, n: usize) -> &CRLR {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(276)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "0x118..0x138 - Capture Fall Lock Register"]
    #[inline(always)]
    pub const fn cflr(&self, n: usize) -> &CFLR {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(280)
                .add(32 * n)
                .cast()
        }
    }
}
#[doc = "pier (rw) register accessor: PWM IRQ Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pier`] module"]
pub type PIER = crate::Reg<pier::PIER_SPEC>;
#[doc = "PWM IRQ Enable Register"]
pub mod pier;
#[doc = "pisr (rw) register accessor: PWM IRQ Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pisr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pisr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pisr`] module"]
pub type PISR = crate::Reg<pisr::PISR_SPEC>;
#[doc = "PWM IRQ Status Register"]
pub mod pisr;
#[doc = "cier (rw) register accessor: Capture IRQ Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cier`] module"]
pub type CIER = crate::Reg<cier::CIER_SPEC>;
#[doc = "Capture IRQ Enable Register"]
pub mod cier;
#[doc = "cisr (rw) register accessor: Capture IRQ Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cisr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cisr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cisr`] module"]
pub type CISR = crate::Reg<cisr::CISR_SPEC>;
#[doc = "Capture IRQ Status Register"]
pub mod cisr;
#[doc = "pccr01 (rw) register accessor: PWM01 Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pccr01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pccr01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pccr01`] module"]
pub type PCCR01 = crate::Reg<pccr01::PCCR01_SPEC>;
#[doc = "PWM01 Clock Configuration Register"]
pub mod pccr01;
#[doc = "pccr23 (rw) register accessor: PWM23 Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pccr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pccr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pccr23`] module"]
pub type PCCR23 = crate::Reg<pccr23::PCCR23_SPEC>;
#[doc = "PWM23 Clock Configuration Register"]
pub mod pccr23;
#[doc = "pccr45 (rw) register accessor: PWM45 Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pccr45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pccr45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pccr45`] module"]
pub type PCCR45 = crate::Reg<pccr45::PCCR45_SPEC>;
#[doc = "PWM45 Clock Configuration Register"]
pub mod pccr45;
#[doc = "pccr67 (rw) register accessor: PWM67 Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pccr67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pccr67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pccr67`] module"]
pub type PCCR67 = crate::Reg<pccr67::PCCR67_SPEC>;
#[doc = "PWM67 Clock Configuration Register"]
pub mod pccr67;
#[doc = "pcgr (rw) register accessor: PWM Clock Gating Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcgr`] module"]
pub type PCGR = crate::Reg<pcgr::PCGR_SPEC>;
#[doc = "PWM Clock Gating Register"]
pub mod pcgr;
#[doc = "pdzcr01 (rw) register accessor: PWM01 Dead Zone Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdzcr01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdzcr01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdzcr01`] module"]
pub type PDZCR01 = crate::Reg<pdzcr01::PDZCR01_SPEC>;
#[doc = "PWM01 Dead Zone Control Register"]
pub mod pdzcr01;
#[doc = "pdzcr23 (rw) register accessor: PWM23 Dead Zone Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdzcr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdzcr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdzcr23`] module"]
pub type PDZCR23 = crate::Reg<pdzcr23::PDZCR23_SPEC>;
#[doc = "PWM23 Dead Zone Control Register"]
pub mod pdzcr23;
#[doc = "pdzcr45 (rw) register accessor: PWM45 Dead Zone Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdzcr45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdzcr45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdzcr45`] module"]
pub type PDZCR45 = crate::Reg<pdzcr45::PDZCR45_SPEC>;
#[doc = "PWM45 Dead Zone Control Register"]
pub mod pdzcr45;
#[doc = "pdzcr67 (rw) register accessor: PWM67 Dead Zone Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdzcr67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdzcr67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdzcr67`] module"]
pub type PDZCR67 = crate::Reg<pdzcr67::PDZCR67_SPEC>;
#[doc = "PWM67 Dead Zone Control Register"]
pub mod pdzcr67;
#[doc = "per (rw) register accessor: PWM Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`per::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`per::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per`] module"]
pub type PER = crate::Reg<per::PER_SPEC>;
#[doc = "PWM Enable Register"]
pub mod per;
#[doc = "pgr (rw) register accessor: PWM Group\\[g\\] Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgr`] module"]
pub type PGR = crate::Reg<pgr::PGR_SPEC>;
#[doc = "PWM Group\\[g\\] Register"]
pub mod pgr;
#[doc = "cer (rw) register accessor: Capture Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cer`] module"]
pub type CER = crate::Reg<cer::CER_SPEC>;
#[doc = "Capture Enable Register"]
pub mod cer;
#[doc = "pcr (rw) register accessor: PWM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr`] module"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "PWM Control Register"]
pub mod pcr;
#[doc = "ppr (rw) register accessor: PWM Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppr`] module"]
pub type PPR = crate::Reg<ppr::PPR_SPEC>;
#[doc = "PWM Period Register"]
pub mod ppr;
#[doc = "pcntr (rw) register accessor: PWM Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcntr`] module"]
pub type PCNTR = crate::Reg<pcntr::PCNTR_SPEC>;
#[doc = "PWM Count Register"]
pub mod pcntr;
#[doc = "ppcntr (r) register accessor: PWM Pulse Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppcntr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppcntr`] module"]
pub type PPCNTR = crate::Reg<ppcntr::PPCNTR_SPEC>;
#[doc = "PWM Pulse Counter Register"]
pub mod ppcntr;
#[doc = "ccr (rw) register accessor: Capture Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Capture Control Register"]
pub mod ccr;
#[doc = "crlr (r) register accessor: Capture Rise Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crlr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crlr`] module"]
pub type CRLR = crate::Reg<crlr::CRLR_SPEC>;
#[doc = "Capture Rise Lock Register"]
pub mod crlr;
#[doc = "cflr (r) register accessor: Capture Fall Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cflr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cflr`] module"]
pub type CFLR = crate::Reg<cflr::CFLR_SPEC>;
#[doc = "Capture Fall Lock Register"]
pub mod cflr;
