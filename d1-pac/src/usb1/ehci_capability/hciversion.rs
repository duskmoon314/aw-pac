#[doc = "Register `hciversion` reader"]
pub type R = crate::R<HCIVERSION_SPEC>;
#[doc = "Field `hciversion` reader - This is a 16 bit register containing a BCD encoding of the EHCI revision number suported by this host controller"]
pub type HCIVERSION_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This is a 16 bit register containing a BCD encoding of the EHCI revision number suported by this host controller"]
    #[inline(always)]
    pub fn hciversion(&self) -> HCIVERSION_R {
        HCIVERSION_R::new(self.bits)
    }
}
#[doc = "EHCI Host Interface Version Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hciversion::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCIVERSION_SPEC;
impl crate::RegisterSpec for HCIVERSION_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hciversion::R`](R) reader structure"]
impl crate::Readable for HCIVERSION_SPEC {}
#[doc = "`reset()` method sets hciversion to value 0"]
impl crate::Resettable for HCIVERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
