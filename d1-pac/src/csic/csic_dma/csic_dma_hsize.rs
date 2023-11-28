#[doc = "Register `csic_dma_hsize` reader"]
pub type R = crate::R<CSIC_DMA_HSIZE_SPEC>;
#[doc = "Register `csic_dma_hsize` writer"]
pub type W = crate::W<CSIC_DMA_HSIZE_SPEC>;
#[doc = "Field `hor_start` reader - Horizontal pixel unit start.\n\nPixel is valid from this pixel."]
pub type HOR_START_R = crate::FieldReader<u16>;
#[doc = "Field `hor_start` writer - Horizontal pixel unit start.\n\nPixel is valid from this pixel."]
pub type HOR_START_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `hor_len` reader - When BK_TOP_EN is enabled, DMA_EN is enabled, these bits indicate Horizontal pixel unit length. Valid pixel of a line in DMA mode."]
pub type HOR_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `hor_len` writer - When BK_TOP_EN is enabled, DMA_EN is enabled, these bits indicate Horizontal pixel unit length. Valid pixel of a line in DMA mode."]
pub type HOR_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Horizontal pixel unit start.\n\nPixel is valid from this pixel."]
    #[inline(always)]
    pub fn hor_start(&self) -> HOR_START_R {
        HOR_START_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - When BK_TOP_EN is enabled, DMA_EN is enabled, these bits indicate Horizontal pixel unit length. Valid pixel of a line in DMA mode."]
    #[inline(always)]
    pub fn hor_len(&self) -> HOR_LEN_R {
        HOR_LEN_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Horizontal pixel unit start.\n\nPixel is valid from this pixel."]
    #[inline(always)]
    #[must_use]
    pub fn hor_start(&mut self) -> HOR_START_W<CSIC_DMA_HSIZE_SPEC> {
        HOR_START_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - When BK_TOP_EN is enabled, DMA_EN is enabled, these bits indicate Horizontal pixel unit length. Valid pixel of a line in DMA mode."]
    #[inline(always)]
    #[must_use]
    pub fn hor_len(&mut self) -> HOR_LEN_W<CSIC_DMA_HSIZE_SPEC> {
        HOR_LEN_W::new(self, 16)
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
#[doc = "CSIC DMA Horizontal Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_hsize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_hsize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_HSIZE_SPEC;
impl crate::RegisterSpec for CSIC_DMA_HSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_hsize::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_HSIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_hsize::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_HSIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_hsize to value 0x0500_0000"]
impl crate::Resettable for CSIC_DMA_HSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0500_0000;
}
