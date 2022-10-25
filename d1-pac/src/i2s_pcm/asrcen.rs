#[doc = "Register `asrcen` reader"]
pub struct R(crate::R<ASRCEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASRCEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASRCEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASRCEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `asrcen` writer"]
pub struct W(crate::W<ASRCEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASRCEN_SPEC>;
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
impl From<crate::W<ASRCEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASRCEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `asrc_fn` reader - ASRC Function Enable"]
pub type ASRC_FN_R = crate::BitReader<ASRC_FN_A>;
#[doc = "ASRC Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASRC_FN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ASRC_FN_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_FN_A) -> Self {
        variant as u8 != 0
    }
}
impl ASRC_FN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASRC_FN_A {
        match self.bits {
            false => ASRC_FN_A::DISABLE,
            true => ASRC_FN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ASRC_FN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ASRC_FN_A::ENABLE
    }
}
#[doc = "Field `asrc_fn` writer - ASRC Function Enable"]
pub type ASRC_FN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASRCEN_SPEC, ASRC_FN_A, O>;
impl<'a, const O: u8> ASRC_FN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ASRC_FN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ASRC_FN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 31 - ASRC Function Enable"]
    #[inline(always)]
    pub fn asrc_fn(&self) -> ASRC_FN_R {
        ASRC_FN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - ASRC Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asrc_fn(&mut self) -> ASRC_FN_W<31> {
        ASRC_FN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASRC Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asrcen](index.html) module"]
pub struct ASRCEN_SPEC;
impl crate::RegisterSpec for ASRCEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asrcen::R](R) reader structure"]
impl crate::Readable for ASRCEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asrcen::W](W) writer structure"]
impl crate::Writable for ASRCEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets asrcen to value 0"]
impl crate::Resettable for ASRCEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
