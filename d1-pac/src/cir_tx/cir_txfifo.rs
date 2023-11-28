#[doc = "Register `cir_txfifo` reader"]
pub type R = crate::R<CIR_TXFIFO_SPEC>;
#[doc = "Register `cir_txfifo` writer"]
pub type W = crate::W<CIR_TXFIFO_SPEC>;
#[doc = "Field `tbf` writer - Transmit Byte FIFO\n\nWhen the transmission is triggered, the data in the FIFO will be transmitted until the data number is transmitted completely."]
pub type TBF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Transmit Byte FIFO\n\nWhen the transmission is triggered, the data in the FIFO will be transmitted until the data number is transmitted completely."]
    #[inline(always)]
    #[must_use]
    pub fn tbf(&mut self) -> TBF_W<CIR_TXFIFO_SPEC> {
        TBF_W::new(self, 0)
    }
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
#[doc = "CIR Transmit FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_txfifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_txfifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIR_TXFIFO_SPEC;
impl crate::RegisterSpec for CIR_TXFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir_txfifo::R`](R) reader structure"]
impl crate::Readable for CIR_TXFIFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cir_txfifo::W`](W) writer structure"]
impl crate::Writable for CIR_TXFIFO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_txfifo to value 0"]
impl crate::Resettable for CIR_TXFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
