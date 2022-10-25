#[doc = "Register `crlr%s` reader"]
pub struct R(crate::R<CRLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `crlr` reader - When the capture channel captures a rising edge, the current value of the 16-bit up-counter is latched to the register."]
pub type CRLR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - When the capture channel captures a rising edge, the current value of the 16-bit up-counter is latched to the register."]
    #[inline(always)]
    pub fn crlr(&self) -> CRLR_R {
        CRLR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Capture Rise Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crlr](index.html) module"]
pub struct CRLR_SPEC;
impl crate::RegisterSpec for CRLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crlr::R](R) reader structure"]
impl crate::Readable for CRLR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets crlr%s to value 0"]
impl crate::Resettable for CRLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
