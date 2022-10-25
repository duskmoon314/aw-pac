#[doc = "Register `csi_clk` reader"]
pub struct R(crate::R<CSI_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSI_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSI_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSI_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csi_clk` writer"]
pub struct W(crate::W<CSI_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSI_CLK_SPEC>;
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
impl From<crate::W<CSI_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSI_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `factor_m` reader - Factor M"]
pub type FACTOR_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `factor_m` writer - Factor M"]
pub type FACTOR_M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSI_CLK_SPEC, u8, u8, 4, O>;
#[doc = "Field `clk_src_sel` reader - Clock Source Select"]
pub type CLK_SRC_SEL_R = crate::FieldReader<u8, CLK_SRC_SEL_A>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_SRC_SEL_A {
    #[doc = "0: `0`"]
    PLL_PERI_2X = 0,
    #[doc = "1: `1`"]
    PLL_VIDEO0_2X = 1,
    #[doc = "2: `10`"]
    PLL_VIDEO1_2X = 2,
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
            0 => Some(CLK_SRC_SEL_A::PLL_PERI_2X),
            1 => Some(CLK_SRC_SEL_A::PLL_VIDEO0_2X),
            2 => Some(CLK_SRC_SEL_A::PLL_VIDEO1_2X),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLL_PERI_2X`"]
    #[inline(always)]
    pub fn is_pll_peri_2x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_PERI_2X
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO0_2X`"]
    #[inline(always)]
    pub fn is_pll_video0_2x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VIDEO0_2X
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO1_2X`"]
    #[inline(always)]
    pub fn is_pll_video1_2x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VIDEO1_2X
    }
}
#[doc = "Field `clk_src_sel` writer - Clock Source Select"]
pub type CLK_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSI_CLK_SPEC, u8, CLK_SRC_SEL_A, 3, O>;
impl<'a, const O: u8> CLK_SRC_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_peri_2x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_2X)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pll_video0_2x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO0_2X)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pll_video1_2x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO1_2X)
    }
}
#[doc = "Field `clk_gating` reader - Gating Clock"]
pub type CLK_GATING_R = crate::BitReader<CLK_GATING_A>;
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[doc = "Field `clk_gating` writer - Gating Clock"]
pub type CLK_GATING_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSI_CLK_SPEC, CLK_GATING_A, O>;
impl<'a, const O: u8> CLK_GATING_W<'a, O> {
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
impl R {
    #[doc = "Bits 0:3 - Factor M"]
    #[inline(always)]
    pub fn factor_m(&self) -> FACTOR_M_R {
        FACTOR_M_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn clk_gating(&self) -> CLK_GATING_R {
        CLK_GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Factor M"]
    #[inline(always)]
    #[must_use]
    pub fn factor_m(&mut self) -> FACTOR_M_W<0> {
        FACTOR_M_W::new(self)
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<24> {
        CLK_SRC_SEL_W::new(self)
    }
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gating(&mut self) -> CLK_GATING_W<31> {
        CLK_GATING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSI Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csi_clk](index.html) module"]
pub struct CSI_CLK_SPEC;
impl crate::RegisterSpec for CSI_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csi_clk::R](R) reader structure"]
impl crate::Readable for CSI_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csi_clk::W](W) writer structure"]
impl crate::Writable for CSI_CLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csi_clk to value 0"]
impl crate::Resettable for CSI_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
