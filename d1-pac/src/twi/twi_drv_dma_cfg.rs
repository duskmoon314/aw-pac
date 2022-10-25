#[doc = "Register `twi_drv_dma_cfg` reader"]
pub struct R(crate::R<TWI_DRV_DMA_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_DRV_DMA_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_DRV_DMA_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_DRV_DMA_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `twi_drv_dma_cfg` writer"]
pub struct W(crate::W<TWI_DRV_DMA_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_DRV_DMA_CFG_SPEC>;
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
impl From<crate::W<TWI_DRV_DMA_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_DRV_DMA_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_trig` reader - "]
pub type TX_TRIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_trig` writer - "]
pub type TX_TRIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TWI_DRV_DMA_CFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `dma_tx_en` reader - "]
pub type DMA_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `dma_tx_en` writer - "]
pub type DMA_TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TWI_DRV_DMA_CFG_SPEC, bool, O>;
#[doc = "Field `rx_trig` reader - "]
pub type RX_TRIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rx_trig` writer - "]
pub type RX_TRIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TWI_DRV_DMA_CFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `dma_rx_en` reader - "]
pub type DMA_RX_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dma_rx_en` writer - "]
pub type DMA_RX_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TWI_DRV_DMA_CFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tx_trig(&self) -> TX_TRIG_R {
        TX_TRIG_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DMA_TX_EN_R {
        DMA_TX_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rx_trig(&self) -> RX_TRIG_R {
        RX_TRIG_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DMA_RX_EN_R {
        DMA_RX_EN_R::new(((self.bits >> 23) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn tx_trig(&mut self) -> TX_TRIG_W<0> {
        TX_TRIG_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_en(&mut self) -> DMA_TX_EN_W<8> {
        DMA_TX_EN_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn rx_trig(&mut self) -> RX_TRIG_W<16> {
        RX_TRIG_W::new(self)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_en(&mut self) -> DMA_RX_EN_W<23> {
        DMA_RX_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI_DRV DMA Configure Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_drv_dma_cfg](index.html) module"]
pub struct TWI_DRV_DMA_CFG_SPEC;
impl crate::RegisterSpec for TWI_DRV_DMA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_drv_dma_cfg::R](R) reader structure"]
impl crate::Readable for TWI_DRV_DMA_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twi_drv_dma_cfg::W](W) writer structure"]
impl crate::Writable for TWI_DRV_DMA_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_drv_dma_cfg to value 0"]
impl crate::Resettable for TWI_DRV_DMA_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
