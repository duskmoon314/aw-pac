#[doc = "Register `usr` reader"]
pub struct R(crate::R<USR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `busy` reader - UART Busy Bit"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "UART Busy Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: `0`"]
    IDLE = 0,
    #[doc = "1: `1`"]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
#[doc = "Field `tfnf` reader - TX FIFO Not Full"]
pub type TFNF_R = crate::BitReader<TFNF_A>;
#[doc = "TX FIFO Not Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFNF_A {
    #[doc = "0: `0`"]
    FULL = 0,
    #[doc = "1: `1`"]
    NOT_FULL = 1,
}
impl From<TFNF_A> for bool {
    #[inline(always)]
    fn from(variant: TFNF_A) -> Self {
        variant as u8 != 0
    }
}
impl TFNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFNF_A {
        match self.bits {
            false => TFNF_A::FULL,
            true => TFNF_A::NOT_FULL,
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TFNF_A::FULL
    }
    #[doc = "Checks if the value of the field is `NOT_FULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TFNF_A::NOT_FULL
    }
}
#[doc = "Field `tfe` reader - TX FIFO Empty"]
pub type TFE_R = crate::BitReader<TFE_A>;
#[doc = "TX FIFO Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFE_A {
    #[doc = "0: `0`"]
    NOT_EMPTY = 0,
    #[doc = "1: `1`"]
    EMPTY = 1,
}
impl From<TFE_A> for bool {
    #[inline(always)]
    fn from(variant: TFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFE_A {
        match self.bits {
            false => TFE_A::NOT_EMPTY,
            true => TFE_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TFE_A::NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TFE_A::EMPTY
    }
}
#[doc = "Field `rfne` reader - RX FIFO Not Empty"]
pub type RFNE_R = crate::BitReader<RFNE_A>;
#[doc = "RX FIFO Not Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFNE_A {
    #[doc = "0: `0`"]
    EMPTY = 0,
    #[doc = "1: `1`"]
    NOT_EMPTY = 1,
}
impl From<RFNE_A> for bool {
    #[inline(always)]
    fn from(variant: RFNE_A) -> Self {
        variant as u8 != 0
    }
}
impl RFNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFNE_A {
        match self.bits {
            false => RFNE_A::EMPTY,
            true => RFNE_A::NOT_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RFNE_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RFNE_A::NOT_EMPTY
    }
}
#[doc = "Field `rff` reader - RX FIFO Full"]
pub type RFF_R = crate::BitReader<RFF_A>;
#[doc = "RX FIFO Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFF_A {
    #[doc = "0: `0`"]
    NOT_FULL = 0,
    #[doc = "1: `1`"]
    FULL = 1,
}
impl From<RFF_A> for bool {
    #[inline(always)]
    fn from(variant: RFF_A) -> Self {
        variant as u8 != 0
    }
}
impl RFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFF_A {
        match self.bits {
            false => RFF_A::NOT_FULL,
            true => RFF_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RFF_A::NOT_FULL
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RFF_A::FULL
    }
}
impl R {
    #[doc = "Bit 0 - UART Busy Bit"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Not Full"]
    #[inline(always)]
    pub fn tfnf(&self) -> TFNF_R {
        TFNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX FIFO Empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Not Empty"]
    #[inline(always)]
    pub fn rfne(&self) -> RFNE_R {
        RFNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Full"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "UART Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usr](index.html) module"]
pub struct USR_SPEC;
impl crate::RegisterSpec for USR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usr::R](R) reader structure"]
impl crate::Readable for USR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets usr to value 0"]
impl crate::Resettable for USR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
