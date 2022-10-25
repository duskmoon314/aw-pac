#[doc = "Register `caplength` reader"]
pub struct R(crate::R<CAPLENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPLENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPLENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPLENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `caplength` reader - The value in these bits indicates an offset to add to register base to find the beginning of the Operational Register Space."]
pub type CAPLENGTH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - The value in these bits indicates an offset to add to register base to find the beginning of the Operational Register Space."]
    #[inline(always)]
    pub fn caplength(&self) -> CAPLENGTH_R {
        CAPLENGTH_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "EHCI Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caplength](index.html) module"]
pub struct CAPLENGTH_SPEC;
impl crate::RegisterSpec for CAPLENGTH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [caplength::R](R) reader structure"]
impl crate::Readable for CAPLENGTH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets caplength to value 0"]
impl crate::Resettable for CAPLENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
