#[doc = "Register `emac_int_en` reader"]
pub type R = crate::R<EMAC_INT_EN_SPEC>;
#[doc = "Register `emac_int_en` writer"]
pub type W = crate::W<EMAC_INT_EN_SPEC>;
#[doc = "Field `tx_int_en` reader - Transmit Interrupt"]
pub type TX_INT_EN_R = crate::BitReader<TX_INT_EN_A>;
#[doc = "Transmit Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_INT_EN_A {
        match self.bits {
            false => TX_INT_EN_A::DISABLE,
            true => TX_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX_INT_EN_A::ENABLE
    }
}
#[doc = "Field `tx_int_en` writer - Transmit Interrupt"]
pub type TX_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, TX_INT_EN_A>;
impl<'a, REG> TX_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `tx_dma_stopped_int_en` reader - Transmit DMA FSM Stopped Interrupt"]
pub type TX_DMA_STOPPED_INT_EN_R = crate::BitReader<TX_DMA_STOPPED_INT_EN_A>;
#[doc = "Transmit DMA FSM Stopped Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_DMA_STOPPED_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_DMA_STOPPED_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_DMA_STOPPED_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_DMA_STOPPED_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_DMA_STOPPED_INT_EN_A {
        match self.bits {
            false => TX_DMA_STOPPED_INT_EN_A::DISABLE,
            true => TX_DMA_STOPPED_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX_DMA_STOPPED_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX_DMA_STOPPED_INT_EN_A::ENABLE
    }
}
#[doc = "Field `tx_dma_stopped_int_en` writer - Transmit DMA FSM Stopped Interrupt"]
pub type TX_DMA_STOPPED_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, TX_DMA_STOPPED_INT_EN_A>;
impl<'a, REG> TX_DMA_STOPPED_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_DMA_STOPPED_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_DMA_STOPPED_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `tx_buf_ua_int_en` reader - Transmit Buffer Available Interrupt"]
pub type TX_BUF_UA_INT_EN_R = crate::BitReader<TX_BUF_UA_INT_EN_A>;
#[doc = "Transmit Buffer Available Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_BUF_UA_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_BUF_UA_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_BUF_UA_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_BUF_UA_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_BUF_UA_INT_EN_A {
        match self.bits {
            false => TX_BUF_UA_INT_EN_A::DISABLE,
            true => TX_BUF_UA_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX_BUF_UA_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX_BUF_UA_INT_EN_A::ENABLE
    }
}
#[doc = "Field `tx_buf_ua_int_en` writer - Transmit Buffer Available Interrupt"]
pub type TX_BUF_UA_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, TX_BUF_UA_INT_EN_A>;
impl<'a, REG> TX_BUF_UA_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_BUF_UA_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_BUF_UA_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `tx_timeout_int_en` reader - Transmit Timeout Interrupt"]
pub type TX_TIMEOUT_INT_EN_R = crate::BitReader<TX_TIMEOUT_INT_EN_A>;
#[doc = "Transmit Timeout Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_TIMEOUT_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_TIMEOUT_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_TIMEOUT_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_TIMEOUT_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_TIMEOUT_INT_EN_A {
        match self.bits {
            false => TX_TIMEOUT_INT_EN_A::DISABLE,
            true => TX_TIMEOUT_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX_TIMEOUT_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX_TIMEOUT_INT_EN_A::ENABLE
    }
}
#[doc = "Field `tx_timeout_int_en` writer - Transmit Timeout Interrupt"]
pub type TX_TIMEOUT_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, TX_TIMEOUT_INT_EN_A>;
impl<'a, REG> TX_TIMEOUT_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_TIMEOUT_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_TIMEOUT_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `tx_underflow_int_en` reader - Transmit Underflow Interrupt"]
pub type TX_UNDERFLOW_INT_EN_R = crate::BitReader<TX_UNDERFLOW_INT_EN_A>;
#[doc = "Transmit Underflow Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_UNDERFLOW_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_UNDERFLOW_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_UNDERFLOW_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_UNDERFLOW_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_UNDERFLOW_INT_EN_A {
        match self.bits {
            false => TX_UNDERFLOW_INT_EN_A::DISABLE,
            true => TX_UNDERFLOW_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX_UNDERFLOW_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX_UNDERFLOW_INT_EN_A::ENABLE
    }
}
#[doc = "Field `tx_underflow_int_en` writer - Transmit Underflow Interrupt"]
pub type TX_UNDERFLOW_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, TX_UNDERFLOW_INT_EN_A>;
impl<'a, REG> TX_UNDERFLOW_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_UNDERFLOW_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_UNDERFLOW_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `tx_early_int_en` reader - Early Transmit Interrupt"]
pub type TX_EARLY_INT_EN_R = crate::BitReader<TX_EARLY_INT_EN_A>;
#[doc = "Early Transmit Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_EARLY_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_EARLY_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_EARLY_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_EARLY_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_EARLY_INT_EN_A {
        match self.bits {
            false => TX_EARLY_INT_EN_A::DISABLE,
            true => TX_EARLY_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX_EARLY_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX_EARLY_INT_EN_A::ENABLE
    }
}
#[doc = "Field `tx_early_int_en` writer - Early Transmit Interrupt"]
pub type TX_EARLY_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, TX_EARLY_INT_EN_A>;
impl<'a, REG> TX_EARLY_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_EARLY_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_EARLY_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `rx_int_en` reader - Receive Interrupt"]
pub type RX_INT_EN_R = crate::BitReader<RX_INT_EN_A>;
#[doc = "Receive Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_INT_EN_A {
        match self.bits {
            false => RX_INT_EN_A::DISABLE,
            true => RX_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_INT_EN_A::ENABLE
    }
}
#[doc = "Field `rx_int_en` writer - Receive Interrupt"]
pub type RX_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, RX_INT_EN_A>;
impl<'a, REG> RX_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `rx_buf_ua_int_en` reader - Receive Buffer Unavailable Interrupt"]
pub type RX_BUF_UA_INT_EN_R = crate::BitReader<RX_BUF_UA_INT_EN_A>;
#[doc = "Receive Buffer Unavailable Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_BUF_UA_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_BUF_UA_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_BUF_UA_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_BUF_UA_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_BUF_UA_INT_EN_A {
        match self.bits {
            false => RX_BUF_UA_INT_EN_A::DISABLE,
            true => RX_BUF_UA_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_BUF_UA_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_BUF_UA_INT_EN_A::ENABLE
    }
}
#[doc = "Field `rx_buf_ua_int_en` writer - Receive Buffer Unavailable Interrupt"]
pub type RX_BUF_UA_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, RX_BUF_UA_INT_EN_A>;
impl<'a, REG> RX_BUF_UA_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_BUF_UA_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_BUF_UA_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `rx_dma_stopped_int_en` reader - Receive DMA FSM Stopped Interrupt"]
pub type RX_DMA_STOPPED_INT_EN_R = crate::BitReader<RX_DMA_STOPPED_INT_EN_A>;
#[doc = "Receive DMA FSM Stopped Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_DMA_STOPPED_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_DMA_STOPPED_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_DMA_STOPPED_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_DMA_STOPPED_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_DMA_STOPPED_INT_EN_A {
        match self.bits {
            false => RX_DMA_STOPPED_INT_EN_A::DISABLE,
            true => RX_DMA_STOPPED_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_DMA_STOPPED_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_DMA_STOPPED_INT_EN_A::ENABLE
    }
}
#[doc = "Field `rx_dma_stopped_int_en` writer - Receive DMA FSM Stopped Interrupt"]
pub type RX_DMA_STOPPED_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, RX_DMA_STOPPED_INT_EN_A>;
impl<'a, REG> RX_DMA_STOPPED_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_DMA_STOPPED_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_DMA_STOPPED_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `rx_timeout_int_en` reader - Receive Timeout Interrupt"]
pub type RX_TIMEOUT_INT_EN_R = crate::BitReader<RX_TIMEOUT_INT_EN_A>;
#[doc = "Receive Timeout Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_TIMEOUT_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_TIMEOUT_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_TIMEOUT_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_TIMEOUT_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_TIMEOUT_INT_EN_A {
        match self.bits {
            false => RX_TIMEOUT_INT_EN_A::DISABLE,
            true => RX_TIMEOUT_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_TIMEOUT_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_TIMEOUT_INT_EN_A::ENABLE
    }
}
#[doc = "Field `rx_timeout_int_en` writer - Receive Timeout Interrupt"]
pub type RX_TIMEOUT_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, RX_TIMEOUT_INT_EN_A>;
impl<'a, REG> RX_TIMEOUT_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_TIMEOUT_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_TIMEOUT_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `rx_overflow_int_en` reader - Receive Overflow Interrupt"]
pub type RX_OVERFLOW_INT_EN_R = crate::BitReader<RX_OVERFLOW_INT_EN_A>;
#[doc = "Receive Overflow Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_OVERFLOW_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_OVERFLOW_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_OVERFLOW_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_OVERFLOW_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_OVERFLOW_INT_EN_A {
        match self.bits {
            false => RX_OVERFLOW_INT_EN_A::DISABLE,
            true => RX_OVERFLOW_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_OVERFLOW_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_OVERFLOW_INT_EN_A::ENABLE
    }
}
#[doc = "Field `rx_overflow_int_en` writer - Receive Overflow Interrupt"]
pub type RX_OVERFLOW_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, RX_OVERFLOW_INT_EN_A>;
impl<'a, REG> RX_OVERFLOW_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_OVERFLOW_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_OVERFLOW_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `rx_early_int_en` reader - Early Receive Interrupt"]
pub type RX_EARLY_INT_EN_R = crate::BitReader<RX_EARLY_INT_EN_A>;
#[doc = "Early Receive Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_EARLY_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_EARLY_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_EARLY_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_EARLY_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_EARLY_INT_EN_A {
        match self.bits {
            false => RX_EARLY_INT_EN_A::DISABLE,
            true => RX_EARLY_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_EARLY_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_EARLY_INT_EN_A::ENABLE
    }
}
#[doc = "Field `rx_early_int_en` writer - Early Receive Interrupt"]
pub type RX_EARLY_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, RX_EARLY_INT_EN_A>;
impl<'a, REG> RX_EARLY_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_EARLY_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_EARLY_INT_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn tx_int_en(&self) -> TX_INT_EN_R {
        TX_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA FSM Stopped Interrupt"]
    #[inline(always)]
    pub fn tx_dma_stopped_int_en(&self) -> TX_DMA_STOPPED_INT_EN_R {
        TX_DMA_STOPPED_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Available Interrupt"]
    #[inline(always)]
    pub fn tx_buf_ua_int_en(&self) -> TX_BUF_UA_INT_EN_R {
        TX_BUF_UA_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Timeout Interrupt"]
    #[inline(always)]
    pub fn tx_timeout_int_en(&self) -> TX_TIMEOUT_INT_EN_R {
        TX_TIMEOUT_INT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Underflow Interrupt"]
    #[inline(always)]
    pub fn tx_underflow_int_en(&self) -> TX_UNDERFLOW_INT_EN_R {
        TX_UNDERFLOW_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn tx_early_int_en(&self) -> TX_EARLY_INT_EN_R {
        TX_EARLY_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Interrupt"]
    #[inline(always)]
    pub fn rx_int_en(&self) -> RX_INT_EN_R {
        RX_INT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Buffer Unavailable Interrupt"]
    #[inline(always)]
    pub fn rx_buf_ua_int_en(&self) -> RX_BUF_UA_INT_EN_R {
        RX_BUF_UA_INT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive DMA FSM Stopped Interrupt"]
    #[inline(always)]
    pub fn rx_dma_stopped_int_en(&self) -> RX_DMA_STOPPED_INT_EN_R {
        RX_DMA_STOPPED_INT_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive Timeout Interrupt"]
    #[inline(always)]
    pub fn rx_timeout_int_en(&self) -> RX_TIMEOUT_INT_EN_R {
        RX_TIMEOUT_INT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Receive Overflow Interrupt"]
    #[inline(always)]
    pub fn rx_overflow_int_en(&self) -> RX_OVERFLOW_INT_EN_R {
        RX_OVERFLOW_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn rx_early_int_en(&self) -> RX_EARLY_INT_EN_R {
        RX_EARLY_INT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tx_int_en(&mut self) -> TX_INT_EN_W<EMAC_INT_EN_SPEC> {
        TX_INT_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit DMA FSM Stopped Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dma_stopped_int_en(&mut self) -> TX_DMA_STOPPED_INT_EN_W<EMAC_INT_EN_SPEC> {
        TX_DMA_STOPPED_INT_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Available Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tx_buf_ua_int_en(&mut self) -> TX_BUF_UA_INT_EN_W<EMAC_INT_EN_SPEC> {
        TX_BUF_UA_INT_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Timeout Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tx_timeout_int_en(&mut self) -> TX_TIMEOUT_INT_EN_W<EMAC_INT_EN_SPEC> {
        TX_TIMEOUT_INT_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit Underflow Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tx_underflow_int_en(&mut self) -> TX_UNDERFLOW_INT_EN_W<EMAC_INT_EN_SPEC> {
        TX_UNDERFLOW_INT_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Early Transmit Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tx_early_int_en(&mut self) -> TX_EARLY_INT_EN_W<EMAC_INT_EN_SPEC> {
        TX_EARLY_INT_EN_W::new(self, 5)
    }
    #[doc = "Bit 8 - Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_int_en(&mut self) -> RX_INT_EN_W<EMAC_INT_EN_SPEC> {
        RX_INT_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Buffer Unavailable Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_buf_ua_int_en(&mut self) -> RX_BUF_UA_INT_EN_W<EMAC_INT_EN_SPEC> {
        RX_BUF_UA_INT_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Receive DMA FSM Stopped Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dma_stopped_int_en(&mut self) -> RX_DMA_STOPPED_INT_EN_W<EMAC_INT_EN_SPEC> {
        RX_DMA_STOPPED_INT_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Receive Timeout Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_timeout_int_en(&mut self) -> RX_TIMEOUT_INT_EN_W<EMAC_INT_EN_SPEC> {
        RX_TIMEOUT_INT_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Receive Overflow Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_overflow_int_en(&mut self) -> RX_OVERFLOW_INT_EN_W<EMAC_INT_EN_SPEC> {
        RX_OVERFLOW_INT_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Early Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_early_int_en(&mut self) -> RX_EARLY_INT_EN_W<EMAC_INT_EN_SPEC> {
        RX_EARLY_INT_EN_W::new(self, 13)
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
#[doc = "EMAC Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMAC_INT_EN_SPEC;
impl crate::RegisterSpec for EMAC_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emac_int_en::R`](R) reader structure"]
impl crate::Readable for EMAC_INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emac_int_en::W`](W) writer structure"]
impl crate::Writable for EMAC_INT_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_int_en to value 0"]
impl crate::Resettable for EMAC_INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
