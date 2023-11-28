#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    dmac_irq_en0: DMAC_IRQ_EN0,
    dmac_irq_en1: DMAC_IRQ_EN1,
    _reserved2: [u8; 0x08],
    dmac_irq_pend0: DMAC_IRQ_PEND0,
    dmac_irq_pend1: DMAC_IRQ_PEND1,
    _reserved4: [u8; 0x10],
    dmac_auto_gate: DMAC_AUTO_GATE,
    _reserved5: [u8; 0x04],
    dmac_sta: DMAC_STA,
    _reserved6: [u8; 0xcc],
    dmac_en: (),
    _reserved7: [u8; 0x04],
    dmac_pau: (),
    _reserved8: [u8; 0x04],
    dmac_desc_addr: (),
    _reserved9: [u8; 0x04],
    dmac_cfg: (),
    _reserved10: [u8; 0x04],
    dmac_cur_src: (),
    _reserved11: [u8; 0x04],
    dmac_cur_dest: (),
    _reserved12: [u8; 0x04],
    dmac_bcnt_left: (),
    _reserved13: [u8; 0x04],
    dmac_para: (),
    _reserved14: [u8; 0x0c],
    dmac_mode: (),
    _reserved15: [u8; 0x04],
    dmac_fdesc_addr: (),
    _reserved16: [u8; 0x04],
    dmac_pkg_num: (),
}
impl RegisterBlock {
    #[doc = "0x00 - DMAC IRQ Enable Register 0"]
    #[inline(always)]
    pub const fn dmac_irq_en0(&self) -> &DMAC_IRQ_EN0 {
        &self.dmac_irq_en0
    }
    #[doc = "0x04 - DMAC IRQ Enable Register 1"]
    #[inline(always)]
    pub const fn dmac_irq_en1(&self) -> &DMAC_IRQ_EN1 {
        &self.dmac_irq_en1
    }
    #[doc = "0x10 - DMAC IRQ Pending Register 0"]
    #[inline(always)]
    pub const fn dmac_irq_pend0(&self) -> &DMAC_IRQ_PEND0 {
        &self.dmac_irq_pend0
    }
    #[doc = "0x14 - DMAC IRQ Pending Register 1"]
    #[inline(always)]
    pub const fn dmac_irq_pend1(&self) -> &DMAC_IRQ_PEND1 {
        &self.dmac_irq_pend1
    }
    #[doc = "0x28 - DMAC Auto Gating Register"]
    #[inline(always)]
    pub const fn dmac_auto_gate(&self) -> &DMAC_AUTO_GATE {
        &self.dmac_auto_gate
    }
    #[doc = "0x30 - DMAC Status Register"]
    #[inline(always)]
    pub const fn dmac_sta(&self) -> &DMAC_STA {
        &self.dmac_sta
    }
    #[doc = "0x100..0x140 - DMAC Channel Enable Register"]
    #[inline(always)]
    pub const fn dmac_en(&self, n: usize) -> &DMAC_EN {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "0x104..0x144 - DMAC Channel Pause Register"]
    #[inline(always)]
    pub const fn dmac_pau(&self, n: usize) -> &DMAC_PAU {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(260)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "0x108..0x148 - DMAC Channel Start Address Register"]
    #[inline(always)]
    pub const fn dmac_desc_addr(&self, n: usize) -> &DMAC_DESC_ADDR {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(264)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "0x10c..0x14c - DMAC Channel Configuration Register"]
    #[inline(always)]
    pub const fn dmac_cfg(&self, n: usize) -> &DMAC_CFG {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(268)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "0x110..0x150 - DMAC Channel Current Source Register"]
    #[inline(always)]
    pub const fn dmac_cur_src(&self, n: usize) -> &DMAC_CUR_SRC {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(272)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "0x114..0x154 - DMAC Channel Current Destination Register"]
    #[inline(always)]
    pub const fn dmac_cur_dest(&self, n: usize) -> &DMAC_CUR_DEST {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(276)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "0x118..0x158 - DMAC Channel Byte Counter Left Register"]
    #[inline(always)]
    pub const fn dmac_bcnt_left(&self, n: usize) -> &DMAC_BCNT_LEFT {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(280)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "0x11c..0x15c - DMAC Channel Parameter Register"]
    #[inline(always)]
    pub const fn dmac_para(&self, n: usize) -> &DMAC_PARA {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(284)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "0x128..0x168 - DMAC Mode Register"]
    #[inline(always)]
    pub const fn dmac_mode(&self, n: usize) -> &DMAC_MODE {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(296)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "0x12c..0x16c - DMAC Former Descriptor Address Register"]
    #[inline(always)]
    pub const fn dmac_fdesc_addr(&self, n: usize) -> &DMAC_FDESC_ADDR {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(300)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "0x130..0x170 - DMAC Package Number Register"]
    #[inline(always)]
    pub const fn dmac_pkg_num(&self, n: usize) -> &DMAC_PKG_NUM {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(304)
                .add(64 * n)
                .cast()
        }
    }
}
#[doc = "dmac_irq_en0 (rw) register accessor: DMAC IRQ Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_irq_en0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_irq_en0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_irq_en0`] module"]
pub type DMAC_IRQ_EN0 = crate::Reg<dmac_irq_en0::DMAC_IRQ_EN0_SPEC>;
#[doc = "DMAC IRQ Enable Register 0"]
pub mod dmac_irq_en0;
#[doc = "dmac_irq_en1 (rw) register accessor: DMAC IRQ Enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_irq_en1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_irq_en1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_irq_en1`] module"]
pub type DMAC_IRQ_EN1 = crate::Reg<dmac_irq_en1::DMAC_IRQ_EN1_SPEC>;
#[doc = "DMAC IRQ Enable Register 1"]
pub mod dmac_irq_en1;
#[doc = "dmac_irq_pend0 (rw) register accessor: DMAC IRQ Pending Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_irq_pend0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_irq_pend0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_irq_pend0`] module"]
pub type DMAC_IRQ_PEND0 = crate::Reg<dmac_irq_pend0::DMAC_IRQ_PEND0_SPEC>;
#[doc = "DMAC IRQ Pending Register 0"]
pub mod dmac_irq_pend0;
#[doc = "dmac_irq_pend1 (rw) register accessor: DMAC IRQ Pending Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_irq_pend1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_irq_pend1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_irq_pend1`] module"]
pub type DMAC_IRQ_PEND1 = crate::Reg<dmac_irq_pend1::DMAC_IRQ_PEND1_SPEC>;
#[doc = "DMAC IRQ Pending Register 1"]
pub mod dmac_irq_pend1;
#[doc = "dmac_auto_gate (rw) register accessor: DMAC Auto Gating Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_auto_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_auto_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_auto_gate`] module"]
pub type DMAC_AUTO_GATE = crate::Reg<dmac_auto_gate::DMAC_AUTO_GATE_SPEC>;
#[doc = "DMAC Auto Gating Register"]
pub mod dmac_auto_gate;
#[doc = "dmac_sta (r) register accessor: DMAC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_sta::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_sta`] module"]
pub type DMAC_STA = crate::Reg<dmac_sta::DMAC_STA_SPEC>;
#[doc = "DMAC Status Register"]
pub mod dmac_sta;
#[doc = "dmac_en (rw) register accessor: DMAC Channel Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_en`] module"]
pub type DMAC_EN = crate::Reg<dmac_en::DMAC_EN_SPEC>;
#[doc = "DMAC Channel Enable Register"]
pub mod dmac_en;
#[doc = "dmac_pau (rw) register accessor: DMAC Channel Pause Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_pau::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_pau::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_pau`] module"]
pub type DMAC_PAU = crate::Reg<dmac_pau::DMAC_PAU_SPEC>;
#[doc = "DMAC Channel Pause Register"]
pub mod dmac_pau;
#[doc = "dmac_desc_addr (rw) register accessor: DMAC Channel Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_desc_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_desc_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_desc_addr`] module"]
pub type DMAC_DESC_ADDR = crate::Reg<dmac_desc_addr::DMAC_DESC_ADDR_SPEC>;
#[doc = "DMAC Channel Start Address Register"]
pub mod dmac_desc_addr;
#[doc = "dmac_cfg (r) register accessor: DMAC Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_cfg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_cfg`] module"]
pub type DMAC_CFG = crate::Reg<dmac_cfg::DMAC_CFG_SPEC>;
#[doc = "DMAC Channel Configuration Register"]
pub mod dmac_cfg;
#[doc = "dmac_cur_src (r) register accessor: DMAC Channel Current Source Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_cur_src::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_cur_src`] module"]
pub type DMAC_CUR_SRC = crate::Reg<dmac_cur_src::DMAC_CUR_SRC_SPEC>;
#[doc = "DMAC Channel Current Source Register"]
pub mod dmac_cur_src;
#[doc = "dmac_cur_dest (r) register accessor: DMAC Channel Current Destination Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_cur_dest::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_cur_dest`] module"]
pub type DMAC_CUR_DEST = crate::Reg<dmac_cur_dest::DMAC_CUR_DEST_SPEC>;
#[doc = "DMAC Channel Current Destination Register"]
pub mod dmac_cur_dest;
#[doc = "dmac_bcnt_left (r) register accessor: DMAC Channel Byte Counter Left Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_bcnt_left::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_bcnt_left`] module"]
pub type DMAC_BCNT_LEFT = crate::Reg<dmac_bcnt_left::DMAC_BCNT_LEFT_SPEC>;
#[doc = "DMAC Channel Byte Counter Left Register"]
pub mod dmac_bcnt_left;
#[doc = "dmac_para (r) register accessor: DMAC Channel Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_para::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_para`] module"]
pub type DMAC_PARA = crate::Reg<dmac_para::DMAC_PARA_SPEC>;
#[doc = "DMAC Channel Parameter Register"]
pub mod dmac_para;
#[doc = "dmac_mode (rw) register accessor: DMAC Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_mode`] module"]
pub type DMAC_MODE = crate::Reg<dmac_mode::DMAC_MODE_SPEC>;
#[doc = "DMAC Mode Register"]
pub mod dmac_mode;
#[doc = "dmac_fdesc_addr (r) register accessor: DMAC Former Descriptor Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_fdesc_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_fdesc_addr`] module"]
pub type DMAC_FDESC_ADDR = crate::Reg<dmac_fdesc_addr::DMAC_FDESC_ADDR_SPEC>;
#[doc = "DMAC Former Descriptor Address Register"]
pub mod dmac_fdesc_addr;
#[doc = "dmac_pkg_num (r) register accessor: DMAC Package Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_pkg_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_pkg_num`] module"]
pub type DMAC_PKG_NUM = crate::Reg<dmac_pkg_num::DMAC_PKG_NUM_SPEC>;
#[doc = "DMAC Package Number Register"]
pub mod dmac_pkg_num;
