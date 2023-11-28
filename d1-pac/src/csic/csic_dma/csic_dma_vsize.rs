#[doc = "Register `csic_dma_vsize` reader"]
pub type R = crate::R<CSIC_DMA_VSIZE_SPEC>;
#[doc = "Register `csic_dma_vsize` writer"]
pub type W = crate::W<CSIC_DMA_VSIZE_SPEC>;
#[doc = "Field `ver_start` reader - Vertical line start\n\nData is valid from this line."]
pub type VER_START_R = crate::FieldReader<u16>;
#[doc = "Field `ver_start` writer - Vertical line start\n\nData is valid from this line."]
pub type VER_START_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `ver_len` reader - When BK_TOP_EN is enabled, DMA_EN is enabled, these bits indicate Valid line number of a frame in DMA mode."]
pub type VER_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `ver_len` writer - When BK_TOP_EN is enabled, DMA_EN is enabled, these bits indicate Valid line number of a frame in DMA mode."]
pub type VER_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Vertical line start\n\nData is valid from this line."]
    #[inline(always)]
    pub fn ver_start(&self) -> VER_START_R {
        VER_START_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - When BK_TOP_EN is enabled, DMA_EN is enabled, these bits indicate Valid line number of a frame in DMA mode."]
    #[inline(always)]
    pub fn ver_len(&self) -> VER_LEN_R {
        VER_LEN_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Vertical line start\n\nData is valid from this line."]
    #[inline(always)]
    #[must_use]
    pub fn ver_start(&mut self) -> VER_START_W<CSIC_DMA_VSIZE_SPEC> {
        VER_START_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - When BK_TOP_EN is enabled, DMA_EN is enabled, these bits indicate Valid line number of a frame in DMA mode."]
    #[inline(always)]
    #[must_use]
    pub fn ver_len(&mut self) -> VER_LEN_W<CSIC_DMA_VSIZE_SPEC> {
        VER_LEN_W::new(self, 16)
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
#[doc = "CSIC DMA Vertical Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_vsize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_vsize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_VSIZE_SPEC;
impl crate::RegisterSpec for CSIC_DMA_VSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_vsize::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_VSIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_vsize::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_VSIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_vsize to value 0x02d0_0000"]
impl crate::Resettable for CSIC_DMA_VSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x02d0_0000;
}
