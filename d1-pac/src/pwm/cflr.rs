#[doc = "Register `cflr%s` reader"]
pub type R = crate::R<CFLR_SPEC>;
#[doc = "Field `cflr` reader - When the capture channel captures a falling edge, the current value of the 16-bit up-counter is latched to the register."]
pub type CFLR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - When the capture channel captures a falling edge, the current value of the 16-bit up-counter is latched to the register."]
    #[inline(always)]
    pub fn cflr(&self) -> CFLR_R {
        CFLR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Capture Fall Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cflr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFLR_SPEC;
impl crate::RegisterSpec for CFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cflr::R`](R) reader structure"]
impl crate::Readable for CFLR_SPEC {}
#[doc = "`reset()` method sets cflr%s to value 0"]
impl crate::Resettable for CFLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
