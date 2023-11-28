#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    dmic_en: DMIC_EN,
    dmic_sr: DMIC_SR,
    dmic_ctr: DMIC_CTR,
    _reserved3: [u8; 0x04],
    dmic_data: DMIC_DATA,
    dmic_intc: DMIC_INTC,
    dmic_ints: DMIC_INTS,
    dmic_rxfifo_ctr: DMIC_RXFIFO_CTR,
    dmic_rxfifo_sta: DMIC_RXFIFO_STA,
    dmic_ch_num: DMIC_CH_NUM,
    dmic_ch_map: DMIC_CH_MAP,
    dmic_cnt: DMIC_CNT,
    data0_data1_vol_ctr: DATA0_DATA1_VOL_CTR,
    data2_data3_vol_ctr: DATA2_DATA3_VOL_CTR,
    hpf_en_ctr: HPF_EN_CTR,
    hpf_coef: HPF_COEF,
    hpf_gain: HPF_GAIN,
}
impl RegisterBlock {
    #[doc = "0x00 - DMIC Enable Control Register"]
    #[inline(always)]
    pub const fn dmic_en(&self) -> &DMIC_EN {
        &self.dmic_en
    }
    #[doc = "0x04 - DMIC Sample Rate Register"]
    #[inline(always)]
    pub const fn dmic_sr(&self) -> &DMIC_SR {
        &self.dmic_sr
    }
    #[doc = "0x08 - DMIC Control Register"]
    #[inline(always)]
    pub const fn dmic_ctr(&self) -> &DMIC_CTR {
        &self.dmic_ctr
    }
    #[doc = "0x10 - DMIC Data Register"]
    #[inline(always)]
    pub const fn dmic_data(&self) -> &DMIC_DATA {
        &self.dmic_data
    }
    #[doc = "0x14 - DMIC Interrupt Control Register"]
    #[inline(always)]
    pub const fn dmic_intc(&self) -> &DMIC_INTC {
        &self.dmic_intc
    }
    #[doc = "0x18 - DMIC Interrupt Status Register"]
    #[inline(always)]
    pub const fn dmic_ints(&self) -> &DMIC_INTS {
        &self.dmic_ints
    }
    #[doc = "0x1c - DMIC RXFIFO Control Register"]
    #[inline(always)]
    pub const fn dmic_rxfifo_ctr(&self) -> &DMIC_RXFIFO_CTR {
        &self.dmic_rxfifo_ctr
    }
    #[doc = "0x20 - DMIC RXFIFO Status Register"]
    #[inline(always)]
    pub const fn dmic_rxfifo_sta(&self) -> &DMIC_RXFIFO_STA {
        &self.dmic_rxfifo_sta
    }
    #[doc = "0x24 - DMIC Channel Numbers Register"]
    #[inline(always)]
    pub const fn dmic_ch_num(&self) -> &DMIC_CH_NUM {
        &self.dmic_ch_num
    }
    #[doc = "0x28 - DMIC Channel Mapping Register"]
    #[inline(always)]
    pub const fn dmic_ch_map(&self) -> &DMIC_CH_MAP {
        &self.dmic_ch_map
    }
    #[doc = "0x2c - DMIC Counter Register"]
    #[inline(always)]
    pub const fn dmic_cnt(&self) -> &DMIC_CNT {
        &self.dmic_cnt
    }
    #[doc = "0x30 - Data0 and Data1 Volume Control Register"]
    #[inline(always)]
    pub const fn data0_data1_vol_ctr(&self) -> &DATA0_DATA1_VOL_CTR {
        &self.data0_data1_vol_ctr
    }
    #[doc = "0x34 - Data2 And Data3 Volume Control Register"]
    #[inline(always)]
    pub const fn data2_data3_vol_ctr(&self) -> &DATA2_DATA3_VOL_CTR {
        &self.data2_data3_vol_ctr
    }
    #[doc = "0x38 - High Pass Filter Enable Control Register"]
    #[inline(always)]
    pub const fn hpf_en_ctr(&self) -> &HPF_EN_CTR {
        &self.hpf_en_ctr
    }
    #[doc = "0x3c - High Pass Filter Coefficient Register"]
    #[inline(always)]
    pub const fn hpf_coef(&self) -> &HPF_COEF {
        &self.hpf_coef
    }
    #[doc = "0x40 - High Pass Filter Gain Register"]
    #[inline(always)]
    pub const fn hpf_gain(&self) -> &HPF_GAIN {
        &self.hpf_gain
    }
}
#[doc = "dmic_en (rw) register accessor: DMIC Enable Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmic_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmic_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmic_en`] module"]
pub type DMIC_EN = crate::Reg<dmic_en::DMIC_EN_SPEC>;
#[doc = "DMIC Enable Control Register"]
pub mod dmic_en;
#[doc = "dmic_sr (rw) register accessor: DMIC Sample Rate Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmic_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmic_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmic_sr`] module"]
pub type DMIC_SR = crate::Reg<dmic_sr::DMIC_SR_SPEC>;
#[doc = "DMIC Sample Rate Register"]
pub mod dmic_sr;
#[doc = "dmic_ctr (rw) register accessor: DMIC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmic_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmic_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmic_ctr`] module"]
pub type DMIC_CTR = crate::Reg<dmic_ctr::DMIC_CTR_SPEC>;
#[doc = "DMIC Control Register"]
pub mod dmic_ctr;
#[doc = "dmic_data (rw) register accessor: DMIC Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmic_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmic_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmic_data`] module"]
pub type DMIC_DATA = crate::Reg<dmic_data::DMIC_DATA_SPEC>;
#[doc = "DMIC Data Register"]
pub mod dmic_data;
#[doc = "dmic_intc (rw) register accessor: DMIC Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmic_intc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmic_intc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmic_intc`] module"]
pub type DMIC_INTC = crate::Reg<dmic_intc::DMIC_INTC_SPEC>;
#[doc = "DMIC Interrupt Control Register"]
pub mod dmic_intc;
#[doc = "dmic_ints (rw) register accessor: DMIC Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmic_ints::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmic_ints::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmic_ints`] module"]
pub type DMIC_INTS = crate::Reg<dmic_ints::DMIC_INTS_SPEC>;
#[doc = "DMIC Interrupt Status Register"]
pub mod dmic_ints;
#[doc = "dmic_rxfifo_ctr (rw) register accessor: DMIC RXFIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmic_rxfifo_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmic_rxfifo_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmic_rxfifo_ctr`] module"]
pub type DMIC_RXFIFO_CTR = crate::Reg<dmic_rxfifo_ctr::DMIC_RXFIFO_CTR_SPEC>;
#[doc = "DMIC RXFIFO Control Register"]
pub mod dmic_rxfifo_ctr;
#[doc = "dmic_rxfifo_sta (rw) register accessor: DMIC RXFIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmic_rxfifo_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmic_rxfifo_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmic_rxfifo_sta`] module"]
pub type DMIC_RXFIFO_STA = crate::Reg<dmic_rxfifo_sta::DMIC_RXFIFO_STA_SPEC>;
#[doc = "DMIC RXFIFO Status Register"]
pub mod dmic_rxfifo_sta;
#[doc = "dmic_ch_num (rw) register accessor: DMIC Channel Numbers Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmic_ch_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmic_ch_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmic_ch_num`] module"]
pub type DMIC_CH_NUM = crate::Reg<dmic_ch_num::DMIC_CH_NUM_SPEC>;
#[doc = "DMIC Channel Numbers Register"]
pub mod dmic_ch_num;
#[doc = "dmic_ch_map (rw) register accessor: DMIC Channel Mapping Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmic_ch_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmic_ch_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmic_ch_map`] module"]
pub type DMIC_CH_MAP = crate::Reg<dmic_ch_map::DMIC_CH_MAP_SPEC>;
#[doc = "DMIC Channel Mapping Register"]
pub mod dmic_ch_map;
#[doc = "dmic_cnt (rw) register accessor: DMIC Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmic_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmic_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmic_cnt`] module"]
pub type DMIC_CNT = crate::Reg<dmic_cnt::DMIC_CNT_SPEC>;
#[doc = "DMIC Counter Register"]
pub mod dmic_cnt;
#[doc = "data0_data1_vol_ctr (rw) register accessor: Data0 and Data1 Volume Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0_data1_vol_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data0_data1_vol_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0_data1_vol_ctr`] module"]
pub type DATA0_DATA1_VOL_CTR = crate::Reg<data0_data1_vol_ctr::DATA0_DATA1_VOL_CTR_SPEC>;
#[doc = "Data0 and Data1 Volume Control Register"]
pub mod data0_data1_vol_ctr;
#[doc = "data2_data3_vol_ctr (rw) register accessor: Data2 And Data3 Volume Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data2_data3_vol_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data2_data3_vol_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2_data3_vol_ctr`] module"]
pub type DATA2_DATA3_VOL_CTR = crate::Reg<data2_data3_vol_ctr::DATA2_DATA3_VOL_CTR_SPEC>;
#[doc = "Data2 And Data3 Volume Control Register"]
pub mod data2_data3_vol_ctr;
#[doc = "hpf_en_ctr (rw) register accessor: High Pass Filter Enable Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpf_en_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpf_en_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpf_en_ctr`] module"]
pub type HPF_EN_CTR = crate::Reg<hpf_en_ctr::HPF_EN_CTR_SPEC>;
#[doc = "High Pass Filter Enable Control Register"]
pub mod hpf_en_ctr;
#[doc = "hpf_coef (rw) register accessor: High Pass Filter Coefficient Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpf_coef::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpf_coef::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpf_coef`] module"]
pub type HPF_COEF = crate::Reg<hpf_coef::HPF_COEF_SPEC>;
#[doc = "High Pass Filter Coefficient Register"]
pub mod hpf_coef;
#[doc = "hpf_gain (rw) register accessor: High Pass Filter Gain Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpf_gain::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpf_gain::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpf_gain`] module"]
pub type HPF_GAIN = crate::Reg<hpf_gain::HPF_GAIN_SPEC>;
#[doc = "High Pass Filter Gain Register"]
pub mod hpf_gain;
