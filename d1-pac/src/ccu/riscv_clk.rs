#[doc = "Register `riscv_clk` reader"]
pub struct R(crate::R<RISCV_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RISCV_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RISCV_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RISCV_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `riscv_clk` writer"]
pub struct W(crate::W<RISCV_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RISCV_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RISCV_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RISCV_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `div_cfg` reader - Factor M"]
pub type DIV_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `div_cfg` writer - Factor M"]
pub type DIV_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RISCV_CLK_SPEC, u8, u8, 5, O>;
#[doc = "Field `axi_div_cfg` reader - Factor N"]
pub type AXI_DIV_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `axi_div_cfg` writer - Factor N"]
pub type AXI_DIV_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RISCV_CLK_SPEC, u8, u8, 2, O>;
#[doc = "Field `clk_src_sel` reader - Clock Source Select"]
pub type CLK_SRC_SEL_R = crate::FieldReader<u8, CLK_SRC_SEL_A>;
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
impl CLK_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK_SRC_SEL_A> {
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
    #[doc = "Checks if the value of the field is `HOSC`"]
    #[inline(always)]
    pub fn is_hosc(&self) -> bool {
        *self == CLK_SRC_SEL_A::HOSC
    }
    #[doc = "Checks if the value of the field is `CLK32K`"]
    #[inline(always)]
    pub fn is_clk32k(&self) -> bool {
        *self == CLK_SRC_SEL_A::CLK32K
    }
    #[doc = "Checks if the value of the field is `CLK16M_RC`"]
    #[inline(always)]
    pub fn is_clk16m_rc(&self) -> bool {
        *self == CLK_SRC_SEL_A::CLK16M_RC
    }
    #[doc = "Checks if the value of the field is `PLL_PERI_800M`"]
    #[inline(always)]
    pub fn is_pll_peri_800m(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_PERI_800M
    }
    #[doc = "Checks if the value of the field is `PLL_PERI_1X`"]
    #[inline(always)]
    pub fn is_pll_peri_1x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_PERI_1X
    }
    #[doc = "Checks if the value of the field is `PLL_CPU`"]
    #[inline(always)]
    pub fn is_pll_cpu(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_CPU
    }
    #[doc = "Checks if the value of the field is `PLL_AUDIO1_DIV2`"]
    #[inline(always)]
    pub fn is_pll_audio1_div2(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_AUDIO1_DIV2
    }
}
#[doc = "Field `clk_src_sel` writer - Clock Source Select"]
pub type CLK_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RISCV_CLK_SPEC, u8, CLK_SRC_SEL_A, 3, O>;
impl<'a, const O: u8> CLK_SRC_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn hosc(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::HOSC)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clk32k(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::CLK32K)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn clk16m_rc(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::CLK16M_RC)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pll_peri_800m(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_800M)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pll_peri_1x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_1X)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pll_cpu(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_CPU)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pll_audio1_div2(self) -> &'a mut W {
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
    pub fn div_cfg(&mut self) -> DIV_CFG_W<0> {
        DIV_CFG_W::new(self)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    #[must_use]
    pub fn axi_div_cfg(&mut self) -> AXI_DIV_CFG_W<8> {
        AXI_DIV_CFG_W::new(self)
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<24> {
        CLK_SRC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RISC-V Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [riscv_clk](index.html) module"]
pub struct RISCV_CLK_SPEC;
impl crate::RegisterSpec for RISCV_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [riscv_clk::R](R) reader structure"]
impl crate::Readable for RISCV_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [riscv_clk::W](W) writer structure"]
impl crate::Writable for RISCV_CLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets riscv_clk to value 0"]
impl crate::Resettable for RISCV_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
