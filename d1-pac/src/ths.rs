#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - THS Control Register"]
    pub ths_ctrl: THS_CTRL,
    #[doc = "0x04 - THS Enable Register"]
    pub ths_en: THS_EN,
    #[doc = "0x08 - THS Period Control Register"]
    pub ths_per: THS_PER,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - THS Data Interrupt Control Register"]
    pub ths_data_intc: THS_DATA_INTC,
    #[doc = "0x14 - THS Shut Interrupt Control Register"]
    pub ths_shut_intc: THS_SHUT_INTC,
    #[doc = "0x18 - THS Alarm Interrupt Control Register"]
    pub ths_alarm_intc: THS_ALARM_INTC,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - THS Data Interrupt Status Register"]
    pub ths_data_ints: THS_DATA_INTS,
    #[doc = "0x24 - THS Shut Interrupt Status Register"]
    pub ths_shut_ints: THS_SHUT_INTS,
    #[doc = "0x28 - THS_ALARM0_INTS"]
    pub ths_alarmo_ints: THS_ALARMO_INTS,
    #[doc = "0x2c - THS Alarm Interrupt Status Register"]
    pub ths_alarm_ints: THS_ALARM_INTS,
    #[doc = "0x30 - THS Median Filter Control Register"]
    pub ths_filter: THS_FILTER,
    _reserved11: [u8; 0x0c],
    #[doc = "0x40 - THS Alarm Threshold Control Register"]
    pub ths_alarm_ctrl: THS_ALARM_CTRL,
    _reserved12: [u8; 0x3c],
    #[doc = "0x80 - THS Shutdown Threshold Control Register"]
    pub ths_shutdown_ctrl: THS_SHUTDOWN_CTRL,
    _reserved13: [u8; 0x1c],
    #[doc = "0xa0 - THS Calibration Data"]
    pub ths_cdata: THS_CDATA,
    _reserved14: [u8; 0x1c],
    #[doc = "0xc0 - THS Data Register"]
    pub ths_data: THS_DATA,
}
#[doc = "ths_ctrl (rw) register accessor: an alias for `Reg<THS_CTRL_SPEC>`"]
pub type THS_CTRL = crate::Reg<ths_ctrl::THS_CTRL_SPEC>;
#[doc = "THS Control Register"]
pub mod ths_ctrl;
#[doc = "ths_en (rw) register accessor: an alias for `Reg<THS_EN_SPEC>`"]
pub type THS_EN = crate::Reg<ths_en::THS_EN_SPEC>;
#[doc = "THS Enable Register"]
pub mod ths_en;
#[doc = "ths_per (rw) register accessor: an alias for `Reg<THS_PER_SPEC>`"]
pub type THS_PER = crate::Reg<ths_per::THS_PER_SPEC>;
#[doc = "THS Period Control Register"]
pub mod ths_per;
#[doc = "ths_data_intc (rw) register accessor: an alias for `Reg<THS_DATA_INTC_SPEC>`"]
pub type THS_DATA_INTC = crate::Reg<ths_data_intc::THS_DATA_INTC_SPEC>;
#[doc = "THS Data Interrupt Control Register"]
pub mod ths_data_intc;
#[doc = "ths_shut_intc (rw) register accessor: an alias for `Reg<THS_SHUT_INTC_SPEC>`"]
pub type THS_SHUT_INTC = crate::Reg<ths_shut_intc::THS_SHUT_INTC_SPEC>;
#[doc = "THS Shut Interrupt Control Register"]
pub mod ths_shut_intc;
#[doc = "ths_alarm_intc (rw) register accessor: an alias for `Reg<THS_ALARM_INTC_SPEC>`"]
pub type THS_ALARM_INTC = crate::Reg<ths_alarm_intc::THS_ALARM_INTC_SPEC>;
#[doc = "THS Alarm Interrupt Control Register"]
pub mod ths_alarm_intc;
#[doc = "ths_data_ints (rw) register accessor: an alias for `Reg<THS_DATA_INTS_SPEC>`"]
pub type THS_DATA_INTS = crate::Reg<ths_data_ints::THS_DATA_INTS_SPEC>;
#[doc = "THS Data Interrupt Status Register"]
pub mod ths_data_ints;
#[doc = "ths_shut_ints (rw) register accessor: an alias for `Reg<THS_SHUT_INTS_SPEC>`"]
pub type THS_SHUT_INTS = crate::Reg<ths_shut_ints::THS_SHUT_INTS_SPEC>;
#[doc = "THS Shut Interrupt Status Register"]
pub mod ths_shut_ints;
#[doc = "ths_alarmo_ints (rw) register accessor: an alias for `Reg<THS_ALARMO_INTS_SPEC>`"]
pub type THS_ALARMO_INTS = crate::Reg<ths_alarmo_ints::THS_ALARMO_INTS_SPEC>;
#[doc = "THS_ALARM0_INTS"]
pub mod ths_alarmo_ints;
#[doc = "ths_alarm_ints (rw) register accessor: an alias for `Reg<THS_ALARM_INTS_SPEC>`"]
pub type THS_ALARM_INTS = crate::Reg<ths_alarm_ints::THS_ALARM_INTS_SPEC>;
#[doc = "THS Alarm Interrupt Status Register"]
pub mod ths_alarm_ints;
#[doc = "ths_filter (rw) register accessor: an alias for `Reg<THS_FILTER_SPEC>`"]
pub type THS_FILTER = crate::Reg<ths_filter::THS_FILTER_SPEC>;
#[doc = "THS Median Filter Control Register"]
pub mod ths_filter;
#[doc = "ths_alarm_ctrl (rw) register accessor: an alias for `Reg<THS_ALARM_CTRL_SPEC>`"]
pub type THS_ALARM_CTRL = crate::Reg<ths_alarm_ctrl::THS_ALARM_CTRL_SPEC>;
#[doc = "THS Alarm Threshold Control Register"]
pub mod ths_alarm_ctrl;
#[doc = "ths_shutdown_ctrl (rw) register accessor: an alias for `Reg<THS_SHUTDOWN_CTRL_SPEC>`"]
pub type THS_SHUTDOWN_CTRL = crate::Reg<ths_shutdown_ctrl::THS_SHUTDOWN_CTRL_SPEC>;
#[doc = "THS Shutdown Threshold Control Register"]
pub mod ths_shutdown_ctrl;
#[doc = "ths_cdata (rw) register accessor: an alias for `Reg<THS_CDATA_SPEC>`"]
pub type THS_CDATA = crate::Reg<ths_cdata::THS_CDATA_SPEC>;
#[doc = "THS Calibration Data"]
pub mod ths_cdata;
#[doc = "ths_data (r) register accessor: an alias for `Reg<THS_DATA_SPEC>`"]
pub type THS_DATA = crate::Reg<ths_data::THS_DATA_SPEC>;
#[doc = "THS Data Register"]
pub mod ths_data;
