#[doc = "Register `cir_rxsta` reader"]
pub struct R(crate::R<CIR_RXSTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_RXSTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_RXSTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_RXSTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cir_rxsta` writer"]
pub struct W(crate::W<CIR_RXSTA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_RXSTA_SPEC>;
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
impl From<crate::W<CIR_RXSTA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_RXSTA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `roi` reader - Receiver FIFO Overrun"]
pub type ROI_R = crate::BitReader<ROI_A>;
#[doc = "Receiver FIFO Overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROI_A {
    #[doc = "0: Receiver FIFO not overrun"]
    NOT_OVERRUN = 0,
    #[doc = "1: Receiver FIFO overrun"]
    OVERRUN = 1,
}
impl From<ROI_A> for bool {
    #[inline(always)]
    fn from(variant: ROI_A) -> Self {
        variant as u8 != 0
    }
}
impl ROI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROI_A {
        match self.bits {
            false => ROI_A::NOT_OVERRUN,
            true => ROI_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_OVERRUN`"]
    #[inline(always)]
    pub fn is_not_overrun(&self) -> bool {
        *self == ROI_A::NOT_OVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == ROI_A::OVERRUN
    }
}
#[doc = "Field `roi` writer - Receiver FIFO Overrun"]
pub type ROI_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CIR_RXSTA_SPEC, ROI_A, O>;
impl<'a, const O: u8> ROI_W<'a, O> {
    #[doc = "Receiver FIFO not overrun"]
    #[inline(always)]
    pub fn not_overrun(self) -> &'a mut W {
        self.variant(ROI_A::NOT_OVERRUN)
    }
    #[doc = "Receiver FIFO overrun"]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut W {
        self.variant(ROI_A::OVERRUN)
    }
}
#[doc = "Field `rpe` reader - Receiver Packet End Flag"]
pub type RPE_R = crate::BitReader<RPE_A>;
#[doc = "Receiver Packet End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPE_A {
    #[doc = "0: STO was not detected. In CIR mode, one CIR symbol is receiving or not detected."]
    NOT_DETECTED = 0,
    #[doc = "1: STO field or packet abort symbol (7'b0000,000 and 8'b0000,0000 for MIR and FIR) is detected. In CIR mode, one CIR symbol is received."]
    STO_FIELD = 1,
}
impl From<RPE_A> for bool {
    #[inline(always)]
    fn from(variant: RPE_A) -> Self {
        variant as u8 != 0
    }
}
impl RPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPE_A {
        match self.bits {
            false => RPE_A::NOT_DETECTED,
            true => RPE_A::STO_FIELD,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == RPE_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `STO_FIELD`"]
    #[inline(always)]
    pub fn is_sto_field(&self) -> bool {
        *self == RPE_A::STO_FIELD
    }
}
#[doc = "Field `rpe` writer - Receiver Packet End Flag"]
pub type RPE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CIR_RXSTA_SPEC, RPE_A, O>;
impl<'a, const O: u8> RPE_W<'a, O> {
    #[doc = "STO was not detected. In CIR mode, one CIR symbol is receiving or not detected."]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(RPE_A::NOT_DETECTED)
    }
    #[doc = "STO field or packet abort symbol (7'b0000,000 and 8'b0000,0000 for MIR and FIR) is detected. In CIR mode, one CIR symbol is received."]
    #[inline(always)]
    pub fn sto_field(self) -> &'a mut W {
        self.variant(RPE_A::STO_FIELD)
    }
}
#[doc = "Field `ra` reader - RX FIFO Available"]
pub type RA_R = crate::BitReader<RA_A>;
#[doc = "RX FIFO Available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RA_A {
    #[doc = "0: RX FIFO not available according to its level"]
    NO_AVAILABLE = 0,
    #[doc = "1: RX FIFO available according to its level Writing 1 clears this bit."]
    AVAILABLE = 1,
}
impl From<RA_A> for bool {
    #[inline(always)]
    fn from(variant: RA_A) -> Self {
        variant as u8 != 0
    }
}
impl RA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RA_A {
        match self.bits {
            false => RA_A::NO_AVAILABLE,
            true => RA_A::AVAILABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_AVAILABLE`"]
    #[inline(always)]
    pub fn is_no_available(&self) -> bool {
        *self == RA_A::NO_AVAILABLE
    }
    #[doc = "Checks if the value of the field is `AVAILABLE`"]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == RA_A::AVAILABLE
    }
}
#[doc = "Field `ra` writer - RX FIFO Available"]
pub type RA_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CIR_RXSTA_SPEC, RA_A, O>;
impl<'a, const O: u8> RA_W<'a, O> {
    #[doc = "RX FIFO not available according to its level"]
    #[inline(always)]
    pub fn no_available(self) -> &'a mut W {
        self.variant(RA_A::NO_AVAILABLE)
    }
    #[doc = "RX FIFO available according to its level Writing 1 clears this bit."]
    #[inline(always)]
    pub fn available(self) -> &'a mut W {
        self.variant(RA_A::AVAILABLE)
    }
}
#[doc = "Field `stat` reader - Status of CIR"]
pub type STAT_R = crate::BitReader<STAT_A>;
#[doc = "Status of CIR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_A {
    #[doc = "0: Idle"]
    IDLE = 0,
    #[doc = "1: Busy"]
    BUSY = 1,
}
impl From<STAT_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_A {
        match self.bits {
            false => STAT_A::IDLE,
            true => STAT_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STAT_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == STAT_A::BUSY
    }
}
#[doc = "Field `rac` reader - RX FIFO Available Counter"]
pub type RAC_R = crate::FieldReader<u8, RAC_A>;
#[doc = "RX FIFO Available Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAC_A {
    #[doc = "0: No available data in RX FIFO"]
    B0 = 0,
    #[doc = "1: 1 byte available in RX FIFO"]
    B1 = 1,
    #[doc = "2: 2 bytes available in RX FIFO"]
    B2 = 2,
    #[doc = "3: 3 bytes available in RX FIFO"]
    B3 = 3,
    #[doc = "4: 4 bytes available in RX FIFO"]
    B4 = 4,
    #[doc = "5: 5 bytes available in RX FIFO"]
    B5 = 5,
    #[doc = "6: 6 bytes available in RX FIFO"]
    B6 = 6,
    #[doc = "7: 7 bytes available in RX FIFO"]
    B7 = 7,
    #[doc = "8: 8 bytes available in RX FIFO"]
    B8 = 8,
    #[doc = "9: 9 bytes available in RX FIFO"]
    B9 = 9,
    #[doc = "10: 10 bytes available in RX FIFO"]
    B10 = 10,
    #[doc = "11: 11 bytes available in RX FIFO"]
    B11 = 11,
    #[doc = "12: 12 bytes available in RX FIFO"]
    B12 = 12,
    #[doc = "13: 13 bytes available in RX FIFO"]
    B13 = 13,
    #[doc = "14: 14 bytes available in RX FIFO"]
    B14 = 14,
    #[doc = "15: 15 bytes available in RX FIFO"]
    B15 = 15,
    #[doc = "16: 16 bytes available in RX FIFO"]
    B16 = 16,
    #[doc = "17: 17 bytes available in RX FIFO"]
    B17 = 17,
    #[doc = "18: 18 bytes available in RX FIFO"]
    B18 = 18,
    #[doc = "19: 19 bytes available in RX FIFO"]
    B19 = 19,
    #[doc = "20: 20 bytes available in RX FIFO"]
    B20 = 20,
    #[doc = "21: 21 bytes available in RX FIFO"]
    B21 = 21,
    #[doc = "22: 22 bytes available in RX FIFO"]
    B22 = 22,
    #[doc = "23: 23 bytes available in RX FIFO"]
    B23 = 23,
    #[doc = "24: 24 bytes available in RX FIFO"]
    B24 = 24,
    #[doc = "25: 25 bytes available in RX FIFO"]
    B25 = 25,
    #[doc = "26: 26 bytes available in RX FIFO"]
    B26 = 26,
    #[doc = "27: 27 bytes available in RX FIFO"]
    B27 = 27,
    #[doc = "28: 28 bytes available in RX FIFO"]
    B28 = 28,
    #[doc = "29: 29 bytes available in RX FIFO"]
    B29 = 29,
    #[doc = "30: 30 bytes available in RX FIFO"]
    B30 = 30,
    #[doc = "31: 31 bytes available in RX FIFO"]
    B31 = 31,
    #[doc = "32: 32 bytes available in RX FIFO"]
    B32 = 32,
    #[doc = "33: 33 bytes available in RX FIFO"]
    B33 = 33,
    #[doc = "34: 34 bytes available in RX FIFO"]
    B34 = 34,
    #[doc = "35: 35 bytes available in RX FIFO"]
    B35 = 35,
    #[doc = "36: 36 bytes available in RX FIFO"]
    B36 = 36,
    #[doc = "37: 37 bytes available in RX FIFO"]
    B37 = 37,
    #[doc = "38: 38 bytes available in RX FIFO"]
    B38 = 38,
    #[doc = "39: 39 bytes available in RX FIFO"]
    B39 = 39,
    #[doc = "40: 40 bytes available in RX FIFO"]
    B40 = 40,
    #[doc = "41: 41 bytes available in RX FIFO"]
    B41 = 41,
    #[doc = "42: 42 bytes available in RX FIFO"]
    B42 = 42,
    #[doc = "43: 43 bytes available in RX FIFO"]
    B43 = 43,
    #[doc = "44: 44 bytes available in RX FIFO"]
    B44 = 44,
    #[doc = "45: 45 bytes available in RX FIFO"]
    B45 = 45,
    #[doc = "46: 46 bytes available in RX FIFO"]
    B46 = 46,
    #[doc = "47: 47 bytes available in RX FIFO"]
    B47 = 47,
    #[doc = "48: 48 bytes available in RX FIFO"]
    B48 = 48,
    #[doc = "49: 49 bytes available in RX FIFO"]
    B49 = 49,
    #[doc = "50: 50 bytes available in RX FIFO"]
    B50 = 50,
    #[doc = "51: 51 bytes available in RX FIFO"]
    B51 = 51,
    #[doc = "52: 52 bytes available in RX FIFO"]
    B52 = 52,
    #[doc = "53: 53 bytes available in RX FIFO"]
    B53 = 53,
    #[doc = "54: 54 bytes available in RX FIFO"]
    B54 = 54,
    #[doc = "55: 55 bytes available in RX FIFO"]
    B55 = 55,
    #[doc = "56: 56 bytes available in RX FIFO"]
    B56 = 56,
    #[doc = "57: 57 bytes available in RX FIFO"]
    B57 = 57,
    #[doc = "58: 58 bytes available in RX FIFO"]
    B58 = 58,
    #[doc = "59: 59 bytes available in RX FIFO"]
    B59 = 59,
    #[doc = "60: 60 bytes available in RX FIFO"]
    B60 = 60,
    #[doc = "61: 61 bytes available in RX FIFO"]
    B61 = 61,
    #[doc = "62: 62 bytes available in RX FIFO"]
    B62 = 62,
    #[doc = "63: 63 bytes available in RX FIFO"]
    B63 = 63,
    #[doc = "64: 64 bytes available in RX FIFO"]
    B64 = 64,
}
impl From<RAC_A> for u8 {
    #[inline(always)]
    fn from(variant: RAC_A) -> Self {
        variant as _
    }
}
impl RAC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAC_A> {
        match self.bits {
            0 => Some(RAC_A::B0),
            1 => Some(RAC_A::B1),
            2 => Some(RAC_A::B2),
            3 => Some(RAC_A::B3),
            4 => Some(RAC_A::B4),
            5 => Some(RAC_A::B5),
            6 => Some(RAC_A::B6),
            7 => Some(RAC_A::B7),
            8 => Some(RAC_A::B8),
            9 => Some(RAC_A::B9),
            10 => Some(RAC_A::B10),
            11 => Some(RAC_A::B11),
            12 => Some(RAC_A::B12),
            13 => Some(RAC_A::B13),
            14 => Some(RAC_A::B14),
            15 => Some(RAC_A::B15),
            16 => Some(RAC_A::B16),
            17 => Some(RAC_A::B17),
            18 => Some(RAC_A::B18),
            19 => Some(RAC_A::B19),
            20 => Some(RAC_A::B20),
            21 => Some(RAC_A::B21),
            22 => Some(RAC_A::B22),
            23 => Some(RAC_A::B23),
            24 => Some(RAC_A::B24),
            25 => Some(RAC_A::B25),
            26 => Some(RAC_A::B26),
            27 => Some(RAC_A::B27),
            28 => Some(RAC_A::B28),
            29 => Some(RAC_A::B29),
            30 => Some(RAC_A::B30),
            31 => Some(RAC_A::B31),
            32 => Some(RAC_A::B32),
            33 => Some(RAC_A::B33),
            34 => Some(RAC_A::B34),
            35 => Some(RAC_A::B35),
            36 => Some(RAC_A::B36),
            37 => Some(RAC_A::B37),
            38 => Some(RAC_A::B38),
            39 => Some(RAC_A::B39),
            40 => Some(RAC_A::B40),
            41 => Some(RAC_A::B41),
            42 => Some(RAC_A::B42),
            43 => Some(RAC_A::B43),
            44 => Some(RAC_A::B44),
            45 => Some(RAC_A::B45),
            46 => Some(RAC_A::B46),
            47 => Some(RAC_A::B47),
            48 => Some(RAC_A::B48),
            49 => Some(RAC_A::B49),
            50 => Some(RAC_A::B50),
            51 => Some(RAC_A::B51),
            52 => Some(RAC_A::B52),
            53 => Some(RAC_A::B53),
            54 => Some(RAC_A::B54),
            55 => Some(RAC_A::B55),
            56 => Some(RAC_A::B56),
            57 => Some(RAC_A::B57),
            58 => Some(RAC_A::B58),
            59 => Some(RAC_A::B59),
            60 => Some(RAC_A::B60),
            61 => Some(RAC_A::B61),
            62 => Some(RAC_A::B62),
            63 => Some(RAC_A::B63),
            64 => Some(RAC_A::B64),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B0`"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RAC_A::B0
    }
    #[doc = "Checks if the value of the field is `B1`"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RAC_A::B1
    }
    #[doc = "Checks if the value of the field is `B2`"]
    #[inline(always)]
    pub fn is_b2(&self) -> bool {
        *self == RAC_A::B2
    }
    #[doc = "Checks if the value of the field is `B3`"]
    #[inline(always)]
    pub fn is_b3(&self) -> bool {
        *self == RAC_A::B3
    }
    #[doc = "Checks if the value of the field is `B4`"]
    #[inline(always)]
    pub fn is_b4(&self) -> bool {
        *self == RAC_A::B4
    }
    #[doc = "Checks if the value of the field is `B5`"]
    #[inline(always)]
    pub fn is_b5(&self) -> bool {
        *self == RAC_A::B5
    }
    #[doc = "Checks if the value of the field is `B6`"]
    #[inline(always)]
    pub fn is_b6(&self) -> bool {
        *self == RAC_A::B6
    }
    #[doc = "Checks if the value of the field is `B7`"]
    #[inline(always)]
    pub fn is_b7(&self) -> bool {
        *self == RAC_A::B7
    }
    #[doc = "Checks if the value of the field is `B8`"]
    #[inline(always)]
    pub fn is_b8(&self) -> bool {
        *self == RAC_A::B8
    }
    #[doc = "Checks if the value of the field is `B9`"]
    #[inline(always)]
    pub fn is_b9(&self) -> bool {
        *self == RAC_A::B9
    }
    #[doc = "Checks if the value of the field is `B10`"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == RAC_A::B10
    }
    #[doc = "Checks if the value of the field is `B11`"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == RAC_A::B11
    }
    #[doc = "Checks if the value of the field is `B12`"]
    #[inline(always)]
    pub fn is_b12(&self) -> bool {
        *self == RAC_A::B12
    }
    #[doc = "Checks if the value of the field is `B13`"]
    #[inline(always)]
    pub fn is_b13(&self) -> bool {
        *self == RAC_A::B13
    }
    #[doc = "Checks if the value of the field is `B14`"]
    #[inline(always)]
    pub fn is_b14(&self) -> bool {
        *self == RAC_A::B14
    }
    #[doc = "Checks if the value of the field is `B15`"]
    #[inline(always)]
    pub fn is_b15(&self) -> bool {
        *self == RAC_A::B15
    }
    #[doc = "Checks if the value of the field is `B16`"]
    #[inline(always)]
    pub fn is_b16(&self) -> bool {
        *self == RAC_A::B16
    }
    #[doc = "Checks if the value of the field is `B17`"]
    #[inline(always)]
    pub fn is_b17(&self) -> bool {
        *self == RAC_A::B17
    }
    #[doc = "Checks if the value of the field is `B18`"]
    #[inline(always)]
    pub fn is_b18(&self) -> bool {
        *self == RAC_A::B18
    }
    #[doc = "Checks if the value of the field is `B19`"]
    #[inline(always)]
    pub fn is_b19(&self) -> bool {
        *self == RAC_A::B19
    }
    #[doc = "Checks if the value of the field is `B20`"]
    #[inline(always)]
    pub fn is_b20(&self) -> bool {
        *self == RAC_A::B20
    }
    #[doc = "Checks if the value of the field is `B21`"]
    #[inline(always)]
    pub fn is_b21(&self) -> bool {
        *self == RAC_A::B21
    }
    #[doc = "Checks if the value of the field is `B22`"]
    #[inline(always)]
    pub fn is_b22(&self) -> bool {
        *self == RAC_A::B22
    }
    #[doc = "Checks if the value of the field is `B23`"]
    #[inline(always)]
    pub fn is_b23(&self) -> bool {
        *self == RAC_A::B23
    }
    #[doc = "Checks if the value of the field is `B24`"]
    #[inline(always)]
    pub fn is_b24(&self) -> bool {
        *self == RAC_A::B24
    }
    #[doc = "Checks if the value of the field is `B25`"]
    #[inline(always)]
    pub fn is_b25(&self) -> bool {
        *self == RAC_A::B25
    }
    #[doc = "Checks if the value of the field is `B26`"]
    #[inline(always)]
    pub fn is_b26(&self) -> bool {
        *self == RAC_A::B26
    }
    #[doc = "Checks if the value of the field is `B27`"]
    #[inline(always)]
    pub fn is_b27(&self) -> bool {
        *self == RAC_A::B27
    }
    #[doc = "Checks if the value of the field is `B28`"]
    #[inline(always)]
    pub fn is_b28(&self) -> bool {
        *self == RAC_A::B28
    }
    #[doc = "Checks if the value of the field is `B29`"]
    #[inline(always)]
    pub fn is_b29(&self) -> bool {
        *self == RAC_A::B29
    }
    #[doc = "Checks if the value of the field is `B30`"]
    #[inline(always)]
    pub fn is_b30(&self) -> bool {
        *self == RAC_A::B30
    }
    #[doc = "Checks if the value of the field is `B31`"]
    #[inline(always)]
    pub fn is_b31(&self) -> bool {
        *self == RAC_A::B31
    }
    #[doc = "Checks if the value of the field is `B32`"]
    #[inline(always)]
    pub fn is_b32(&self) -> bool {
        *self == RAC_A::B32
    }
    #[doc = "Checks if the value of the field is `B33`"]
    #[inline(always)]
    pub fn is_b33(&self) -> bool {
        *self == RAC_A::B33
    }
    #[doc = "Checks if the value of the field is `B34`"]
    #[inline(always)]
    pub fn is_b34(&self) -> bool {
        *self == RAC_A::B34
    }
    #[doc = "Checks if the value of the field is `B35`"]
    #[inline(always)]
    pub fn is_b35(&self) -> bool {
        *self == RAC_A::B35
    }
    #[doc = "Checks if the value of the field is `B36`"]
    #[inline(always)]
    pub fn is_b36(&self) -> bool {
        *self == RAC_A::B36
    }
    #[doc = "Checks if the value of the field is `B37`"]
    #[inline(always)]
    pub fn is_b37(&self) -> bool {
        *self == RAC_A::B37
    }
    #[doc = "Checks if the value of the field is `B38`"]
    #[inline(always)]
    pub fn is_b38(&self) -> bool {
        *self == RAC_A::B38
    }
    #[doc = "Checks if the value of the field is `B39`"]
    #[inline(always)]
    pub fn is_b39(&self) -> bool {
        *self == RAC_A::B39
    }
    #[doc = "Checks if the value of the field is `B40`"]
    #[inline(always)]
    pub fn is_b40(&self) -> bool {
        *self == RAC_A::B40
    }
    #[doc = "Checks if the value of the field is `B41`"]
    #[inline(always)]
    pub fn is_b41(&self) -> bool {
        *self == RAC_A::B41
    }
    #[doc = "Checks if the value of the field is `B42`"]
    #[inline(always)]
    pub fn is_b42(&self) -> bool {
        *self == RAC_A::B42
    }
    #[doc = "Checks if the value of the field is `B43`"]
    #[inline(always)]
    pub fn is_b43(&self) -> bool {
        *self == RAC_A::B43
    }
    #[doc = "Checks if the value of the field is `B44`"]
    #[inline(always)]
    pub fn is_b44(&self) -> bool {
        *self == RAC_A::B44
    }
    #[doc = "Checks if the value of the field is `B45`"]
    #[inline(always)]
    pub fn is_b45(&self) -> bool {
        *self == RAC_A::B45
    }
    #[doc = "Checks if the value of the field is `B46`"]
    #[inline(always)]
    pub fn is_b46(&self) -> bool {
        *self == RAC_A::B46
    }
    #[doc = "Checks if the value of the field is `B47`"]
    #[inline(always)]
    pub fn is_b47(&self) -> bool {
        *self == RAC_A::B47
    }
    #[doc = "Checks if the value of the field is `B48`"]
    #[inline(always)]
    pub fn is_b48(&self) -> bool {
        *self == RAC_A::B48
    }
    #[doc = "Checks if the value of the field is `B49`"]
    #[inline(always)]
    pub fn is_b49(&self) -> bool {
        *self == RAC_A::B49
    }
    #[doc = "Checks if the value of the field is `B50`"]
    #[inline(always)]
    pub fn is_b50(&self) -> bool {
        *self == RAC_A::B50
    }
    #[doc = "Checks if the value of the field is `B51`"]
    #[inline(always)]
    pub fn is_b51(&self) -> bool {
        *self == RAC_A::B51
    }
    #[doc = "Checks if the value of the field is `B52`"]
    #[inline(always)]
    pub fn is_b52(&self) -> bool {
        *self == RAC_A::B52
    }
    #[doc = "Checks if the value of the field is `B53`"]
    #[inline(always)]
    pub fn is_b53(&self) -> bool {
        *self == RAC_A::B53
    }
    #[doc = "Checks if the value of the field is `B54`"]
    #[inline(always)]
    pub fn is_b54(&self) -> bool {
        *self == RAC_A::B54
    }
    #[doc = "Checks if the value of the field is `B55`"]
    #[inline(always)]
    pub fn is_b55(&self) -> bool {
        *self == RAC_A::B55
    }
    #[doc = "Checks if the value of the field is `B56`"]
    #[inline(always)]
    pub fn is_b56(&self) -> bool {
        *self == RAC_A::B56
    }
    #[doc = "Checks if the value of the field is `B57`"]
    #[inline(always)]
    pub fn is_b57(&self) -> bool {
        *self == RAC_A::B57
    }
    #[doc = "Checks if the value of the field is `B58`"]
    #[inline(always)]
    pub fn is_b58(&self) -> bool {
        *self == RAC_A::B58
    }
    #[doc = "Checks if the value of the field is `B59`"]
    #[inline(always)]
    pub fn is_b59(&self) -> bool {
        *self == RAC_A::B59
    }
    #[doc = "Checks if the value of the field is `B60`"]
    #[inline(always)]
    pub fn is_b60(&self) -> bool {
        *self == RAC_A::B60
    }
    #[doc = "Checks if the value of the field is `B61`"]
    #[inline(always)]
    pub fn is_b61(&self) -> bool {
        *self == RAC_A::B61
    }
    #[doc = "Checks if the value of the field is `B62`"]
    #[inline(always)]
    pub fn is_b62(&self) -> bool {
        *self == RAC_A::B62
    }
    #[doc = "Checks if the value of the field is `B63`"]
    #[inline(always)]
    pub fn is_b63(&self) -> bool {
        *self == RAC_A::B63
    }
    #[doc = "Checks if the value of the field is `B64`"]
    #[inline(always)]
    pub fn is_b64(&self) -> bool {
        *self == RAC_A::B64
    }
}
impl R {
    #[doc = "Bit 0 - Receiver FIFO Overrun"]
    #[inline(always)]
    pub fn roi(&self) -> ROI_R {
        ROI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver Packet End Flag"]
    #[inline(always)]
    pub fn rpe(&self) -> RPE_R {
        RPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Available"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Status of CIR"]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - RX FIFO Available Counter"]
    #[inline(always)]
    pub fn rac(&self) -> RAC_R {
        RAC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver FIFO Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn roi(&mut self) -> ROI_W<0> {
        ROI_W::new(self)
    }
    #[doc = "Bit 1 - Receiver Packet End Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rpe(&mut self) -> RPE_W<1> {
        RPE_W::new(self)
    }
    #[doc = "Bit 4 - RX FIFO Available"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<4> {
        RA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CIR Receiver Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir_rxsta](index.html) module"]
pub struct CIR_RXSTA_SPEC;
impl crate::RegisterSpec for CIR_RXSTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir_rxsta::R](R) reader structure"]
impl crate::Readable for CIR_RXSTA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir_rxsta::W](W) writer structure"]
impl crate::Writable for CIR_RXSTA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x13;
}
#[doc = "`reset()` method sets cir_rxsta to value 0"]
impl crate::Resettable for CIR_RXSTA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
