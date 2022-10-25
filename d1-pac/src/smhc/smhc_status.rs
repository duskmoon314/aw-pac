#[doc = "Register `smhc_status` reader"]
pub struct R(crate::R<SMHC_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `fifo_rx_level` reader - FIFO RX Water Level Flag"]
pub type FIFO_RX_LEVEL_R = crate::BitReader<FIFO_RX_LEVEL_A>;
#[doc = "FIFO RX Water Level Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_RX_LEVEL_A {
    #[doc = "0: FIFO does not reach the receive trigger level"]
    NOT_REACH = 0,
    #[doc = "1: FIFO reaches the receive trigger level"]
    REACH = 1,
}
impl From<FIFO_RX_LEVEL_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_RX_LEVEL_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_RX_LEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_RX_LEVEL_A {
        match self.bits {
            false => FIFO_RX_LEVEL_A::NOT_REACH,
            true => FIFO_RX_LEVEL_A::REACH,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_REACH`"]
    #[inline(always)]
    pub fn is_not_reach(&self) -> bool {
        *self == FIFO_RX_LEVEL_A::NOT_REACH
    }
    #[doc = "Checks if the value of the field is `REACH`"]
    #[inline(always)]
    pub fn is_reach(&self) -> bool {
        *self == FIFO_RX_LEVEL_A::REACH
    }
}
#[doc = "Field `fifo_tx_level` reader - FIFO TX Water Level Flag"]
pub type FIFO_TX_LEVEL_R = crate::BitReader<FIFO_TX_LEVEL_A>;
#[doc = "FIFO TX Water Level Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_TX_LEVEL_A {
    #[doc = "0: FIFO does not reach the transmit trigger level"]
    NOT_REACH = 0,
    #[doc = "1: FIFO reaches the transmit trigger level"]
    REACH = 1,
}
impl From<FIFO_TX_LEVEL_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_TX_LEVEL_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_TX_LEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_TX_LEVEL_A {
        match self.bits {
            false => FIFO_TX_LEVEL_A::NOT_REACH,
            true => FIFO_TX_LEVEL_A::REACH,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_REACH`"]
    #[inline(always)]
    pub fn is_not_reach(&self) -> bool {
        *self == FIFO_TX_LEVEL_A::NOT_REACH
    }
    #[doc = "Checks if the value of the field is `REACH`"]
    #[inline(always)]
    pub fn is_reach(&self) -> bool {
        *self == FIFO_TX_LEVEL_A::REACH
    }
}
#[doc = "Field `fifo_empty` reader - FIFO Empty"]
pub type FIFO_EMPTY_R = crate::BitReader<FIFO_EMPTY_A>;
#[doc = "FIFO Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_EMPTY_A {
    #[doc = "0: FIFO is not empty"]
    NOT_SEMPTY = 0,
    #[doc = "1: FIFO is empty"]
    EMPTY = 1,
}
impl From<FIFO_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_EMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_EMPTY_A {
        match self.bits {
            false => FIFO_EMPTY_A::NOT_SEMPTY,
            true => FIFO_EMPTY_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_SEMPTY`"]
    #[inline(always)]
    pub fn is_not_sempty(&self) -> bool {
        *self == FIFO_EMPTY_A::NOT_SEMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FIFO_EMPTY_A::EMPTY
    }
}
#[doc = "Field `fifo_full` reader - sFIFO Full"]
pub type FIFO_FULL_R = crate::BitReader<FIFO_FULL_A>;
#[doc = "sFIFO Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_FULL_A {
    #[doc = "0: FIFO is not full"]
    NOT_FULL = 0,
    #[doc = "1: FIFO is full"]
    FULL = 1,
}
impl From<FIFO_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_FULL_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_FULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_FULL_A {
        match self.bits {
            false => FIFO_FULL_A::NOT_FULL,
            true => FIFO_FULL_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == FIFO_FULL_A::NOT_FULL
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FIFO_FULL_A::FULL
    }
}
#[doc = "Field `fsm_sta` reader - Command FSM States"]
pub type FSM_STA_R = crate::FieldReader<u8, FSM_STA_A>;
#[doc = "Command FSM States\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSM_STA_A {
    #[doc = "0: Idle"]
    IDLE = 0,
    #[doc = "1: Send init sequence"]
    SIS = 1,
    #[doc = "2: TX CMD start bit"]
    TXCSB = 2,
    #[doc = "3: TX CMD TX bit"]
    TXCTB = 3,
    #[doc = "4: TX CMD index + argument"]
    TXCIA = 4,
    #[doc = "5: TX CMD CRC7"]
    TXCC = 5,
    #[doc = "6: TX CMD end bit"]
    TXCEB = 6,
    #[doc = "7: RX response start bit"]
    RXRSB = 7,
    #[doc = "8: RX response IRQ responses"]
    RXRIR = 8,
    #[doc = "9: RX response TX bit"]
    RXRTB = 9,
    #[doc = "10: RX response CMD index"]
    RXRCI = 10,
    #[doc = "11: RX response data"]
    RXRD = 11,
    #[doc = "12: RX response CRC7"]
    RXRC = 12,
    #[doc = "13: RX response end bit"]
    RXREB = 13,
    #[doc = "14: CMD path wait NCC"]
    CPWN = 14,
    #[doc = "15: Wait; CMD-to-response turn around"]
    WAIT = 15,
}
impl From<FSM_STA_A> for u8 {
    #[inline(always)]
    fn from(variant: FSM_STA_A) -> Self {
        variant as _
    }
}
impl FSM_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSM_STA_A {
        match self.bits {
            0 => FSM_STA_A::IDLE,
            1 => FSM_STA_A::SIS,
            2 => FSM_STA_A::TXCSB,
            3 => FSM_STA_A::TXCTB,
            4 => FSM_STA_A::TXCIA,
            5 => FSM_STA_A::TXCC,
            6 => FSM_STA_A::TXCEB,
            7 => FSM_STA_A::RXRSB,
            8 => FSM_STA_A::RXRIR,
            9 => FSM_STA_A::RXRTB,
            10 => FSM_STA_A::RXRCI,
            11 => FSM_STA_A::RXRD,
            12 => FSM_STA_A::RXRC,
            13 => FSM_STA_A::RXREB,
            14 => FSM_STA_A::CPWN,
            15 => FSM_STA_A::WAIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == FSM_STA_A::IDLE
    }
    #[doc = "Checks if the value of the field is `SIS`"]
    #[inline(always)]
    pub fn is_sis(&self) -> bool {
        *self == FSM_STA_A::SIS
    }
    #[doc = "Checks if the value of the field is `TXCSB`"]
    #[inline(always)]
    pub fn is_txcsb(&self) -> bool {
        *self == FSM_STA_A::TXCSB
    }
    #[doc = "Checks if the value of the field is `TXCTB`"]
    #[inline(always)]
    pub fn is_txctb(&self) -> bool {
        *self == FSM_STA_A::TXCTB
    }
    #[doc = "Checks if the value of the field is `TXCIA`"]
    #[inline(always)]
    pub fn is_txcia(&self) -> bool {
        *self == FSM_STA_A::TXCIA
    }
    #[doc = "Checks if the value of the field is `TXCC`"]
    #[inline(always)]
    pub fn is_txcc(&self) -> bool {
        *self == FSM_STA_A::TXCC
    }
    #[doc = "Checks if the value of the field is `TXCEB`"]
    #[inline(always)]
    pub fn is_txceb(&self) -> bool {
        *self == FSM_STA_A::TXCEB
    }
    #[doc = "Checks if the value of the field is `RXRSB`"]
    #[inline(always)]
    pub fn is_rxrsb(&self) -> bool {
        *self == FSM_STA_A::RXRSB
    }
    #[doc = "Checks if the value of the field is `RXRIR`"]
    #[inline(always)]
    pub fn is_rxrir(&self) -> bool {
        *self == FSM_STA_A::RXRIR
    }
    #[doc = "Checks if the value of the field is `RXRTB`"]
    #[inline(always)]
    pub fn is_rxrtb(&self) -> bool {
        *self == FSM_STA_A::RXRTB
    }
    #[doc = "Checks if the value of the field is `RXRCI`"]
    #[inline(always)]
    pub fn is_rxrci(&self) -> bool {
        *self == FSM_STA_A::RXRCI
    }
    #[doc = "Checks if the value of the field is `RXRD`"]
    #[inline(always)]
    pub fn is_rxrd(&self) -> bool {
        *self == FSM_STA_A::RXRD
    }
    #[doc = "Checks if the value of the field is `RXRC`"]
    #[inline(always)]
    pub fn is_rxrc(&self) -> bool {
        *self == FSM_STA_A::RXRC
    }
    #[doc = "Checks if the value of the field is `RXREB`"]
    #[inline(always)]
    pub fn is_rxreb(&self) -> bool {
        *self == FSM_STA_A::RXREB
    }
    #[doc = "Checks if the value of the field is `CPWN`"]
    #[inline(always)]
    pub fn is_cpwn(&self) -> bool {
        *self == FSM_STA_A::CPWN
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == FSM_STA_A::WAIT
    }
}
#[doc = "Field `card_present` reader - Data\\[3\\] Statuss"]
pub type CARD_PRESENT_R = crate::BitReader<CARD_PRESENT_A>;
#[doc = "Data\\[3\\] Statuss\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARD_PRESENT_A {
    #[doc = "0: The card is not present"]
    NOT_PRESENT = 0,
    #[doc = "1: The card is present"]
    PRESENT = 1,
}
impl From<CARD_PRESENT_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_PRESENT_A) -> Self {
        variant as u8 != 0
    }
}
impl CARD_PRESENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARD_PRESENT_A {
        match self.bits {
            false => CARD_PRESENT_A::NOT_PRESENT,
            true => CARD_PRESENT_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == CARD_PRESENT_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == CARD_PRESENT_A::PRESENT
    }
}
#[doc = "Field `card_busy` reader - Card Data Busy"]
pub type CARD_BUSY_R = crate::BitReader<CARD_BUSY_A>;
#[doc = "Card Data Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARD_BUSY_A {
    #[doc = "0: Card data is not busy"]
    NOT_BUSY = 0,
    #[doc = "1: Card data is busy"]
    BUSY = 1,
}
impl From<CARD_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl CARD_BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARD_BUSY_A {
        match self.bits {
            false => CARD_BUSY_A::NOT_BUSY,
            true => CARD_BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == CARD_BUSY_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == CARD_BUSY_A::BUSY
    }
}
#[doc = "Field `fsm_busy` reader - Data FSM Busy"]
pub type FSM_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `resp_idx` reader - Response Index"]
pub type RESP_IDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fifo_level` reader - FIFO Level"]
pub type FIFO_LEVEL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `dma_req` reader - DMA Request"]
pub type DMA_REQ_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - FIFO RX Water Level Flag"]
    #[inline(always)]
    pub fn fifo_rx_level(&self) -> FIFO_RX_LEVEL_R {
        FIFO_RX_LEVEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO TX Water Level Flag"]
    #[inline(always)]
    pub fn fifo_tx_level(&self) -> FIFO_TX_LEVEL_R {
        FIFO_TX_LEVEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO Empty"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sFIFO Full"]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Command FSM States"]
    #[inline(always)]
    pub fn fsm_sta(&self) -> FSM_STA_R {
        FSM_STA_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Data\\[3\\] Statuss"]
    #[inline(always)]
    pub fn card_present(&self) -> CARD_PRESENT_R {
        CARD_PRESENT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Card Data Busy"]
    #[inline(always)]
    pub fn card_busy(&self) -> CARD_BUSY_R {
        CARD_BUSY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data FSM Busy"]
    #[inline(always)]
    pub fn fsm_busy(&self) -> FSM_BUSY_R {
        FSM_BUSY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:16 - Response Index"]
    #[inline(always)]
    pub fn resp_idx(&self) -> RESP_IDX_R {
        RESP_IDX_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:25 - FIFO Level"]
    #[inline(always)]
    pub fn fifo_level(&self) -> FIFO_LEVEL_R {
        FIFO_LEVEL_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - DMA Request"]
    #[inline(always)]
    pub fn dma_req(&self) -> DMA_REQ_R {
        DMA_REQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_status](index.html) module"]
pub struct SMHC_STATUS_SPEC;
impl crate::RegisterSpec for SMHC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_status::R](R) reader structure"]
impl crate::Readable for SMHC_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets smhc_status to value 0"]
impl crate::Resettable for SMHC_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
