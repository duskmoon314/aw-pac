#[doc = r"Register block"]
#[repr(C)]
pub struct CSIC_CCU {
    ccu_clk_mode: CCU_CLK_MODE,
    ccu_parser_clk_en: CCU_PARSER_CLK_EN,
    _reserved2: [u8; 0x04],
    ccu_post0_clk_en: CCU_POST0_CLK_EN,
}
impl CSIC_CCU {
    #[doc = "0x00 - CCU Clock Mode Register"]
    #[inline(always)]
    pub const fn ccu_clk_mode(&self) -> &CCU_CLK_MODE {
        &self.ccu_clk_mode
    }
    #[doc = "0x04 - CCU Parser Clock Enable Register"]
    #[inline(always)]
    pub const fn ccu_parser_clk_en(&self) -> &CCU_PARSER_CLK_EN {
        &self.ccu_parser_clk_en
    }
    #[doc = "0x0c - CCU Post0 Clock Enable Register"]
    #[inline(always)]
    pub const fn ccu_post0_clk_en(&self) -> &CCU_POST0_CLK_EN {
        &self.ccu_post0_clk_en
    }
}
#[doc = "ccu_clk_mode (rw) register accessor: CCU Clock Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccu_clk_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccu_clk_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccu_clk_mode`] module"]
pub type CCU_CLK_MODE = crate::Reg<ccu_clk_mode::CCU_CLK_MODE_SPEC>;
#[doc = "CCU Clock Mode Register"]
pub mod ccu_clk_mode;
#[doc = "ccu_parser_clk_en (rw) register accessor: CCU Parser Clock Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccu_parser_clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccu_parser_clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccu_parser_clk_en`] module"]
pub type CCU_PARSER_CLK_EN = crate::Reg<ccu_parser_clk_en::CCU_PARSER_CLK_EN_SPEC>;
#[doc = "CCU Parser Clock Enable Register"]
pub mod ccu_parser_clk_en;
#[doc = "ccu_post0_clk_en (rw) register accessor: CCU Post0 Clock Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccu_post0_clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccu_post0_clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccu_post0_clk_en`] module"]
pub type CCU_POST0_CLK_EN = crate::Reg<ccu_post0_clk_en::CCU_POST0_CLK_EN_SPEC>;
#[doc = "CCU Post0 Clock Enable Register"]
pub mod ccu_post0_clk_en;
