#[doc = "Register `clk27m_fan` reader"]
pub struct R(crate::R<CLK27M_FAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK27M_FAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK27M_FAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK27M_FAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clk27m_fan` writer"]
pub struct W(crate::W<CLK27M_FAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK27M_FAN_SPEC>;
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
impl From<crate::W<CLK27M_FAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK27M_FAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `div0` reader - Factor M"]
pub type DIV0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `div0` writer - Factor M"]
pub type DIV0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK27M_FAN_SPEC, u8, u8, 5, O>;
#[doc = "Field `div1` reader - Factor N"]
pub type DIV1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `div1` writer - Factor N"]
pub type DIV1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK27M_FAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `clk_src_sel` reader - Clock Source Select"]
pub type CLK_SRC_SEL_R = crate::FieldReader<u8, CLK_SRC_SEL_A>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_SRC_SEL_A {
    #[doc = "0: `0`"]
    PLL_VIDEO0_1X = 0,
    #[doc = "1: `1`"]
    PLL_VIDEO1_1X = 1,
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
            0 => Some(CLK_SRC_SEL_A::PLL_VIDEO0_1X),
            1 => Some(CLK_SRC_SEL_A::PLL_VIDEO1_1X),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO0_1X`"]
    #[inline(always)]
    pub fn is_pll_video0_1x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VIDEO0_1X
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO1_1X`"]
    #[inline(always)]
    pub fn is_pll_video1_1x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VIDEO1_1X
    }
}
#[doc = "Field `clk_src_sel` writer - Clock Source Select"]
pub type CLK_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK27M_FAN_SPEC, u8, CLK_SRC_SEL_A, 2, O>;
impl<'a, const O: u8> CLK_SRC_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_video0_1x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO0_1X)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pll_video1_1x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO1_1X)
    }
}
#[doc = "Field `gating` reader - Gating for CLK27M"]
pub type GATING_R = crate::BitReader<GATING_A>;
#[doc = "Gating for CLK27M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GATING_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<GATING_A> for bool {
    #[inline(always)]
    fn from(variant: GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GATING_A {
        match self.bits {
            false => GATING_A::OFF,
            true => GATING_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == GATING_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == GATING_A::ON
    }
}
#[doc = "Field `gating` writer - Gating for CLK27M"]
pub type GATING_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK27M_FAN_SPEC, GATING_A, O>;
impl<'a, const O: u8> GATING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(GATING_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(GATING_A::ON)
    }
}
impl R {
    #[doc = "Bits 0:4 - Factor M"]
    #[inline(always)]
    pub fn div0(&self) -> DIV0_R {
        DIV0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    pub fn div1(&self) -> DIV1_R {
        DIV1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 31 - Gating for CLK27M"]
    #[inline(always)]
    pub fn gating(&self) -> GATING_R {
        GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Factor M"]
    #[inline(always)]
    #[must_use]
    pub fn div0(&mut self) -> DIV0_W<0> {
        DIV0_W::new(self)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    #[must_use]
    pub fn div1(&mut self) -> DIV1_W<8> {
        DIV1_W::new(self)
    }
    #[doc = "Bits 24:25 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<24> {
        CLK_SRC_SEL_W::new(self)
    }
    #[doc = "Bit 31 - Gating for CLK27M"]
    #[inline(always)]
    #[must_use]
    pub fn gating(&mut self) -> GATING_W<31> {
        GATING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLK27M FANOUT Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk27m_fan](index.html) module"]
pub struct CLK27M_FAN_SPEC;
impl crate::RegisterSpec for CLK27M_FAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk27m_fan::R](R) reader structure"]
impl crate::Readable for CLK27M_FAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk27m_fan::W](W) writer structure"]
impl crate::Writable for CLK27M_FAN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clk27m_fan to value 0"]
impl crate::Resettable for CLK27M_FAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
