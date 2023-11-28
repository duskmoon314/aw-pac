#[doc = "Register `csic_dma_buf_len` reader"]
pub type R = crate::R<CSIC_DMA_BUF_LEN_SPEC>;
#[doc = "Register `csic_dma_buf_len` writer"]
pub type W = crate::W<CSIC_DMA_BUF_LEN_SPEC>;
#[doc = "Field `buf_len` reader - DMA_MODE:Buffer length of luminance Y in a line. Unit is byte.\n\nLBC_MODE: Buffer length Stride of luminance Y and chroma C in YC line. Unit is byte.\n\nOnly Readable when BUF_LENGTH_CFG_MODE is set 0."]
pub type BUF_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `buf_len` writer - DMA_MODE:Buffer length of luminance Y in a line. Unit is byte.\n\nLBC_MODE: Buffer length Stride of luminance Y and chroma C in YC line. Unit is byte.\n\nOnly Readable when BUF_LENGTH_CFG_MODE is set 0."]
pub type BUF_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `buf_len_c` reader - DMA_MODE: Buffer length of chroma C in a line. Unit is byte.\n\nLBC_MODE: Buffer length Stride of luminance Y in ONLY Y line. Unit is byte.\n\nOnly Readable when BUF_LENGTH_CFG_MODE is set 0."]
pub type BUF_LEN_C_R = crate::FieldReader<u16>;
#[doc = "Field `buf_len_c` writer - DMA_MODE: Buffer length of chroma C in a line. Unit is byte.\n\nLBC_MODE: Buffer length Stride of luminance Y in ONLY Y line. Unit is byte.\n\nOnly Readable when BUF_LENGTH_CFG_MODE is set 0."]
pub type BUF_LEN_C_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - DMA_MODE:Buffer length of luminance Y in a line. Unit is byte.\n\nLBC_MODE: Buffer length Stride of luminance Y and chroma C in YC line. Unit is byte.\n\nOnly Readable when BUF_LENGTH_CFG_MODE is set 0."]
    #[inline(always)]
    pub fn buf_len(&self) -> BUF_LEN_R {
        BUF_LEN_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - DMA_MODE: Buffer length of chroma C in a line. Unit is byte.\n\nLBC_MODE: Buffer length Stride of luminance Y in ONLY Y line. Unit is byte.\n\nOnly Readable when BUF_LENGTH_CFG_MODE is set 0."]
    #[inline(always)]
    pub fn buf_len_c(&self) -> BUF_LEN_C_R {
        BUF_LEN_C_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - DMA_MODE:Buffer length of luminance Y in a line. Unit is byte.\n\nLBC_MODE: Buffer length Stride of luminance Y and chroma C in YC line. Unit is byte.\n\nOnly Readable when BUF_LENGTH_CFG_MODE is set 0."]
    #[inline(always)]
    #[must_use]
    pub fn buf_len(&mut self) -> BUF_LEN_W<CSIC_DMA_BUF_LEN_SPEC> {
        BUF_LEN_W::new(self, 0)
    }
    #[doc = "Bits 16:29 - DMA_MODE: Buffer length of chroma C in a line. Unit is byte.\n\nLBC_MODE: Buffer length Stride of luminance Y in ONLY Y line. Unit is byte.\n\nOnly Readable when BUF_LENGTH_CFG_MODE is set 0."]
    #[inline(always)]
    #[must_use]
    pub fn buf_len_c(&mut self) -> BUF_LEN_C_W<CSIC_DMA_BUF_LEN_SPEC> {
        BUF_LEN_C_W::new(self, 16)
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
#[doc = "CSIC DMA Buffer Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_buf_len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_buf_len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_BUF_LEN_SPEC;
impl crate::RegisterSpec for CSIC_DMA_BUF_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_buf_len::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_BUF_LEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_buf_len::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_BUF_LEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_buf_len to value 0x0280_0500"]
impl crate::Resettable for CSIC_DMA_BUF_LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0280_0500;
}
