#[doc = "Register `csic_dma_flip_size` reader"]
pub struct R(crate::R<CSIC_DMA_FLIP_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_FLIP_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_FLIP_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_FLIP_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_flip_size` writer"]
pub struct W(crate::W<CSIC_DMA_FLIP_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_FLIP_SIZE_SPEC>;
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
impl From<crate::W<CSIC_DMA_FLIP_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_FLIP_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `valid_len` reader - Valid components of a line when in HFLIP mode. Unit is pixel component.\n\nOnly Readable when FLIP_SIZE_CFG_MODE is set to 0."]
pub type VALID_LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `valid_len` writer - Valid components of a line when in HFLIP mode. Unit is pixel component.\n\nOnly Readable when FLIP_SIZE_CFG_MODE is set to 0."]
pub type VALID_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_FLIP_SIZE_SPEC, u16, u16, 14, O>;
#[doc = "Field `ver_len` reader - Vertical line number when in VFLIP mode. Unit is line.\n\nOnly Readable when FLIP_SIZE_CFG_MODE is set to 0."]
pub type VER_LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ver_len` writer - Vertical line number when in VFLIP mode. Unit is line.\n\nOnly Readable when FLIP_SIZE_CFG_MODE is set to 0."]
pub type VER_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_FLIP_SIZE_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:13 - Valid components of a line when in HFLIP mode. Unit is pixel component.\n\nOnly Readable when FLIP_SIZE_CFG_MODE is set to 0."]
    #[inline(always)]
    pub fn valid_len(&self) -> VALID_LEN_R {
        VALID_LEN_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:28 - Vertical line number when in VFLIP mode. Unit is line.\n\nOnly Readable when FLIP_SIZE_CFG_MODE is set to 0."]
    #[inline(always)]
    pub fn ver_len(&self) -> VER_LEN_R {
        VER_LEN_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Valid components of a line when in HFLIP mode. Unit is pixel component.\n\nOnly Readable when FLIP_SIZE_CFG_MODE is set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn valid_len(&mut self) -> VALID_LEN_W<0> {
        VALID_LEN_W::new(self)
    }
    #[doc = "Bits 16:28 - Vertical line number when in VFLIP mode. Unit is line.\n\nOnly Readable when FLIP_SIZE_CFG_MODE is set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn ver_len(&mut self) -> VER_LEN_W<16> {
        VER_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC DMA Flip Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_flip_size](index.html) module"]
pub struct CSIC_DMA_FLIP_SIZE_SPEC;
impl crate::RegisterSpec for CSIC_DMA_FLIP_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_flip_size::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_FLIP_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_flip_size::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_FLIP_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_flip_size to value 0x02d0_0500"]
impl crate::Resettable for CSIC_DMA_FLIP_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x02d0_0500;
}
