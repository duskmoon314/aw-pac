#[doc = "Register `rfl` reader"]
pub type R = crate::R<RFL_SPEC>;
#[doc = "Field `rfl` reader - RX FIFO Level"]
pub type RFL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - RX FIFO Level"]
    #[inline(always)]
    pub fn rfl(&self) -> RFL_R {
        RFL_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "UART Receive FIFO Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFL_SPEC;
impl crate::RegisterSpec for RFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfl::R`](R) reader structure"]
impl crate::Readable for RFL_SPEC {}
#[doc = "`reset()` method sets rfl to value 0"]
impl crate::Resettable for RFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
