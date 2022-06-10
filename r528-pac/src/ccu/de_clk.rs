#[doc = "Register `DE_CLK` reader"]
pub struct R(crate::R<DE_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DE_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DE_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DE_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DE_CLK` writer"]
pub struct W(crate::W<DE_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DE_CLK_SPEC>;
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
impl From<crate::W<DE_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DE_CLK_SPEC>) -> Self {
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
pub type CLK_GATING_W<'a> = crate::BitWriter<'a, u32, DE_CLK_SPEC, CLK_GATING_A, 31>;
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
#[repr(u8)]
pub enum CLK_SRC_SEL_A {
    #[doc = "0: `0`"]
    PLL_PERI_2X = 0,
    #[doc = "1: `1`"]
    PLL_VIDEO0_4X = 1,
    #[doc = "2: `10`"]
    PLL_VIDEO1_4X = 2,
    #[doc = "3: `11`"]
    PLL_AUDIO1_DIV2 = 3,
}
impl From<CLK_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SRC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLK_SRC_SEL` reader - Clock Source Select"]
pub type CLK_SRC_SEL_R = crate::FieldReader<u8, CLK_SRC_SEL_A>;
impl CLK_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK_SRC_SEL_A> {
        match self.bits {
            0 => Some(CLK_SRC_SEL_A::PLL_PERI_2X),
            1 => Some(CLK_SRC_SEL_A::PLL_VIDEO0_4X),
            2 => Some(CLK_SRC_SEL_A::PLL_VIDEO1_4X),
            3 => Some(CLK_SRC_SEL_A::PLL_AUDIO1_DIV2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLL_PERI_2X`"]
    #[inline(always)]
    pub fn is_pll_peri_2x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_PERI_2X
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO0_4X`"]
    #[inline(always)]
    pub fn is_pll_video0_4x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VIDEO0_4X
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO1_4X`"]
    #[inline(always)]
    pub fn is_pll_video1_4x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VIDEO1_4X
    }
    #[doc = "Checks if the value of the field is `PLL_AUDIO1_DIV2`"]
    #[inline(always)]
    pub fn is_pll_audio1_div2(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_AUDIO1_DIV2
    }
}
#[doc = "Field `CLK_SRC_SEL` writer - Clock Source Select"]
pub type CLK_SRC_SEL_W<'a> = crate::FieldWriter<'a, u32, DE_CLK_SPEC, u8, CLK_SRC_SEL_A, 3, 24>;
impl<'a> CLK_SRC_SEL_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_peri_2x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_2X)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pll_video0_4x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO0_4X)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pll_video1_4x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO1_4X)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pll_audio1_div2(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_AUDIO1_DIV2)
    }
}
#[doc = "Field `FACTOR_M` reader - Factor M"]
pub type FACTOR_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FACTOR_M` writer - Factor M"]
pub type FACTOR_M_W<'a> = crate::FieldWriter<'a, u32, DE_CLK_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn clk_gating(&self) -> CLK_GATING_R {
        CLK_GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 24) & 7) as u8)
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
    #[doc = "Bits 24:26 - Clock Source Select"]
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
#[doc = "DE Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [de_clk](index.html) module"]
pub struct DE_CLK_SPEC;
impl crate::RegisterSpec for DE_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [de_clk::R](R) reader structure"]
impl crate::Readable for DE_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [de_clk::W](W) writer structure"]
impl crate::Writable for DE_CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DE_CLK to value 0"]
impl crate::Resettable for DE_CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
