#[doc = "Register `rxdma_lmt` reader"]
pub type R = crate::R<RXDMA_LMT_SPEC>;
#[doc = "Register `rxdma_lmt` writer"]
pub type W = crate::W<RXDMA_LMT_SPEC>;
#[doc = "Field `limit_size` reader - "]
pub type LIMIT_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `limit_size` writer - "]
pub type LIMIT_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn limit_size(&self) -> LIMIT_SIZE_R {
        LIMIT_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn limit_size(&mut self) -> LIMIT_SIZE_W<RXDMA_LMT_SPEC> {
        LIMIT_SIZE_W::new(self, 0)
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
#[doc = "UART RXDMA Limit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_lmt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_lmt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDMA_LMT_SPEC;
impl crate::RegisterSpec for RXDMA_LMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdma_lmt::R`](R) reader structure"]
impl crate::Readable for RXDMA_LMT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxdma_lmt::W`](W) writer structure"]
impl crate::Writable for RXDMA_LMT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxdma_lmt to value 0"]
impl crate::Resettable for RXDMA_LMT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
