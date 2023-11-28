#[doc = "Register `rxdma_str` reader"]
pub type R = crate::R<RXDMA_STR_SPEC>;
#[doc = "Register `rxdma_str` writer"]
pub type W = crate::W<RXDMA_STR_SPEC>;
#[doc = "Field `start` reader - "]
pub type START_R = crate::BitReader;
#[doc = "Field `start` writer - "]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<RXDMA_STR_SPEC> {
        START_W::new(self, 0)
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
#[doc = "UART RXDMA Start Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_str::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_str::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDMA_STR_SPEC;
impl crate::RegisterSpec for RXDMA_STR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdma_str::R`](R) reader structure"]
impl crate::Readable for RXDMA_STR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxdma_str::W`](W) writer structure"]
impl crate::Writable for RXDMA_STR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxdma_str to value 0"]
impl crate::Resettable for RXDMA_STR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
