#[doc = "Register `dmac_auto_gate` reader"]
pub struct R(crate::R<DMAC_AUTO_GATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_AUTO_GATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_AUTO_GATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_AUTO_GATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dmac_auto_gate` writer"]
pub struct W(crate::W<DMAC_AUTO_GATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_AUTO_GATE_SPEC>;
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
impl From<crate::W<DMAC_AUTO_GATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_AUTO_GATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma_chan_circuit` reader - Auto gating bit of DMA channel circuit"]
pub type DMA_CHAN_CIRCUIT_R = crate::BitReader<DMA_CHAN_CIRCUIT_A>;
#[doc = "Auto gating bit of DMA channel circuit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_CHAN_CIRCUIT_A {
    #[doc = "0: `0`"]
    ENABLED = 0,
    #[doc = "1: `1`"]
    DISABLED = 1,
}
impl From<DMA_CHAN_CIRCUIT_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_CHAN_CIRCUIT_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_CHAN_CIRCUIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_CHAN_CIRCUIT_A {
        match self.bits {
            false => DMA_CHAN_CIRCUIT_A::ENABLED,
            true => DMA_CHAN_CIRCUIT_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA_CHAN_CIRCUIT_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_CHAN_CIRCUIT_A::DISABLED
    }
}
#[doc = "Field `dma_chan_circuit` writer - Auto gating bit of DMA channel circuit"]
pub type DMA_CHAN_CIRCUIT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC_AUTO_GATE_SPEC, DMA_CHAN_CIRCUIT_A, O>;
impl<'a, const O: u8> DMA_CHAN_CIRCUIT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA_CHAN_CIRCUIT_A::ENABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_CHAN_CIRCUIT_A::DISABLED)
    }
}
#[doc = "Field `dma_common_circuit` reader - Auto gating bit of DMA common circuit"]
pub type DMA_COMMON_CIRCUIT_R = crate::BitReader<DMA_COMMON_CIRCUIT_A>;
#[doc = "Auto gating bit of DMA common circuit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_COMMON_CIRCUIT_A {
    #[doc = "0: `0`"]
    ENABLED = 0,
    #[doc = "1: `1`"]
    DISABLED = 1,
}
impl From<DMA_COMMON_CIRCUIT_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_COMMON_CIRCUIT_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_COMMON_CIRCUIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_COMMON_CIRCUIT_A {
        match self.bits {
            false => DMA_COMMON_CIRCUIT_A::ENABLED,
            true => DMA_COMMON_CIRCUIT_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA_COMMON_CIRCUIT_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_COMMON_CIRCUIT_A::DISABLED
    }
}
#[doc = "Field `dma_common_circuit` writer - Auto gating bit of DMA common circuit"]
pub type DMA_COMMON_CIRCUIT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC_AUTO_GATE_SPEC, DMA_COMMON_CIRCUIT_A, O>;
impl<'a, const O: u8> DMA_COMMON_CIRCUIT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA_COMMON_CIRCUIT_A::ENABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_COMMON_CIRCUIT_A::DISABLED)
    }
}
#[doc = "Field `dma_mclk_circuit` reader - Auto gating bit of DMA MCLK interfact circuit"]
pub type DMA_MCLK_CIRCUIT_R = crate::BitReader<DMA_MCLK_CIRCUIT_A>;
#[doc = "Auto gating bit of DMA MCLK interfact circuit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_MCLK_CIRCUIT_A {
    #[doc = "0: `0`"]
    ENABLED = 0,
    #[doc = "1: `1`"]
    DISABLED = 1,
}
impl From<DMA_MCLK_CIRCUIT_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_MCLK_CIRCUIT_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_MCLK_CIRCUIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_MCLK_CIRCUIT_A {
        match self.bits {
            false => DMA_MCLK_CIRCUIT_A::ENABLED,
            true => DMA_MCLK_CIRCUIT_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA_MCLK_CIRCUIT_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_MCLK_CIRCUIT_A::DISABLED
    }
}
#[doc = "Field `dma_mclk_circuit` writer - Auto gating bit of DMA MCLK interfact circuit"]
pub type DMA_MCLK_CIRCUIT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC_AUTO_GATE_SPEC, DMA_MCLK_CIRCUIT_A, O>;
impl<'a, const O: u8> DMA_MCLK_CIRCUIT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA_MCLK_CIRCUIT_A::ENABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_MCLK_CIRCUIT_A::DISABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Auto gating bit of DMA channel circuit"]
    #[inline(always)]
    pub fn dma_chan_circuit(&self) -> DMA_CHAN_CIRCUIT_R {
        DMA_CHAN_CIRCUIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto gating bit of DMA common circuit"]
    #[inline(always)]
    pub fn dma_common_circuit(&self) -> DMA_COMMON_CIRCUIT_R {
        DMA_COMMON_CIRCUIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto gating bit of DMA MCLK interfact circuit"]
    #[inline(always)]
    pub fn dma_mclk_circuit(&self) -> DMA_MCLK_CIRCUIT_R {
        DMA_MCLK_CIRCUIT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auto gating bit of DMA channel circuit"]
    #[inline(always)]
    #[must_use]
    pub fn dma_chan_circuit(&mut self) -> DMA_CHAN_CIRCUIT_W<0> {
        DMA_CHAN_CIRCUIT_W::new(self)
    }
    #[doc = "Bit 1 - Auto gating bit of DMA common circuit"]
    #[inline(always)]
    #[must_use]
    pub fn dma_common_circuit(&mut self) -> DMA_COMMON_CIRCUIT_W<1> {
        DMA_COMMON_CIRCUIT_W::new(self)
    }
    #[doc = "Bit 2 - Auto gating bit of DMA MCLK interfact circuit"]
    #[inline(always)]
    #[must_use]
    pub fn dma_mclk_circuit(&mut self) -> DMA_MCLK_CIRCUIT_W<2> {
        DMA_MCLK_CIRCUIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Auto Gating Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_auto_gate](index.html) module"]
pub struct DMAC_AUTO_GATE_SPEC;
impl crate::RegisterSpec for DMAC_AUTO_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_auto_gate::R](R) reader structure"]
impl crate::Readable for DMAC_AUTO_GATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_auto_gate::W](W) writer structure"]
impl crate::Writable for DMAC_AUTO_GATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dmac_auto_gate to value 0"]
impl crate::Resettable for DMAC_AUTO_GATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
