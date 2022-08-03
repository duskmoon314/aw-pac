#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Low Oscillator Control Register"]
    pub losc_ctrl: LOSC_CTRL,
    #[doc = "0x04 - LOSC Auto Switch Status Register"]
    pub losc_auto_swt_sta: LOSC_AUTO_SWT_STA,
    #[doc = "0x08 - Internal OSC Clock Pre-scalar Register"]
    pub intosc_clk_prescal: INTOSC_CLK_PRESCAL,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - RTC Year-Month-Day Register"]
    pub rtc_day: RTC_DAY,
    #[doc = "0x14 - RTC Hour-Minute-Second Register"]
    pub rtc_hh_mm_ss: RTC_HH_MM_SS,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - Alarm 0 Day Setting Register"]
    pub alarm0_day_set: ALARM0_DAY_SET,
    #[doc = "0x24 - Alarm 0 Counter Current Value Register"]
    pub alarm0_cur_vlu: ALARM0_CUR_VLU,
    #[doc = "0x28 - Alarm 0 Enable Register"]
    pub alarm0_enable: ALARM0_ENABLE,
    #[doc = "0x2c - Alarm 0 IRQ Enable Register"]
    pub alarm0_irq_en: ALARM0_IRQ_EN,
    #[doc = "0x30 - Alarm 0 IRQ Status Register"]
    pub alarm0_irq_sta: ALARM0_IRQ_STA,
    _reserved10: [u8; 0x1c],
    #[doc = "0x50 - Alarm Configuration Register"]
    pub alarm_config: ALARM_CONFIG,
    _reserved11: [u8; 0x0c],
    #[doc = "0x60 - 32K Fanout Control Gating Register"]
    pub fout_32k_ctrl_gating: FOUT_32K_CTRL_GATING,
    _reserved12: [u8; 0x9c],
    #[doc = "0x100..0x120 - General Purpose Register"]
    pub gp_data: [GP_DATA; 8],
    #[doc = "0x120..0x128 - Fast Boot Information Register \\[01\\]"]
    pub fboot_info: [FBOOT_INFO; 2],
    _reserved14: [u8; 0x38],
    #[doc = "0x160 - DCXO Control Register"]
    pub dcxo_ctrl: DCXO_CTRL,
    _reserved15: [u8; 0x2c],
    #[doc = "0x190 - RTC_VIO Regulation Register"]
    pub rtc_vio: RTC_VIO,
    _reserved16: [u8; 0x5c],
    #[doc = "0x1f0 - IC Characteristic Register"]
    pub ic_chara: IC_CHARA,
    #[doc = "0x1f4 - VDD Off Gating Control Register"]
    pub vdd_off_gating_ctrl: VDD_OFF_GATING_CTRL,
    _reserved18: [u8; 0x0c],
    #[doc = "0x204 - Efuse High Voltage Power Switch Control Register"]
    pub efuse_hv_pwrswt_ctrl: EFUSE_HV_PWRSWT_CTRL,
    _reserved19: [u8; 0x0108],
    #[doc = "0x310 - RTC SPI Clock Control Register"]
    pub rtc_spi_clk_ctrl: RTC_SPI_CLK_CTRL,
}
#[doc = "losc_ctrl (rw) register accessor: an alias for `Reg<LOSC_CTRL_SPEC>`"]
pub type LOSC_CTRL = crate::Reg<losc_ctrl::LOSC_CTRL_SPEC>;
#[doc = "Low Oscillator Control Register"]
pub mod losc_ctrl;
#[doc = "losc_auto_swt_sta (rw) register accessor: an alias for `Reg<LOSC_AUTO_SWT_STA_SPEC>`"]
pub type LOSC_AUTO_SWT_STA = crate::Reg<losc_auto_swt_sta::LOSC_AUTO_SWT_STA_SPEC>;
#[doc = "LOSC Auto Switch Status Register"]
pub mod losc_auto_swt_sta;
#[doc = "intosc_clk_prescal (rw) register accessor: an alias for `Reg<INTOSC_CLK_PRESCAL_SPEC>`"]
pub type INTOSC_CLK_PRESCAL = crate::Reg<intosc_clk_prescal::INTOSC_CLK_PRESCAL_SPEC>;
#[doc = "Internal OSC Clock Pre-scalar Register"]
pub mod intosc_clk_prescal;
#[doc = "rtc_day (rw) register accessor: an alias for `Reg<RTC_DAY_SPEC>`"]
pub type RTC_DAY = crate::Reg<rtc_day::RTC_DAY_SPEC>;
#[doc = "RTC Year-Month-Day Register"]
pub mod rtc_day;
#[doc = "rtc_hh_mm_ss (rw) register accessor: an alias for `Reg<RTC_HH_MM_SS_SPEC>`"]
pub type RTC_HH_MM_SS = crate::Reg<rtc_hh_mm_ss::RTC_HH_MM_SS_SPEC>;
#[doc = "RTC Hour-Minute-Second Register"]
pub mod rtc_hh_mm_ss;
#[doc = "alarm0_day_set (rw) register accessor: an alias for `Reg<ALARM0_DAY_SET_SPEC>`"]
pub type ALARM0_DAY_SET = crate::Reg<alarm0_day_set::ALARM0_DAY_SET_SPEC>;
#[doc = "Alarm 0 Day Setting Register"]
pub mod alarm0_day_set;
#[doc = "alarm0_cur_vlu (rw) register accessor: an alias for `Reg<ALARM0_CUR_VLU_SPEC>`"]
pub type ALARM0_CUR_VLU = crate::Reg<alarm0_cur_vlu::ALARM0_CUR_VLU_SPEC>;
#[doc = "Alarm 0 Counter Current Value Register"]
pub mod alarm0_cur_vlu;
#[doc = "alarm0_enable (rw) register accessor: an alias for `Reg<ALARM0_ENABLE_SPEC>`"]
pub type ALARM0_ENABLE = crate::Reg<alarm0_enable::ALARM0_ENABLE_SPEC>;
#[doc = "Alarm 0 Enable Register"]
pub mod alarm0_enable;
#[doc = "alarm0_irq_en (rw) register accessor: an alias for `Reg<ALARM0_IRQ_EN_SPEC>`"]
pub type ALARM0_IRQ_EN = crate::Reg<alarm0_irq_en::ALARM0_IRQ_EN_SPEC>;
#[doc = "Alarm 0 IRQ Enable Register"]
pub mod alarm0_irq_en;
#[doc = "alarm0_irq_sta (rw) register accessor: an alias for `Reg<ALARM0_IRQ_STA_SPEC>`"]
pub type ALARM0_IRQ_STA = crate::Reg<alarm0_irq_sta::ALARM0_IRQ_STA_SPEC>;
#[doc = "Alarm 0 IRQ Status Register"]
pub mod alarm0_irq_sta;
#[doc = "alarm_config (rw) register accessor: an alias for `Reg<ALARM_CONFIG_SPEC>`"]
pub type ALARM_CONFIG = crate::Reg<alarm_config::ALARM_CONFIG_SPEC>;
#[doc = "Alarm Configuration Register"]
pub mod alarm_config;
#[doc = "fout_32k_ctrl_gating (rw) register accessor: an alias for `Reg<FOUT_32K_CTRL_GATING_SPEC>`"]
pub type FOUT_32K_CTRL_GATING = crate::Reg<fout_32k_ctrl_gating::FOUT_32K_CTRL_GATING_SPEC>;
#[doc = "32K Fanout Control Gating Register"]
pub mod fout_32k_ctrl_gating;
#[doc = "gp_data (rw) register accessor: an alias for `Reg<GP_DATA_SPEC>`"]
pub type GP_DATA = crate::Reg<gp_data::GP_DATA_SPEC>;
#[doc = "General Purpose Register"]
pub mod gp_data;
#[doc = "fboot_info (rw) register accessor: an alias for `Reg<FBOOT_INFO_SPEC>`"]
pub type FBOOT_INFO = crate::Reg<fboot_info::FBOOT_INFO_SPEC>;
#[doc = "Fast Boot Information Register \\[01\\]"]
pub mod fboot_info;
#[doc = "dcxo_ctrl (rw) register accessor: an alias for `Reg<DCXO_CTRL_SPEC>`"]
pub type DCXO_CTRL = crate::Reg<dcxo_ctrl::DCXO_CTRL_SPEC>;
#[doc = "DCXO Control Register"]
pub mod dcxo_ctrl;
#[doc = "rtc_vio (rw) register accessor: an alias for `Reg<RTC_VIO_SPEC>`"]
pub type RTC_VIO = crate::Reg<rtc_vio::RTC_VIO_SPEC>;
#[doc = "RTC_VIO Regulation Register"]
pub mod rtc_vio;
#[doc = "ic_chara (rw) register accessor: an alias for `Reg<IC_CHARA_SPEC>`"]
pub type IC_CHARA = crate::Reg<ic_chara::IC_CHARA_SPEC>;
#[doc = "IC Characteristic Register"]
pub mod ic_chara;
#[doc = "vdd_off_gating_ctrl (rw) register accessor: an alias for `Reg<VDD_OFF_GATING_CTRL_SPEC>`"]
pub type VDD_OFF_GATING_CTRL = crate::Reg<vdd_off_gating_ctrl::VDD_OFF_GATING_CTRL_SPEC>;
#[doc = "VDD Off Gating Control Register"]
pub mod vdd_off_gating_ctrl;
#[doc = "efuse_hv_pwrswt_ctrl (rw) register accessor: an alias for `Reg<EFUSE_HV_PWRSWT_CTRL_SPEC>`"]
pub type EFUSE_HV_PWRSWT_CTRL = crate::Reg<efuse_hv_pwrswt_ctrl::EFUSE_HV_PWRSWT_CTRL_SPEC>;
#[doc = "Efuse High Voltage Power Switch Control Register"]
pub mod efuse_hv_pwrswt_ctrl;
#[doc = "rtc_spi_clk_ctrl (rw) register accessor: an alias for `Reg<RTC_SPI_CLK_CTRL_SPEC>`"]
pub type RTC_SPI_CLK_CTRL = crate::Reg<rtc_spi_clk_ctrl::RTC_SPI_CLK_CTRL_SPEC>;
#[doc = "RTC SPI Clock Control Register"]
pub mod rtc_spi_clk_ctrl;
