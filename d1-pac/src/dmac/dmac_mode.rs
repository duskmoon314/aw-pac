#[doc = "Register `dmac_mode%s` reader"]
pub type R = crate::R<DMAC_MODE_SPEC>;
#[doc = "Register `dmac_mode%s` writer"]
pub type W = crate::W<DMAC_MODE_SPEC>;
#[doc = "Field `dma_src_mode` reader - Source Communication Mode Select"]
pub type DMA_SRC_MODE_R = crate::BitReader<DMA_SRC_MODE_A>;
#[doc = "Source Communication Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_SRC_MODE_A {
    #[doc = "0: `0`"]
    WAITING = 0,
    #[doc = "1: `1`"]
    HANDSHAKE = 1,
}
impl From<DMA_SRC_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_SRC_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_SRC_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_SRC_MODE_A {
        match self.bits {
            false => DMA_SRC_MODE_A::WAITING,
            true => DMA_SRC_MODE_A::HANDSHAKE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_waiting(&self) -> bool {
        *self == DMA_SRC_MODE_A::WAITING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_handshake(&self) -> bool {
        *self == DMA_SRC_MODE_A::HANDSHAKE
    }
}
#[doc = "Field `dma_src_mode` writer - Source Communication Mode Select"]
pub type DMA_SRC_MODE_W<'a, REG> = crate::BitWriter<'a, REG, DMA_SRC_MODE_A>;
impl<'a, REG> DMA_SRC_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn waiting(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_SRC_MODE_A::WAITING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn handshake(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_SRC_MODE_A::HANDSHAKE)
    }
}
#[doc = "Field `dma_dst_mode` reader - Destination Communication Mode Select"]
pub type DMA_DST_MODE_R = crate::BitReader<DMA_DST_MODE_A>;
#[doc = "Destination Communication Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_DST_MODE_A {
    #[doc = "0: `0`"]
    WAITING = 0,
    #[doc = "1: `1`"]
    HANDSHAKE = 1,
}
impl From<DMA_DST_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_DST_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_DST_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_DST_MODE_A {
        match self.bits {
            false => DMA_DST_MODE_A::WAITING,
            true => DMA_DST_MODE_A::HANDSHAKE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_waiting(&self) -> bool {
        *self == DMA_DST_MODE_A::WAITING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_handshake(&self) -> bool {
        *self == DMA_DST_MODE_A::HANDSHAKE
    }
}
#[doc = "Field `dma_dst_mode` writer - Destination Communication Mode Select"]
pub type DMA_DST_MODE_W<'a, REG> = crate::BitWriter<'a, REG, DMA_DST_MODE_A>;
impl<'a, REG> DMA_DST_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn waiting(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_DST_MODE_A::WAITING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn handshake(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_DST_MODE_A::HANDSHAKE)
    }
}
impl R {
    #[doc = "Bit 2 - Source Communication Mode Select"]
    #[inline(always)]
    pub fn dma_src_mode(&self) -> DMA_SRC_MODE_R {
        DMA_SRC_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination Communication Mode Select"]
    #[inline(always)]
    pub fn dma_dst_mode(&self) -> DMA_DST_MODE_R {
        DMA_DST_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Source Communication Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn dma_src_mode(&mut self) -> DMA_SRC_MODE_W<DMAC_MODE_SPEC> {
        DMA_SRC_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Destination Communication Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn dma_dst_mode(&mut self) -> DMA_DST_MODE_W<DMAC_MODE_SPEC> {
        DMA_DST_MODE_W::new(self, 3)
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
#[doc = "DMAC Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAC_MODE_SPEC;
impl crate::RegisterSpec for DMAC_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_mode::R`](R) reader structure"]
impl crate::Readable for DMAC_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmac_mode::W`](W) writer structure"]
impl crate::Writable for DMAC_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dmac_mode%s to value 0"]
impl crate::Resettable for DMAC_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
