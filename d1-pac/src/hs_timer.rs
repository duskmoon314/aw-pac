#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    hs_tmr_irq_en: HS_TMR_IRQ_EN,
    hs_tmr_irq_stas: HS_TMR_IRQ_STAS,
    _reserved2: [u8; 0x18],
    hs_tmr_ctrl: (),
    _reserved3: [u8; 0x04],
    hs_tmr_intv_lo: (),
    _reserved4: [u8; 0x04],
    hs_tmr_intv_hi: (),
    _reserved5: [u8; 0x04],
    hs_tmr_curnt_lo: (),
    _reserved6: [u8; 0x04],
    hs_tmr_curnt_hi: (),
}
impl RegisterBlock {
    #[doc = "0x00 - HS Timer IRQ Enable Register"]
    #[inline(always)]
    pub const fn hs_tmr_irq_en(&self) -> &HS_TMR_IRQ_EN {
        &self.hs_tmr_irq_en
    }
    #[doc = "0x04 - HS Timer Status Register"]
    #[inline(always)]
    pub const fn hs_tmr_irq_stas(&self) -> &HS_TMR_IRQ_STAS {
        &self.hs_tmr_irq_stas
    }
    #[doc = "0x20..0x28 - HS Timer Control Register"]
    #[inline(always)]
    pub const fn hs_tmr_ctrl(&self, n: usize) -> &HS_TMR_CTRL {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(32)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "0x20 - HS Timer Control Register"]
    #[inline(always)]
    pub const fn hs_tmr0_ctrl(&self) -> &HS_TMR_CTRL {
        self.hs_tmr_ctrl(0)
    }
    #[doc = "0x40 - HS Timer Control Register"]
    #[inline(always)]
    pub const fn hs_tmr1_ctrl(&self) -> &HS_TMR_CTRL {
        self.hs_tmr_ctrl(1)
    }
    #[doc = "0x24..0x2c - HS Timer Interval Value Low Register"]
    #[inline(always)]
    pub const fn hs_tmr_intv_lo(&self, n: usize) -> &HS_TMR_INTV_LO {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(36)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "0x24 - HS Timer Interval Value Low Register"]
    #[inline(always)]
    pub const fn hs_tmr0_intv_lo(&self) -> &HS_TMR_INTV_LO {
        self.hs_tmr_intv_lo(0)
    }
    #[doc = "0x44 - HS Timer Interval Value Low Register"]
    #[inline(always)]
    pub const fn hs_tmr1_intv_lo(&self) -> &HS_TMR_INTV_LO {
        self.hs_tmr_intv_lo(1)
    }
    #[doc = "0x28..0x30 - HS Timer Interval Value High Register"]
    #[inline(always)]
    pub const fn hs_tmr_intv_hi(&self, n: usize) -> &HS_TMR_INTV_HI {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(40)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "0x28 - HS Timer Interval Value High Register"]
    #[inline(always)]
    pub const fn hs_tmr0_intv_hi(&self) -> &HS_TMR_INTV_HI {
        self.hs_tmr_intv_hi(0)
    }
    #[doc = "0x48 - HS Timer Interval Value High Register"]
    #[inline(always)]
    pub const fn hs_tmr1_intv_hi(&self) -> &HS_TMR_INTV_HI {
        self.hs_tmr_intv_hi(1)
    }
    #[doc = "0x2c..0x34 - HS Timer Current Value Low Register"]
    #[inline(always)]
    pub const fn hs_tmr_curnt_lo(&self, n: usize) -> &HS_TMR_CURNT_LO {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(44)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "0x2c - HS Timer Current Value Low Register"]
    #[inline(always)]
    pub const fn hs_tmr0_curnt_lo(&self) -> &HS_TMR_CURNT_LO {
        self.hs_tmr_curnt_lo(0)
    }
    #[doc = "0x4c - HS Timer Current Value Low Register"]
    #[inline(always)]
    pub const fn hs_tmr1_curnt_lo(&self) -> &HS_TMR_CURNT_LO {
        self.hs_tmr_curnt_lo(1)
    }
    #[doc = "0x30..0x38 - HS Timer Current Value High Register"]
    #[inline(always)]
    pub const fn hs_tmr_curnt_hi(&self, n: usize) -> &HS_TMR_CURNT_HI {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(48)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "0x30 - HS Timer Current Value High Register"]
    #[inline(always)]
    pub const fn hs_tmr0_curnt_hi(&self) -> &HS_TMR_CURNT_HI {
        self.hs_tmr_curnt_hi(0)
    }
    #[doc = "0x50 - HS Timer Current Value High Register"]
    #[inline(always)]
    pub const fn hs_tmr1_curnt_hi(&self) -> &HS_TMR_CURNT_HI {
        self.hs_tmr_curnt_hi(1)
    }
}
#[doc = "hs_tmr_irq_en (rw) register accessor: HS Timer IRQ Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_tmr_irq_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_tmr_irq_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hs_tmr_irq_en`] module"]
pub type HS_TMR_IRQ_EN = crate::Reg<hs_tmr_irq_en::HS_TMR_IRQ_EN_SPEC>;
#[doc = "HS Timer IRQ Enable Register"]
pub mod hs_tmr_irq_en;
#[doc = "hs_tmr_irq_stas (rw) register accessor: HS Timer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_tmr_irq_stas::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_tmr_irq_stas::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hs_tmr_irq_stas`] module"]
pub type HS_TMR_IRQ_STAS = crate::Reg<hs_tmr_irq_stas::HS_TMR_IRQ_STAS_SPEC>;
#[doc = "HS Timer Status Register"]
pub mod hs_tmr_irq_stas;
#[doc = "hs_tmr_ctrl (rw) register accessor: HS Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_tmr_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_tmr_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hs_tmr_ctrl`] module"]
pub type HS_TMR_CTRL = crate::Reg<hs_tmr_ctrl::HS_TMR_CTRL_SPEC>;
#[doc = "HS Timer Control Register"]
pub mod hs_tmr_ctrl;
#[doc = "hs_tmr_intv_lo (rw) register accessor: HS Timer Interval Value Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_tmr_intv_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_tmr_intv_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hs_tmr_intv_lo`] module"]
pub type HS_TMR_INTV_LO = crate::Reg<hs_tmr_intv_lo::HS_TMR_INTV_LO_SPEC>;
#[doc = "HS Timer Interval Value Low Register"]
pub mod hs_tmr_intv_lo;
#[doc = "hs_tmr_intv_hi (rw) register accessor: HS Timer Interval Value High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_tmr_intv_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_tmr_intv_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hs_tmr_intv_hi`] module"]
pub type HS_TMR_INTV_HI = crate::Reg<hs_tmr_intv_hi::HS_TMR_INTV_HI_SPEC>;
#[doc = "HS Timer Interval Value High Register"]
pub mod hs_tmr_intv_hi;
#[doc = "hs_tmr_curnt_lo (rw) register accessor: HS Timer Current Value Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_tmr_curnt_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_tmr_curnt_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hs_tmr_curnt_lo`] module"]
pub type HS_TMR_CURNT_LO = crate::Reg<hs_tmr_curnt_lo::HS_TMR_CURNT_LO_SPEC>;
#[doc = "HS Timer Current Value Low Register"]
pub mod hs_tmr_curnt_lo;
#[doc = "hs_tmr_curnt_hi (rw) register accessor: HS Timer Current Value High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_tmr_curnt_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_tmr_curnt_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hs_tmr_curnt_hi`] module"]
pub type HS_TMR_CURNT_HI = crate::Reg<hs_tmr_curnt_hi::HS_TMR_CURNT_HI_SPEC>;
#[doc = "HS Timer Current Value High Register"]
pub mod hs_tmr_curnt_hi;
