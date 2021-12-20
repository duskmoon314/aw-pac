#[doc = "Register `LCR` reader"]
pub struct R(crate::R<LCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCR` writer"]
pub struct W(crate::W<LCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCR_SPEC>;
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
impl From<crate::W<LCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Divisor Latch Access Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLAB_A {
    #[doc = "0: `0`"]
    RX_BUFFER = 0,
    #[doc = "1: `1`"]
    DIVISOR_LATCH = 1,
}
impl From<DLAB_A> for bool {
    #[inline(always)]
    fn from(variant: DLAB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dlab` reader - Divisor Latch Access Bit"]
pub struct DLAB_R(crate::FieldReader<bool, DLAB_A>);
impl DLAB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLAB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLAB_A {
        match self.bits {
            false => DLAB_A::RX_BUFFER,
            true => DLAB_A::DIVISOR_LATCH,
        }
    }
    #[doc = "Checks if the value of the field is `RX_BUFFER`"]
    #[inline(always)]
    pub fn is_rx_buffer(&self) -> bool {
        **self == DLAB_A::RX_BUFFER
    }
    #[doc = "Checks if the value of the field is `DIVISOR_LATCH`"]
    #[inline(always)]
    pub fn is_divisor_latch(&self) -> bool {
        **self == DLAB_A::DIVISOR_LATCH
    }
}
impl core::ops::Deref for DLAB_R {
    type Target = crate::FieldReader<bool, DLAB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dlab` writer - Divisor Latch Access Bit"]
pub struct DLAB_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLAB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rx_buffer(self) -> &'a mut W {
        self.variant(DLAB_A::RX_BUFFER)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn divisor_latch(self) -> &'a mut W {
        self.variant(DLAB_A::DIVISOR_LATCH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `bc` reader - Break Control Bit"]
pub struct BC_R(crate::FieldReader<bool, bool>);
impl BC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bc` writer - Break Control Bit"]
pub struct BC_W<'a> {
    w: &'a mut W,
}
impl<'a> BC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Even Parity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPS_A {
    #[doc = "0: `0`"]
    ODD = 0,
    #[doc = "1: `1`"]
    EVEN = 1,
    #[doc = "2: `10`"]
    RS485_DATA = 2,
    #[doc = "3: `11`"]
    RS485_ADDR = 3,
}
impl From<EPS_A> for u8 {
    #[inline(always)]
    fn from(variant: EPS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `eps` reader - Even Parity Select"]
pub struct EPS_R(crate::FieldReader<u8, EPS_A>);
impl EPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPS_A {
        match self.bits {
            0 => EPS_A::ODD,
            1 => EPS_A::EVEN,
            2 => EPS_A::RS485_DATA,
            3 => EPS_A::RS485_ADDR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        **self == EPS_A::ODD
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        **self == EPS_A::EVEN
    }
    #[doc = "Checks if the value of the field is `RS485_DATA`"]
    #[inline(always)]
    pub fn is_rs485_data(&self) -> bool {
        **self == EPS_A::RS485_DATA
    }
    #[doc = "Checks if the value of the field is `RS485_ADDR`"]
    #[inline(always)]
    pub fn is_rs485_addr(&self) -> bool {
        **self == EPS_A::RS485_ADDR
    }
}
impl core::ops::Deref for EPS_R {
    type Target = crate::FieldReader<u8, EPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `eps` writer - Even Parity Select"]
pub struct EPS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(EPS_A::ODD)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(EPS_A::EVEN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rs485_data(self) -> &'a mut W {
        self.variant(EPS_A::RS485_DATA)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rs485_addr(self) -> &'a mut W {
        self.variant(EPS_A::RS485_ADDR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<PEN_A> for bool {
    #[inline(always)]
    fn from(variant: PEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pen` reader - Parity Enable"]
pub struct PEN_R(crate::FieldReader<bool, PEN_A>);
impl PEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN_A {
        match self.bits {
            false => PEN_A::DISABLED,
            true => PEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PEN_A::ENABLED
    }
}
impl core::ops::Deref for PEN_R {
    type Target = crate::FieldReader<bool, PEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pen` writer - Parity Enable"]
pub struct PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PEN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PEN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Number of stop bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_A {
    #[doc = "0: 1 stop bit"]
    ONE = 0,
    #[doc = "1: 1.5 stop bits when DLS(LCR\\[1:0\\]) is zero, else 2 stop bits"]
    TWO = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `stop` reader - Number of stop bits"]
pub struct STOP_R(crate::FieldReader<bool, STOP_A>);
impl STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::ONE,
            true => STOP_A::TWO,
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == STOP_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        **self == STOP_A::TWO
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<bool, STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `stop` writer - Number of stop bits"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(STOP_A::ONE)
    }
    #[doc = "1.5 stop bits when DLS(LCR\\[1:0\\]) is zero, else 2 stop bits"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(STOP_A::TWO)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Data Length Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DLS_A {
    #[doc = "0: 5 bits"]
    FIVE = 0,
    #[doc = "1: 6 bits"]
    SIX = 1,
    #[doc = "2: 7 bits"]
    SEVEN = 2,
    #[doc = "3: 8 bits"]
    EIGHT = 3,
}
impl From<DLS_A> for u8 {
    #[inline(always)]
    fn from(variant: DLS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `dls` reader - Data Length Select"]
pub struct DLS_R(crate::FieldReader<u8, DLS_A>);
impl DLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLS_A {
        match self.bits {
            0 => DLS_A::FIVE,
            1 => DLS_A::SIX,
            2 => DLS_A::SEVEN,
            3 => DLS_A::EIGHT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FIVE`"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        **self == DLS_A::FIVE
    }
    #[doc = "Checks if the value of the field is `SIX`"]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        **self == DLS_A::SIX
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        **self == DLS_A::SEVEN
    }
    #[doc = "Checks if the value of the field is `EIGHT`"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        **self == DLS_A::EIGHT
    }
}
impl core::ops::Deref for DLS_R {
    type Target = crate::FieldReader<u8, DLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dls` writer - Data Length Select"]
pub struct DLS_W<'a> {
    w: &'a mut W,
}
impl<'a> DLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn five(self) -> &'a mut W {
        self.variant(DLS_A::FIVE)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn six(self) -> &'a mut W {
        self.variant(DLS_A::SIX)
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut W {
        self.variant(DLS_A::SEVEN)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut W {
        self.variant(DLS_A::EIGHT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn dlab(&self) -> DLAB_R {
        DLAB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Break Control Bit"]
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Even Parity Select"]
    #[inline(always)]
    pub fn eps(&self) -> EPS_R {
        EPS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Parity Enable"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Number of stop bits"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - Data Length Select"]
    #[inline(always)]
    pub fn dls(&self) -> DLS_R {
        DLS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn dlab(&mut self) -> DLAB_W {
        DLAB_W { w: self }
    }
    #[doc = "Bit 6 - Break Control Bit"]
    #[inline(always)]
    pub fn bc(&mut self) -> BC_W {
        BC_W { w: self }
    }
    #[doc = "Bits 4:5 - Even Parity Select"]
    #[inline(always)]
    pub fn eps(&mut self) -> EPS_W {
        EPS_W { w: self }
    }
    #[doc = "Bit 3 - Parity Enable"]
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W {
        PEN_W { w: self }
    }
    #[doc = "Bit 2 - Number of stop bits"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bits 0:1 - Data Length Select"]
    #[inline(always)]
    pub fn dls(&mut self) -> DLS_W {
        DLS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Line Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcr](index.html) module"]
pub struct LCR_SPEC;
impl crate::RegisterSpec for LCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcr::R](R) reader structure"]
impl crate::Readable for LCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcr::W](W) writer structure"]
impl crate::Writable for LCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCR to value 0"]
impl crate::Resettable for LCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
