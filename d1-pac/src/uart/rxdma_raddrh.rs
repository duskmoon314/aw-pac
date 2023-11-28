#[doc = "Register `rxdma_raddrh` reader"]
pub type R = crate::R<RXDMA_RADDRH_SPEC>;
#[doc = "Register `rxdma_raddrh` writer"]
pub type W = crate::W<RXDMA_RADDRH_SPEC>;
#[doc = "Field `raddr` reader - RXDMA Current Read Address \\[33:32\\]"]
pub type RADDR_R = crate::FieldReader;
#[doc = "Field `raddr` writer - RXDMA Current Read Address \\[33:32\\]"]
pub type RADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - RXDMA Current Read Address \\[33:32\\]"]
    #[inline(always)]
    pub fn raddr(&self) -> RADDR_R {
        RADDR_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RXDMA Current Read Address \\[33:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn raddr(&mut self) -> RADDR_W<RXDMA_RADDRH_SPEC> {
        RADDR_W::new(self, 0)
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
#[doc = "UART RXDMA Read Address High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_raddrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_raddrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDMA_RADDRH_SPEC;
impl crate::RegisterSpec for RXDMA_RADDRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdma_raddrh::R`](R) reader structure"]
impl crate::Readable for RXDMA_RADDRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxdma_raddrh::W`](W) writer structure"]
impl crate::Writable for RXDMA_RADDRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxdma_raddrh to value 0"]
impl crate::Resettable for RXDMA_RADDRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
