#[doc = "Register `dmac_irq_pend0` reader"]
pub struct R(crate::R<DMAC_IRQ_PEND0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_IRQ_PEND0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_IRQ_PEND0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_IRQ_PEND0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dmac_irq_pend0` writer"]
pub struct W(crate::W<DMAC_IRQ_PEND0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_IRQ_PEND0_SPEC>;
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
impl From<crate::W<DMAC_IRQ_PEND0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_IRQ_PEND0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma_hlaf_irq_pend[0-7]` reader - The IRQ pending bit for the half package interrupt of DMA"]
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
    pub fn variant(&self) -> DMA_HLAF_IRQ_PEND_A {
        match self.bits {
            false => DMA_HLAF_IRQ_PEND_A::NO_EFFECT,
            true => DMA_HLAF_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMA_HLAF_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DMA_HLAF_IRQ_PEND_A::PENDING
    }
}
#[doc = "Field `dma_hlaf_irq_pend[0-7]` writer - The IRQ pending bit for the half package interrupt of DMA"]
pub type DMA_HLAF_IRQ_PEND_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC_IRQ_PEND0_SPEC, DMA_HLAF_IRQ_PEND_A, O>;
impl<'a, const O: u8> DMA_HLAF_IRQ_PEND_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DMA_HLAF_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(DMA_HLAF_IRQ_PEND_A::PENDING)
    }
}
#[doc = "Field `dma_pkg_irq_pend[0-7]` reader - The IRQ pending bit for the package end interrupt of DMA"]
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
    pub fn variant(&self) -> DMA_PKG_IRQ_PEND_A {
        match self.bits {
            false => DMA_PKG_IRQ_PEND_A::NO_EFFECT,
            true => DMA_PKG_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMA_PKG_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DMA_PKG_IRQ_PEND_A::PENDING
    }
}
#[doc = "Field `dma_pkg_irq_pend[0-7]` writer - The IRQ pending bit for the package end interrupt of DMA"]
pub type DMA_PKG_IRQ_PEND_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC_IRQ_PEND0_SPEC, DMA_PKG_IRQ_PEND_A, O>;
impl<'a, const O: u8> DMA_PKG_IRQ_PEND_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DMA_PKG_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(DMA_PKG_IRQ_PEND_A::PENDING)
    }
}
#[doc = "Field `dma_queue_irq_pend[0-7]` reader - The IRQ pending bit for the queue end interrupt of DMA"]
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
    pub fn variant(&self) -> DMA_QUEUE_IRQ_PEND_A {
        match self.bits {
            false => DMA_QUEUE_IRQ_PEND_A::NO_EFFECT,
            true => DMA_QUEUE_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMA_QUEUE_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DMA_QUEUE_IRQ_PEND_A::PENDING
    }
}
#[doc = "Field `dma_queue_irq_pend[0-7]` writer - The IRQ pending bit for the queue end interrupt of DMA"]
pub type DMA_QUEUE_IRQ_PEND_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC_IRQ_PEND0_SPEC, DMA_QUEUE_IRQ_PEND_A, O>;
impl<'a, const O: u8> DMA_QUEUE_IRQ_PEND_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DMA_QUEUE_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(DMA_QUEUE_IRQ_PEND_A::PENDING)
    }
}
impl R {
    #[doc = "The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_hlaf_irq_pend(&self, n: u8) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    #[doc = "Bit 0 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma0_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma1_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma2_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma3_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma4_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma5_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma6_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma7_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_pkg_irq_pend(&self, n: u8) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma0_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma1_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma2_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma3_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma4_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma5_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma6_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma7_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_queue_irq_pend(&self, n: u8) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma0_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma1_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma2_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma3_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma4_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma5_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma6_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma7_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn dma_hlaf_irq_pend<const O: u8>(&mut self) -> DMA_HLAF_IRQ_PEND_W<O> {
        DMA_HLAF_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 0 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma0_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<0> {
        DMA_HLAF_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 4 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<4> {
        DMA_HLAF_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 8 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma2_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<8> {
        DMA_HLAF_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 12 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma3_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<12> {
        DMA_HLAF_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 16 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma4_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<16> {
        DMA_HLAF_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 20 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma5_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<20> {
        DMA_HLAF_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 24 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma6_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<24> {
        DMA_HLAF_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 28 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma7_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<28> {
        DMA_HLAF_IRQ_PEND_W::new(self)
    }
    #[doc = "The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn dma_pkg_irq_pend<const O: u8>(&mut self) -> DMA_PKG_IRQ_PEND_W<O> {
        DMA_PKG_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 1 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma0_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<1> {
        DMA_PKG_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 5 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<5> {
        DMA_PKG_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 9 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma2_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<9> {
        DMA_PKG_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 13 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma3_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<13> {
        DMA_PKG_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 17 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma4_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<17> {
        DMA_PKG_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 21 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma5_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<21> {
        DMA_PKG_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 25 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma6_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<25> {
        DMA_PKG_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 29 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma7_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<29> {
        DMA_PKG_IRQ_PEND_W::new(self)
    }
    #[doc = "The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn dma_queue_irq_pend<const O: u8>(&mut self) -> DMA_QUEUE_IRQ_PEND_W<O> {
        DMA_QUEUE_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 2 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma0_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<2> {
        DMA_QUEUE_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 6 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<6> {
        DMA_QUEUE_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 10 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma2_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<10> {
        DMA_QUEUE_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 14 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma3_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<14> {
        DMA_QUEUE_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 18 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma4_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<18> {
        DMA_QUEUE_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 22 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma5_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<22> {
        DMA_QUEUE_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 26 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma6_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<26> {
        DMA_QUEUE_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 30 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma7_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<30> {
        DMA_QUEUE_IRQ_PEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC IRQ Pending Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_irq_pend0](index.html) module"]
pub struct DMAC_IRQ_PEND0_SPEC;
impl crate::RegisterSpec for DMAC_IRQ_PEND0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_irq_pend0::R](R) reader structure"]
impl crate::Readable for DMAC_IRQ_PEND0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_irq_pend0::W](W) writer structure"]
impl crate::Writable for DMAC_IRQ_PEND0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dmac_irq_pend0 to value 0"]
impl crate::Resettable for DMAC_IRQ_PEND0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
