#[doc = "Register `avs_clk` reader"]
pub struct R(crate::R<AVS_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AVS_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AVS_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AVS_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `avs_clk` writer"]
pub struct W(crate::W<AVS_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AVS_CLK_SPEC>;
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
impl From<crate::W<AVS_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AVS_CLK_SPEC>) -> Self {
        W(writer)
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
pub type CLK_GATING_W<'a, const O: u8> = crate::BitWriter<'a, u32, AVS_CLK_SPEC, CLK_GATING_A, O>;
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
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn clk_gating(&self) -> CLK_GATING_R {
        CLK_GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
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
#[doc = "AVS Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [avs_clk](index.html) module"]
pub struct AVS_CLK_SPEC;
impl crate::RegisterSpec for AVS_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [avs_clk::R](R) reader structure"]
impl crate::Readable for AVS_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [avs_clk::W](W) writer structure"]
impl crate::Writable for AVS_CLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets avs_clk to value 0"]
impl crate::Resettable for AVS_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
