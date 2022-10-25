#[doc = "Register `emac_int_sta` reader"]
pub struct R(crate::R<EMAC_INT_STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_INT_STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_INT_STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_INT_STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `emac_int_sta` writer"]
pub struct W(crate::W<EMAC_INT_STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_INT_STA_SPEC>;
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
impl From<crate::W<EMAC_INT_STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_INT_STA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_p` reader - Frame Transmission Interrupt Pending"]
pub type TX_P_R = crate::BitReader<TX_P_A>;
#[doc = "Frame Transmission Interrupt Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_P_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<TX_P_A> for bool {
    #[inline(always)]
    fn from(variant: TX_P_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_P_A {
        match self.bits {
            false => TX_P_A::NO_PENDING,
            true => TX_P_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == TX_P_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TX_P_A::PENDING
    }
}
#[doc = "Field `tx_p` writer - Frame Transmission Interrupt Pending"]
pub type TX_P_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EMAC_INT_STA_SPEC, TX_P_A, O>;
impl<'a, const O: u8> TX_P_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(TX_P_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(TX_P_A::PENDING)
    }
}
#[doc = "Field `tx_dma_stopped_p` reader - Transmission DMA Stopped Interrupt Pending"]
pub type TX_DMA_STOPPED_P_R = crate::BitReader<TX_DMA_STOPPED_P_A>;
#[doc = "Transmission DMA Stopped Interrupt Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_DMA_STOPPED_P_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<TX_DMA_STOPPED_P_A> for bool {
    #[inline(always)]
    fn from(variant: TX_DMA_STOPPED_P_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_DMA_STOPPED_P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_DMA_STOPPED_P_A {
        match self.bits {
            false => TX_DMA_STOPPED_P_A::NO_PENDING,
            true => TX_DMA_STOPPED_P_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == TX_DMA_STOPPED_P_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TX_DMA_STOPPED_P_A::PENDING
    }
}
#[doc = "Field `tx_dma_stopped_p` writer - Transmission DMA Stopped Interrupt Pending"]
pub type TX_DMA_STOPPED_P_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, EMAC_INT_STA_SPEC, TX_DMA_STOPPED_P_A, O>;
impl<'a, const O: u8> TX_DMA_STOPPED_P_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(TX_DMA_STOPPED_P_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(TX_DMA_STOPPED_P_A::PENDING)
    }
}
#[doc = "Field `tx_buf_ua_p` reader - TX Buffer UA Interrupt Pending"]
pub type TX_BUF_UA_P_R = crate::BitReader<TX_BUF_UA_P_A>;
#[doc = "TX Buffer UA Interrupt Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_BUF_UA_P_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<TX_BUF_UA_P_A> for bool {
    #[inline(always)]
    fn from(variant: TX_BUF_UA_P_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_BUF_UA_P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_BUF_UA_P_A {
        match self.bits {
            false => TX_BUF_UA_P_A::NO_PENDING,
            true => TX_BUF_UA_P_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == TX_BUF_UA_P_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TX_BUF_UA_P_A::PENDING
    }
}
#[doc = "Field `tx_buf_ua_p` writer - TX Buffer UA Interrupt Pending"]
pub type TX_BUF_UA_P_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, EMAC_INT_STA_SPEC, TX_BUF_UA_P_A, O>;
impl<'a, const O: u8> TX_BUF_UA_P_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(TX_BUF_UA_P_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(TX_BUF_UA_P_A::PENDING)
    }
}
#[doc = "Field `tx_timeout_p` reader - Transmitter Timeout Interrupt Pending"]
pub type TX_TIMEOUT_P_R = crate::BitReader<TX_TIMEOUT_P_A>;
#[doc = "Transmitter Timeout Interrupt Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_TIMEOUT_P_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<TX_TIMEOUT_P_A> for bool {
    #[inline(always)]
    fn from(variant: TX_TIMEOUT_P_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_TIMEOUT_P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_TIMEOUT_P_A {
        match self.bits {
            false => TX_TIMEOUT_P_A::NO_PENDING,
            true => TX_TIMEOUT_P_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == TX_TIMEOUT_P_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TX_TIMEOUT_P_A::PENDING
    }
}
#[doc = "Field `tx_timeout_p` writer - Transmitter Timeout Interrupt Pending"]
pub type TX_TIMEOUT_P_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, EMAC_INT_STA_SPEC, TX_TIMEOUT_P_A, O>;
impl<'a, const O: u8> TX_TIMEOUT_P_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(TX_TIMEOUT_P_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(TX_TIMEOUT_P_A::PENDING)
    }
}
#[doc = "Field `tx_underflow_p` reader - TX FIFO Underflow Interrupt Pending"]
pub type TX_UNDERFLOW_P_R = crate::BitReader<TX_UNDERFLOW_P_A>;
#[doc = "TX FIFO Underflow Interrupt Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_UNDERFLOW_P_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<TX_UNDERFLOW_P_A> for bool {
    #[inline(always)]
    fn from(variant: TX_UNDERFLOW_P_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_UNDERFLOW_P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_UNDERFLOW_P_A {
        match self.bits {
            false => TX_UNDERFLOW_P_A::NO_PENDING,
            true => TX_UNDERFLOW_P_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == TX_UNDERFLOW_P_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TX_UNDERFLOW_P_A::PENDING
    }
}
#[doc = "Field `tx_underflow_p` writer - TX FIFO Underflow Interrupt Pending"]
pub type TX_UNDERFLOW_P_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, EMAC_INT_STA_SPEC, TX_UNDERFLOW_P_A, O>;
impl<'a, const O: u8> TX_UNDERFLOW_P_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(TX_UNDERFLOW_P_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(TX_UNDERFLOW_P_A::PENDING)
    }
}
#[doc = "Field `tx_early_p` reader - Total interrupt pending which the frame is transmitted to FIFO"]
pub type TX_EARLY_P_R = crate::BitReader<TX_EARLY_P_A>;
#[doc = "Total interrupt pending which the frame is transmitted to FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_EARLY_P_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<TX_EARLY_P_A> for bool {
    #[inline(always)]
    fn from(variant: TX_EARLY_P_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_EARLY_P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_EARLY_P_A {
        match self.bits {
            false => TX_EARLY_P_A::NO_PENDING,
            true => TX_EARLY_P_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == TX_EARLY_P_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TX_EARLY_P_A::PENDING
    }
}
#[doc = "Field `tx_early_p` writer - Total interrupt pending which the frame is transmitted to FIFO"]
pub type TX_EARLY_P_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, EMAC_INT_STA_SPEC, TX_EARLY_P_A, O>;
impl<'a, const O: u8> TX_EARLY_P_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(TX_EARLY_P_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(TX_EARLY_P_A::PENDING)
    }
}
#[doc = "Field `rx_p` reader - Frame RX Completed Interrupt Pending"]
pub type RX_P_R = crate::BitReader<RX_P_A>;
#[doc = "Frame RX Completed Interrupt Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_P_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<RX_P_A> for bool {
    #[inline(always)]
    fn from(variant: RX_P_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_P_A {
        match self.bits {
            false => RX_P_A::NO_PENDING,
            true => RX_P_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == RX_P_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RX_P_A::PENDING
    }
}
#[doc = "Field `rx_p` writer - Frame RX Completed Interrupt Pending"]
pub type RX_P_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EMAC_INT_STA_SPEC, RX_P_A, O>;
impl<'a, const O: u8> RX_P_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(RX_P_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(RX_P_A::PENDING)
    }
}
#[doc = "Field `rx_buf_ua_p` reader - RX Buffer UA Interrupt Pending"]
pub type RX_BUF_UA_P_R = crate::BitReader<RX_BUF_UA_P_A>;
#[doc = "RX Buffer UA Interrupt Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_BUF_UA_P_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<RX_BUF_UA_P_A> for bool {
    #[inline(always)]
    fn from(variant: RX_BUF_UA_P_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_BUF_UA_P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_BUF_UA_P_A {
        match self.bits {
            false => RX_BUF_UA_P_A::NO_PENDING,
            true => RX_BUF_UA_P_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == RX_BUF_UA_P_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RX_BUF_UA_P_A::PENDING
    }
}
#[doc = "Field `rx_buf_ua_p` writer - RX Buffer UA Interrupt Pending"]
pub type RX_BUF_UA_P_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, EMAC_INT_STA_SPEC, RX_BUF_UA_P_A, O>;
impl<'a, const O: u8> RX_BUF_UA_P_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(RX_BUF_UA_P_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(RX_BUF_UA_P_A::PENDING)
    }
}
#[doc = "Field `rx_dma_stopped_p` reader - When this bit asserted, the RX DMA FSM is stopped."]
pub type RX_DMA_STOPPED_P_R = crate::BitReader<bool>;
#[doc = "Field `rx_dma_stopped_p` writer - When this bit asserted, the RX DMA FSM is stopped."]
pub type RX_DMA_STOPPED_P_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, EMAC_INT_STA_SPEC, bool, O>;
#[doc = "Field `rx_timeout_p` reader - RX Timeout Interrupt Pending"]
pub type RX_TIMEOUT_P_R = crate::BitReader<RX_TIMEOUT_P_A>;
#[doc = "RX Timeout Interrupt Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_TIMEOUT_P_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<RX_TIMEOUT_P_A> for bool {
    #[inline(always)]
    fn from(variant: RX_TIMEOUT_P_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_TIMEOUT_P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_TIMEOUT_P_A {
        match self.bits {
            false => RX_TIMEOUT_P_A::NO_PENDING,
            true => RX_TIMEOUT_P_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == RX_TIMEOUT_P_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RX_TIMEOUT_P_A::PENDING
    }
}
#[doc = "Field `rx_timeout_p` writer - RX Timeout Interrupt Pending"]
pub type RX_TIMEOUT_P_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, EMAC_INT_STA_SPEC, RX_TIMEOUT_P_A, O>;
impl<'a, const O: u8> RX_TIMEOUT_P_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(RX_TIMEOUT_P_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(RX_TIMEOUT_P_A::PENDING)
    }
}
#[doc = "Field `rx_overflow_p` reader - RX FIFO Overflow Error Interrupt Pending"]
pub type RX_OVERFLOW_P_R = crate::BitReader<RX_OVERFLOW_P_A>;
#[doc = "RX FIFO Overflow Error Interrupt Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_OVERFLOW_P_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<RX_OVERFLOW_P_A> for bool {
    #[inline(always)]
    fn from(variant: RX_OVERFLOW_P_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_OVERFLOW_P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_OVERFLOW_P_A {
        match self.bits {
            false => RX_OVERFLOW_P_A::NO_PENDING,
            true => RX_OVERFLOW_P_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == RX_OVERFLOW_P_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RX_OVERFLOW_P_A::PENDING
    }
}
#[doc = "Field `rx_overflow_p` writer - RX FIFO Overflow Error Interrupt Pending"]
pub type RX_OVERFLOW_P_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, EMAC_INT_STA_SPEC, RX_OVERFLOW_P_A, O>;
impl<'a, const O: u8> RX_OVERFLOW_P_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(RX_OVERFLOW_P_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(RX_OVERFLOW_P_A::PENDING)
    }
}
#[doc = "Field `rx_early_p` reader - RX DMA Filled First Data Buffer of the Receive Frame Interrupt Pending"]
pub type RX_EARLY_P_R = crate::BitReader<RX_EARLY_P_A>;
#[doc = "RX DMA Filled First Data Buffer of the Receive Frame Interrupt Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_EARLY_P_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<RX_EARLY_P_A> for bool {
    #[inline(always)]
    fn from(variant: RX_EARLY_P_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_EARLY_P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_EARLY_P_A {
        match self.bits {
            false => RX_EARLY_P_A::NO_PENDING,
            true => RX_EARLY_P_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == RX_EARLY_P_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RX_EARLY_P_A::PENDING
    }
}
#[doc = "Field `rx_early_p` writer - RX DMA Filled First Data Buffer of the Receive Frame Interrupt Pending"]
pub type RX_EARLY_P_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, EMAC_INT_STA_SPEC, RX_EARLY_P_A, O>;
impl<'a, const O: u8> RX_EARLY_P_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(RX_EARLY_P_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(RX_EARLY_P_A::PENDING)
    }
}
#[doc = "Field `rgmii_link_sta_p` reader - RMII Link Status Changed Interrupt Pending"]
pub type RGMII_LINK_STA_P_R = crate::BitReader<RGMII_LINK_STA_P_A>;
#[doc = "RMII Link Status Changed Interrupt Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGMII_LINK_STA_P_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<RGMII_LINK_STA_P_A> for bool {
    #[inline(always)]
    fn from(variant: RGMII_LINK_STA_P_A) -> Self {
        variant as u8 != 0
    }
}
impl RGMII_LINK_STA_P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGMII_LINK_STA_P_A {
        match self.bits {
            false => RGMII_LINK_STA_P_A::NO_PENDING,
            true => RGMII_LINK_STA_P_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == RGMII_LINK_STA_P_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RGMII_LINK_STA_P_A::PENDING
    }
}
#[doc = "Field `rgmii_link_sta_p` writer - RMII Link Status Changed Interrupt Pending"]
pub type RGMII_LINK_STA_P_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, EMAC_INT_STA_SPEC, RGMII_LINK_STA_P_A, O>;
impl<'a, const O: u8> RGMII_LINK_STA_P_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(RGMII_LINK_STA_P_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(RGMII_LINK_STA_P_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 0 - Frame Transmission Interrupt Pending"]
    #[inline(always)]
    pub fn tx_p(&self) -> TX_P_R {
        TX_P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission DMA Stopped Interrupt Pending"]
    #[inline(always)]
    pub fn tx_dma_stopped_p(&self) -> TX_DMA_STOPPED_P_R {
        TX_DMA_STOPPED_P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX Buffer UA Interrupt Pending"]
    #[inline(always)]
    pub fn tx_buf_ua_p(&self) -> TX_BUF_UA_P_R {
        TX_BUF_UA_P_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Timeout Interrupt Pending"]
    #[inline(always)]
    pub fn tx_timeout_p(&self) -> TX_TIMEOUT_P_R {
        TX_TIMEOUT_P_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO Underflow Interrupt Pending"]
    #[inline(always)]
    pub fn tx_underflow_p(&self) -> TX_UNDERFLOW_P_R {
        TX_UNDERFLOW_P_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Total interrupt pending which the frame is transmitted to FIFO"]
    #[inline(always)]
    pub fn tx_early_p(&self) -> TX_EARLY_P_R {
        TX_EARLY_P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Frame RX Completed Interrupt Pending"]
    #[inline(always)]
    pub fn rx_p(&self) -> RX_P_R {
        RX_P_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RX Buffer UA Interrupt Pending"]
    #[inline(always)]
    pub fn rx_buf_ua_p(&self) -> RX_BUF_UA_P_R {
        RX_BUF_UA_P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When this bit asserted, the RX DMA FSM is stopped."]
    #[inline(always)]
    pub fn rx_dma_stopped_p(&self) -> RX_DMA_STOPPED_P_R {
        RX_DMA_STOPPED_P_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RX Timeout Interrupt Pending"]
    #[inline(always)]
    pub fn rx_timeout_p(&self) -> RX_TIMEOUT_P_R {
        RX_TIMEOUT_P_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RX FIFO Overflow Error Interrupt Pending"]
    #[inline(always)]
    pub fn rx_overflow_p(&self) -> RX_OVERFLOW_P_R {
        RX_OVERFLOW_P_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RX DMA Filled First Data Buffer of the Receive Frame Interrupt Pending"]
    #[inline(always)]
    pub fn rx_early_p(&self) -> RX_EARLY_P_R {
        RX_EARLY_P_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - RMII Link Status Changed Interrupt Pending"]
    #[inline(always)]
    pub fn rgmii_link_sta_p(&self) -> RGMII_LINK_STA_P_R {
        RGMII_LINK_STA_P_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame Transmission Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn tx_p(&mut self) -> TX_P_W<0> {
        TX_P_W::new(self)
    }
    #[doc = "Bit 1 - Transmission DMA Stopped Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dma_stopped_p(&mut self) -> TX_DMA_STOPPED_P_W<1> {
        TX_DMA_STOPPED_P_W::new(self)
    }
    #[doc = "Bit 2 - TX Buffer UA Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn tx_buf_ua_p(&mut self) -> TX_BUF_UA_P_W<2> {
        TX_BUF_UA_P_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter Timeout Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn tx_timeout_p(&mut self) -> TX_TIMEOUT_P_W<3> {
        TX_TIMEOUT_P_W::new(self)
    }
    #[doc = "Bit 4 - TX FIFO Underflow Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn tx_underflow_p(&mut self) -> TX_UNDERFLOW_P_W<4> {
        TX_UNDERFLOW_P_W::new(self)
    }
    #[doc = "Bit 5 - Total interrupt pending which the frame is transmitted to FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn tx_early_p(&mut self) -> TX_EARLY_P_W<5> {
        TX_EARLY_P_W::new(self)
    }
    #[doc = "Bit 8 - Frame RX Completed Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn rx_p(&mut self) -> RX_P_W<8> {
        RX_P_W::new(self)
    }
    #[doc = "Bit 9 - RX Buffer UA Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn rx_buf_ua_p(&mut self) -> RX_BUF_UA_P_W<9> {
        RX_BUF_UA_P_W::new(self)
    }
    #[doc = "Bit 10 - When this bit asserted, the RX DMA FSM is stopped."]
    #[inline(always)]
    #[must_use]
    pub fn rx_dma_stopped_p(&mut self) -> RX_DMA_STOPPED_P_W<10> {
        RX_DMA_STOPPED_P_W::new(self)
    }
    #[doc = "Bit 11 - RX Timeout Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn rx_timeout_p(&mut self) -> RX_TIMEOUT_P_W<11> {
        RX_TIMEOUT_P_W::new(self)
    }
    #[doc = "Bit 12 - RX FIFO Overflow Error Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn rx_overflow_p(&mut self) -> RX_OVERFLOW_P_W<12> {
        RX_OVERFLOW_P_W::new(self)
    }
    #[doc = "Bit 13 - RX DMA Filled First Data Buffer of the Receive Frame Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn rx_early_p(&mut self) -> RX_EARLY_P_W<13> {
        RX_EARLY_P_W::new(self)
    }
    #[doc = "Bit 16 - RMII Link Status Changed Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn rgmii_link_sta_p(&mut self) -> RGMII_LINK_STA_P_W<16> {
        RGMII_LINK_STA_P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_int_sta](index.html) module"]
pub struct EMAC_INT_STA_SPEC;
impl crate::RegisterSpec for EMAC_INT_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_int_sta::R](R) reader structure"]
impl crate::Readable for EMAC_INT_STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_int_sta::W](W) writer structure"]
impl crate::Writable for EMAC_INT_STA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0001_3f3f;
}
#[doc = "`reset()` method sets emac_int_sta to value 0"]
impl crate::Resettable for EMAC_INT_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
