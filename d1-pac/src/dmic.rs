#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMIC Enable Control Register"]
    pub dmic_en: DMIC_EN,
    #[doc = "0x04 - DMIC Sample Rate Register"]
    pub dmic_sr: DMIC_SR,
    #[doc = "0x08 - DMIC Control Register"]
    pub dmic_ctr: DMIC_CTR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - DMIC Data Register"]
    pub dmic_data: DMIC_DATA,
    #[doc = "0x14 - DMIC Interrupt Control Register"]
    pub dmic_intc: DMIC_INTC,
    #[doc = "0x18 - DMIC Interrupt Status Register"]
    pub dmic_ints: DMIC_INTS,
    #[doc = "0x1c - DMIC RXFIFO Control Register"]
    pub dmic_rxfifo_ctr: DMIC_RXFIFO_CTR,
    #[doc = "0x20 - DMIC RXFIFO Status Register"]
    pub dmic_rxfifo_sta: DMIC_RXFIFO_STA,
    #[doc = "0x24 - DMIC Channel Numbers Register"]
    pub dmic_ch_num: DMIC_CH_NUM,
    #[doc = "0x28 - DMIC Channel Mapping Register"]
    pub dmic_ch_map: DMIC_CH_MAP,
    #[doc = "0x2c - DMIC Counter Register"]
    pub dmic_cnt: DMIC_CNT,
    #[doc = "0x30 - Data0 and Data1 Volume Control Register"]
    pub data0_data1_vol_ctr: DATA0_DATA1_VOL_CTR,
    #[doc = "0x34 - Data2 And Data3 Volume Control Register"]
    pub data2_data3_vol_ctr: DATA2_DATA3_VOL_CTR,
    #[doc = "0x38 - High Pass Filter Enable Control Register"]
    pub hpf_en_ctr: HPF_EN_CTR,
    #[doc = "0x3c - High Pass Filter Coefficient Register"]
    pub hpf_coef: HPF_COEF,
    #[doc = "0x40 - High Pass Filter Gain Register"]
    pub hpf_gain: HPF_GAIN,
}
#[doc = "dmic_en (rw) register accessor: an alias for `Reg<DMIC_EN_SPEC>`"]
pub type DMIC_EN = crate::Reg<dmic_en::DMIC_EN_SPEC>;
#[doc = "DMIC Enable Control Register"]
pub mod dmic_en;
#[doc = "dmic_sr (rw) register accessor: an alias for `Reg<DMIC_SR_SPEC>`"]
pub type DMIC_SR = crate::Reg<dmic_sr::DMIC_SR_SPEC>;
#[doc = "DMIC Sample Rate Register"]
pub mod dmic_sr;
#[doc = "dmic_ctr (rw) register accessor: an alias for `Reg<DMIC_CTR_SPEC>`"]
pub type DMIC_CTR = crate::Reg<dmic_ctr::DMIC_CTR_SPEC>;
#[doc = "DMIC Control Register"]
pub mod dmic_ctr;
#[doc = "dmic_data (rw) register accessor: an alias for `Reg<DMIC_DATA_SPEC>`"]
pub type DMIC_DATA = crate::Reg<dmic_data::DMIC_DATA_SPEC>;
#[doc = "DMIC Data Register"]
pub mod dmic_data;
#[doc = "dmic_intc (rw) register accessor: an alias for `Reg<DMIC_INTC_SPEC>`"]
pub type DMIC_INTC = crate::Reg<dmic_intc::DMIC_INTC_SPEC>;
#[doc = "DMIC Interrupt Control Register"]
pub mod dmic_intc;
#[doc = "dmic_ints (rw) register accessor: an alias for `Reg<DMIC_INTS_SPEC>`"]
pub type DMIC_INTS = crate::Reg<dmic_ints::DMIC_INTS_SPEC>;
#[doc = "DMIC Interrupt Status Register"]
pub mod dmic_ints;
#[doc = "dmic_rxfifo_ctr (rw) register accessor: an alias for `Reg<DMIC_RXFIFO_CTR_SPEC>`"]
pub type DMIC_RXFIFO_CTR = crate::Reg<dmic_rxfifo_ctr::DMIC_RXFIFO_CTR_SPEC>;
#[doc = "DMIC RXFIFO Control Register"]
pub mod dmic_rxfifo_ctr;
#[doc = "dmic_rxfifo_sta (rw) register accessor: an alias for `Reg<DMIC_RXFIFO_STA_SPEC>`"]
pub type DMIC_RXFIFO_STA = crate::Reg<dmic_rxfifo_sta::DMIC_RXFIFO_STA_SPEC>;
#[doc = "DMIC RXFIFO Status Register"]
pub mod dmic_rxfifo_sta;
#[doc = "dmic_ch_num (rw) register accessor: an alias for `Reg<DMIC_CH_NUM_SPEC>`"]
pub type DMIC_CH_NUM = crate::Reg<dmic_ch_num::DMIC_CH_NUM_SPEC>;
#[doc = "DMIC Channel Numbers Register"]
pub mod dmic_ch_num;
#[doc = "dmic_ch_map (rw) register accessor: an alias for `Reg<DMIC_CH_MAP_SPEC>`"]
pub type DMIC_CH_MAP = crate::Reg<dmic_ch_map::DMIC_CH_MAP_SPEC>;
#[doc = "DMIC Channel Mapping Register"]
pub mod dmic_ch_map;
#[doc = "dmic_cnt (rw) register accessor: an alias for `Reg<DMIC_CNT_SPEC>`"]
pub type DMIC_CNT = crate::Reg<dmic_cnt::DMIC_CNT_SPEC>;
#[doc = "DMIC Counter Register"]
pub mod dmic_cnt;
#[doc = "data0_data1_vol_ctr (rw) register accessor: an alias for `Reg<DATA0_DATA1_VOL_CTR_SPEC>`"]
pub type DATA0_DATA1_VOL_CTR = crate::Reg<data0_data1_vol_ctr::DATA0_DATA1_VOL_CTR_SPEC>;
#[doc = "Data0 and Data1 Volume Control Register"]
pub mod data0_data1_vol_ctr;
#[doc = "data2_data3_vol_ctr (rw) register accessor: an alias for `Reg<DATA2_DATA3_VOL_CTR_SPEC>`"]
pub type DATA2_DATA3_VOL_CTR = crate::Reg<data2_data3_vol_ctr::DATA2_DATA3_VOL_CTR_SPEC>;
#[doc = "Data2 And Data3 Volume Control Register"]
pub mod data2_data3_vol_ctr;
#[doc = "hpf_en_ctr (rw) register accessor: an alias for `Reg<HPF_EN_CTR_SPEC>`"]
pub type HPF_EN_CTR = crate::Reg<hpf_en_ctr::HPF_EN_CTR_SPEC>;
#[doc = "High Pass Filter Enable Control Register"]
pub mod hpf_en_ctr;
#[doc = "hpf_coef (rw) register accessor: an alias for `Reg<HPF_COEF_SPEC>`"]
pub type HPF_COEF = crate::Reg<hpf_coef::HPF_COEF_SPEC>;
#[doc = "High Pass Filter Coefficient Register"]
pub mod hpf_coef;
#[doc = "hpf_gain (rw) register accessor: an alias for `Reg<HPF_GAIN_SPEC>`"]
pub type HPF_GAIN = crate::Reg<hpf_gain::HPF_GAIN_SPEC>;
#[doc = "High Pass Filter Gain Register"]
pub mod hpf_gain;
