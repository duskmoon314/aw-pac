#[doc = "Register `cflr%s` reader"]
pub struct R(crate::R<CFLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `cflr` reader - When the capture channel captures a falling edge, the current value of the 16-bit up-counter is latched to the register."]
pub type CFLR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - When the capture channel captures a falling edge, the current value of the 16-bit up-counter is latched to the register."]
    #[inline(always)]
    pub fn cflr(&self) -> CFLR_R {
        CFLR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Capture Fall Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cflr](index.html) module"]
pub struct CFLR_SPEC;
impl crate::RegisterSpec for CFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cflr::R](R) reader structure"]
impl crate::Readable for CFLR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets cflr%s to value 0"]
impl crate::Resettable for CFLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
