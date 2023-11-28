#[doc = "Register `twi_drv_dma_cfg` reader"]
pub type R = crate::R<TWI_DRV_DMA_CFG_SPEC>;
#[doc = "Register `twi_drv_dma_cfg` writer"]
pub type W = crate::W<TWI_DRV_DMA_CFG_SPEC>;
#[doc = "Field `tx_trig` reader - "]
pub type TX_TRIG_R = crate::FieldReader;
#[doc = "Field `tx_trig` writer - "]
pub type TX_TRIG_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `dma_tx_en` reader - "]
pub type DMA_TX_EN_R = crate::BitReader;
#[doc = "Field `dma_tx_en` writer - "]
pub type DMA_TX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_trig` reader - "]
pub type RX_TRIG_R = crate::FieldReader;
#[doc = "Field `rx_trig` writer - "]
pub type RX_TRIG_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `dma_rx_en` reader - "]
pub type DMA_RX_EN_R = crate::FieldReader;
#[doc = "Field `dma_rx_en` writer - "]
pub type DMA_RX_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    pub fn tx_trig(&mut self) -> TX_TRIG_W<TWI_DRV_DMA_CFG_SPEC> {
        TX_TRIG_W::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_en(&mut self) -> DMA_TX_EN_W<TWI_DRV_DMA_CFG_SPEC> {
        DMA_TX_EN_W::new(self, 8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn rx_trig(&mut self) -> RX_TRIG_W<TWI_DRV_DMA_CFG_SPEC> {
        RX_TRIG_W::new(self, 16)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_en(&mut self) -> DMA_RX_EN_W<TWI_DRV_DMA_CFG_SPEC> {
        DMA_RX_EN_W::new(self, 23)
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
#[doc = "TWI_DRV DMA Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_dma_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_dma_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_DRV_DMA_CFG_SPEC;
impl crate::RegisterSpec for TWI_DRV_DMA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_drv_dma_cfg::R`](R) reader structure"]
impl crate::Readable for TWI_DRV_DMA_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twi_drv_dma_cfg::W`](W) writer structure"]
impl crate::Writable for TWI_DRV_DMA_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_drv_dma_cfg to value 0"]
impl crate::Resettable for TWI_DRV_DMA_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
