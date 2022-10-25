#[doc = "Register `csic_feature` reader"]
pub struct R(crate::R<CSIC_FEATURE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_FEATURE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_FEATURE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_FEATURE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_feature` writer"]
pub struct W(crate::W<CSIC_FEATURE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_FEATURE_SPEC>;
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
impl From<crate::W<CSIC_FEATURE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_FEATURE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma0_embedded_fbc` reader - "]
pub type DMA0_EMBEDDED_FBC_R = crate::BitReader<DMA0_EMBEDDED_FBC_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA0_EMBEDDED_FBC_A {
    #[doc = "0: No Embedded DMA"]
    NO_E_MBEDDED = 0,
    #[doc = "1: Embedded FBC"]
    E_MBEDDED = 1,
}
impl From<DMA0_EMBEDDED_FBC_A> for bool {
    #[inline(always)]
    fn from(variant: DMA0_EMBEDDED_FBC_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA0_EMBEDDED_FBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0_EMBEDDED_FBC_A {
        match self.bits {
            false => DMA0_EMBEDDED_FBC_A::NO_E_MBEDDED,
            true => DMA0_EMBEDDED_FBC_A::E_MBEDDED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_E_MBEDDED`"]
    #[inline(always)]
    pub fn is_no_e_mbedded(&self) -> bool {
        *self == DMA0_EMBEDDED_FBC_A::NO_E_MBEDDED
    }
    #[doc = "Checks if the value of the field is `E_MBEDDED`"]
    #[inline(always)]
    pub fn is_e_mbedded(&self) -> bool {
        *self == DMA0_EMBEDDED_FBC_A::E_MBEDDED
    }
}
#[doc = "Field `dma0_embedded_lbc` reader - "]
pub type DMA0_EMBEDDED_LBC_R = crate::BitReader<DMA0_EMBEDDED_LBC_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA0_EMBEDDED_LBC_A {
    #[doc = "0: No Embedded LBC"]
    NO_E_MBEDDED = 0,
    #[doc = "1: Embedded LBC"]
    E_MBEDDED = 1,
}
impl From<DMA0_EMBEDDED_LBC_A> for bool {
    #[inline(always)]
    fn from(variant: DMA0_EMBEDDED_LBC_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA0_EMBEDDED_LBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0_EMBEDDED_LBC_A {
        match self.bits {
            false => DMA0_EMBEDDED_LBC_A::NO_E_MBEDDED,
            true => DMA0_EMBEDDED_LBC_A::E_MBEDDED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_E_MBEDDED`"]
    #[inline(always)]
    pub fn is_no_e_mbedded(&self) -> bool {
        *self == DMA0_EMBEDDED_LBC_A::NO_E_MBEDDED
    }
    #[doc = "Checks if the value of the field is `E_MBEDDED`"]
    #[inline(always)]
    pub fn is_e_mbedded(&self) -> bool {
        *self == DMA0_EMBEDDED_LBC_A::E_MBEDDED
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma0_embedded_fbc(&self) -> DMA0_EMBEDDED_FBC_R {
        DMA0_EMBEDDED_FBC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma0_embedded_lbc(&self) -> DMA0_EMBEDDED_LBC_R {
        DMA0_EMBEDDED_LBC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC DMA Feature List Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_feature](index.html) module"]
pub struct CSIC_FEATURE_SPEC;
impl crate::RegisterSpec for CSIC_FEATURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_feature::R](R) reader structure"]
impl crate::Readable for CSIC_FEATURE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_feature::W](W) writer structure"]
impl crate::Writable for CSIC_FEATURE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_feature to value 0"]
impl crate::Resettable for CSIC_FEATURE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
