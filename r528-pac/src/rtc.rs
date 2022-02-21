#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Low Oscillator Control Register"]
    pub losc_ctrl_reg: crate::Reg<losc_ctrl_reg::LOSC_CTRL_REG_SPEC>,
    #[doc = "0x04 - LOSC Auto Switch Status Register"]
    pub losc_auto_swt_sta_reg: crate::Reg<losc_auto_swt_sta_reg::LOSC_AUTO_SWT_STA_REG_SPEC>,
    #[doc = "0x08 - Internal OSC Clock Pre-scalar Register"]
    pub intosc_clk_prescal_reg: crate::Reg<intosc_clk_prescal_reg::INTOSC_CLK_PRESCAL_REG_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - RTC Year-Month-Day Register"]
    pub rtc_day_reg: crate::Reg<rtc_day_reg::RTC_DAY_REG_SPEC>,
    #[doc = "0x14 - RTC Hour-Minute-Second Register"]
    pub rtc_hh_mm_ss_reg: crate::Reg<rtc_hh_mm_ss_reg::RTC_HH_MM_SS_REG_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - Alarm 0 Day Setting Register"]
    pub alarm0_day_set_reg: crate::Reg<alarm0_day_set_reg::ALARM0_DAY_SET_REG_SPEC>,
    #[doc = "0x24 - Alarm 0 Counter Current Value Register"]
    pub alarm0_cur_vlu_reg: crate::Reg<alarm0_cur_vlu_reg::ALARM0_CUR_VLU_REG_SPEC>,
    #[doc = "0x28 - Alarm 0 Enable Register"]
    pub alarm0_enable_reg: crate::Reg<alarm0_enable_reg::ALARM0_ENABLE_REG_SPEC>,
    #[doc = "0x2c - Alarm 0 IRQ Enable Register"]
    pub alarm0_irq_en: crate::Reg<alarm0_irq_en::ALARM0_IRQ_EN_SPEC>,
    #[doc = "0x30 - Alarm 0 IRQ Status Register"]
    pub alarm0_irq_sta_reg: crate::Reg<alarm0_irq_sta_reg::ALARM0_IRQ_STA_REG_SPEC>,
    _reserved10: [u8; 0x1c],
    #[doc = "0x50 - Alarm Configuration Register"]
    pub alarm_config_reg: crate::Reg<alarm_config_reg::ALARM_CONFIG_REG_SPEC>,
    _reserved11: [u8; 0x0c],
    #[doc = "0x60 - 32K Fanout Control Gating Register"]
    pub _32k_fout_ctrl_gating_reg:
        crate::Reg<_32k_fout_ctrl_gating_reg::_32K_FOUT_CTRL_GATING_REG_SPEC>,
    _reserved12: [u8; 0x9c],
    #[doc = "0x100..0x120 - General Purpose Register"]
    pub gp_data_reg: [crate::Reg<gp_data_reg::GP_DATA_REG_SPEC>; 8],
    #[doc = "0x120 - Fast Boot Information Register0"]
    pub fboot_info_reg0: crate::Reg<fboot_info_reg0::FBOOT_INFO_REG0_SPEC>,
    #[doc = "0x124 - Fast Boot Information Register1"]
    pub fboot_info_reg1: crate::Reg<fboot_info_reg1::FBOOT_INFO_REG1_SPEC>,
    _reserved15: [u8; 0x38],
    #[doc = "0x160 - DCXO Control Register"]
    pub dcxo_ctrl_reg: crate::Reg<dcxo_ctrl_reg::DCXO_CTRL_REG_SPEC>,
    _reserved16: [u8; 0x2c],
    #[doc = "0x190 - RTC_VIO Regulation Register"]
    pub rtc_vio_reg: crate::Reg<rtc_vio_reg::RTC_VIO_REG_SPEC>,
    _reserved17: [u8; 0x5c],
    #[doc = "0x1f0 - IC Characteristic Register"]
    pub ic_chara_reg: crate::Reg<ic_chara_reg::IC_CHARA_REG_SPEC>,
    #[doc = "0x1f4 - VDD Off Gating Control Register"]
    pub vdd_off_gating_ctrl_reg: crate::Reg<vdd_off_gating_ctrl_reg::VDD_OFF_GATING_CTRL_REG_SPEC>,
    _reserved19: [u8; 0x0c],
    #[doc = "0x204 - Efuse High Voltage Power Switch Control Register"]
    pub efuse_hv_pwrswt_ctrl_reg:
        crate::Reg<efuse_hv_pwrswt_ctrl_reg::EFUSE_HV_PWRSWT_CTRL_REG_SPEC>,
    _reserved20: [u8; 0x0108],
    #[doc = "0x310 - RTC SPI Clock Control Register"]
    pub rtc_spi_clk_ctrl_reg: crate::Reg<rtc_spi_clk_ctrl_reg::RTC_SPI_CLK_CTRL_REG_SPEC>,
}
#[doc = "LOSC_CTRL_REG register accessor: an alias for `Reg<LOSC_CTRL_REG_SPEC>`"]
pub type LOSC_CTRL_REG = crate::Reg<losc_ctrl_reg::LOSC_CTRL_REG_SPEC>;
#[doc = "Low Oscillator Control Register"]
pub mod losc_ctrl_reg;
#[doc = "LOSC_AUTO_SWT_STA_REG register accessor: an alias for `Reg<LOSC_AUTO_SWT_STA_REG_SPEC>`"]
pub type LOSC_AUTO_SWT_STA_REG = crate::Reg<losc_auto_swt_sta_reg::LOSC_AUTO_SWT_STA_REG_SPEC>;
#[doc = "LOSC Auto Switch Status Register"]
pub mod losc_auto_swt_sta_reg;
#[doc = "INTOSC_CLK_PRESCAL_REG register accessor: an alias for `Reg<INTOSC_CLK_PRESCAL_REG_SPEC>`"]
pub type INTOSC_CLK_PRESCAL_REG = crate::Reg<intosc_clk_prescal_reg::INTOSC_CLK_PRESCAL_REG_SPEC>;
#[doc = "Internal OSC Clock Pre-scalar Register"]
pub mod intosc_clk_prescal_reg;
#[doc = "RTC_DAY_REG register accessor: an alias for `Reg<RTC_DAY_REG_SPEC>`"]
pub type RTC_DAY_REG = crate::Reg<rtc_day_reg::RTC_DAY_REG_SPEC>;
#[doc = "RTC Year-Month-Day Register"]
pub mod rtc_day_reg;
#[doc = "RTC_HH_MM_SS_REG register accessor: an alias for `Reg<RTC_HH_MM_SS_REG_SPEC>`"]
pub type RTC_HH_MM_SS_REG = crate::Reg<rtc_hh_mm_ss_reg::RTC_HH_MM_SS_REG_SPEC>;
#[doc = "RTC Hour-Minute-Second Register"]
pub mod rtc_hh_mm_ss_reg;
#[doc = "ALARM0_DAY_SET_REG register accessor: an alias for `Reg<ALARM0_DAY_SET_REG_SPEC>`"]
pub type ALARM0_DAY_SET_REG = crate::Reg<alarm0_day_set_reg::ALARM0_DAY_SET_REG_SPEC>;
#[doc = "Alarm 0 Day Setting Register"]
pub mod alarm0_day_set_reg;
#[doc = "ALARM0_CUR_VLU_REG register accessor: an alias for `Reg<ALARM0_CUR_VLU_REG_SPEC>`"]
pub type ALARM0_CUR_VLU_REG = crate::Reg<alarm0_cur_vlu_reg::ALARM0_CUR_VLU_REG_SPEC>;
#[doc = "Alarm 0 Counter Current Value Register"]
pub mod alarm0_cur_vlu_reg;
#[doc = "ALARM0_ENABLE_REG register accessor: an alias for `Reg<ALARM0_ENABLE_REG_SPEC>`"]
pub type ALARM0_ENABLE_REG = crate::Reg<alarm0_enable_reg::ALARM0_ENABLE_REG_SPEC>;
#[doc = "Alarm 0 Enable Register"]
pub mod alarm0_enable_reg;
#[doc = "ALARM0_IRQ_EN register accessor: an alias for `Reg<ALARM0_IRQ_EN_SPEC>`"]
pub type ALARM0_IRQ_EN = crate::Reg<alarm0_irq_en::ALARM0_IRQ_EN_SPEC>;
#[doc = "Alarm 0 IRQ Enable Register"]
pub mod alarm0_irq_en;
#[doc = "ALARM0_IRQ_STA_REG register accessor: an alias for `Reg<ALARM0_IRQ_STA_REG_SPEC>`"]
pub type ALARM0_IRQ_STA_REG = crate::Reg<alarm0_irq_sta_reg::ALARM0_IRQ_STA_REG_SPEC>;
#[doc = "Alarm 0 IRQ Status Register"]
pub mod alarm0_irq_sta_reg;
#[doc = "ALARM_CONFIG_REG register accessor: an alias for `Reg<ALARM_CONFIG_REG_SPEC>`"]
pub type ALARM_CONFIG_REG = crate::Reg<alarm_config_reg::ALARM_CONFIG_REG_SPEC>;
#[doc = "Alarm Configuration Register"]
pub mod alarm_config_reg;
#[doc = "_32K_FOUT_CTRL_GATING_REG register accessor: an alias for `Reg<_32K_FOUT_CTRL_GATING_REG_SPEC>`"]
pub type _32K_FOUT_CTRL_GATING_REG =
    crate::Reg<_32k_fout_ctrl_gating_reg::_32K_FOUT_CTRL_GATING_REG_SPEC>;
#[doc = "32K Fanout Control Gating Register"]
pub mod _32k_fout_ctrl_gating_reg;
#[doc = "GP_DATA_REG register accessor: an alias for `Reg<GP_DATA_REG_SPEC>`"]
pub type GP_DATA_REG = crate::Reg<gp_data_reg::GP_DATA_REG_SPEC>;
#[doc = "General Purpose Register"]
pub mod gp_data_reg;
#[doc = "FBOOT_INFO_REG0 register accessor: an alias for `Reg<FBOOT_INFO_REG0_SPEC>`"]
pub type FBOOT_INFO_REG0 = crate::Reg<fboot_info_reg0::FBOOT_INFO_REG0_SPEC>;
#[doc = "Fast Boot Information Register0"]
pub mod fboot_info_reg0;
#[doc = "FBOOT_INFO_REG1 register accessor: an alias for `Reg<FBOOT_INFO_REG1_SPEC>`"]
pub type FBOOT_INFO_REG1 = crate::Reg<fboot_info_reg1::FBOOT_INFO_REG1_SPEC>;
#[doc = "Fast Boot Information Register1"]
pub mod fboot_info_reg1;
#[doc = "DCXO_CTRL_REG register accessor: an alias for `Reg<DCXO_CTRL_REG_SPEC>`"]
pub type DCXO_CTRL_REG = crate::Reg<dcxo_ctrl_reg::DCXO_CTRL_REG_SPEC>;
#[doc = "DCXO Control Register"]
pub mod dcxo_ctrl_reg;
#[doc = "RTC_VIO_REG register accessor: an alias for `Reg<RTC_VIO_REG_SPEC>`"]
pub type RTC_VIO_REG = crate::Reg<rtc_vio_reg::RTC_VIO_REG_SPEC>;
#[doc = "RTC_VIO Regulation Register"]
pub mod rtc_vio_reg;
#[doc = "IC_CHARA_REG register accessor: an alias for `Reg<IC_CHARA_REG_SPEC>`"]
pub type IC_CHARA_REG = crate::Reg<ic_chara_reg::IC_CHARA_REG_SPEC>;
#[doc = "IC Characteristic Register"]
pub mod ic_chara_reg;
#[doc = "VDD_OFF_GATING_CTRL_REG register accessor: an alias for `Reg<VDD_OFF_GATING_CTRL_REG_SPEC>`"]
pub type VDD_OFF_GATING_CTRL_REG =
    crate::Reg<vdd_off_gating_ctrl_reg::VDD_OFF_GATING_CTRL_REG_SPEC>;
#[doc = "VDD Off Gating Control Register"]
pub mod vdd_off_gating_ctrl_reg;
#[doc = "EFUSE_HV_PWRSWT_CTRL_REG register accessor: an alias for `Reg<EFUSE_HV_PWRSWT_CTRL_REG_SPEC>`"]
pub type EFUSE_HV_PWRSWT_CTRL_REG =
    crate::Reg<efuse_hv_pwrswt_ctrl_reg::EFUSE_HV_PWRSWT_CTRL_REG_SPEC>;
#[doc = "Efuse High Voltage Power Switch Control Register"]
pub mod efuse_hv_pwrswt_ctrl_reg;
#[doc = "RTC_SPI_CLK_CTRL_REG register accessor: an alias for `Reg<RTC_SPI_CLK_CTRL_REG_SPEC>`"]
pub type RTC_SPI_CLK_CTRL_REG = crate::Reg<rtc_spi_clk_ctrl_reg::RTC_SPI_CLK_CTRL_REG_SPEC>;
#[doc = "RTC SPI Clock Control Register"]
pub mod rtc_spi_clk_ctrl_reg;
