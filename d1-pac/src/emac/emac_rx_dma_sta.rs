#[doc = "Register `emac_rx_dma_sta` reader"]
pub type R = crate::R<EMAC_RX_DMA_STA_SPEC>;
#[doc = "Field `rx_dma_sta` reader - The State of RX DMA FSM"]
pub type RX_DMA_STA_R = crate::FieldReader<RX_DMA_STA_A>;
#[doc = "The State of RX DMA FSM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_DMA_STA_A {
    #[doc = "0: `0`"]
    STOP = 0,
    #[doc = "1: `1`"]
    RUN_FETCH_DESC = 1,
    #[doc = "3: `11`"]
    RUN_WAIT_FRM = 3,
    #[doc = "4: `100`"]
    SUSPEND = 4,
    #[doc = "5: `101`"]
    RUN_CLOSE_DESC = 5,
    #[doc = "7: `111`"]
    RUN_TRANS_DATA = 7,
}
impl From<RX_DMA_STA_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_DMA_STA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RX_DMA_STA_A {
    type Ux = u8;
}
impl RX_DMA_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RX_DMA_STA_A> {
        match self.bits {
            0 => Some(RX_DMA_STA_A::STOP),
            1 => Some(RX_DMA_STA_A::RUN_FETCH_DESC),
            3 => Some(RX_DMA_STA_A::RUN_WAIT_FRM),
            4 => Some(RX_DMA_STA_A::SUSPEND),
            5 => Some(RX_DMA_STA_A::RUN_CLOSE_DESC),
            7 => Some(RX_DMA_STA_A::RUN_TRANS_DATA),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RX_DMA_STA_A::STOP
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_run_fetch_desc(&self) -> bool {
        *self == RX_DMA_STA_A::RUN_FETCH_DESC
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_run_wait_frm(&self) -> bool {
        *self == RX_DMA_STA_A::RUN_WAIT_FRM
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == RX_DMA_STA_A::SUSPEND
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_run_close_desc(&self) -> bool {
        *self == RX_DMA_STA_A::RUN_CLOSE_DESC
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_run_trans_data(&self) -> bool {
        *self == RX_DMA_STA_A::RUN_TRANS_DATA
    }
}
impl R {
    #[doc = "Bits 0:2 - The State of RX DMA FSM"]
    #[inline(always)]
    pub fn rx_dma_sta(&self) -> RX_DMA_STA_R {
        RX_DMA_STA_R::new((self.bits & 7) as u8)
    }
}
#[doc = "EMAC Receive DMA Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_rx_dma_sta::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMAC_RX_DMA_STA_SPEC;
impl crate::RegisterSpec for EMAC_RX_DMA_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emac_rx_dma_sta::R`](R) reader structure"]
impl crate::Readable for EMAC_RX_DMA_STA_SPEC {}
#[doc = "`reset()` method sets emac_rx_dma_sta to value 0"]
impl crate::Resettable for EMAC_RX_DMA_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
