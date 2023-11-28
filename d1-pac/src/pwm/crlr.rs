#[doc = "Register `crlr%s` reader"]
pub type R = crate::R<CRLR_SPEC>;
#[doc = "Field `crlr` reader - When the capture channel captures a rising edge, the current value of the 16-bit up-counter is latched to the register."]
pub type CRLR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - When the capture channel captures a rising edge, the current value of the 16-bit up-counter is latched to the register."]
    #[inline(always)]
    pub fn crlr(&self) -> CRLR_R {
        CRLR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Capture Rise Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crlr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRLR_SPEC;
impl crate::RegisterSpec for CRLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crlr::R`](R) reader structure"]
impl crate::Readable for CRLR_SPEC {}
#[doc = "`reset()` method sets crlr%s to value 0"]
impl crate::Resettable for CRLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
