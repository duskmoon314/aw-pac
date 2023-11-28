#[doc = "Register `tfl` reader"]
pub type R = crate::R<TFL_SPEC>;
#[doc = "Field `tfl` reader - TX FIFO Level"]
pub type TFL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - TX FIFO Level"]
    #[inline(always)]
    pub fn tfl(&self) -> TFL_R {
        TFL_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "UART Transmit FIFO Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFL_SPEC;
impl crate::RegisterSpec for TFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfl::R`](R) reader structure"]
impl crate::Readable for TFL_SPEC {}
#[doc = "`reset()` method sets tfl to value 0"]
impl crate::Resettable for TFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
