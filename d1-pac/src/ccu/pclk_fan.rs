#[doc = "Register `pclk_fan` reader"]
pub struct R(crate::R<PCLK_FAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCLK_FAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCLK_FAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCLK_FAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pclk_fan` writer"]
pub struct W(crate::W<PCLK_FAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCLK_FAN_SPEC>;
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
impl From<crate::W<PCLK_FAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCLK_FAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `div` reader - Factor M"]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `div` writer - Factor M"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCLK_FAN_SPEC, u8, u8, 5, O>;
#[doc = "Field `gating` reader - Gating for PCLK"]
pub type GATING_R = crate::BitReader<GATING_A>;
#[doc = "Gating for PCLK\n\nValue on reset: 0"]
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
#[doc = "Field `gating` writer - Gating for PCLK"]
pub type GATING_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLK_FAN_SPEC, GATING_A, O>;
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
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Gating for PCLK"]
    #[inline(always)]
    pub fn gating(&self) -> GATING_R {
        GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Factor M"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Bit 31 - Gating for PCLK"]
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
#[doc = "PCLK FANOUT Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclk_fan](index.html) module"]
pub struct PCLK_FAN_SPEC;
impl crate::RegisterSpec for PCLK_FAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pclk_fan::R](R) reader structure"]
impl crate::Readable for PCLK_FAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pclk_fan::W](W) writer structure"]
impl crate::Writable for PCLK_FAN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pclk_fan to value 0"]
impl crate::Resettable for PCLK_FAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
