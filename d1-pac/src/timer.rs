#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tmr_irq_en: TMR_IRQ_EN,
    tmr_irq_sta: TMR_IRQ_STA,
    _reserved2: [u8; 0x08],
    tmr_ctrl: (),
    _reserved3: [u8; 0x04],
    tmr_intv_value: (),
    _reserved4: [u8; 0x04],
    tmr_cur_value: (),
    _reserved5: [u8; 0x88],
    wdog_irq_en: WDOG_IRQ_EN,
    wdog_irq_sta: WDOG_IRQ_STA,
    wdog_soft_rst: WDOG_SOFT_RST,
    _reserved8: [u8; 0x04],
    wdog_ctrl: WDOG_CTRL,
    wdog_cfg: WDOG_CFG,
    wdog_mode: WDOG_MODE,
    wdog_output_cfg: WDOG_OUTPUT_CFG,
    avs_cnt_ctl: AVS_CNT_CTL,
    avs_cnt0: AVS_CNT0,
    avs_cnt1: AVS_CNT1,
    avs_cnt_div: AVS_CNT_DIV,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer IRQ Enable Register"]
    #[inline(always)]
    pub const fn tmr_irq_en(&self) -> &TMR_IRQ_EN {
        &self.tmr_irq_en
    }
    #[doc = "0x04 - Timer Status Register"]
    #[inline(always)]
    pub const fn tmr_irq_sta(&self) -> &TMR_IRQ_STA {
        &self.tmr_irq_sta
    }
    #[doc = "0x10..0x18 - Timer IRQ Enable Register"]
    #[inline(always)]
    pub const fn tmr_ctrl(&self, n: usize) -> &TMR_CTRL {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "0x10 - Timer IRQ Enable Register"]
    #[inline(always)]
    pub const fn tmr0_ctrl(&self) -> &TMR_CTRL {
        self.tmr_ctrl(0)
    }
    #[doc = "0x20 - Timer IRQ Enable Register"]
    #[inline(always)]
    pub const fn tmr1_ctrl(&self) -> &TMR_CTRL {
        self.tmr_ctrl(1)
    }
    #[doc = "0x14..0x1c - Timer Interval Value Register"]
    #[inline(always)]
    pub const fn tmr_intv_value(&self, n: usize) -> &TMR_INTV_VALUE {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(20)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "0x14 - Timer Interval Value Register"]
    #[inline(always)]
    pub const fn tmr0_intv_value(&self) -> &TMR_INTV_VALUE {
        self.tmr_intv_value(0)
    }
    #[doc = "0x24 - Timer Interval Value Register"]
    #[inline(always)]
    pub const fn tmr1_intv_value(&self) -> &TMR_INTV_VALUE {
        self.tmr_intv_value(1)
    }
    #[doc = "0x18..0x20 - Timer Current Value Register"]
    #[inline(always)]
    pub const fn tmr_cur_value(&self, n: usize) -> &TMR_CUR_VALUE {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(24)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "0x18 - Timer Current Value Register"]
    #[inline(always)]
    pub const fn tmr0_cur_value(&self) -> &TMR_CUR_VALUE {
        self.tmr_cur_value(0)
    }
    #[doc = "0x28 - Timer Current Value Register"]
    #[inline(always)]
    pub const fn tmr1_cur_value(&self) -> &TMR_CUR_VALUE {
        self.tmr_cur_value(1)
    }
    #[doc = "0xa0 - Watchdog IRQ Enable Register"]
    #[inline(always)]
    pub const fn wdog_irq_en(&self) -> &WDOG_IRQ_EN {
        &self.wdog_irq_en
    }
    #[doc = "0xa4 - Watchdog Status Register"]
    #[inline(always)]
    pub const fn wdog_irq_sta(&self) -> &WDOG_IRQ_STA {
        &self.wdog_irq_sta
    }
    #[doc = "0xa8 - Watchdog Software Reset Register"]
    #[inline(always)]
    pub const fn wdog_soft_rst(&self) -> &WDOG_SOFT_RST {
        &self.wdog_soft_rst
    }
    #[doc = "0xb0 - Watchdog Control Register"]
    #[inline(always)]
    pub const fn wdog_ctrl(&self) -> &WDOG_CTRL {
        &self.wdog_ctrl
    }
    #[doc = "0xb4 - Watchdog Configuration Register"]
    #[inline(always)]
    pub const fn wdog_cfg(&self) -> &WDOG_CFG {
        &self.wdog_cfg
    }
    #[doc = "0xb8 - Watchdog Mode Register"]
    #[inline(always)]
    pub const fn wdog_mode(&self) -> &WDOG_MODE {
        &self.wdog_mode
    }
    #[doc = "0xbc - Watchdog Output Configuration Register"]
    #[inline(always)]
    pub const fn wdog_output_cfg(&self) -> &WDOG_OUTPUT_CFG {
        &self.wdog_output_cfg
    }
    #[doc = "0xc0 - AVS Counter Control Register"]
    #[inline(always)]
    pub const fn avs_cnt_ctl(&self) -> &AVS_CNT_CTL {
        &self.avs_cnt_ctl
    }
    #[doc = "0xc4 - AVS Counter 0 Register"]
    #[inline(always)]
    pub const fn avs_cnt0(&self) -> &AVS_CNT0 {
        &self.avs_cnt0
    }
    #[doc = "0xc8 - AVS Counter 1 Register"]
    #[inline(always)]
    pub const fn avs_cnt1(&self) -> &AVS_CNT1 {
        &self.avs_cnt1
    }
    #[doc = "0xcc - AVS Counter Divisor Register"]
    #[inline(always)]
    pub const fn avs_cnt_div(&self) -> &AVS_CNT_DIV {
        &self.avs_cnt_div
    }
}
#[doc = "tmr_irq_en (rw) register accessor: Timer IRQ Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr_irq_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr_irq_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr_irq_en`] module"]
pub type TMR_IRQ_EN = crate::Reg<tmr_irq_en::TMR_IRQ_EN_SPEC>;
#[doc = "Timer IRQ Enable Register"]
pub mod tmr_irq_en;
#[doc = "tmr_irq_sta (rw) register accessor: Timer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr_irq_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr_irq_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr_irq_sta`] module"]
pub type TMR_IRQ_STA = crate::Reg<tmr_irq_sta::TMR_IRQ_STA_SPEC>;
#[doc = "Timer Status Register"]
pub mod tmr_irq_sta;
#[doc = "tmr_ctrl (rw) register accessor: Timer IRQ Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr_ctrl`] module"]
pub type TMR_CTRL = crate::Reg<tmr_ctrl::TMR_CTRL_SPEC>;
#[doc = "Timer IRQ Enable Register"]
pub mod tmr_ctrl;
#[doc = "tmr_intv_value (rw) register accessor: Timer Interval Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr_intv_value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr_intv_value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr_intv_value`] module"]
pub type TMR_INTV_VALUE = crate::Reg<tmr_intv_value::TMR_INTV_VALUE_SPEC>;
#[doc = "Timer Interval Value Register"]
pub mod tmr_intv_value;
#[doc = "tmr_cur_value (rw) register accessor: Timer Current Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr_cur_value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr_cur_value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr_cur_value`] module"]
pub type TMR_CUR_VALUE = crate::Reg<tmr_cur_value::TMR_CUR_VALUE_SPEC>;
#[doc = "Timer Current Value Register"]
pub mod tmr_cur_value;
#[doc = "wdog_irq_en (rw) register accessor: Watchdog IRQ Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdog_irq_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdog_irq_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdog_irq_en`] module"]
pub type WDOG_IRQ_EN = crate::Reg<wdog_irq_en::WDOG_IRQ_EN_SPEC>;
#[doc = "Watchdog IRQ Enable Register"]
pub mod wdog_irq_en;
#[doc = "wdog_irq_sta (rw) register accessor: Watchdog Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdog_irq_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdog_irq_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdog_irq_sta`] module"]
pub type WDOG_IRQ_STA = crate::Reg<wdog_irq_sta::WDOG_IRQ_STA_SPEC>;
#[doc = "Watchdog Status Register"]
pub mod wdog_irq_sta;
#[doc = "wdog_soft_rst (rw) register accessor: Watchdog Software Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdog_soft_rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdog_soft_rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdog_soft_rst`] module"]
pub type WDOG_SOFT_RST = crate::Reg<wdog_soft_rst::WDOG_SOFT_RST_SPEC>;
#[doc = "Watchdog Software Reset Register"]
pub mod wdog_soft_rst;
#[doc = "wdog_ctrl (rw) register accessor: Watchdog Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdog_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdog_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdog_ctrl`] module"]
pub type WDOG_CTRL = crate::Reg<wdog_ctrl::WDOG_CTRL_SPEC>;
#[doc = "Watchdog Control Register"]
pub mod wdog_ctrl;
#[doc = "wdog_cfg (rw) register accessor: Watchdog Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdog_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdog_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdog_cfg`] module"]
pub type WDOG_CFG = crate::Reg<wdog_cfg::WDOG_CFG_SPEC>;
#[doc = "Watchdog Configuration Register"]
pub mod wdog_cfg;
#[doc = "wdog_mode (rw) register accessor: Watchdog Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdog_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdog_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdog_mode`] module"]
pub type WDOG_MODE = crate::Reg<wdog_mode::WDOG_MODE_SPEC>;
#[doc = "Watchdog Mode Register"]
pub mod wdog_mode;
#[doc = "wdog_output_cfg (rw) register accessor: Watchdog Output Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdog_output_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdog_output_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdog_output_cfg`] module"]
pub type WDOG_OUTPUT_CFG = crate::Reg<wdog_output_cfg::WDOG_OUTPUT_CFG_SPEC>;
#[doc = "Watchdog Output Configuration Register"]
pub mod wdog_output_cfg;
#[doc = "avs_cnt_ctl (rw) register accessor: AVS Counter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`avs_cnt_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`avs_cnt_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@avs_cnt_ctl`] module"]
pub type AVS_CNT_CTL = crate::Reg<avs_cnt_ctl::AVS_CNT_CTL_SPEC>;
#[doc = "AVS Counter Control Register"]
pub mod avs_cnt_ctl;
#[doc = "avs_cnt0 (rw) register accessor: AVS Counter 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`avs_cnt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`avs_cnt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@avs_cnt0`] module"]
pub type AVS_CNT0 = crate::Reg<avs_cnt0::AVS_CNT0_SPEC>;
#[doc = "AVS Counter 0 Register"]
pub mod avs_cnt0;
#[doc = "avs_cnt1 (rw) register accessor: AVS Counter 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`avs_cnt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`avs_cnt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@avs_cnt1`] module"]
pub type AVS_CNT1 = crate::Reg<avs_cnt1::AVS_CNT1_SPEC>;
#[doc = "AVS Counter 1 Register"]
pub mod avs_cnt1;
#[doc = "avs_cnt_div (rw) register accessor: AVS Counter Divisor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`avs_cnt_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`avs_cnt_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@avs_cnt_div`] module"]
pub type AVS_CNT_DIV = crate::Reg<avs_cnt_div::AVS_CNT_DIV_SPEC>;
#[doc = "AVS Counter Divisor Register"]
pub mod avs_cnt_div;
