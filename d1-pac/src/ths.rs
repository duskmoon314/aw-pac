#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ths_ctrl: THS_CTRL,
    ths_en: THS_EN,
    ths_per: THS_PER,
    _reserved3: [u8; 0x04],
    ths_data_intc: THS_DATA_INTC,
    ths_shut_intc: THS_SHUT_INTC,
    ths_alarm_intc: THS_ALARM_INTC,
    _reserved6: [u8; 0x04],
    ths_data_ints: THS_DATA_INTS,
    ths_shut_ints: THS_SHUT_INTS,
    ths_alarmo_ints: THS_ALARMO_INTS,
    ths_alarm_ints: THS_ALARM_INTS,
    ths_filter: THS_FILTER,
    _reserved11: [u8; 0x0c],
    ths_alarm_ctrl: THS_ALARM_CTRL,
    _reserved12: [u8; 0x3c],
    ths_shutdown_ctrl: THS_SHUTDOWN_CTRL,
    _reserved13: [u8; 0x1c],
    ths_cdata: THS_CDATA,
    _reserved14: [u8; 0x1c],
    ths_data: THS_DATA,
}
impl RegisterBlock {
    #[doc = "0x00 - THS Control Register"]
    #[inline(always)]
    pub const fn ths_ctrl(&self) -> &THS_CTRL {
        &self.ths_ctrl
    }
    #[doc = "0x04 - THS Enable Register"]
    #[inline(always)]
    pub const fn ths_en(&self) -> &THS_EN {
        &self.ths_en
    }
    #[doc = "0x08 - THS Period Control Register"]
    #[inline(always)]
    pub const fn ths_per(&self) -> &THS_PER {
        &self.ths_per
    }
    #[doc = "0x10 - THS Data Interrupt Control Register"]
    #[inline(always)]
    pub const fn ths_data_intc(&self) -> &THS_DATA_INTC {
        &self.ths_data_intc
    }
    #[doc = "0x14 - THS Shut Interrupt Control Register"]
    #[inline(always)]
    pub const fn ths_shut_intc(&self) -> &THS_SHUT_INTC {
        &self.ths_shut_intc
    }
    #[doc = "0x18 - THS Alarm Interrupt Control Register"]
    #[inline(always)]
    pub const fn ths_alarm_intc(&self) -> &THS_ALARM_INTC {
        &self.ths_alarm_intc
    }
    #[doc = "0x20 - THS Data Interrupt Status Register"]
    #[inline(always)]
    pub const fn ths_data_ints(&self) -> &THS_DATA_INTS {
        &self.ths_data_ints
    }
    #[doc = "0x24 - THS Shut Interrupt Status Register"]
    #[inline(always)]
    pub const fn ths_shut_ints(&self) -> &THS_SHUT_INTS {
        &self.ths_shut_ints
    }
    #[doc = "0x28 - THS_ALARM0_INTS"]
    #[inline(always)]
    pub const fn ths_alarmo_ints(&self) -> &THS_ALARMO_INTS {
        &self.ths_alarmo_ints
    }
    #[doc = "0x2c - THS Alarm Interrupt Status Register"]
    #[inline(always)]
    pub const fn ths_alarm_ints(&self) -> &THS_ALARM_INTS {
        &self.ths_alarm_ints
    }
    #[doc = "0x30 - THS Median Filter Control Register"]
    #[inline(always)]
    pub const fn ths_filter(&self) -> &THS_FILTER {
        &self.ths_filter
    }
    #[doc = "0x40 - THS Alarm Threshold Control Register"]
    #[inline(always)]
    pub const fn ths_alarm_ctrl(&self) -> &THS_ALARM_CTRL {
        &self.ths_alarm_ctrl
    }
    #[doc = "0x80 - THS Shutdown Threshold Control Register"]
    #[inline(always)]
    pub const fn ths_shutdown_ctrl(&self) -> &THS_SHUTDOWN_CTRL {
        &self.ths_shutdown_ctrl
    }
    #[doc = "0xa0 - THS Calibration Data"]
    #[inline(always)]
    pub const fn ths_cdata(&self) -> &THS_CDATA {
        &self.ths_cdata
    }
    #[doc = "0xc0 - THS Data Register"]
    #[inline(always)]
    pub const fn ths_data(&self) -> &THS_DATA {
        &self.ths_data
    }
}
#[doc = "ths_ctrl (rw) register accessor: THS Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ths_ctrl`] module"]
pub type THS_CTRL = crate::Reg<ths_ctrl::THS_CTRL_SPEC>;
#[doc = "THS Control Register"]
pub mod ths_ctrl;
#[doc = "ths_en (rw) register accessor: THS Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ths_en`] module"]
pub type THS_EN = crate::Reg<ths_en::THS_EN_SPEC>;
#[doc = "THS Enable Register"]
pub mod ths_en;
#[doc = "ths_per (rw) register accessor: THS Period Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_per::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_per::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ths_per`] module"]
pub type THS_PER = crate::Reg<ths_per::THS_PER_SPEC>;
#[doc = "THS Period Control Register"]
pub mod ths_per;
#[doc = "ths_data_intc (rw) register accessor: THS Data Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_data_intc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_data_intc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ths_data_intc`] module"]
pub type THS_DATA_INTC = crate::Reg<ths_data_intc::THS_DATA_INTC_SPEC>;
#[doc = "THS Data Interrupt Control Register"]
pub mod ths_data_intc;
#[doc = "ths_shut_intc (rw) register accessor: THS Shut Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_shut_intc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_shut_intc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ths_shut_intc`] module"]
pub type THS_SHUT_INTC = crate::Reg<ths_shut_intc::THS_SHUT_INTC_SPEC>;
#[doc = "THS Shut Interrupt Control Register"]
pub mod ths_shut_intc;
#[doc = "ths_alarm_intc (rw) register accessor: THS Alarm Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_alarm_intc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_alarm_intc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ths_alarm_intc`] module"]
pub type THS_ALARM_INTC = crate::Reg<ths_alarm_intc::THS_ALARM_INTC_SPEC>;
#[doc = "THS Alarm Interrupt Control Register"]
pub mod ths_alarm_intc;
#[doc = "ths_data_ints (rw) register accessor: THS Data Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_data_ints::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_data_ints::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ths_data_ints`] module"]
pub type THS_DATA_INTS = crate::Reg<ths_data_ints::THS_DATA_INTS_SPEC>;
#[doc = "THS Data Interrupt Status Register"]
pub mod ths_data_ints;
#[doc = "ths_shut_ints (rw) register accessor: THS Shut Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_shut_ints::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_shut_ints::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ths_shut_ints`] module"]
pub type THS_SHUT_INTS = crate::Reg<ths_shut_ints::THS_SHUT_INTS_SPEC>;
#[doc = "THS Shut Interrupt Status Register"]
pub mod ths_shut_ints;
#[doc = "ths_alarmo_ints (rw) register accessor: THS_ALARM0_INTS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_alarmo_ints::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_alarmo_ints::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ths_alarmo_ints`] module"]
pub type THS_ALARMO_INTS = crate::Reg<ths_alarmo_ints::THS_ALARMO_INTS_SPEC>;
#[doc = "THS_ALARM0_INTS"]
pub mod ths_alarmo_ints;
#[doc = "ths_alarm_ints (rw) register accessor: THS Alarm Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_alarm_ints::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_alarm_ints::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ths_alarm_ints`] module"]
pub type THS_ALARM_INTS = crate::Reg<ths_alarm_ints::THS_ALARM_INTS_SPEC>;
#[doc = "THS Alarm Interrupt Status Register"]
pub mod ths_alarm_ints;
#[doc = "ths_filter (rw) register accessor: THS Median Filter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_filter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_filter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ths_filter`] module"]
pub type THS_FILTER = crate::Reg<ths_filter::THS_FILTER_SPEC>;
#[doc = "THS Median Filter Control Register"]
pub mod ths_filter;
#[doc = "ths_alarm_ctrl (rw) register accessor: THS Alarm Threshold Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_alarm_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_alarm_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ths_alarm_ctrl`] module"]
pub type THS_ALARM_CTRL = crate::Reg<ths_alarm_ctrl::THS_ALARM_CTRL_SPEC>;
#[doc = "THS Alarm Threshold Control Register"]
pub mod ths_alarm_ctrl;
#[doc = "ths_shutdown_ctrl (rw) register accessor: THS Shutdown Threshold Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_shutdown_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_shutdown_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ths_shutdown_ctrl`] module"]
pub type THS_SHUTDOWN_CTRL = crate::Reg<ths_shutdown_ctrl::THS_SHUTDOWN_CTRL_SPEC>;
#[doc = "THS Shutdown Threshold Control Register"]
pub mod ths_shutdown_ctrl;
#[doc = "ths_cdata (rw) register accessor: THS Calibration Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_cdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_cdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ths_cdata`] module"]
pub type THS_CDATA = crate::Reg<ths_cdata::THS_CDATA_SPEC>;
#[doc = "THS Calibration Data"]
pub mod ths_cdata;
#[doc = "ths_data (r) register accessor: THS Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ths_data`] module"]
pub type THS_DATA = crate::Reg<ths_data::THS_DATA_SPEC>;
#[doc = "THS Data Register"]
pub mod ths_data;
