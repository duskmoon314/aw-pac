#[doc = "Register `rxdma_saddrh` reader"]
pub type R = crate::R<RXDMA_SADDRH_SPEC>;
#[doc = "Register `rxdma_saddrh` writer"]
pub type W = crate::W<RXDMA_SADDRH_SPEC>;
#[doc = "Field `saddr` reader - RXDMA Buffer Start Address \\[33:32\\]"]
pub type SADDR_R = crate::FieldReader;
#[doc = "Field `saddr` writer - RXDMA Buffer Start Address \\[33:32\\]"]
pub type SADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - RXDMA Buffer Start Address \\[33:32\\]"]
    #[inline(always)]
    pub fn saddr(&self) -> SADDR_R {
        SADDR_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RXDMA Buffer Start Address \\[33:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn saddr(&mut self) -> SADDR_W<RXDMA_SADDRH_SPEC> {
        SADDR_W::new(self, 0)
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
#[doc = "UART RXDMA Buffer Start Address High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_saddrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_saddrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDMA_SADDRH_SPEC;
impl crate::RegisterSpec for RXDMA_SADDRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdma_saddrh::R`](R) reader structure"]
impl crate::Readable for RXDMA_SADDRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxdma_saddrh::W`](W) writer structure"]
impl crate::Writable for RXDMA_SADDRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxdma_saddrh to value 0"]
impl crate::Resettable for RXDMA_SADDRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
