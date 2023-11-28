#[doc = "Register `rbr` reader"]
pub type R = crate::R<RBR_SPEC>;
#[doc = "Field `rbr` reader - "]
pub type RBR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rbr(&self) -> RBR_R {
        RBR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UART Receive Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RBR_SPEC;
impl crate::RegisterSpec for RBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbr::R`](R) reader structure"]
impl crate::Readable for RBR_SPEC {}
#[doc = "`reset()` method sets rbr to value 0"]
impl crate::Resettable for RBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
