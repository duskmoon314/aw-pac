#[doc = "Register `emac_tx_dma_sta` reader"]
pub type R = crate::R<EMAC_TX_DMA_STA_SPEC>;
#[doc = "Field `tx_dma_sta` reader - The State of Transmit DMA FSM"]
pub type TX_DMA_STA_R = crate::FieldReader<TX_DMA_STA_A>;
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
impl crate::FieldSpec for TX_DMA_STA_A {
    type Ux = u8;
}
impl TX_DMA_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TX_DMA_STA_A> {
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
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == TX_DMA_STA_A::STOP
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_run_fetch_desc(&self) -> bool {
        *self == TX_DMA_STA_A::RUN_FETCH_DESC
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_run_wait_sta(&self) -> bool {
        *self == TX_DMA_STA_A::RUN_WAIT_STA
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_run_trans_data(&self) -> bool {
        *self == TX_DMA_STA_A::RUN_TRANS_DATA
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == TX_DMA_STA_A::SUSPEND
    }
    #[doc = "`111`"]
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
#[doc = "EMAC Transmit DMA Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_tx_dma_sta::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMAC_TX_DMA_STA_SPEC;
impl crate::RegisterSpec for EMAC_TX_DMA_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emac_tx_dma_sta::R`](R) reader structure"]
impl crate::Readable for EMAC_TX_DMA_STA_SPEC {}
#[doc = "`reset()` method sets emac_tx_dma_sta to value 0"]
impl crate::Resettable for EMAC_TX_DMA_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
