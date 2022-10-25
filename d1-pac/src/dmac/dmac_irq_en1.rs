#[doc = "Register `dmac_irq_en1` reader"]
pub struct R(crate::R<DMAC_IRQ_EN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_IRQ_EN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_IRQ_EN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_IRQ_EN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dmac_irq_en1` writer"]
pub struct W(crate::W<DMAC_IRQ_EN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_IRQ_EN1_SPEC>;
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
impl From<crate::W<DMAC_IRQ_EN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_IRQ_EN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma_hlaf_irq_en[8-15]` reader - Enable the half package interrupt of DMA"]
pub type DMA_HLAF_IRQ_EN_R = crate::BitReader<DMA_HLAF_IRQ_EN_A>;
#[doc = "Enable the half package interrupt of DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_HLAF_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<DMA_HLAF_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_HLAF_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_HLAF_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_HLAF_IRQ_EN_A {
        match self.bits {
            false => DMA_HLAF_IRQ_EN_A::DISABLED,
            true => DMA_HLAF_IRQ_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_HLAF_IRQ_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA_HLAF_IRQ_EN_A::ENABLED
    }
}
#[doc = "Field `dma_hlaf_irq_en[8-15]` writer - Enable the half package interrupt of DMA"]
pub type DMA_HLAF_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC_IRQ_EN1_SPEC, DMA_HLAF_IRQ_EN_A, O>;
impl<'a, const O: u8> DMA_HLAF_IRQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_HLAF_IRQ_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA_HLAF_IRQ_EN_A::ENABLED)
    }
}
#[doc = "Field `dma_pkg_irq_en[8-15]` reader - Enable the package end interrupt of DMA"]
pub type DMA_PKG_IRQ_EN_R = crate::BitReader<DMA_PKG_IRQ_EN_A>;
#[doc = "Enable the package end interrupt of DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_PKG_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<DMA_PKG_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_PKG_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_PKG_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_PKG_IRQ_EN_A {
        match self.bits {
            false => DMA_PKG_IRQ_EN_A::DISABLED,
            true => DMA_PKG_IRQ_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_PKG_IRQ_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA_PKG_IRQ_EN_A::ENABLED
    }
}
#[doc = "Field `dma_pkg_irq_en[8-15]` writer - Enable the package end interrupt of DMA"]
pub type DMA_PKG_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC_IRQ_EN1_SPEC, DMA_PKG_IRQ_EN_A, O>;
impl<'a, const O: u8> DMA_PKG_IRQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_PKG_IRQ_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA_PKG_IRQ_EN_A::ENABLED)
    }
}
#[doc = "Field `dma_queue_irq_en[8-15]` reader - Enable the queue end interrupt of DMA"]
pub type DMA_QUEUE_IRQ_EN_R = crate::BitReader<DMA_QUEUE_IRQ_EN_A>;
#[doc = "Enable the queue end interrupt of DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_QUEUE_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<DMA_QUEUE_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_QUEUE_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_QUEUE_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_QUEUE_IRQ_EN_A {
        match self.bits {
            false => DMA_QUEUE_IRQ_EN_A::DISABLED,
            true => DMA_QUEUE_IRQ_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_QUEUE_IRQ_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA_QUEUE_IRQ_EN_A::ENABLED
    }
}
#[doc = "Field `dma_queue_irq_en[8-15]` writer - Enable the queue end interrupt of DMA"]
pub type DMA_QUEUE_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC_IRQ_EN1_SPEC, DMA_QUEUE_IRQ_EN_A, O>;
impl<'a, const O: u8> DMA_QUEUE_IRQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_QUEUE_IRQ_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA_QUEUE_IRQ_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_hlaf_irq_en(&self, n: u8) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new(((self.bits >> ((n - 8) * 4)) & 1) != 0)
    }
    #[doc = "Bit 0 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma8_hlaf_irq_en(&self) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma9_hlaf_irq_en(&self) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma10_hlaf_irq_en(&self) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma11_hlaf_irq_en(&self) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma12_hlaf_irq_en(&self) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma13_hlaf_irq_en(&self) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma14_hlaf_irq_en(&self) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma15_hlaf_irq_en(&self) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_pkg_irq_en(&self, n: u8) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> ((n - 8) * 4 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma8_pkg_irq_en(&self) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma9_pkg_irq_en(&self) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma10_pkg_irq_en(&self) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma11_pkg_irq_en(&self) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma12_pkg_irq_en(&self) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma13_pkg_irq_en(&self) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma14_pkg_irq_en(&self) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma15_pkg_irq_en(&self) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_queue_irq_en(&self, n: u8) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> ((n - 8) * 4 + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma8_queue_irq_en(&self) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma9_queue_irq_en(&self) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma10_queue_irq_en(&self) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma11_queue_irq_en(&self) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma12_queue_irq_en(&self) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma13_queue_irq_en(&self) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma14_queue_irq_en(&self) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma15_queue_irq_en(&self) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Enable the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn dma_hlaf_irq_en<const O: u8>(&mut self) -> DMA_HLAF_IRQ_EN_W<O> {
        DMA_HLAF_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 0 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma8_hlaf_irq_en(&mut self) -> DMA_HLAF_IRQ_EN_W<0> {
        DMA_HLAF_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 4 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma9_hlaf_irq_en(&mut self) -> DMA_HLAF_IRQ_EN_W<4> {
        DMA_HLAF_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 8 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma10_hlaf_irq_en(&mut self) -> DMA_HLAF_IRQ_EN_W<8> {
        DMA_HLAF_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 12 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma11_hlaf_irq_en(&mut self) -> DMA_HLAF_IRQ_EN_W<12> {
        DMA_HLAF_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 16 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma12_hlaf_irq_en(&mut self) -> DMA_HLAF_IRQ_EN_W<16> {
        DMA_HLAF_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 20 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma13_hlaf_irq_en(&mut self) -> DMA_HLAF_IRQ_EN_W<20> {
        DMA_HLAF_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 24 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma14_hlaf_irq_en(&mut self) -> DMA_HLAF_IRQ_EN_W<24> {
        DMA_HLAF_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 28 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma15_hlaf_irq_en(&mut self) -> DMA_HLAF_IRQ_EN_W<28> {
        DMA_HLAF_IRQ_EN_W::new(self)
    }
    #[doc = "Enable the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn dma_pkg_irq_en<const O: u8>(&mut self) -> DMA_PKG_IRQ_EN_W<O> {
        DMA_PKG_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 1 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma8_pkg_irq_en(&mut self) -> DMA_PKG_IRQ_EN_W<1> {
        DMA_PKG_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 5 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma9_pkg_irq_en(&mut self) -> DMA_PKG_IRQ_EN_W<5> {
        DMA_PKG_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 9 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma10_pkg_irq_en(&mut self) -> DMA_PKG_IRQ_EN_W<9> {
        DMA_PKG_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 13 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma11_pkg_irq_en(&mut self) -> DMA_PKG_IRQ_EN_W<13> {
        DMA_PKG_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 17 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma12_pkg_irq_en(&mut self) -> DMA_PKG_IRQ_EN_W<17> {
        DMA_PKG_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 21 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma13_pkg_irq_en(&mut self) -> DMA_PKG_IRQ_EN_W<21> {
        DMA_PKG_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 25 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma14_pkg_irq_en(&mut self) -> DMA_PKG_IRQ_EN_W<25> {
        DMA_PKG_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 29 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma15_pkg_irq_en(&mut self) -> DMA_PKG_IRQ_EN_W<29> {
        DMA_PKG_IRQ_EN_W::new(self)
    }
    #[doc = "Enable the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn dma_queue_irq_en<const O: u8>(&mut self) -> DMA_QUEUE_IRQ_EN_W<O> {
        DMA_QUEUE_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 2 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma8_queue_irq_en(&mut self) -> DMA_QUEUE_IRQ_EN_W<2> {
        DMA_QUEUE_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 6 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma9_queue_irq_en(&mut self) -> DMA_QUEUE_IRQ_EN_W<6> {
        DMA_QUEUE_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 10 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma10_queue_irq_en(&mut self) -> DMA_QUEUE_IRQ_EN_W<10> {
        DMA_QUEUE_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 14 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma11_queue_irq_en(&mut self) -> DMA_QUEUE_IRQ_EN_W<14> {
        DMA_QUEUE_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 18 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma12_queue_irq_en(&mut self) -> DMA_QUEUE_IRQ_EN_W<18> {
        DMA_QUEUE_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 22 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma13_queue_irq_en(&mut self) -> DMA_QUEUE_IRQ_EN_W<22> {
        DMA_QUEUE_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 26 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma14_queue_irq_en(&mut self) -> DMA_QUEUE_IRQ_EN_W<26> {
        DMA_QUEUE_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 30 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma15_queue_irq_en(&mut self) -> DMA_QUEUE_IRQ_EN_W<30> {
        DMA_QUEUE_IRQ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC IRQ Enable Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_irq_en1](index.html) module"]
pub struct DMAC_IRQ_EN1_SPEC;
impl crate::RegisterSpec for DMAC_IRQ_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_irq_en1::R](R) reader structure"]
impl crate::Readable for DMAC_IRQ_EN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_irq_en1::W](W) writer structure"]
impl crate::Writable for DMAC_IRQ_EN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dmac_irq_en1 to value 0"]
impl crate::Resettable for DMAC_IRQ_EN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
