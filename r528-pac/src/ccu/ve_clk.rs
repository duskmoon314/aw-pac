#[doc = "Register `VE_CLK` reader"]
pub struct R(crate::R<VE_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VE_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VE_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VE_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VE_CLK` writer"]
pub struct W(crate::W<VE_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VE_CLK_SPEC>;
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
impl From<crate::W<VE_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VE_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_GATING_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLK_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_GATING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_GATING` reader - Gating Clock"]
pub type CLK_GATING_R = crate::BitReader<CLK_GATING_A>;
impl CLK_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_GATING_A {
        match self.bits {
            false => CLK_GATING_A::OFF,
            true => CLK_GATING_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLK_GATING_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CLK_GATING_A::ON
    }
}
#[doc = "Field `CLK_GATING` writer - Gating Clock"]
pub type CLK_GATING_W<'a> = crate::BitWriter<'a, u32, VE_CLK_SPEC, CLK_GATING_A, 31>;
impl<'a> CLK_GATING_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLK_GATING_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CLK_GATING_A::ON)
    }
}
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_SRC_SEL_A {
    #[doc = "0: `0`"]
    VEPLL = 0,
    #[doc = "1: `1`"]
    PLL_PERI_2X = 1,
}
impl From<CLK_SRC_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_SRC_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_SRC_SEL` reader - Clock Source Select"]
pub type CLK_SRC_SEL_R = crate::BitReader<CLK_SRC_SEL_A>;
impl CLK_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_SRC_SEL_A {
        match self.bits {
            false => CLK_SRC_SEL_A::VEPLL,
            true => CLK_SRC_SEL_A::PLL_PERI_2X,
        }
    }
    #[doc = "Checks if the value of the field is `VEPLL`"]
    #[inline(always)]
    pub fn is_vepll(&self) -> bool {
        *self == CLK_SRC_SEL_A::VEPLL
    }
    #[doc = "Checks if the value of the field is `PLL_PERI_2X`"]
    #[inline(always)]
    pub fn is_pll_peri_2x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_PERI_2X
    }
}
#[doc = "Field `CLK_SRC_SEL` writer - Clock Source Select"]
pub type CLK_SRC_SEL_W<'a> = crate::BitWriter<'a, u32, VE_CLK_SPEC, CLK_SRC_SEL_A, 24>;
impl<'a> CLK_SRC_SEL_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn vepll(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::VEPLL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pll_peri_2x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_2X)
    }
}
#[doc = "Field `FACTOR_M` reader - Factor M"]
pub type FACTOR_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FACTOR_M` writer - Factor M"]
pub type FACTOR_M_W<'a> = crate::FieldWriter<'a, u32, VE_CLK_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn clk_gating(&self) -> CLK_GATING_R {
        CLK_GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 24 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 0:4 - Factor M"]
    #[inline(always)]
    pub fn factor_m(&self) -> FACTOR_M_R {
        FACTOR_M_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn clk_gating(&mut self) -> CLK_GATING_W {
        CLK_GATING_W::new(self)
    }
    #[doc = "Bit 24 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W {
        CLK_SRC_SEL_W::new(self)
    }
    #[doc = "Bits 0:4 - Factor M"]
    #[inline(always)]
    pub fn factor_m(&mut self) -> FACTOR_M_W {
        FACTOR_M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VE Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ve_clk](index.html) module"]
pub struct VE_CLK_SPEC;
impl crate::RegisterSpec for VE_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ve_clk::R](R) reader structure"]
impl crate::Readable for VE_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ve_clk::W](W) writer structure"]
impl crate::Writable for VE_CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VE_CLK to value 0"]
impl crate::Resettable for VE_CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
