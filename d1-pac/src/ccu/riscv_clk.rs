#[doc = "Register `riscv_clk` reader"]
pub type R = crate::R<RISCV_CLK_SPEC>;
#[doc = "Register `riscv_clk` writer"]
pub type W = crate::W<RISCV_CLK_SPEC>;
#[doc = "Field `div_cfg` reader - Factor M"]
pub type DIV_CFG_R = crate::FieldReader;
#[doc = "Field `div_cfg` writer - Factor M"]
pub type DIV_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `axi_div_cfg` reader - Factor N"]
pub type AXI_DIV_CFG_R = crate::FieldReader;
#[doc = "Field `axi_div_cfg` writer - Factor N"]
pub type AXI_DIV_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clk_src_sel` reader - Clock Source Select"]
pub type CLK_SRC_SEL_R = crate::FieldReader<CLK_SRC_SEL_A>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_SRC_SEL_A {
    #[doc = "0: `0`"]
    HOSC = 0,
    #[doc = "1: `1`"]
    CLK32K = 1,
    #[doc = "2: `10`"]
    CLK16M_RC = 2,
    #[doc = "3: `11`"]
    PLL_PERI_800M = 3,
    #[doc = "4: `100`"]
    PLL_PERI_1X = 4,
    #[doc = "5: `101`"]
    PLL_CPU = 5,
    #[doc = "6: `110`"]
    PLL_AUDIO1_DIV2 = 6,
}
impl From<CLK_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLK_SRC_SEL_A {
    type Ux = u8;
}
impl CLK_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLK_SRC_SEL_A> {
        match self.bits {
            0 => Some(CLK_SRC_SEL_A::HOSC),
            1 => Some(CLK_SRC_SEL_A::CLK32K),
            2 => Some(CLK_SRC_SEL_A::CLK16M_RC),
            3 => Some(CLK_SRC_SEL_A::PLL_PERI_800M),
            4 => Some(CLK_SRC_SEL_A::PLL_PERI_1X),
            5 => Some(CLK_SRC_SEL_A::PLL_CPU),
            6 => Some(CLK_SRC_SEL_A::PLL_AUDIO1_DIV2),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_hosc(&self) -> bool {
        *self == CLK_SRC_SEL_A::HOSC
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clk32k(&self) -> bool {
        *self == CLK_SRC_SEL_A::CLK32K
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_clk16m_rc(&self) -> bool {
        *self == CLK_SRC_SEL_A::CLK16M_RC
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_pll_peri_800m(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_PERI_800M
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_pll_peri_1x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_PERI_1X
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_pll_cpu(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_CPU
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_pll_audio1_div2(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_AUDIO1_DIV2
    }
}
#[doc = "Field `clk_src_sel` writer - Clock Source Select"]
pub type CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CLK_SRC_SEL_A>;
impl<'a, REG> CLK_SRC_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn hosc(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::HOSC)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clk32k(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::CLK32K)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn clk16m_rc(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::CLK16M_RC)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pll_peri_800m(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_800M)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pll_peri_1x(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_1X)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pll_cpu(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_CPU)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pll_audio1_div2(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_AUDIO1_DIV2)
    }
}
impl R {
    #[doc = "Bits 0:4 - Factor M"]
    #[inline(always)]
    pub fn div_cfg(&self) -> DIV_CFG_R {
        DIV_CFG_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    pub fn axi_div_cfg(&self) -> AXI_DIV_CFG_R {
        AXI_DIV_CFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Factor M"]
    #[inline(always)]
    #[must_use]
    pub fn div_cfg(&mut self) -> DIV_CFG_W<RISCV_CLK_SPEC> {
        DIV_CFG_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    #[must_use]
    pub fn axi_div_cfg(&mut self) -> AXI_DIV_CFG_W<RISCV_CLK_SPEC> {
        AXI_DIV_CFG_W::new(self, 8)
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<RISCV_CLK_SPEC> {
        CLK_SRC_SEL_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RISC-V Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`riscv_clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RISCV_CLK_SPEC;
impl crate::RegisterSpec for RISCV_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`riscv_clk::R`](R) reader structure"]
impl crate::Readable for RISCV_CLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`riscv_clk::W`](W) writer structure"]
impl crate::Writable for RISCV_CLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets riscv_clk to value 0"]
impl crate::Resettable for RISCV_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
