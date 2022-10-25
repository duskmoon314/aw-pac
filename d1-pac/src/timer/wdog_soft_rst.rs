#[doc = "Register `wdog_soft_rst` reader"]
pub struct R(crate::R<WDOG_SOFT_RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOG_SOFT_RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOG_SOFT_RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOG_SOFT_RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wdog_soft_rst` writer"]
pub struct W(crate::W<WDOG_SOFT_RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOG_SOFT_RST_SPEC>;
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
impl From<crate::W<WDOG_SOFT_RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOG_SOFT_RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `soft_rst_en` reader - Soft Reset Enable"]
pub type SOFT_RST_EN_R = crate::BitReader<SOFT_RST_EN_A>;
#[doc = "Soft Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFT_RST_EN_A {
    #[doc = "0: `0`"]
    DEASSERT = 0,
    #[doc = "1: `1`"]
    RESET = 1,
}
impl From<SOFT_RST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SOFT_RST_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SOFT_RST_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFT_RST_EN_A {
        match self.bits {
            false => SOFT_RST_EN_A::DEASSERT,
            true => SOFT_RST_EN_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == SOFT_RST_EN_A::DEASSERT
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SOFT_RST_EN_A::RESET
    }
}
#[doc = "Field `soft_rst_en` writer - Soft Reset Enable"]
pub type SOFT_RST_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WDOG_SOFT_RST_SPEC, SOFT_RST_EN_A, O>;
impl<'a, const O: u8> SOFT_RST_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(SOFT_RST_EN_A::DEASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SOFT_RST_EN_A::RESET)
    }
}
#[doc = "Field `key_field` writer - Key Field"]
pub type KEY_FIELD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WDOG_SOFT_RST_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Soft Reset Enable"]
    #[inline(always)]
    pub fn soft_rst_en(&self) -> SOFT_RST_EN_R {
        SOFT_RST_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Soft Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn soft_rst_en(&mut self) -> SOFT_RST_EN_W<0> {
        SOFT_RST_EN_W::new(self)
    }
    #[doc = "Bits 16:31 - Key Field"]
    #[inline(always)]
    #[must_use]
    pub fn key_field(&mut self) -> KEY_FIELD_W<16> {
        KEY_FIELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Software Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdog_soft_rst](index.html) module"]
pub struct WDOG_SOFT_RST_SPEC;
impl crate::RegisterSpec for WDOG_SOFT_RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdog_soft_rst::R](R) reader structure"]
impl crate::Readable for WDOG_SOFT_RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdog_soft_rst::W](W) writer structure"]
impl crate::Writable for WDOG_SOFT_RST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wdog_soft_rst to value 0"]
impl crate::Resettable for WDOG_SOFT_RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
