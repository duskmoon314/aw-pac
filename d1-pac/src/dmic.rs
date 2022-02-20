#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMIC Enable Control Register"]
    pub dmic_en: crate::Reg<dmic_en::DMIC_EN_SPEC>,
    #[doc = "0x04 - DMIC Sample Rate Register"]
    pub dmic_sr: crate::Reg<dmic_sr::DMIC_SR_SPEC>,
    #[doc = "0x08 - DMIC Control Register"]
    pub dmic_ctr: crate::Reg<dmic_ctr::DMIC_CTR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - DMIC Data Register"]
    pub dmic_data: crate::Reg<dmic_data::DMIC_DATA_SPEC>,
    #[doc = "0x14 - DMIC Interrupt Control Register"]
    pub dmic_intc: crate::Reg<dmic_intc::DMIC_INTC_SPEC>,
    #[doc = "0x18 - DMIC Interrupt Status Register"]
    pub dmic_ints: crate::Reg<dmic_ints::DMIC_INTS_SPEC>,
    #[doc = "0x1c - DMIC RXFIFO Control Register"]
    pub dmic_rxfifo_ctr: crate::Reg<dmic_rxfifo_ctr::DMIC_RXFIFO_CTR_SPEC>,
    #[doc = "0x20 - DMIC RXFIFO Status Register"]
    pub dmic_rxfifo_sta: crate::Reg<dmic_rxfifo_sta::DMIC_RXFIFO_STA_SPEC>,
    #[doc = "0x24 - DMIC Channel Numbers Register"]
    pub dmic_ch_num: crate::Reg<dmic_ch_num::DMIC_CH_NUM_SPEC>,
    #[doc = "0x28 - DMIC Channel Mapping Register"]
    pub dmic_ch_map: crate::Reg<dmic_ch_map::DMIC_CH_MAP_SPEC>,
    #[doc = "0x2c - DMIC Counter Register"]
    pub dmic_cnt: crate::Reg<dmic_cnt::DMIC_CNT_SPEC>,
    #[doc = "0x30 - Data0 and Data1 Volume Control Register"]
    pub data0_data1_vol_ctr: crate::Reg<data0_data1_vol_ctr::DATA0_DATA1_VOL_CTR_SPEC>,
    #[doc = "0x34 - Data2 And Data3 Volume Control Register"]
    pub data2_data3_vol_ctr: crate::Reg<data2_data3_vol_ctr::DATA2_DATA3_VOL_CTR_SPEC>,
    #[doc = "0x38 - High Pass Filter Enable Control Register"]
    pub hpf_en_ctr: crate::Reg<hpf_en_ctr::HPF_EN_CTR_SPEC>,
    #[doc = "0x3c - High Pass Filter Coefficient Register"]
    pub hpf_coef_reg: crate::Reg<hpf_coef_reg::HPF_COEF_REG_SPEC>,
    #[doc = "0x40 - High Pass Filter Gain Register"]
    pub hpf_gain_reg: crate::Reg<hpf_gain_reg::HPF_GAIN_REG_SPEC>,
}
#[doc = "DMIC_EN register accessor: an alias for `Reg<DMIC_EN_SPEC>`"]
pub type DMIC_EN = crate::Reg<dmic_en::DMIC_EN_SPEC>;
#[doc = "DMIC Enable Control Register"]
pub mod dmic_en;
#[doc = "DMIC_SR register accessor: an alias for `Reg<DMIC_SR_SPEC>`"]
pub type DMIC_SR = crate::Reg<dmic_sr::DMIC_SR_SPEC>;
#[doc = "DMIC Sample Rate Register"]
pub mod dmic_sr;
#[doc = "DMIC_CTR register accessor: an alias for `Reg<DMIC_CTR_SPEC>`"]
pub type DMIC_CTR = crate::Reg<dmic_ctr::DMIC_CTR_SPEC>;
#[doc = "DMIC Control Register"]
pub mod dmic_ctr;
#[doc = "DMIC_DATA register accessor: an alias for `Reg<DMIC_DATA_SPEC>`"]
pub type DMIC_DATA = crate::Reg<dmic_data::DMIC_DATA_SPEC>;
#[doc = "DMIC Data Register"]
pub mod dmic_data;
#[doc = "DMIC_INTC register accessor: an alias for `Reg<DMIC_INTC_SPEC>`"]
pub type DMIC_INTC = crate::Reg<dmic_intc::DMIC_INTC_SPEC>;
#[doc = "DMIC Interrupt Control Register"]
pub mod dmic_intc;
#[doc = "DMIC_INTS register accessor: an alias for `Reg<DMIC_INTS_SPEC>`"]
pub type DMIC_INTS = crate::Reg<dmic_ints::DMIC_INTS_SPEC>;
#[doc = "DMIC Interrupt Status Register"]
pub mod dmic_ints;
#[doc = "DMIC_RXFIFO_CTR register accessor: an alias for `Reg<DMIC_RXFIFO_CTR_SPEC>`"]
pub type DMIC_RXFIFO_CTR = crate::Reg<dmic_rxfifo_ctr::DMIC_RXFIFO_CTR_SPEC>;
#[doc = "DMIC RXFIFO Control Register"]
pub mod dmic_rxfifo_ctr;
#[doc = "DMIC_RXFIFO_STA register accessor: an alias for `Reg<DMIC_RXFIFO_STA_SPEC>`"]
pub type DMIC_RXFIFO_STA = crate::Reg<dmic_rxfifo_sta::DMIC_RXFIFO_STA_SPEC>;
#[doc = "DMIC RXFIFO Status Register"]
pub mod dmic_rxfifo_sta;
#[doc = "DMIC_CH_NUM register accessor: an alias for `Reg<DMIC_CH_NUM_SPEC>`"]
pub type DMIC_CH_NUM = crate::Reg<dmic_ch_num::DMIC_CH_NUM_SPEC>;
#[doc = "DMIC Channel Numbers Register"]
pub mod dmic_ch_num;
#[doc = "DMIC_CH_MAP register accessor: an alias for `Reg<DMIC_CH_MAP_SPEC>`"]
pub type DMIC_CH_MAP = crate::Reg<dmic_ch_map::DMIC_CH_MAP_SPEC>;
#[doc = "DMIC Channel Mapping Register"]
pub mod dmic_ch_map;
#[doc = "DMIC_CNT register accessor: an alias for `Reg<DMIC_CNT_SPEC>`"]
pub type DMIC_CNT = crate::Reg<dmic_cnt::DMIC_CNT_SPEC>;
#[doc = "DMIC Counter Register"]
pub mod dmic_cnt;
#[doc = "DATA0_DATA1_VOL_CTR register accessor: an alias for `Reg<DATA0_DATA1_VOL_CTR_SPEC>`"]
pub type DATA0_DATA1_VOL_CTR = crate::Reg<data0_data1_vol_ctr::DATA0_DATA1_VOL_CTR_SPEC>;
#[doc = "Data0 and Data1 Volume Control Register"]
pub mod data0_data1_vol_ctr;
#[doc = "DATA2_DATA3_VOL_CTR register accessor: an alias for `Reg<DATA2_DATA3_VOL_CTR_SPEC>`"]
pub type DATA2_DATA3_VOL_CTR = crate::Reg<data2_data3_vol_ctr::DATA2_DATA3_VOL_CTR_SPEC>;
#[doc = "Data2 And Data3 Volume Control Register"]
pub mod data2_data3_vol_ctr;
#[doc = "HPF_EN_CTR register accessor: an alias for `Reg<HPF_EN_CTR_SPEC>`"]
pub type HPF_EN_CTR = crate::Reg<hpf_en_ctr::HPF_EN_CTR_SPEC>;
#[doc = "High Pass Filter Enable Control Register"]
pub mod hpf_en_ctr;
#[doc = "HPF_COEF_REG register accessor: an alias for `Reg<HPF_COEF_REG_SPEC>`"]
pub type HPF_COEF_REG = crate::Reg<hpf_coef_reg::HPF_COEF_REG_SPEC>;
#[doc = "High Pass Filter Coefficient Register"]
pub mod hpf_coef_reg;
#[doc = "HPF_GAIN_REG register accessor: an alias for `Reg<HPF_GAIN_REG_SPEC>`"]
pub type HPF_GAIN_REG = crate::Reg<hpf_gain_reg::HPF_GAIN_REG_SPEC>;
#[doc = "High Pass Filter Gain Register"]
pub mod hpf_gain_reg;
