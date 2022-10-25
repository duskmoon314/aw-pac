#[doc = "Register `smhc_ctype` reader"]
pub struct R(crate::R<SMHC_CTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_CTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_CTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_CTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `smhc_ctype` writer"]
pub struct W(crate::W<SMHC_CTYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_CTYPE_SPEC>;
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
impl From<crate::W<SMHC_CTYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_CTYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `card_wid` reader - Card Width"]
pub type CARD_WID_R = crate::FieldReader<u8, CARD_WID_A>;
#[doc = "Card Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CARD_WID_A {
    #[doc = "0: 1-bit width"]
    B1 = 0,
    #[doc = "1: 4-bit width"]
    B4 = 1,
    #[doc = "2: 8-bit width"]
    B8 = 2,
}
impl From<CARD_WID_A> for u8 {
    #[inline(always)]
    fn from(variant: CARD_WID_A) -> Self {
        variant as _
    }
}
impl CARD_WID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CARD_WID_A> {
        match self.bits {
            0 => Some(CARD_WID_A::B1),
            1 => Some(CARD_WID_A::B4),
            2 => Some(CARD_WID_A::B8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B1`"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CARD_WID_A::B1
    }
    #[doc = "Checks if the value of the field is `B4`"]
    #[inline(always)]
    pub fn is_b4(&self) -> bool {
        *self == CARD_WID_A::B4
    }
    #[doc = "Checks if the value of the field is `B8`"]
    #[inline(always)]
    pub fn is_b8(&self) -> bool {
        *self == CARD_WID_A::B8
    }
}
#[doc = "Field `card_wid` writer - Card Width"]
pub type CARD_WID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SMHC_CTYPE_SPEC, u8, CARD_WID_A, 2, O>;
impl<'a, const O: u8> CARD_WID_W<'a, O> {
    #[doc = "1-bit width"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut W {
        self.variant(CARD_WID_A::B1)
    }
    #[doc = "4-bit width"]
    #[inline(always)]
    pub fn b4(self) -> &'a mut W {
        self.variant(CARD_WID_A::B4)
    }
    #[doc = "8-bit width"]
    #[inline(always)]
    pub fn b8(self) -> &'a mut W {
        self.variant(CARD_WID_A::B8)
    }
}
impl R {
    #[doc = "Bits 0:1 - Card Width"]
    #[inline(always)]
    pub fn card_wid(&self) -> CARD_WID_R {
        CARD_WID_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Card Width"]
    #[inline(always)]
    #[must_use]
    pub fn card_wid(&mut self) -> CARD_WID_W<0> {
        CARD_WID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Width Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_ctype](index.html) module"]
pub struct SMHC_CTYPE_SPEC;
impl crate::RegisterSpec for SMHC_CTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_ctype::R](R) reader structure"]
impl crate::Readable for SMHC_CTYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_ctype::W](W) writer structure"]
impl crate::Writable for SMHC_CTYPE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_ctype to value 0"]
impl crate::Resettable for SMHC_CTYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
