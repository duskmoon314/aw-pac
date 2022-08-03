#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer IRQ Enable Register"]
    pub tmr_irq_en: TMR_IRQ_EN,
    #[doc = "0x04 - Timer Status Register"]
    pub tmr_irq_sta: TMR_IRQ_STA,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Timer IRQ Enable Register"]
    pub tmr0_ctrl: TMR_CTRL,
    #[doc = "0x14 - Timer Interval Value Register"]
    pub tmr0_intv_value: TMR_INTV_VALUE,
    #[doc = "0x18 - Timer Current Value Register"]
    pub tmr0_cur_value: TMR_CUR_VALUE,
    _reserved5: [u8; 0x04],
    #[doc = "0x20 - Timer IRQ Enable Register"]
    pub tmr1_ctrl: TMR_CTRL,
    #[doc = "0x24 - Timer Interval Value Register"]
    pub tmr1_intv_value: TMR_INTV_VALUE,
    #[doc = "0x28 - Timer Current Value Register"]
    pub tmr1_cur_value: TMR_CUR_VALUE,
    _reserved8: [u8; 0x74],
    #[doc = "0xa0 - Watchdog IRQ Enable Register"]
    pub wdog_irq_en: WDOG_IRQ_EN,
    #[doc = "0xa4 - Watchdog Status Register"]
    pub wdog_irq_sta: WDOG_IRQ_STA,
    #[doc = "0xa8 - Watchdog Software Reset Register"]
    pub wdog_soft_rst: WDOG_SOFT_RST,
    _reserved11: [u8; 0x04],
    #[doc = "0xb0 - Watchdog Control Register"]
    pub wdog_ctrl: WDOG_CTRL,
    #[doc = "0xb4 - Watchdog Configuration Register"]
    pub wdog_cfg: WDOG_CFG,
    #[doc = "0xb8 - Watchdog Mode Register"]
    pub wdog_mode: WDOG_MODE,
    #[doc = "0xbc - Watchdog Output Configuration Register"]
    pub wdog_output_cfg: WDOG_OUTPUT_CFG,
    #[doc = "0xc0 - AVS Counter Control Register"]
    pub avs_cnt_ctl: AVS_CNT_CTL,
    #[doc = "0xc4 - AVS Counter 0 Register"]
    pub avs_cnt0: AVS_CNT0,
    #[doc = "0xc8 - AVS Counter 1 Register"]
    pub avs_cnt1: AVS_CNT1,
    #[doc = "0xcc - AVS Counter Divisor Register"]
    pub avs_cnt_div: AVS_CNT_DIV,
}
#[doc = "tmr_irq_en (rw) register accessor: an alias for `Reg<TMR_IRQ_EN_SPEC>`"]
pub type TMR_IRQ_EN = crate::Reg<tmr_irq_en::TMR_IRQ_EN_SPEC>;
#[doc = "Timer IRQ Enable Register"]
pub mod tmr_irq_en;
#[doc = "tmr_irq_sta (rw) register accessor: an alias for `Reg<TMR_IRQ_STA_SPEC>`"]
pub type TMR_IRQ_STA = crate::Reg<tmr_irq_sta::TMR_IRQ_STA_SPEC>;
#[doc = "Timer Status Register"]
pub mod tmr_irq_sta;
#[doc = "tmr_ctrl (rw) register accessor: an alias for `Reg<TMR_CTRL_SPEC>`"]
pub type TMR_CTRL = crate::Reg<tmr_ctrl::TMR_CTRL_SPEC>;
#[doc = "Timer IRQ Enable Register"]
pub mod tmr_ctrl;
#[doc = "tmr_intv_value (rw) register accessor: an alias for `Reg<TMR_INTV_VALUE_SPEC>`"]
pub type TMR_INTV_VALUE = crate::Reg<tmr_intv_value::TMR_INTV_VALUE_SPEC>;
#[doc = "Timer Interval Value Register"]
pub mod tmr_intv_value;
#[doc = "tmr_cur_value (rw) register accessor: an alias for `Reg<TMR_CUR_VALUE_SPEC>`"]
pub type TMR_CUR_VALUE = crate::Reg<tmr_cur_value::TMR_CUR_VALUE_SPEC>;
#[doc = "Timer Current Value Register"]
pub mod tmr_cur_value;
#[doc = "wdog_irq_en (rw) register accessor: an alias for `Reg<WDOG_IRQ_EN_SPEC>`"]
pub type WDOG_IRQ_EN = crate::Reg<wdog_irq_en::WDOG_IRQ_EN_SPEC>;
#[doc = "Watchdog IRQ Enable Register"]
pub mod wdog_irq_en;
#[doc = "wdog_irq_sta (rw) register accessor: an alias for `Reg<WDOG_IRQ_STA_SPEC>`"]
pub type WDOG_IRQ_STA = crate::Reg<wdog_irq_sta::WDOG_IRQ_STA_SPEC>;
#[doc = "Watchdog Status Register"]
pub mod wdog_irq_sta;
#[doc = "wdog_soft_rst (rw) register accessor: an alias for `Reg<WDOG_SOFT_RST_SPEC>`"]
pub type WDOG_SOFT_RST = crate::Reg<wdog_soft_rst::WDOG_SOFT_RST_SPEC>;
#[doc = "Watchdog Software Reset Register"]
pub mod wdog_soft_rst;
#[doc = "wdog_ctrl (rw) register accessor: an alias for `Reg<WDOG_CTRL_SPEC>`"]
pub type WDOG_CTRL = crate::Reg<wdog_ctrl::WDOG_CTRL_SPEC>;
#[doc = "Watchdog Control Register"]
pub mod wdog_ctrl;
#[doc = "wdog_cfg (rw) register accessor: an alias for `Reg<WDOG_CFG_SPEC>`"]
pub type WDOG_CFG = crate::Reg<wdog_cfg::WDOG_CFG_SPEC>;
#[doc = "Watchdog Configuration Register"]
pub mod wdog_cfg;
#[doc = "wdog_mode (rw) register accessor: an alias for `Reg<WDOG_MODE_SPEC>`"]
pub type WDOG_MODE = crate::Reg<wdog_mode::WDOG_MODE_SPEC>;
#[doc = "Watchdog Mode Register"]
pub mod wdog_mode;
#[doc = "wdog_output_cfg (rw) register accessor: an alias for `Reg<WDOG_OUTPUT_CFG_SPEC>`"]
pub type WDOG_OUTPUT_CFG = crate::Reg<wdog_output_cfg::WDOG_OUTPUT_CFG_SPEC>;
#[doc = "Watchdog Output Configuration Register"]
pub mod wdog_output_cfg;
#[doc = "avs_cnt_ctl (rw) register accessor: an alias for `Reg<AVS_CNT_CTL_SPEC>`"]
pub type AVS_CNT_CTL = crate::Reg<avs_cnt_ctl::AVS_CNT_CTL_SPEC>;
#[doc = "AVS Counter Control Register"]
pub mod avs_cnt_ctl;
#[doc = "avs_cnt0 (rw) register accessor: an alias for `Reg<AVS_CNT0_SPEC>`"]
pub type AVS_CNT0 = crate::Reg<avs_cnt0::AVS_CNT0_SPEC>;
#[doc = "AVS Counter 0 Register"]
pub mod avs_cnt0;
#[doc = "avs_cnt1 (rw) register accessor: an alias for `Reg<AVS_CNT1_SPEC>`"]
pub type AVS_CNT1 = crate::Reg<avs_cnt1::AVS_CNT1_SPEC>;
#[doc = "AVS Counter 1 Register"]
pub mod avs_cnt1;
#[doc = "avs_cnt_div (rw) register accessor: an alias for `Reg<AVS_CNT_DIV_SPEC>`"]
pub type AVS_CNT_DIV = crate::Reg<avs_cnt_div::AVS_CNT_DIV_SPEC>;
#[doc = "AVS Counter Divisor Register"]
pub mod avs_cnt_div;
