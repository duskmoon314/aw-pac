#[doc = "Register `iir` reader"]
pub struct R(crate::R<IIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `iid` reader - Interrupt ID"]
pub type IID_R = crate::FieldReader<u8, IID_A>;
#[doc = "Interrupt ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IID_A {
    #[doc = "0: `0`"]
    MODEM_STATUS = 0,
    #[doc = "1: `1`"]
    NO_INTERRUPT_PENDING = 1,
    #[doc = "2: `10`"]
    THR_EMPTY = 2,
    #[doc = "3: `11`"]
    RS485_INTERRUPT = 3,
    #[doc = "4: `100`"]
    RECEIVED_DATA_AVAILABLE = 4,
    #[doc = "6: `110`"]
    RECEIVER_LINE_STATUS = 6,
    #[doc = "7: `111`"]
    BUSY_DETECT = 7,
    #[doc = "12: `1100`"]
    CHARACTER_TIMEOUT = 12,
}
impl From<IID_A> for u8 {
    #[inline(always)]
    fn from(variant: IID_A) -> Self {
        variant as _
    }
}
impl IID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IID_A> {
        match self.bits {
            0 => Some(IID_A::MODEM_STATUS),
            1 => Some(IID_A::NO_INTERRUPT_PENDING),
            2 => Some(IID_A::THR_EMPTY),
            3 => Some(IID_A::RS485_INTERRUPT),
            4 => Some(IID_A::RECEIVED_DATA_AVAILABLE),
            6 => Some(IID_A::RECEIVER_LINE_STATUS),
            7 => Some(IID_A::BUSY_DETECT),
            12 => Some(IID_A::CHARACTER_TIMEOUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MODEM_STATUS`"]
    #[inline(always)]
    pub fn is_modem_status(&self) -> bool {
        *self == IID_A::MODEM_STATUS
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT_PENDING`"]
    #[inline(always)]
    pub fn is_no_interrupt_pending(&self) -> bool {
        *self == IID_A::NO_INTERRUPT_PENDING
    }
    #[doc = "Checks if the value of the field is `THR_EMPTY`"]
    #[inline(always)]
    pub fn is_thr_empty(&self) -> bool {
        *self == IID_A::THR_EMPTY
    }
    #[doc = "Checks if the value of the field is `RS485_INTERRUPT`"]
    #[inline(always)]
    pub fn is_rs485_interrupt(&self) -> bool {
        *self == IID_A::RS485_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `RECEIVED_DATA_AVAILABLE`"]
    #[inline(always)]
    pub fn is_received_data_available(&self) -> bool {
        *self == IID_A::RECEIVED_DATA_AVAILABLE
    }
    #[doc = "Checks if the value of the field is `RECEIVER_LINE_STATUS`"]
    #[inline(always)]
    pub fn is_receiver_line_status(&self) -> bool {
        *self == IID_A::RECEIVER_LINE_STATUS
    }
    #[doc = "Checks if the value of the field is `BUSY_DETECT`"]
    #[inline(always)]
    pub fn is_busy_detect(&self) -> bool {
        *self == IID_A::BUSY_DETECT
    }
    #[doc = "Checks if the value of the field is `CHARACTER_TIMEOUT`"]
    #[inline(always)]
    pub fn is_character_timeout(&self) -> bool {
        *self == IID_A::CHARACTER_TIMEOUT
    }
}
#[doc = "Field `feflag` reader - FIFOs Enable Flag"]
pub type FEFLAG_R = crate::FieldReader<u8, FEFLAG_A>;
#[doc = "FIFOs Enable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FEFLAG_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "3: `11`"]
    ENABLE = 3,
}
impl From<FEFLAG_A> for u8 {
    #[inline(always)]
    fn from(variant: FEFLAG_A) -> Self {
        variant as _
    }
}
impl FEFLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FEFLAG_A> {
        match self.bits {
            0 => Some(FEFLAG_A::DISABLE),
            3 => Some(FEFLAG_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FEFLAG_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FEFLAG_A::ENABLE
    }
}
impl R {
    #[doc = "Bits 0:3 - Interrupt ID"]
    #[inline(always)]
    pub fn iid(&self) -> IID_R {
        IID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - FIFOs Enable Flag"]
    #[inline(always)]
    pub fn feflag(&self) -> FEFLAG_R {
        FEFLAG_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "UART Interrupt Identity Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iir](index.html) module"]
pub struct IIR_SPEC;
impl crate::RegisterSpec for IIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iir::R](R) reader structure"]
impl crate::Readable for IIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets iir to value 0"]
impl crate::Resettable for IIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
