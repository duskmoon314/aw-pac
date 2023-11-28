#[doc = "Register `rxdma_dcnt` reader"]
pub type R = crate::R<RXDMA_DCNT_SPEC>;
#[doc = "Register `rxdma_dcnt` writer"]
pub type W = crate::W<RXDMA_DCNT_SPEC>;
#[doc = "Field `data_count` reader - "]
pub type DATA_COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `data_count` writer - "]
pub type DATA_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn data_count(&self) -> DATA_COUNT_R {
        DATA_COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn data_count(&mut self) -> DATA_COUNT_W<RXDMA_DCNT_SPEC> {
        DATA_COUNT_W::new(self, 0)
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
#[doc = "UART RXDMA Data Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_dcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_dcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDMA_DCNT_SPEC;
impl crate::RegisterSpec for RXDMA_DCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdma_dcnt::R`](R) reader structure"]
impl crate::Readable for RXDMA_DCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxdma_dcnt::W`](W) writer structure"]
impl crate::Writable for RXDMA_DCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxdma_dcnt to value 0"]
impl crate::Resettable for RXDMA_DCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
