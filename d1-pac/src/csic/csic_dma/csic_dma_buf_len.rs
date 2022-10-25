#[doc = "Register `csic_dma_buf_len` reader"]
pub struct R(crate::R<CSIC_DMA_BUF_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_BUF_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_BUF_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_BUF_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_buf_len` writer"]
pub struct W(crate::W<CSIC_DMA_BUF_LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_BUF_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CSIC_DMA_BUF_LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_BUF_LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `buf_len` reader - DMA_MODE:Buffer length of luminance Y in a line. Unit is byte.\n\nLBC_MODE: Buffer length Stride of luminance Y and chroma C in YC line. Unit is byte.\n\nOnly Readable when BUF_LENGTH_CFG_MODE is set 0."]
pub type BUF_LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `buf_len` writer - DMA_MODE:Buffer length of luminance Y in a line. Unit is byte.\n\nLBC_MODE: Buffer length Stride of luminance Y and chroma C in YC line. Unit is byte.\n\nOnly Readable when BUF_LENGTH_CFG_MODE is set 0."]
pub type BUF_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_BUF_LEN_SPEC, u16, u16, 14, O>;
#[doc = "Field `buf_len_c` reader - DMA_MODE: Buffer length of chroma C in a line. Unit is byte.\n\nLBC_MODE: Buffer length Stride of luminance Y in ONLY Y line. Unit is byte.\n\nOnly Readable when BUF_LENGTH_CFG_MODE is set 0."]
pub type BUF_LEN_C_R = crate::FieldReader<u16, u16>;
#[doc = "Field `buf_len_c` writer - DMA_MODE: Buffer length of chroma C in a line. Unit is byte.\n\nLBC_MODE: Buffer length Stride of luminance Y in ONLY Y line. Unit is byte.\n\nOnly Readable when BUF_LENGTH_CFG_MODE is set 0."]
pub type BUF_LEN_C_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_BUF_LEN_SPEC, u16, u16, 14, O>;
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
    pub fn buf_len(&mut self) -> BUF_LEN_W<0> {
        BUF_LEN_W::new(self)
    }
    #[doc = "Bits 16:29 - DMA_MODE: Buffer length of chroma C in a line. Unit is byte.\n\nLBC_MODE: Buffer length Stride of luminance Y in ONLY Y line. Unit is byte.\n\nOnly Readable when BUF_LENGTH_CFG_MODE is set 0."]
    #[inline(always)]
    #[must_use]
    pub fn buf_len_c(&mut self) -> BUF_LEN_C_W<16> {
        BUF_LEN_C_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC DMA Buffer Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_buf_len](index.html) module"]
pub struct CSIC_DMA_BUF_LEN_SPEC;
impl crate::RegisterSpec for CSIC_DMA_BUF_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_buf_len::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_BUF_LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_buf_len::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_BUF_LEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_buf_len to value 0x0280_0500"]
impl crate::Resettable for CSIC_DMA_BUF_LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0280_0500;
}
