#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    gp_sr_con: GP_SR_CON,
    gp_ctrl: GP_CTRL,
    gp_cs_en: GP_CS_EN,
    gp_fifo_intc: GP_FIFO_INTC,
    gp_fifo_ints: GP_FIFO_INTS,
    gp_fifo_data: GP_FIFO_DATA,
    gp_cdata: GP_CDATA,
    _reserved7: [u8; 0x04],
    gp_datal_intc: GP_DATAL_INTC,
    gp_datah_intc: GP_DATAH_INTC,
    gp_data_intc: GP_DATA_INTC,
    _reserved10: [u8; 0x04],
    gp_datal_ints: GP_DATAL_INTS,
    gp_datah_ints: GP_DATAH_INTS,
    gp_data_ints: GP_DATA_INTS,
    _reserved13: [u8; 0x04],
    gp_ch0_cmp_data: GP_CH0_CMP_DATA,
    gp_ch1_cmp_data: GP_CH1_CMP_DATA,
    _reserved15: [u8; 0x38],
    gp_ch0_data: GP_CH0_DATA,
    gp_ch1_data: GP_CH1_DATA,
}
impl RegisterBlock {
    #[doc = "0x00 - GPADC Sample Rate Configure Register"]
    #[inline(always)]
    pub const fn gp_sr_con(&self) -> &GP_SR_CON {
        &self.gp_sr_con
    }
    #[doc = "0x04 - GPADC Control Register"]
    #[inline(always)]
    pub const fn gp_ctrl(&self) -> &GP_CTRL {
        &self.gp_ctrl
    }
    #[doc = "0x08 - GPADC Compare and Select Enable Register"]
    #[inline(always)]
    pub const fn gp_cs_en(&self) -> &GP_CS_EN {
        &self.gp_cs_en
    }
    #[doc = "0x0c - GPADC FIFO Interrupt Control Register"]
    #[inline(always)]
    pub const fn gp_fifo_intc(&self) -> &GP_FIFO_INTC {
        &self.gp_fifo_intc
    }
    #[doc = "0x10 - GPADC FIFO Interrupt Status Register"]
    #[inline(always)]
    pub const fn gp_fifo_ints(&self) -> &GP_FIFO_INTS {
        &self.gp_fifo_ints
    }
    #[doc = "0x14 - GPADC FIFO Data Register"]
    #[inline(always)]
    pub const fn gp_fifo_data(&self) -> &GP_FIFO_DATA {
        &self.gp_fifo_data
    }
    #[doc = "0x18 - GPADC Calibration Data Register"]
    #[inline(always)]
    pub const fn gp_cdata(&self) -> &GP_CDATA {
        &self.gp_cdata
    }
    #[doc = "0x20 - GPADC Data Low Interrupt Configure Register"]
    #[inline(always)]
    pub const fn gp_datal_intc(&self) -> &GP_DATAL_INTC {
        &self.gp_datal_intc
    }
    #[doc = "0x24 - GPADC Data High Interrupt Configure Register"]
    #[inline(always)]
    pub const fn gp_datah_intc(&self) -> &GP_DATAH_INTC {
        &self.gp_datah_intc
    }
    #[doc = "0x28 - GPADC Data Interrupt Configure Register"]
    #[inline(always)]
    pub const fn gp_data_intc(&self) -> &GP_DATA_INTC {
        &self.gp_data_intc
    }
    #[doc = "0x30 - GPADC Data Low Interrupt Status Register"]
    #[inline(always)]
    pub const fn gp_datal_ints(&self) -> &GP_DATAL_INTS {
        &self.gp_datal_ints
    }
    #[doc = "0x34 - GPADC Data High Interrupt Status Register"]
    #[inline(always)]
    pub const fn gp_datah_ints(&self) -> &GP_DATAH_INTS {
        &self.gp_datah_ints
    }
    #[doc = "0x38 - GPADC Data Interrupt Status Register"]
    #[inline(always)]
    pub const fn gp_data_ints(&self) -> &GP_DATA_INTS {
        &self.gp_data_ints
    }
    #[doc = "0x40 - GPADC CH0 Compare Data Register"]
    #[inline(always)]
    pub const fn gp_ch0_cmp_data(&self) -> &GP_CH0_CMP_DATA {
        &self.gp_ch0_cmp_data
    }
    #[doc = "0x44 - GPADC CH1 Compare Data Register"]
    #[inline(always)]
    pub const fn gp_ch1_cmp_data(&self) -> &GP_CH1_CMP_DATA {
        &self.gp_ch1_cmp_data
    }
    #[doc = "0x80 - GPADC CH0 Data Register"]
    #[inline(always)]
    pub const fn gp_ch0_data(&self) -> &GP_CH0_DATA {
        &self.gp_ch0_data
    }
    #[doc = "0x84 - GPADC CH1 Data Register"]
    #[inline(always)]
    pub const fn gp_ch1_data(&self) -> &GP_CH1_DATA {
        &self.gp_ch1_data
    }
}
#[doc = "gp_sr_con (rw) register accessor: GPADC Sample Rate Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_sr_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_sr_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_sr_con`] module"]
pub type GP_SR_CON = crate::Reg<gp_sr_con::GP_SR_CON_SPEC>;
#[doc = "GPADC Sample Rate Configure Register"]
pub mod gp_sr_con;
#[doc = "gp_ctrl (rw) register accessor: GPADC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_ctrl`] module"]
pub type GP_CTRL = crate::Reg<gp_ctrl::GP_CTRL_SPEC>;
#[doc = "GPADC Control Register"]
pub mod gp_ctrl;
#[doc = "gp_cs_en (rw) register accessor: GPADC Compare and Select Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_cs_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_cs_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_cs_en`] module"]
pub type GP_CS_EN = crate::Reg<gp_cs_en::GP_CS_EN_SPEC>;
#[doc = "GPADC Compare and Select Enable Register"]
pub mod gp_cs_en;
#[doc = "gp_fifo_intc (rw) register accessor: GPADC FIFO Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_fifo_intc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_fifo_intc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_fifo_intc`] module"]
pub type GP_FIFO_INTC = crate::Reg<gp_fifo_intc::GP_FIFO_INTC_SPEC>;
#[doc = "GPADC FIFO Interrupt Control Register"]
pub mod gp_fifo_intc;
#[doc = "gp_fifo_ints (rw) register accessor: GPADC FIFO Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_fifo_ints::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_fifo_ints::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_fifo_ints`] module"]
pub type GP_FIFO_INTS = crate::Reg<gp_fifo_ints::GP_FIFO_INTS_SPEC>;
#[doc = "GPADC FIFO Interrupt Status Register"]
pub mod gp_fifo_ints;
#[doc = "gp_fifo_data (rw) register accessor: GPADC FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_fifo_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_fifo_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_fifo_data`] module"]
pub type GP_FIFO_DATA = crate::Reg<gp_fifo_data::GP_FIFO_DATA_SPEC>;
#[doc = "GPADC FIFO Data Register"]
pub mod gp_fifo_data;
#[doc = "gp_cdata (rw) register accessor: GPADC Calibration Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_cdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_cdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_cdata`] module"]
pub type GP_CDATA = crate::Reg<gp_cdata::GP_CDATA_SPEC>;
#[doc = "GPADC Calibration Data Register"]
pub mod gp_cdata;
#[doc = "gp_datal_intc (rw) register accessor: GPADC Data Low Interrupt Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_datal_intc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_datal_intc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_datal_intc`] module"]
pub type GP_DATAL_INTC = crate::Reg<gp_datal_intc::GP_DATAL_INTC_SPEC>;
#[doc = "GPADC Data Low Interrupt Configure Register"]
pub mod gp_datal_intc;
#[doc = "gp_datah_intc (rw) register accessor: GPADC Data High Interrupt Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_datah_intc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_datah_intc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_datah_intc`] module"]
pub type GP_DATAH_INTC = crate::Reg<gp_datah_intc::GP_DATAH_INTC_SPEC>;
#[doc = "GPADC Data High Interrupt Configure Register"]
pub mod gp_datah_intc;
#[doc = "gp_data_intc (rw) register accessor: GPADC Data Interrupt Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_data_intc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_data_intc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_data_intc`] module"]
pub type GP_DATA_INTC = crate::Reg<gp_data_intc::GP_DATA_INTC_SPEC>;
#[doc = "GPADC Data Interrupt Configure Register"]
pub mod gp_data_intc;
#[doc = "gp_datal_ints (rw) register accessor: GPADC Data Low Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_datal_ints::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_datal_ints::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_datal_ints`] module"]
pub type GP_DATAL_INTS = crate::Reg<gp_datal_ints::GP_DATAL_INTS_SPEC>;
#[doc = "GPADC Data Low Interrupt Status Register"]
pub mod gp_datal_ints;
#[doc = "gp_datah_ints (rw) register accessor: GPADC Data High Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_datah_ints::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_datah_ints::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_datah_ints`] module"]
pub type GP_DATAH_INTS = crate::Reg<gp_datah_ints::GP_DATAH_INTS_SPEC>;
#[doc = "GPADC Data High Interrupt Status Register"]
pub mod gp_datah_ints;
#[doc = "gp_data_ints (rw) register accessor: GPADC Data Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_data_ints::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_data_ints::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_data_ints`] module"]
pub type GP_DATA_INTS = crate::Reg<gp_data_ints::GP_DATA_INTS_SPEC>;
#[doc = "GPADC Data Interrupt Status Register"]
pub mod gp_data_ints;
#[doc = "gp_ch0_cmp_data (rw) register accessor: GPADC CH0 Compare Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_ch0_cmp_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_ch0_cmp_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_ch0_cmp_data`] module"]
pub type GP_CH0_CMP_DATA = crate::Reg<gp_ch0_cmp_data::GP_CH0_CMP_DATA_SPEC>;
#[doc = "GPADC CH0 Compare Data Register"]
pub mod gp_ch0_cmp_data;
#[doc = "gp_ch1_cmp_data (rw) register accessor: GPADC CH1 Compare Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_ch1_cmp_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_ch1_cmp_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_ch1_cmp_data`] module"]
pub type GP_CH1_CMP_DATA = crate::Reg<gp_ch1_cmp_data::GP_CH1_CMP_DATA_SPEC>;
#[doc = "GPADC CH1 Compare Data Register"]
pub mod gp_ch1_cmp_data;
#[doc = "gp_ch0_data (rw) register accessor: GPADC CH0 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_ch0_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_ch0_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_ch0_data`] module"]
pub type GP_CH0_DATA = crate::Reg<gp_ch0_data::GP_CH0_DATA_SPEC>;
#[doc = "GPADC CH0 Data Register"]
pub mod gp_ch0_data;
#[doc = "gp_ch1_data (rw) register accessor: GPADC CH1 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_ch1_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_ch1_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_ch1_data`] module"]
pub type GP_CH1_DATA = crate::Reg<gp_ch1_data::GP_CH1_DATA_SPEC>;
#[doc = "GPADC CH1 Data Register"]
pub mod gp_ch1_data;
