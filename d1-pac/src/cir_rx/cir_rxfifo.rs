#[doc = "Register `cir_rxfifo` reader"]
pub type R = crate::R<CIR_RXFIFO_SPEC>;
#[doc = "Register `cir_rxfifo` writer"]
pub type W = crate::W<CIR_RXFIFO_SPEC>;
#[doc = "Field `rbf` reader - Receiver Byte FIFO"]
pub type RBF_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receiver Byte FIFO"]
    #[inline(always)]
    pub fn rbf(&self) -> RBF_R {
        RBF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CIR Receiver FIFO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_rxfifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_rxfifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIR_RXFIFO_SPEC;
impl crate::RegisterSpec for CIR_RXFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir_rxfifo::R`](R) reader structure"]
impl crate::Readable for CIR_RXFIFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cir_rxfifo::W`](W) writer structure"]
impl crate::Writable for CIR_RXFIFO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_rxfifo to value 0"]
impl crate::Resettable for CIR_RXFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
