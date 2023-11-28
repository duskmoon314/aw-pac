#[doc = "Register `asrcfifostat` reader"]
pub type R = crate::R<ASRCFIFOSTAT_SPEC>;
#[doc = "Register `asrcfifostat` writer"]
pub type W = crate::W<ASRCFIFOSTAT_SPEC>;
#[doc = "Field `asrc_rx_fifo_full_leval` reader - ASRC RXFIFO Full Level\n\nThe manually-configured FIFO fill level for the ratio value of the received data."]
pub type ASRC_RX_FIFO_FULL_LEVAL_R = crate::FieldReader<u16>;
#[doc = "Field `asrc_rx_fifo_full_leval` writer - ASRC RXFIFO Full Level\n\nThe manually-configured FIFO fill level for the ratio value of the received data."]
pub type ASRC_RX_FIFO_FULL_LEVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - ASRC RXFIFO Full Level\n\nThe manually-configured FIFO fill level for the ratio value of the received data."]
    #[inline(always)]
    pub fn asrc_rx_fifo_full_leval(&self) -> ASRC_RX_FIFO_FULL_LEVAL_R {
        ASRC_RX_FIFO_FULL_LEVAL_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - ASRC RXFIFO Full Level\n\nThe manually-configured FIFO fill level for the ratio value of the received data."]
    #[inline(always)]
    #[must_use]
    pub fn asrc_rx_fifo_full_leval(&mut self) -> ASRC_RX_FIFO_FULL_LEVAL_W<ASRCFIFOSTAT_SPEC> {
        ASRC_RX_FIFO_FULL_LEVAL_W::new(self, 0)
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
#[doc = "ASRC FIFO Level Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asrcfifostat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asrcfifostat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASRCFIFOSTAT_SPEC;
impl crate::RegisterSpec for ASRCFIFOSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asrcfifostat::R`](R) reader structure"]
impl crate::Readable for ASRCFIFOSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`asrcfifostat::W`](W) writer structure"]
impl crate::Writable for ASRCFIFOSTAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets asrcfifostat to value 0"]
impl crate::Resettable for ASRCFIFOSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
