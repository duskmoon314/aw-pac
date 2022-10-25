#[doc = "Register `hciversion` reader"]
pub struct R(crate::R<HCIVERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCIVERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCIVERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCIVERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `hciversion` reader - This is a 16 bit register containing a BCD encoding of the EHCI revision number suported by this host controller"]
pub type HCIVERSION_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This is a 16 bit register containing a BCD encoding of the EHCI revision number suported by this host controller"]
    #[inline(always)]
    pub fn hciversion(&self) -> HCIVERSION_R {
        HCIVERSION_R::new(self.bits)
    }
}
#[doc = "EHCI Host Interface Version Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hciversion](index.html) module"]
pub struct HCIVERSION_SPEC;
impl crate::RegisterSpec for HCIVERSION_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [hciversion::R](R) reader structure"]
impl crate::Readable for HCIVERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets hciversion to value 0"]
impl crate::Resettable for HCIVERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
