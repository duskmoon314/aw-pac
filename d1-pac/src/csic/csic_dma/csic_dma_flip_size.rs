#[doc = "Register `csic_dma_flip_size` reader"]
pub type R = crate::R<CSIC_DMA_FLIP_SIZE_SPEC>;
#[doc = "Register `csic_dma_flip_size` writer"]
pub type W = crate::W<CSIC_DMA_FLIP_SIZE_SPEC>;
#[doc = "Field `valid_len` reader - Valid components of a line when in HFLIP mode. Unit is pixel component.\n\nOnly Readable when FLIP_SIZE_CFG_MODE is set to 0."]
pub type VALID_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `valid_len` writer - Valid components of a line when in HFLIP mode. Unit is pixel component.\n\nOnly Readable when FLIP_SIZE_CFG_MODE is set to 0."]
pub type VALID_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `ver_len` reader - Vertical line number when in VFLIP mode. Unit is line.\n\nOnly Readable when FLIP_SIZE_CFG_MODE is set to 0."]
pub type VER_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `ver_len` writer - Vertical line number when in VFLIP mode. Unit is line.\n\nOnly Readable when FLIP_SIZE_CFG_MODE is set to 0."]
pub type VER_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
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
    pub fn valid_len(&mut self) -> VALID_LEN_W<CSIC_DMA_FLIP_SIZE_SPEC> {
        VALID_LEN_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Vertical line number when in VFLIP mode. Unit is line.\n\nOnly Readable when FLIP_SIZE_CFG_MODE is set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn ver_len(&mut self) -> VER_LEN_W<CSIC_DMA_FLIP_SIZE_SPEC> {
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
#[doc = "CSIC DMA Flip Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_flip_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_flip_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_FLIP_SIZE_SPEC;
impl crate::RegisterSpec for CSIC_DMA_FLIP_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_flip_size::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_FLIP_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_flip_size::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_FLIP_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_flip_size to value 0x02d0_0500"]
impl crate::Resettable for CSIC_DMA_FLIP_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x02d0_0500;
}
