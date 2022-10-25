#[doc = "Register `mcr` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mcr` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dtr` reader - Data Terminal Ready"]
pub type DTR_R = crate::BitReader<DTR_A>;
#[doc = "Data Terminal Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTR_A {
    #[doc = "0: `0`"]
    DEASSERTED = 0,
    #[doc = "1: `1`"]
    ASSERTED = 1,
}
impl From<DTR_A> for bool {
    #[inline(always)]
    fn from(variant: DTR_A) -> Self {
        variant as u8 != 0
    }
}
impl DTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTR_A {
        match self.bits {
            false => DTR_A::DEASSERTED,
            true => DTR_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `DEASSERTED`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        *self == DTR_A::DEASSERTED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == DTR_A::ASSERTED
    }
}
#[doc = "Field `dtr` writer - Data Terminal Ready"]
pub type DTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, DTR_A, O>;
impl<'a, const O: u8> DTR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn deasserted(self) -> &'a mut W {
        self.variant(DTR_A::DEASSERTED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(DTR_A::ASSERTED)
    }
}
#[doc = "Field `rts` reader - Request to Send"]
pub type RTS_R = crate::BitReader<RTS_A>;
#[doc = "Request to Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTS_A {
    #[doc = "0: `0`"]
    DEASSERTED = 0,
    #[doc = "1: `1`"]
    ASSERTED = 1,
}
impl From<RTS_A> for bool {
    #[inline(always)]
    fn from(variant: RTS_A) -> Self {
        variant as u8 != 0
    }
}
impl RTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTS_A {
        match self.bits {
            false => RTS_A::DEASSERTED,
            true => RTS_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `DEASSERTED`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        *self == RTS_A::DEASSERTED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == RTS_A::ASSERTED
    }
}
#[doc = "Field `rts` writer - Request to Send"]
pub type RTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, RTS_A, O>;
impl<'a, const O: u8> RTS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn deasserted(self) -> &'a mut W {
        self.variant(RTS_A::DEASSERTED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(RTS_A::ASSERTED)
    }
}
#[doc = "Field `loop` reader - Loop Back Mode"]
pub type LOOP_R = crate::BitReader<LOOP_A>;
#[doc = "Loop Back Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOP_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    LOOP_BACK = 1,
}
impl From<LOOP_A> for bool {
    #[inline(always)]
    fn from(variant: LOOP_A) -> Self {
        variant as u8 != 0
    }
}
impl LOOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOP_A {
        match self.bits {
            false => LOOP_A::NORMAL,
            true => LOOP_A::LOOP_BACK,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LOOP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `LOOP_BACK`"]
    #[inline(always)]
    pub fn is_loop_back(&self) -> bool {
        *self == LOOP_A::LOOP_BACK
    }
}
#[doc = "Field `loop` writer - Loop Back Mode"]
pub type LOOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, LOOP_A, O>;
impl<'a, const O: u8> LOOP_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(LOOP_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn loop_back(self) -> &'a mut W {
        self.variant(LOOP_A::LOOP_BACK)
    }
}
#[doc = "Field `afce` reader - Auto Flow Control Enable"]
pub type AFCE_R = crate::BitReader<AFCE_A>;
#[doc = "Auto Flow Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFCE_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<AFCE_A> for bool {
    #[inline(always)]
    fn from(variant: AFCE_A) -> Self {
        variant as u8 != 0
    }
}
impl AFCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFCE_A {
        match self.bits {
            false => AFCE_A::DISABLED,
            true => AFCE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AFCE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AFCE_A::ENABLED
    }
}
#[doc = "Field `afce` writer - Auto Flow Control Enable"]
pub type AFCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, AFCE_A, O>;
impl<'a, const O: u8> AFCE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFCE_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFCE_A::ENABLED)
    }
}
#[doc = "Field `function` reader - UART Function: Select IrDA or RS485"]
pub type FUNCTION_R = crate::FieldReader<u8, FUNCTION_A>;
#[doc = "UART Function: Select IrDA or RS485\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FUNCTION_A {
    #[doc = "0: `0`"]
    UART = 0,
    #[doc = "1: `1`"]
    IR_DA_SIR = 1,
    #[doc = "2: `10`"]
    RS485 = 2,
}
impl From<FUNCTION_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNCTION_A) -> Self {
        variant as _
    }
}
impl FUNCTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FUNCTION_A {
        match self.bits {
            0 => FUNCTION_A::UART,
            1 => FUNCTION_A::IR_DA_SIR,
            2 => FUNCTION_A::RS485,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UART`"]
    #[inline(always)]
    pub fn is_uart(&self) -> bool {
        *self == FUNCTION_A::UART
    }
    #[doc = "Checks if the value of the field is `IR_DA_SIR`"]
    #[inline(always)]
    pub fn is_ir_da_sir(&self) -> bool {
        *self == FUNCTION_A::IR_DA_SIR
    }
    #[doc = "Checks if the value of the field is `RS485`"]
    #[inline(always)]
    pub fn is_rs485(&self) -> bool {
        *self == FUNCTION_A::RS485
    }
}
#[doc = "Field `function` writer - UART Function: Select IrDA or RS485"]
pub type FUNCTION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR_SPEC, u8, FUNCTION_A, 2, O>;
impl<'a, const O: u8> FUNCTION_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn uart(self) -> &'a mut W {
        self.variant(FUNCTION_A::UART)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ir_da_sir(self) -> &'a mut W {
        self.variant(FUNCTION_A::IR_DA_SIR)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rs485(self) -> &'a mut W {
        self.variant(FUNCTION_A::RS485)
    }
}
impl R {
    #[doc = "Bit 0 - Data Terminal Ready"]
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Request to Send"]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Loop Back Mode"]
    #[inline(always)]
    pub fn loop_(&self) -> LOOP_R {
        LOOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Auto Flow Control Enable"]
    #[inline(always)]
    pub fn afce(&self) -> AFCE_R {
        AFCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - UART Function: Select IrDA or RS485"]
    #[inline(always)]
    pub fn function(&self) -> FUNCTION_R {
        FUNCTION_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Data Terminal Ready"]
    #[inline(always)]
    #[must_use]
    pub fn dtr(&mut self) -> DTR_W<0> {
        DTR_W::new(self)
    }
    #[doc = "Bit 1 - Request to Send"]
    #[inline(always)]
    #[must_use]
    pub fn rts(&mut self) -> RTS_W<1> {
        RTS_W::new(self)
    }
    #[doc = "Bit 4 - Loop Back Mode"]
    #[inline(always)]
    #[must_use]
    pub fn loop_(&mut self) -> LOOP_W<4> {
        LOOP_W::new(self)
    }
    #[doc = "Bit 5 - Auto Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn afce(&mut self) -> AFCE_W<5> {
        AFCE_W::new(self)
    }
    #[doc = "Bits 6:7 - UART Function: Select IrDA or RS485"]
    #[inline(always)]
    #[must_use]
    pub fn function(&mut self) -> FUNCTION_W<6> {
        FUNCTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Modem Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mcr to value 0"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
