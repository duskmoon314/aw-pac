#[doc = "Register `dmac_irq_pend1` reader"]
pub type R = crate::R<DMAC_IRQ_PEND1_SPEC>;
#[doc = "Register `dmac_irq_pend1` writer"]
pub type W = crate::W<DMAC_IRQ_PEND1_SPEC>;
#[doc = "Field `dma_hlaf_irq_pend[8-15]` reader - The IRQ pending bit for the half package interrupt of DMA"]
pub type DMA_HLAF_IRQ_PEND_R = crate::BitReader<DMA_HLAF_IRQ_PEND_A>;
#[doc = "The IRQ pending bit for the half package interrupt of DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_HLAF_IRQ_PEND_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<DMA_HLAF_IRQ_PEND_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_HLAF_IRQ_PEND_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_HLAF_IRQ_PEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_HLAF_IRQ_PEND_A {
        match self.bits {
            false => DMA_HLAF_IRQ_PEND_A::NO_EFFECT,
            true => DMA_HLAF_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMA_HLAF_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DMA_HLAF_IRQ_PEND_A::PENDING
    }
}
#[doc = "Field `dma_hlaf_irq_pend[8-15]` writer - The IRQ pending bit for the half package interrupt of DMA"]
pub type DMA_HLAF_IRQ_PEND_W<'a, REG> = crate::BitWriter<'a, REG, DMA_HLAF_IRQ_PEND_A>;
impl<'a, REG> DMA_HLAF_IRQ_PEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_HLAF_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_HLAF_IRQ_PEND_A::PENDING)
    }
}
#[doc = "Field `dma_pkg_irq_pend[8-15]` reader - The IRQ pending bit for the package end interrupt of DMA"]
pub type DMA_PKG_IRQ_PEND_R = crate::BitReader<DMA_PKG_IRQ_PEND_A>;
#[doc = "The IRQ pending bit for the package end interrupt of DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_PKG_IRQ_PEND_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<DMA_PKG_IRQ_PEND_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_PKG_IRQ_PEND_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_PKG_IRQ_PEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_PKG_IRQ_PEND_A {
        match self.bits {
            false => DMA_PKG_IRQ_PEND_A::NO_EFFECT,
            true => DMA_PKG_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMA_PKG_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DMA_PKG_IRQ_PEND_A::PENDING
    }
}
#[doc = "Field `dma_pkg_irq_pend[8-15]` writer - The IRQ pending bit for the package end interrupt of DMA"]
pub type DMA_PKG_IRQ_PEND_W<'a, REG> = crate::BitWriter<'a, REG, DMA_PKG_IRQ_PEND_A>;
impl<'a, REG> DMA_PKG_IRQ_PEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_PKG_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_PKG_IRQ_PEND_A::PENDING)
    }
}
#[doc = "Field `dma_queue_irq_pend[8-15]` reader - The IRQ pending bit for the queue end interrupt of DMA"]
pub type DMA_QUEUE_IRQ_PEND_R = crate::BitReader<DMA_QUEUE_IRQ_PEND_A>;
#[doc = "The IRQ pending bit for the queue end interrupt of DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_QUEUE_IRQ_PEND_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<DMA_QUEUE_IRQ_PEND_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_QUEUE_IRQ_PEND_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_QUEUE_IRQ_PEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_QUEUE_IRQ_PEND_A {
        match self.bits {
            false => DMA_QUEUE_IRQ_PEND_A::NO_EFFECT,
            true => DMA_QUEUE_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMA_QUEUE_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DMA_QUEUE_IRQ_PEND_A::PENDING
    }
}
#[doc = "Field `dma_queue_irq_pend[8-15]` writer - The IRQ pending bit for the queue end interrupt of DMA"]
pub type DMA_QUEUE_IRQ_PEND_W<'a, REG> = crate::BitWriter<'a, REG, DMA_QUEUE_IRQ_PEND_A>;
impl<'a, REG> DMA_QUEUE_IRQ_PEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_QUEUE_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_QUEUE_IRQ_PEND_A::PENDING)
    }
}
impl R {
    #[doc = "The IRQ pending bit for the half package interrupt of DMA\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `dma8_hlaf_irq_pend` field"]
    #[inline(always)]
    pub fn dma_hlaf_irq_pend(&self, n: u8) -> DMA_HLAF_IRQ_PEND_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    #[doc = "Bit 0 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma8_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma9_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma10_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma11_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma12_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma13_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma14_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma15_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "The IRQ pending bit for the package end interrupt of DMA\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `dma8_pkg_irq_pend` field"]
    #[inline(always)]
    pub fn dma_pkg_irq_pend(&self, n: u8) -> DMA_PKG_IRQ_PEND_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma8_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma9_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma10_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma11_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma12_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma13_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma14_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma15_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "The IRQ pending bit for the queue end interrupt of DMA\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `dma8_queue_irq_pend` field"]
    #[inline(always)]
    pub fn dma_queue_irq_pend(&self, n: u8) -> DMA_QUEUE_IRQ_PEND_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma8_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma9_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma10_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma11_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma12_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma13_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma14_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma15_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "The IRQ pending bit for the half package interrupt of DMA\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `dma8_hlaf_irq_pend` field"]
    #[inline(always)]
    #[must_use]
    pub fn dma_hlaf_irq_pend(&mut self, n: u8) -> DMA_HLAF_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DMA_HLAF_IRQ_PEND_W::new(self, n * 4)
    }
    #[doc = "Bit 0 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma8_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_HLAF_IRQ_PEND_W::new(self, 0)
    }
    #[doc = "Bit 4 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma9_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_HLAF_IRQ_PEND_W::new(self, 4)
    }
    #[doc = "Bit 8 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma10_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_HLAF_IRQ_PEND_W::new(self, 8)
    }
    #[doc = "Bit 12 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma11_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_HLAF_IRQ_PEND_W::new(self, 12)
    }
    #[doc = "Bit 16 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma12_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_HLAF_IRQ_PEND_W::new(self, 16)
    }
    #[doc = "Bit 20 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma13_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_HLAF_IRQ_PEND_W::new(self, 20)
    }
    #[doc = "Bit 24 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma14_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_HLAF_IRQ_PEND_W::new(self, 24)
    }
    #[doc = "Bit 28 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma15_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_HLAF_IRQ_PEND_W::new(self, 28)
    }
    #[doc = "The IRQ pending bit for the package end interrupt of DMA\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `dma8_pkg_irq_pend` field"]
    #[inline(always)]
    #[must_use]
    pub fn dma_pkg_irq_pend(&mut self, n: u8) -> DMA_PKG_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DMA_PKG_IRQ_PEND_W::new(self, n * 4 + 1)
    }
    #[doc = "Bit 1 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma8_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_PKG_IRQ_PEND_W::new(self, 1)
    }
    #[doc = "Bit 5 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma9_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_PKG_IRQ_PEND_W::new(self, 5)
    }
    #[doc = "Bit 9 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma10_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_PKG_IRQ_PEND_W::new(self, 9)
    }
    #[doc = "Bit 13 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma11_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_PKG_IRQ_PEND_W::new(self, 13)
    }
    #[doc = "Bit 17 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma12_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_PKG_IRQ_PEND_W::new(self, 17)
    }
    #[doc = "Bit 21 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma13_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_PKG_IRQ_PEND_W::new(self, 21)
    }
    #[doc = "Bit 25 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma14_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_PKG_IRQ_PEND_W::new(self, 25)
    }
    #[doc = "Bit 29 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma15_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_PKG_IRQ_PEND_W::new(self, 29)
    }
    #[doc = "The IRQ pending bit for the queue end interrupt of DMA\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `dma8_queue_irq_pend` field"]
    #[inline(always)]
    #[must_use]
    pub fn dma_queue_irq_pend(&mut self, n: u8) -> DMA_QUEUE_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DMA_QUEUE_IRQ_PEND_W::new(self, n * 4 + 2)
    }
    #[doc = "Bit 2 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma8_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_QUEUE_IRQ_PEND_W::new(self, 2)
    }
    #[doc = "Bit 6 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma9_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_QUEUE_IRQ_PEND_W::new(self, 6)
    }
    #[doc = "Bit 10 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma10_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_QUEUE_IRQ_PEND_W::new(self, 10)
    }
    #[doc = "Bit 14 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma11_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_QUEUE_IRQ_PEND_W::new(self, 14)
    }
    #[doc = "Bit 18 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma12_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_QUEUE_IRQ_PEND_W::new(self, 18)
    }
    #[doc = "Bit 22 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma13_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_QUEUE_IRQ_PEND_W::new(self, 22)
    }
    #[doc = "Bit 26 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma14_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_QUEUE_IRQ_PEND_W::new(self, 26)
    }
    #[doc = "Bit 30 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma15_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<DMAC_IRQ_PEND1_SPEC> {
        DMA_QUEUE_IRQ_PEND_W::new(self, 30)
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
#[doc = "DMAC IRQ Pending Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_irq_pend1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_irq_pend1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAC_IRQ_PEND1_SPEC;
impl crate::RegisterSpec for DMAC_IRQ_PEND1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_irq_pend1::R`](R) reader structure"]
impl crate::Readable for DMAC_IRQ_PEND1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmac_irq_pend1::W`](W) writer structure"]
impl crate::Writable for DMAC_IRQ_PEND1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dmac_irq_pend1 to value 0"]
impl crate::Resettable for DMAC_IRQ_PEND1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
