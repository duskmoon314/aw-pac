#[doc = "Register `DMAC_AUTO_GATE_REG` reader"]
pub struct R(crate::R<DMAC_AUTO_GATE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_AUTO_GATE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_AUTO_GATE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_AUTO_GATE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_AUTO_GATE_REG` writer"]
pub struct W(crate::W<DMAC_AUTO_GATE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_AUTO_GATE_REG_SPEC>;
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
impl From<crate::W<DMAC_AUTO_GATE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_AUTO_GATE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Auto gating bit of DMA MCLK interfact circuit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DMA_MCLK_CIRCUIT` reader - Auto gating bit of DMA MCLK interfact circuit"]
pub type DMA_MCLK_CIRCUIT_R = crate::BitReader<DMA_MCLK_CIRCUIT_A>;
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
#[doc = "Field `DMA_MCLK_CIRCUIT` writer - Auto gating bit of DMA MCLK interfact circuit"]
pub type DMA_MCLK_CIRCUIT_W<'a> =
    crate::BitWriter<'a, u32, DMAC_AUTO_GATE_REG_SPEC, DMA_MCLK_CIRCUIT_A, 2>;
impl<'a> DMA_MCLK_CIRCUIT_W<'a> {
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
#[doc = "Auto gating bit of DMA common circuit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DMA_COMMON_CIRCUIT` reader - Auto gating bit of DMA common circuit"]
pub type DMA_COMMON_CIRCUIT_R = crate::BitReader<DMA_COMMON_CIRCUIT_A>;
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
#[doc = "Field `DMA_COMMON_CIRCUIT` writer - Auto gating bit of DMA common circuit"]
pub type DMA_COMMON_CIRCUIT_W<'a> =
    crate::BitWriter<'a, u32, DMAC_AUTO_GATE_REG_SPEC, DMA_COMMON_CIRCUIT_A, 1>;
impl<'a> DMA_COMMON_CIRCUIT_W<'a> {
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
#[doc = "Auto gating bit of DMA channel circuit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DMA_CHAN_CIRCUIT` reader - Auto gating bit of DMA channel circuit"]
pub type DMA_CHAN_CIRCUIT_R = crate::BitReader<DMA_CHAN_CIRCUIT_A>;
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
#[doc = "Field `DMA_CHAN_CIRCUIT` writer - Auto gating bit of DMA channel circuit"]
pub type DMA_CHAN_CIRCUIT_W<'a> =
    crate::BitWriter<'a, u32, DMAC_AUTO_GATE_REG_SPEC, DMA_CHAN_CIRCUIT_A, 0>;
impl<'a> DMA_CHAN_CIRCUIT_W<'a> {
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
impl R {
    #[doc = "Bit 2 - Auto gating bit of DMA MCLK interfact circuit"]
    #[inline(always)]
    pub fn dma_mclk_circuit(&self) -> DMA_MCLK_CIRCUIT_R {
        DMA_MCLK_CIRCUIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Auto gating bit of DMA common circuit"]
    #[inline(always)]
    pub fn dma_common_circuit(&self) -> DMA_COMMON_CIRCUIT_R {
        DMA_COMMON_CIRCUIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Auto gating bit of DMA channel circuit"]
    #[inline(always)]
    pub fn dma_chan_circuit(&self) -> DMA_CHAN_CIRCUIT_R {
        DMA_CHAN_CIRCUIT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Auto gating bit of DMA MCLK interfact circuit"]
    #[inline(always)]
    pub fn dma_mclk_circuit(&mut self) -> DMA_MCLK_CIRCUIT_W {
        DMA_MCLK_CIRCUIT_W::new(self)
    }
    #[doc = "Bit 1 - Auto gating bit of DMA common circuit"]
    #[inline(always)]
    pub fn dma_common_circuit(&mut self) -> DMA_COMMON_CIRCUIT_W {
        DMA_COMMON_CIRCUIT_W::new(self)
    }
    #[doc = "Bit 0 - Auto gating bit of DMA channel circuit"]
    #[inline(always)]
    pub fn dma_chan_circuit(&mut self) -> DMA_CHAN_CIRCUIT_W {
        DMA_CHAN_CIRCUIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Auto Gating Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_auto_gate_reg](index.html) module"]
pub struct DMAC_AUTO_GATE_REG_SPEC;
impl crate::RegisterSpec for DMAC_AUTO_GATE_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_auto_gate_reg::R](R) reader structure"]
impl crate::Readable for DMAC_AUTO_GATE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_auto_gate_reg::W](W) writer structure"]
impl crate::Writable for DMAC_AUTO_GATE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_AUTO_GATE_REG to value 0"]
impl crate::Resettable for DMAC_AUTO_GATE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
