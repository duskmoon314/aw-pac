#[doc = "Register `cir_dma_ctl` reader"]
pub type R = crate::R<CIR_DMA_CTL_SPEC>;
#[doc = "Register `cir_dma_ctl` writer"]
pub type W = crate::W<CIR_DMA_CTL_SPEC>;
#[doc = "Field `dma` reader - Handshake Configuration"]
pub type DMA_R = crate::FieldReader<DMA_A>;
#[doc = "Handshake Configuration\n\nValue on reset: 165"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA_A {
    #[doc = "165: DMA waiting cycle mode"]
    WAITING_CYCLE = 165,
    #[doc = "234: DMA handshake mode"]
    HANDSHAKE = 234,
}
impl From<DMA_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA_A {
    type Ux = u8;
}
impl DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DMA_A> {
        match self.bits {
            165 => Some(DMA_A::WAITING_CYCLE),
            234 => Some(DMA_A::HANDSHAKE),
            _ => None,
        }
    }
    #[doc = "DMA waiting cycle mode"]
    #[inline(always)]
    pub fn is_waiting_cycle(&self) -> bool {
        *self == DMA_A::WAITING_CYCLE
    }
    #[doc = "DMA handshake mode"]
    #[inline(always)]
    pub fn is_handshake(&self) -> bool {
        *self == DMA_A::HANDSHAKE
    }
}
#[doc = "Field `dma` writer - Handshake Configuration"]
pub type DMA_W<'a, REG> = crate::FieldWriter<'a, REG, 8, DMA_A>;
impl<'a, REG> DMA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA waiting cycle mode"]
    #[inline(always)]
    pub fn waiting_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_A::WAITING_CYCLE)
    }
    #[doc = "DMA handshake mode"]
    #[inline(always)]
    pub fn handshake(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_A::HANDSHAKE)
    }
}
impl R {
    #[doc = "Bits 0:7 - Handshake Configuration"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Handshake Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<CIR_DMA_CTL_SPEC> {
        DMA_W::new(self, 0)
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
#[doc = "CIR DMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_dma_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_dma_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIR_DMA_CTL_SPEC;
impl crate::RegisterSpec for CIR_DMA_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir_dma_ctl::R`](R) reader structure"]
impl crate::Readable for CIR_DMA_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cir_dma_ctl::W`](W) writer structure"]
impl crate::Writable for CIR_DMA_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_dma_ctl to value 0xa5"]
impl crate::Resettable for CIR_DMA_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xa5;
}
