#[doc = "Register `emac_tx_dma_sta` reader"]
pub struct R(crate::R<EMAC_TX_DMA_STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_TX_DMA_STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_TX_DMA_STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_TX_DMA_STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tx_dma_sta` reader - The State of Transmit DMA FSM"]
pub type TX_DMA_STA_R = crate::FieldReader<u8, TX_DMA_STA_A>;
#[doc = "The State of Transmit DMA FSM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_DMA_STA_A {
    #[doc = "0: `0`"]
    STOP = 0,
    #[doc = "1: `1`"]
    RUN_FETCH_DESC = 1,
    #[doc = "2: `10`"]
    RUN_WAIT_STA = 2,
    #[doc = "3: `11`"]
    RUN_TRANS_DATA = 3,
    #[doc = "6: `110`"]
    SUSPEND = 6,
    #[doc = "7: `111`"]
    RUN_CLOSE_DESC = 7,
}
impl From<TX_DMA_STA_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_DMA_STA_A) -> Self {
        variant as _
    }
}
impl TX_DMA_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_DMA_STA_A> {
        match self.bits {
            0 => Some(TX_DMA_STA_A::STOP),
            1 => Some(TX_DMA_STA_A::RUN_FETCH_DESC),
            2 => Some(TX_DMA_STA_A::RUN_WAIT_STA),
            3 => Some(TX_DMA_STA_A::RUN_TRANS_DATA),
            6 => Some(TX_DMA_STA_A::SUSPEND),
            7 => Some(TX_DMA_STA_A::RUN_CLOSE_DESC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == TX_DMA_STA_A::STOP
    }
    #[doc = "Checks if the value of the field is `RUN_FETCH_DESC`"]
    #[inline(always)]
    pub fn is_run_fetch_desc(&self) -> bool {
        *self == TX_DMA_STA_A::RUN_FETCH_DESC
    }
    #[doc = "Checks if the value of the field is `RUN_WAIT_STA`"]
    #[inline(always)]
    pub fn is_run_wait_sta(&self) -> bool {
        *self == TX_DMA_STA_A::RUN_WAIT_STA
    }
    #[doc = "Checks if the value of the field is `RUN_TRANS_DATA`"]
    #[inline(always)]
    pub fn is_run_trans_data(&self) -> bool {
        *self == TX_DMA_STA_A::RUN_TRANS_DATA
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == TX_DMA_STA_A::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RUN_CLOSE_DESC`"]
    #[inline(always)]
    pub fn is_run_close_desc(&self) -> bool {
        *self == TX_DMA_STA_A::RUN_CLOSE_DESC
    }
}
impl R {
    #[doc = "Bits 0:2 - The State of Transmit DMA FSM"]
    #[inline(always)]
    pub fn tx_dma_sta(&self) -> TX_DMA_STA_R {
        TX_DMA_STA_R::new((self.bits & 7) as u8)
    }
}
#[doc = "EMAC Transmit DMA Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_tx_dma_sta](index.html) module"]
pub struct EMAC_TX_DMA_STA_SPEC;
impl crate::RegisterSpec for EMAC_TX_DMA_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_tx_dma_sta::R](R) reader structure"]
impl crate::Readable for EMAC_TX_DMA_STA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets emac_tx_dma_sta to value 0"]
impl crate::Resettable for EMAC_TX_DMA_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
