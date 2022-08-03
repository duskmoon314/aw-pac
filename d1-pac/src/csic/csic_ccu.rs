#[doc = r"Register block"]
#[repr(C)]
pub struct CSIC_CCU {
    #[doc = "0x00 - CCU Clock Mode Register"]
    pub ccu_clk_mode: CCU_CLK_MODE,
    #[doc = "0x04 - CCU Parser Clock Enable Register"]
    pub ccu_parser_clk_en: CCU_PARSER_CLK_EN,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - CCU Post0 Clock Enable Register"]
    pub ccu_post0_clk_en: CCU_POST0_CLK_EN,
}
#[doc = "ccu_clk_mode (rw) register accessor: an alias for `Reg<CCU_CLK_MODE_SPEC>`"]
pub type CCU_CLK_MODE = crate::Reg<ccu_clk_mode::CCU_CLK_MODE_SPEC>;
#[doc = "CCU Clock Mode Register"]
pub mod ccu_clk_mode;
#[doc = "ccu_parser_clk_en (rw) register accessor: an alias for `Reg<CCU_PARSER_CLK_EN_SPEC>`"]
pub type CCU_PARSER_CLK_EN = crate::Reg<ccu_parser_clk_en::CCU_PARSER_CLK_EN_SPEC>;
#[doc = "CCU Parser Clock Enable Register"]
pub mod ccu_parser_clk_en;
#[doc = "ccu_post0_clk_en (rw) register accessor: an alias for `Reg<CCU_POST0_CLK_EN_SPEC>`"]
pub type CCU_POST0_CLK_EN = crate::Reg<ccu_post0_clk_en::CCU_POST0_CLK_EN_SPEC>;
#[doc = "CCU Post0 Clock Enable Register"]
pub mod ccu_post0_clk_en;
