#[doc = "Register `SPI_ISR` reader"]
pub struct R(crate::R<SPI_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_ISR` writer"]
pub struct W(crate::W<SPI_ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_ISR_SPEC>;
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
impl From<crate::W<SPI_ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ssi` reader - SS Invalid Enable"]
pub struct SSI_R(crate::FieldReader<bool, bool>);
impl SSI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ssi` writer - SS Invalid Enable"]
pub struct SSI_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Transfer Completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC_A {
    #[doc = "0: `0`"]
    BUSY = 0,
    #[doc = "1: `1`"]
    TRANSFER_COMPLETED = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tc` reader - Transfer Completed"]
pub struct TC_R(crate::FieldReader<bool, TC_A>);
impl TC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::BUSY,
            true => TC_A::TRANSFER_COMPLETED,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == TC_A::BUSY
    }
    #[doc = "Checks if the value of the field is `TRANSFER_COMPLETED`"]
    #[inline(always)]
    pub fn is_transfer_completed(&self) -> bool {
        **self == TC_A::TRANSFER_COMPLETED
    }
}
impl core::ops::Deref for TC_R {
    type Target = crate::FieldReader<bool, TC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tc` writer - Transfer Completed"]
pub struct TC_W<'a> {
    w: &'a mut W,
}
impl<'a> TC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(TC_A::BUSY)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn transfer_completed(self) -> &'a mut W {
        self.variant(TC_A::TRANSFER_COMPLETED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "TXFIFO Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF_UDR_A {
    #[doc = "0: `0`"]
    NOT_UNDERRUN = 0,
    #[doc = "1: `1`"]
    UNDERRUN = 1,
}
impl From<TF_UDR_A> for bool {
    #[inline(always)]
    fn from(variant: TF_UDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tf_udr` reader - TXFIFO Underrun"]
pub struct TF_UDR_R(crate::FieldReader<bool, TF_UDR_A>);
impl TF_UDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TF_UDR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF_UDR_A {
        match self.bits {
            false => TF_UDR_A::NOT_UNDERRUN,
            true => TF_UDR_A::UNDERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_UNDERRUN`"]
    #[inline(always)]
    pub fn is_not_underrun(&self) -> bool {
        **self == TF_UDR_A::NOT_UNDERRUN
    }
    #[doc = "Checks if the value of the field is `UNDERRUN`"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        **self == TF_UDR_A::UNDERRUN
    }
}
impl core::ops::Deref for TF_UDR_R {
    type Target = crate::FieldReader<bool, TF_UDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tf_udr` writer - TXFIFO Underrun"]
pub struct TF_UDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TF_UDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TF_UDR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_underrun(self) -> &'a mut W {
        self.variant(TF_UDR_A::NOT_UNDERRUN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn underrun(self) -> &'a mut W {
        self.variant(TF_UDR_A::UNDERRUN)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "TXFIFO Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF_OVF_A {
    #[doc = "0: `0`"]
    NOT_OVERFLOW = 0,
    #[doc = "1: `1`"]
    OVERFLOW = 1,
}
impl From<TF_OVF_A> for bool {
    #[inline(always)]
    fn from(variant: TF_OVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tf_ovf` reader - TXFIFO Overflow"]
pub struct TF_OVF_R(crate::FieldReader<bool, TF_OVF_A>);
impl TF_OVF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TF_OVF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF_OVF_A {
        match self.bits {
            false => TF_OVF_A::NOT_OVERFLOW,
            true => TF_OVF_A::OVERFLOW,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_OVERFLOW`"]
    #[inline(always)]
    pub fn is_not_overflow(&self) -> bool {
        **self == TF_OVF_A::NOT_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        **self == TF_OVF_A::OVERFLOW
    }
}
impl core::ops::Deref for TF_OVF_R {
    type Target = crate::FieldReader<bool, TF_OVF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tf_ovf` writer - TXFIFO Overflow"]
pub struct TF_OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> TF_OVF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TF_OVF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_overflow(self) -> &'a mut W {
        self.variant(TF_OVF_A::NOT_OVERFLOW)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut W {
        self.variant(TF_OVF_A::OVERFLOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "RXFIFO Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_UDR_A {
    #[doc = "0: `0`"]
    NOT_UNDERRUN = 0,
    #[doc = "1: `1`"]
    UNDERRUN = 1,
}
impl From<RF_UDR_A> for bool {
    #[inline(always)]
    fn from(variant: RF_UDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rf_udr` reader - RXFIFO Underrun"]
pub struct RF_UDR_R(crate::FieldReader<bool, RF_UDR_A>);
impl RF_UDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_UDR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_UDR_A {
        match self.bits {
            false => RF_UDR_A::NOT_UNDERRUN,
            true => RF_UDR_A::UNDERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_UNDERRUN`"]
    #[inline(always)]
    pub fn is_not_underrun(&self) -> bool {
        **self == RF_UDR_A::NOT_UNDERRUN
    }
    #[doc = "Checks if the value of the field is `UNDERRUN`"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        **self == RF_UDR_A::UNDERRUN
    }
}
impl core::ops::Deref for RF_UDR_R {
    type Target = crate::FieldReader<bool, RF_UDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_udr` writer - RXFIFO Underrun"]
pub struct RF_UDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_UDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF_UDR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_underrun(self) -> &'a mut W {
        self.variant(RF_UDR_A::NOT_UNDERRUN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn underrun(self) -> &'a mut W {
        self.variant(RF_UDR_A::UNDERRUN)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "RXFIFO Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_OVF_A {
    #[doc = "0: `0`"]
    NOT_OVERFLOW = 0,
    #[doc = "1: `1`"]
    OVERFLOW = 1,
}
impl From<RF_OVF_A> for bool {
    #[inline(always)]
    fn from(variant: RF_OVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rf_ovf` reader - RXFIFO Overflow"]
pub struct RF_OVF_R(crate::FieldReader<bool, RF_OVF_A>);
impl RF_OVF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_OVF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_OVF_A {
        match self.bits {
            false => RF_OVF_A::NOT_OVERFLOW,
            true => RF_OVF_A::OVERFLOW,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_OVERFLOW`"]
    #[inline(always)]
    pub fn is_not_overflow(&self) -> bool {
        **self == RF_OVF_A::NOT_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        **self == RF_OVF_A::OVERFLOW
    }
}
impl core::ops::Deref for RF_OVF_R {
    type Target = crate::FieldReader<bool, RF_OVF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_ovf` writer - RXFIFO Overflow"]
pub struct RF_OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_OVF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF_OVF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_overflow(self) -> &'a mut W {
        self.variant(RF_OVF_A::NOT_OVERFLOW)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut W {
        self.variant(RF_OVF_A::OVERFLOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "TXFIFO Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF_FULL_A {
    #[doc = "0: `0`"]
    NOT_FULL = 0,
    #[doc = "1: `1`"]
    FULL = 1,
}
impl From<TF_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: TF_FULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tf_full` reader - TXFIFO Full"]
pub struct TF_FULL_R(crate::FieldReader<bool, TF_FULL_A>);
impl TF_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TF_FULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF_FULL_A {
        match self.bits {
            false => TF_FULL_A::NOT_FULL,
            true => TF_FULL_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        **self == TF_FULL_A::NOT_FULL
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        **self == TF_FULL_A::FULL
    }
}
impl core::ops::Deref for TF_FULL_R {
    type Target = crate::FieldReader<bool, TF_FULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tf_full` writer - TXFIFO Full"]
pub struct TF_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> TF_FULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TF_FULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_full(self) -> &'a mut W {
        self.variant(TF_FULL_A::NOT_FULL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(TF_FULL_A::FULL)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "TXFIFO Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF_EMP_A {
    #[doc = "0: `0`"]
    NOT_EMPTY = 0,
    #[doc = "1: `1`"]
    EMPTY = 1,
}
impl From<TF_EMP_A> for bool {
    #[inline(always)]
    fn from(variant: TF_EMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tf_emp` reader - TXFIFO Empty"]
pub struct TF_EMP_R(crate::FieldReader<bool, TF_EMP_A>);
impl TF_EMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TF_EMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF_EMP_A {
        match self.bits {
            false => TF_EMP_A::NOT_EMPTY,
            true => TF_EMP_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        **self == TF_EMP_A::NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        **self == TF_EMP_A::EMPTY
    }
}
impl core::ops::Deref for TF_EMP_R {
    type Target = crate::FieldReader<bool, TF_EMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tf_emp` writer - TXFIFO Empty"]
pub struct TF_EMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TF_EMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TF_EMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut W {
        self.variant(TF_EMP_A::NOT_EMPTY)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TF_EMP_A::EMPTY)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `tf_ready` reader - TXFIFO Ready"]
pub struct TF_READY_R(crate::FieldReader<bool, bool>);
impl TF_READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TF_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tf_ready` writer - TXFIFO Ready"]
pub struct TF_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> TF_READY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "RXFIFO Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_FULL_A {
    #[doc = "0: `0`"]
    NOT_FULL = 0,
    #[doc = "1: `1`"]
    FULL = 1,
}
impl From<RF_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: RF_FULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rf_full` reader - RXFIFO Full"]
pub struct RF_FULL_R(crate::FieldReader<bool, RF_FULL_A>);
impl RF_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_FULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_FULL_A {
        match self.bits {
            false => RF_FULL_A::NOT_FULL,
            true => RF_FULL_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        **self == RF_FULL_A::NOT_FULL
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        **self == RF_FULL_A::FULL
    }
}
impl core::ops::Deref for RF_FULL_R {
    type Target = crate::FieldReader<bool, RF_FULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_full` writer - RXFIFO Full"]
pub struct RF_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF_FULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_full(self) -> &'a mut W {
        self.variant(RF_FULL_A::NOT_FULL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(RF_FULL_A::FULL)
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
#[doc = "RXFIFO Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_EMP_A {
    #[doc = "0: `0`"]
    NOT_EMPTY = 0,
    #[doc = "1: `1`"]
    EMPTY = 1,
}
impl From<RF_EMP_A> for bool {
    #[inline(always)]
    fn from(variant: RF_EMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rf_emp` reader - RXFIFO Empty"]
pub struct RF_EMP_R(crate::FieldReader<bool, RF_EMP_A>);
impl RF_EMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_EMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_EMP_A {
        match self.bits {
            false => RF_EMP_A::NOT_EMPTY,
            true => RF_EMP_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        **self == RF_EMP_A::NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        **self == RF_EMP_A::EMPTY
    }
}
impl core::ops::Deref for RF_EMP_R {
    type Target = crate::FieldReader<bool, RF_EMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_emp` writer - RXFIFO Empty"]
pub struct RF_EMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_EMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF_EMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut W {
        self.variant(RF_EMP_A::NOT_EMPTY)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(RF_EMP_A::EMPTY)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `rf_rdy` reader - RXFIFO Ready"]
pub struct RF_RDY_R(crate::FieldReader<bool, bool>);
impl RF_RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_rdy` writer - RXFIFO Ready"]
pub struct RF_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 13 - SS Invalid Enable"]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transfer Completed"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TXFIFO Underrun"]
    #[inline(always)]
    pub fn tf_udr(&self) -> TF_UDR_R {
        TF_UDR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TXFIFO Overflow"]
    #[inline(always)]
    pub fn tf_ovf(&self) -> TF_OVF_R {
        TF_OVF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RXFIFO Underrun"]
    #[inline(always)]
    pub fn rf_udr(&self) -> RF_UDR_R {
        RF_UDR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RXFIFO Overflow"]
    #[inline(always)]
    pub fn rf_ovf(&self) -> RF_OVF_R {
        RF_OVF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TXFIFO Full"]
    #[inline(always)]
    pub fn tf_full(&self) -> TF_FULL_R {
        TF_FULL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TXFIFO Empty"]
    #[inline(always)]
    pub fn tf_emp(&self) -> TF_EMP_R {
        TF_EMP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TXFIFO Ready"]
    #[inline(always)]
    pub fn tf_ready(&self) -> TF_READY_R {
        TF_READY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RXFIFO Full"]
    #[inline(always)]
    pub fn rf_full(&self) -> RF_FULL_R {
        RF_FULL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - RXFIFO Empty"]
    #[inline(always)]
    pub fn rf_emp(&self) -> RF_EMP_R {
        RF_EMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RXFIFO Ready"]
    #[inline(always)]
    pub fn rf_rdy(&self) -> RF_RDY_R {
        RF_RDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - SS Invalid Enable"]
    #[inline(always)]
    pub fn ssi(&mut self) -> SSI_W {
        SSI_W { w: self }
    }
    #[doc = "Bit 12 - Transfer Completed"]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W {
        TC_W { w: self }
    }
    #[doc = "Bit 11 - TXFIFO Underrun"]
    #[inline(always)]
    pub fn tf_udr(&mut self) -> TF_UDR_W {
        TF_UDR_W { w: self }
    }
    #[doc = "Bit 10 - TXFIFO Overflow"]
    #[inline(always)]
    pub fn tf_ovf(&mut self) -> TF_OVF_W {
        TF_OVF_W { w: self }
    }
    #[doc = "Bit 9 - RXFIFO Underrun"]
    #[inline(always)]
    pub fn rf_udr(&mut self) -> RF_UDR_W {
        RF_UDR_W { w: self }
    }
    #[doc = "Bit 8 - RXFIFO Overflow"]
    #[inline(always)]
    pub fn rf_ovf(&mut self) -> RF_OVF_W {
        RF_OVF_W { w: self }
    }
    #[doc = "Bit 6 - TXFIFO Full"]
    #[inline(always)]
    pub fn tf_full(&mut self) -> TF_FULL_W {
        TF_FULL_W { w: self }
    }
    #[doc = "Bit 5 - TXFIFO Empty"]
    #[inline(always)]
    pub fn tf_emp(&mut self) -> TF_EMP_W {
        TF_EMP_W { w: self }
    }
    #[doc = "Bit 4 - TXFIFO Ready"]
    #[inline(always)]
    pub fn tf_ready(&mut self) -> TF_READY_W {
        TF_READY_W { w: self }
    }
    #[doc = "Bit 2 - RXFIFO Full"]
    #[inline(always)]
    pub fn rf_full(&mut self) -> RF_FULL_W {
        RF_FULL_W { w: self }
    }
    #[doc = "Bit 1 - RXFIFO Empty"]
    #[inline(always)]
    pub fn rf_emp(&mut self) -> RF_EMP_W {
        RF_EMP_W { w: self }
    }
    #[doc = "Bit 0 - RXFIFO Ready"]
    #[inline(always)]
    pub fn rf_rdy(&mut self) -> RF_RDY_W {
        RF_RDY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_isr](index.html) module"]
pub struct SPI_ISR_SPEC;
impl crate::RegisterSpec for SPI_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_isr::R](R) reader structure"]
impl crate::Readable for SPI_ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_isr::W](W) writer structure"]
impl crate::Writable for SPI_ISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_ISR to value 0"]
impl crate::Resettable for SPI_ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
