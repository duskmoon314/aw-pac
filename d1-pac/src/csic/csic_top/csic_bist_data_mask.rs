#[doc = "Register `csic_bist_data_mask` reader"]
pub struct R(crate::R<CSIC_BIST_DATA_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_BIST_DATA_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_BIST_DATA_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_BIST_DATA_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_bist_data_mask` writer"]
pub struct W(crate::W<CSIC_BIST_DATA_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_BIST_DATA_MASK_SPEC>;
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
impl From<crate::W<CSIC_BIST_DATA_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_BIST_DATA_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bist_data_mask` reader - BIST data mask"]
pub type BIST_DATA_MASK_R = crate::FieldReader<u32, BIST_DATA_MASK_A>;
#[doc = "BIST data mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum BIST_DATA_MASK_A {
    #[doc = "0: `0`"]
    UNMASK = 0,
    #[doc = "1: `1`"]
    MASK = 1,
}
impl From<BIST_DATA_MASK_A> for u32 {
    #[inline(always)]
    fn from(variant: BIST_DATA_MASK_A) -> Self {
        variant as _
    }
}
impl BIST_DATA_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BIST_DATA_MASK_A> {
        match self.bits {
            0 => Some(BIST_DATA_MASK_A::UNMASK),
            1 => Some(BIST_DATA_MASK_A::MASK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == BIST_DATA_MASK_A::UNMASK
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == BIST_DATA_MASK_A::MASK
    }
}
#[doc = "Field `bist_data_mask` writer - BIST data mask"]
pub type BIST_DATA_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_BIST_DATA_MASK_SPEC, u32, BIST_DATA_MASK_A, 32, O>;
impl<'a, const O: u8> BIST_DATA_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(BIST_DATA_MASK_A::UNMASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(BIST_DATA_MASK_A::MASK)
    }
}
impl R {
    #[doc = "Bits 0:31 - BIST data mask"]
    #[inline(always)]
    pub fn bist_data_mask(&self) -> BIST_DATA_MASK_R {
        BIST_DATA_MASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BIST data mask"]
    #[inline(always)]
    #[must_use]
    pub fn bist_data_mask(&mut self) -> BIST_DATA_MASK_W<0> {
        BIST_DATA_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC BIST Data Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_bist_data_mask](index.html) module"]
pub struct CSIC_BIST_DATA_MASK_SPEC;
impl crate::RegisterSpec for CSIC_BIST_DATA_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_bist_data_mask::R](R) reader structure"]
impl crate::Readable for CSIC_BIST_DATA_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_bist_data_mask::W](W) writer structure"]
impl crate::Writable for CSIC_BIST_DATA_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_bist_data_mask to value 0"]
impl crate::Resettable for CSIC_BIST_DATA_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
