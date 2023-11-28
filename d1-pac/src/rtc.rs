#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    losc_ctrl: LOSC_CTRL,
    losc_auto_swt_sta: LOSC_AUTO_SWT_STA,
    intosc_clk_prescal: INTOSC_CLK_PRESCAL,
    _reserved3: [u8; 0x04],
    rtc_day: RTC_DAY,
    rtc_hh_mm_ss: RTC_HH_MM_SS,
    _reserved5: [u8; 0x08],
    alarm0_day_set: ALARM0_DAY_SET,
    alarm0_cur_vlu: ALARM0_CUR_VLU,
    alarm0_enable: ALARM0_ENABLE,
    alarm0_irq_en: ALARM0_IRQ_EN,
    alarm0_irq_sta: ALARM0_IRQ_STA,
    _reserved10: [u8; 0x1c],
    alarm_config: ALARM_CONFIG,
    _reserved11: [u8; 0x0c],
    fout_32k_ctrl_gating: FOUT_32K_CTRL_GATING,
    _reserved12: [u8; 0x9c],
    gp_data: [GP_DATA; 8],
    fboot_info: [FBOOT_INFO; 2],
    _reserved14: [u8; 0x38],
    dcxo_ctrl: DCXO_CTRL,
    _reserved15: [u8; 0x2c],
    rtc_vio: RTC_VIO,
    _reserved16: [u8; 0x5c],
    ic_chara: IC_CHARA,
    vdd_off_gating_ctrl: VDD_OFF_GATING_CTRL,
    _reserved18: [u8; 0x0c],
    efuse_hv_pwrswt_ctrl: EFUSE_HV_PWRSWT_CTRL,
    _reserved19: [u8; 0x0108],
    rtc_spi_clk_ctrl: RTC_SPI_CLK_CTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Low Oscillator Control Register"]
    #[inline(always)]
    pub const fn losc_ctrl(&self) -> &LOSC_CTRL {
        &self.losc_ctrl
    }
    #[doc = "0x04 - LOSC Auto Switch Status Register"]
    #[inline(always)]
    pub const fn losc_auto_swt_sta(&self) -> &LOSC_AUTO_SWT_STA {
        &self.losc_auto_swt_sta
    }
    #[doc = "0x08 - Internal OSC Clock Pre-scalar Register"]
    #[inline(always)]
    pub const fn intosc_clk_prescal(&self) -> &INTOSC_CLK_PRESCAL {
        &self.intosc_clk_prescal
    }
    #[doc = "0x10 - RTC Year-Month-Day Register"]
    #[inline(always)]
    pub const fn rtc_day(&self) -> &RTC_DAY {
        &self.rtc_day
    }
    #[doc = "0x14 - RTC Hour-Minute-Second Register"]
    #[inline(always)]
    pub const fn rtc_hh_mm_ss(&self) -> &RTC_HH_MM_SS {
        &self.rtc_hh_mm_ss
    }
    #[doc = "0x20 - Alarm 0 Day Setting Register"]
    #[inline(always)]
    pub const fn alarm0_day_set(&self) -> &ALARM0_DAY_SET {
        &self.alarm0_day_set
    }
    #[doc = "0x24 - Alarm 0 Counter Current Value Register"]
    #[inline(always)]
    pub const fn alarm0_cur_vlu(&self) -> &ALARM0_CUR_VLU {
        &self.alarm0_cur_vlu
    }
    #[doc = "0x28 - Alarm 0 Enable Register"]
    #[inline(always)]
    pub const fn alarm0_enable(&self) -> &ALARM0_ENABLE {
        &self.alarm0_enable
    }
    #[doc = "0x2c - Alarm 0 IRQ Enable Register"]
    #[inline(always)]
    pub const fn alarm0_irq_en(&self) -> &ALARM0_IRQ_EN {
        &self.alarm0_irq_en
    }
    #[doc = "0x30 - Alarm 0 IRQ Status Register"]
    #[inline(always)]
    pub const fn alarm0_irq_sta(&self) -> &ALARM0_IRQ_STA {
        &self.alarm0_irq_sta
    }
    #[doc = "0x50 - Alarm Configuration Register"]
    #[inline(always)]
    pub const fn alarm_config(&self) -> &ALARM_CONFIG {
        &self.alarm_config
    }
    #[doc = "0x60 - 32K Fanout Control Gating Register"]
    #[inline(always)]
    pub const fn fout_32k_ctrl_gating(&self) -> &FOUT_32K_CTRL_GATING {
        &self.fout_32k_ctrl_gating
    }
    #[doc = "0x100..0x120 - General Purpose Register"]
    #[inline(always)]
    pub const fn gp_data(&self, n: usize) -> &GP_DATA {
        &self.gp_data[n]
    }
    #[doc = "0x120..0x128 - Fast Boot Information Register \\[01\\]"]
    #[inline(always)]
    pub const fn fboot_info(&self, n: usize) -> &FBOOT_INFO {
        &self.fboot_info[n]
    }
    #[doc = "0x160 - DCXO Control Register"]
    #[inline(always)]
    pub const fn dcxo_ctrl(&self) -> &DCXO_CTRL {
        &self.dcxo_ctrl
    }
    #[doc = "0x190 - RTC_VIO Regulation Register"]
    #[inline(always)]
    pub const fn rtc_vio(&self) -> &RTC_VIO {
        &self.rtc_vio
    }
    #[doc = "0x1f0 - IC Characteristic Register"]
    #[inline(always)]
    pub const fn ic_chara(&self) -> &IC_CHARA {
        &self.ic_chara
    }
    #[doc = "0x1f4 - VDD Off Gating Control Register"]
    #[inline(always)]
    pub const fn vdd_off_gating_ctrl(&self) -> &VDD_OFF_GATING_CTRL {
        &self.vdd_off_gating_ctrl
    }
    #[doc = "0x204 - Efuse High Voltage Power Switch Control Register"]
    #[inline(always)]
    pub const fn efuse_hv_pwrswt_ctrl(&self) -> &EFUSE_HV_PWRSWT_CTRL {
        &self.efuse_hv_pwrswt_ctrl
    }
    #[doc = "0x310 - RTC SPI Clock Control Register"]
    #[inline(always)]
    pub const fn rtc_spi_clk_ctrl(&self) -> &RTC_SPI_CLK_CTRL {
        &self.rtc_spi_clk_ctrl
    }
}
#[doc = "losc_ctrl (rw) register accessor: Low Oscillator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`losc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`losc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@losc_ctrl`] module"]
pub type LOSC_CTRL = crate::Reg<losc_ctrl::LOSC_CTRL_SPEC>;
#[doc = "Low Oscillator Control Register"]
pub mod losc_ctrl;
#[doc = "losc_auto_swt_sta (rw) register accessor: LOSC Auto Switch Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`losc_auto_swt_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`losc_auto_swt_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@losc_auto_swt_sta`] module"]
pub type LOSC_AUTO_SWT_STA = crate::Reg<losc_auto_swt_sta::LOSC_AUTO_SWT_STA_SPEC>;
#[doc = "LOSC Auto Switch Status Register"]
pub mod losc_auto_swt_sta;
#[doc = "intosc_clk_prescal (rw) register accessor: Internal OSC Clock Pre-scalar Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intosc_clk_prescal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intosc_clk_prescal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intosc_clk_prescal`] module"]
pub type INTOSC_CLK_PRESCAL = crate::Reg<intosc_clk_prescal::INTOSC_CLK_PRESCAL_SPEC>;
#[doc = "Internal OSC Clock Pre-scalar Register"]
pub mod intosc_clk_prescal;
#[doc = "rtc_day (rw) register accessor: RTC Year-Month-Day Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_day::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_day::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_day`] module"]
pub type RTC_DAY = crate::Reg<rtc_day::RTC_DAY_SPEC>;
#[doc = "RTC Year-Month-Day Register"]
pub mod rtc_day;
#[doc = "rtc_hh_mm_ss (rw) register accessor: RTC Hour-Minute-Second Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_hh_mm_ss::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_hh_mm_ss::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_hh_mm_ss`] module"]
pub type RTC_HH_MM_SS = crate::Reg<rtc_hh_mm_ss::RTC_HH_MM_SS_SPEC>;
#[doc = "RTC Hour-Minute-Second Register"]
pub mod rtc_hh_mm_ss;
#[doc = "alarm0_day_set (rw) register accessor: Alarm 0 Day Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm0_day_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0_day_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm0_day_set`] module"]
pub type ALARM0_DAY_SET = crate::Reg<alarm0_day_set::ALARM0_DAY_SET_SPEC>;
#[doc = "Alarm 0 Day Setting Register"]
pub mod alarm0_day_set;
#[doc = "alarm0_cur_vlu (rw) register accessor: Alarm 0 Counter Current Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm0_cur_vlu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0_cur_vlu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm0_cur_vlu`] module"]
pub type ALARM0_CUR_VLU = crate::Reg<alarm0_cur_vlu::ALARM0_CUR_VLU_SPEC>;
#[doc = "Alarm 0 Counter Current Value Register"]
pub mod alarm0_cur_vlu;
#[doc = "alarm0_enable (rw) register accessor: Alarm 0 Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm0_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm0_enable`] module"]
pub type ALARM0_ENABLE = crate::Reg<alarm0_enable::ALARM0_ENABLE_SPEC>;
#[doc = "Alarm 0 Enable Register"]
pub mod alarm0_enable;
#[doc = "alarm0_irq_en (rw) register accessor: Alarm 0 IRQ Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm0_irq_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0_irq_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm0_irq_en`] module"]
pub type ALARM0_IRQ_EN = crate::Reg<alarm0_irq_en::ALARM0_IRQ_EN_SPEC>;
#[doc = "Alarm 0 IRQ Enable Register"]
pub mod alarm0_irq_en;
#[doc = "alarm0_irq_sta (rw) register accessor: Alarm 0 IRQ Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm0_irq_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0_irq_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm0_irq_sta`] module"]
pub type ALARM0_IRQ_STA = crate::Reg<alarm0_irq_sta::ALARM0_IRQ_STA_SPEC>;
#[doc = "Alarm 0 IRQ Status Register"]
pub mod alarm0_irq_sta;
#[doc = "alarm_config (rw) register accessor: Alarm Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm_config`] module"]
pub type ALARM_CONFIG = crate::Reg<alarm_config::ALARM_CONFIG_SPEC>;
#[doc = "Alarm Configuration Register"]
pub mod alarm_config;
#[doc = "fout_32k_ctrl_gating (rw) register accessor: 32K Fanout Control Gating Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fout_32k_ctrl_gating::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fout_32k_ctrl_gating::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fout_32k_ctrl_gating`] module"]
pub type FOUT_32K_CTRL_GATING = crate::Reg<fout_32k_ctrl_gating::FOUT_32K_CTRL_GATING_SPEC>;
#[doc = "32K Fanout Control Gating Register"]
pub mod fout_32k_ctrl_gating;
#[doc = "gp_data (rw) register accessor: General Purpose Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_data`] module"]
pub type GP_DATA = crate::Reg<gp_data::GP_DATA_SPEC>;
#[doc = "General Purpose Register"]
pub mod gp_data;
#[doc = "fboot_info (rw) register accessor: Fast Boot Information Register \\[01\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fboot_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fboot_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fboot_info`] module"]
pub type FBOOT_INFO = crate::Reg<fboot_info::FBOOT_INFO_SPEC>;
#[doc = "Fast Boot Information Register \\[01\\]"]
pub mod fboot_info;
#[doc = "dcxo_ctrl (rw) register accessor: DCXO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcxo_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcxo_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcxo_ctrl`] module"]
pub type DCXO_CTRL = crate::Reg<dcxo_ctrl::DCXO_CTRL_SPEC>;
#[doc = "DCXO Control Register"]
pub mod dcxo_ctrl;
#[doc = "rtc_vio (rw) register accessor: RTC_VIO Regulation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_vio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_vio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_vio`] module"]
pub type RTC_VIO = crate::Reg<rtc_vio::RTC_VIO_SPEC>;
#[doc = "RTC_VIO Regulation Register"]
pub mod rtc_vio;
#[doc = "ic_chara (rw) register accessor: IC Characteristic Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ic_chara::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ic_chara::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ic_chara`] module"]
pub type IC_CHARA = crate::Reg<ic_chara::IC_CHARA_SPEC>;
#[doc = "IC Characteristic Register"]
pub mod ic_chara;
#[doc = "vdd_off_gating_ctrl (rw) register accessor: VDD Off Gating Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdd_off_gating_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdd_off_gating_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdd_off_gating_ctrl`] module"]
pub type VDD_OFF_GATING_CTRL = crate::Reg<vdd_off_gating_ctrl::VDD_OFF_GATING_CTRL_SPEC>;
#[doc = "VDD Off Gating Control Register"]
pub mod vdd_off_gating_ctrl;
#[doc = "efuse_hv_pwrswt_ctrl (rw) register accessor: Efuse High Voltage Power Switch Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse_hv_pwrswt_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuse_hv_pwrswt_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_hv_pwrswt_ctrl`] module"]
pub type EFUSE_HV_PWRSWT_CTRL = crate::Reg<efuse_hv_pwrswt_ctrl::EFUSE_HV_PWRSWT_CTRL_SPEC>;
#[doc = "Efuse High Voltage Power Switch Control Register"]
pub mod efuse_hv_pwrswt_ctrl;
#[doc = "rtc_spi_clk_ctrl (rw) register accessor: RTC SPI Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_spi_clk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_spi_clk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_spi_clk_ctrl`] module"]
pub type RTC_SPI_CLK_CTRL = crate::Reg<rtc_spi_clk_ctrl::RTC_SPI_CLK_CTRL_SPEC>;
#[doc = "RTC SPI Clock Control Register"]
pub mod rtc_spi_clk_ctrl;
