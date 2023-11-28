#[doc = "Register `rxdma_bl` reader"]
pub type R = crate::R<RXDMA_BL_SPEC>;
#[doc = "Register `rxdma_bl` writer"]
pub type W = crate::W<RXDMA_BL_SPEC>;
#[doc = "Field `buffer_length` reader - "]
pub type BUFFER_LENGTH_R = crate::FieldReader<u16>;
#[doc = "Field `buffer_length` writer - "]
pub type BUFFER_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn buffer_length(&self) -> BUFFER_LENGTH_R {
        BUFFER_LENGTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn buffer_length(&mut self) -> BUFFER_LENGTH_W<RXDMA_BL_SPEC> {
        BUFFER_LENGTH_W::new(self, 0)
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
#[doc = "UART RXDMA Buffer Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_bl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_bl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDMA_BL_SPEC;
impl crate::RegisterSpec for RXDMA_BL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdma_bl::R`](R) reader structure"]
impl crate::Readable for RXDMA_BL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxdma_bl::W`](W) writer structure"]
impl crate::Writable for RXDMA_BL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxdma_bl to value 0"]
impl crate::Resettable for RXDMA_BL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
