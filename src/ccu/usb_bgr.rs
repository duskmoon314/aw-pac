#[doc = "Register `USB_BGR` reader"]
pub struct R(crate::R<USB_BGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_BGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_BGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_BGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_BGR` writer"]
pub struct W(crate::W<USB_BGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_BGR_SPEC>;
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
impl From<crate::W<USB_BGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_BGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USBOTG0 Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBOTG0_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<USBOTG0_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USBOTG0_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBOTG0_RST` reader - USBOTG0 Reset"]
pub struct USBOTG0_RST_R(crate::FieldReader<bool, USBOTG0_RST_A>);
impl USBOTG0_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBOTG0_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBOTG0_RST_A {
        match self.bits {
            false => USBOTG0_RST_A::ASSERT,
            true => USBOTG0_RST_A::DEASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        **self == USBOTG0_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        **self == USBOTG0_RST_A::DEASSERT
    }
}
impl core::ops::Deref for USBOTG0_RST_R {
    type Target = crate::FieldReader<bool, USBOTG0_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBOTG0_RST` writer - USBOTG0 Reset"]
pub struct USBOTG0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBOTG0_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBOTG0_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(USBOTG0_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(USBOTG0_RST_A::DEASSERT)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "USBEHCI Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBEHCI_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<USBEHCI_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USBEHCI_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `USBEHCI(0-1)_RST` reader - USBEHCI Reset"]
pub struct USBEHCI_RST_R(crate::FieldReader<bool, USBEHCI_RST_A>);
impl USBEHCI_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBEHCI_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBEHCI_RST_A {
        match self.bits {
            false => USBEHCI_RST_A::ASSERT,
            true => USBEHCI_RST_A::DEASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        **self == USBEHCI_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        **self == USBEHCI_RST_A::DEASSERT
    }
}
impl core::ops::Deref for USBEHCI_RST_R {
    type Target = crate::FieldReader<bool, USBEHCI_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `USBEHCI(0-1)_RST` writer - USBEHCI Reset"]
pub struct USBEHCI_RST_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> USBEHCI_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBEHCI_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(USBEHCI_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(USBEHCI_RST_A::DEASSERT)
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
        self.w.bits =
            (self.w.bits & !(0x01 << self.offset)) | ((value as u32 & 0x01) << self.offset);
        self.w
    }
}
#[doc = "Fields `USBEHCI(0-1)_RST` const generic writer - USBEHCI Reset"]
pub struct USBEHCI_RST_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> USBEHCI_RST_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBEHCI_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(USBEHCI_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(USBEHCI_RST_A::DEASSERT)
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
        self.w.bits = (self.w.bits & !(0x01 << O)) | ((value as u32 & 0x01) << O);
        self.w
    }
}
#[doc = "USBOHCI Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBOHCI_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<USBOHCI_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USBOHCI_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `USBOHCI(0-1)_RST` reader - USBOHCI Reset"]
pub struct USBOHCI_RST_R(crate::FieldReader<bool, USBOHCI_RST_A>);
impl USBOHCI_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBOHCI_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBOHCI_RST_A {
        match self.bits {
            false => USBOHCI_RST_A::ASSERT,
            true => USBOHCI_RST_A::DEASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        **self == USBOHCI_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        **self == USBOHCI_RST_A::DEASSERT
    }
}
impl core::ops::Deref for USBOHCI_RST_R {
    type Target = crate::FieldReader<bool, USBOHCI_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `USBOHCI(0-1)_RST` writer - USBOHCI Reset"]
pub struct USBOHCI_RST_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> USBOHCI_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBOHCI_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(USBOHCI_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(USBOHCI_RST_A::DEASSERT)
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
        self.w.bits =
            (self.w.bits & !(0x01 << self.offset)) | ((value as u32 & 0x01) << self.offset);
        self.w
    }
}
#[doc = "Fields `USBOHCI(0-1)_RST` const generic writer - USBOHCI Reset"]
pub struct USBOHCI_RST_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> USBOHCI_RST_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBOHCI_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(USBOHCI_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(USBOHCI_RST_A::DEASSERT)
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
        self.w.bits = (self.w.bits & !(0x01 << O)) | ((value as u32 & 0x01) << O);
        self.w
    }
}
#[doc = "USBOTG0 Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBOTG0_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<USBOTG0_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: USBOTG0_GATING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBOTG0_GATING` reader - USBOTG0 Gating Clock"]
pub struct USBOTG0_GATING_R(crate::FieldReader<bool, USBOTG0_GATING_A>);
impl USBOTG0_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBOTG0_GATING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBOTG0_GATING_A {
        match self.bits {
            false => USBOTG0_GATING_A::MASK,
            true => USBOTG0_GATING_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == USBOTG0_GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == USBOTG0_GATING_A::PASS
    }
}
impl core::ops::Deref for USBOTG0_GATING_R {
    type Target = crate::FieldReader<bool, USBOTG0_GATING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBOTG0_GATING` writer - USBOTG0 Gating Clock"]
pub struct USBOTG0_GATING_W<'a> {
    w: &'a mut W,
}
impl<'a> USBOTG0_GATING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBOTG0_GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(USBOTG0_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(USBOTG0_GATING_A::PASS)
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
#[doc = "USBEHCI Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBEHCI_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<USBEHCI_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: USBEHCI_GATING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `USBEHCI(0-1)_GATING` reader - USBEHCI Gating Clock"]
pub struct USBEHCI_GATING_R(crate::FieldReader<bool, USBEHCI_GATING_A>);
impl USBEHCI_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBEHCI_GATING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBEHCI_GATING_A {
        match self.bits {
            false => USBEHCI_GATING_A::MASK,
            true => USBEHCI_GATING_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == USBEHCI_GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == USBEHCI_GATING_A::PASS
    }
}
impl core::ops::Deref for USBEHCI_GATING_R {
    type Target = crate::FieldReader<bool, USBEHCI_GATING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `USBEHCI(0-1)_GATING` writer - USBEHCI Gating Clock"]
pub struct USBEHCI_GATING_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> USBEHCI_GATING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBEHCI_GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(USBEHCI_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(USBEHCI_GATING_A::PASS)
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
        self.w.bits =
            (self.w.bits & !(0x01 << self.offset)) | ((value as u32 & 0x01) << self.offset);
        self.w
    }
}
#[doc = "Fields `USBEHCI(0-1)_GATING` const generic writer - USBEHCI Gating Clock"]
pub struct USBEHCI_GATING_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> USBEHCI_GATING_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBEHCI_GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(USBEHCI_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(USBEHCI_GATING_A::PASS)
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
        self.w.bits = (self.w.bits & !(0x01 << O)) | ((value as u32 & 0x01) << O);
        self.w
    }
}
#[doc = "USBOHCI Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBOHCI_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<USBOHCI_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: USBOHCI_GATING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `USBOHCI(0-1)_GATING` reader - USBOHCI Gating Clock"]
pub struct USBOHCI_GATING_R(crate::FieldReader<bool, USBOHCI_GATING_A>);
impl USBOHCI_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBOHCI_GATING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBOHCI_GATING_A {
        match self.bits {
            false => USBOHCI_GATING_A::MASK,
            true => USBOHCI_GATING_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == USBOHCI_GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == USBOHCI_GATING_A::PASS
    }
}
impl core::ops::Deref for USBOHCI_GATING_R {
    type Target = crate::FieldReader<bool, USBOHCI_GATING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `USBOHCI(0-1)_GATING` writer - USBOHCI Gating Clock"]
pub struct USBOHCI_GATING_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> USBOHCI_GATING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBOHCI_GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(USBOHCI_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(USBOHCI_GATING_A::PASS)
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
        self.w.bits =
            (self.w.bits & !(0x01 << self.offset)) | ((value as u32 & 0x01) << self.offset);
        self.w
    }
}
#[doc = "Fields `USBOHCI(0-1)_GATING` const generic writer - USBOHCI Gating Clock"]
pub struct USBOHCI_GATING_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> USBOHCI_GATING_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBOHCI_GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(USBOHCI_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(USBOHCI_GATING_A::PASS)
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
        self.w.bits = (self.w.bits & !(0x01 << O)) | ((value as u32 & 0x01) << O);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - USBOTG0 Reset"]
    #[inline(always)]
    pub fn usbotg0_rst(&self) -> USBOTG0_RST_R {
        USBOTG0_RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "USBEHCI Reset"]
    #[inline(always)]
    pub unsafe fn usbehci_rst(&self, n: usize) -> USBEHCI_RST_R {
        USBEHCI_RST_R::new(((self.bits >> (n + 20)) & 0x01) != 0)
    }
    #[doc = "Bit 20 - USBEHCI Reset"]
    #[inline(always)]
    pub fn usbehci0_rst(&self) -> USBEHCI_RST_R {
        USBEHCI_RST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - USBEHCI Reset"]
    #[inline(always)]
    pub fn usbehci1_rst(&self) -> USBEHCI_RST_R {
        USBEHCI_RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "USBOHCI Reset"]
    #[inline(always)]
    pub unsafe fn usbohci_rst(&self, n: usize) -> USBOHCI_RST_R {
        USBOHCI_RST_R::new(((self.bits >> (n + 16)) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USBOHCI Reset"]
    #[inline(always)]
    pub fn usbohci0_rst(&self) -> USBOHCI_RST_R {
        USBOHCI_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - USBOHCI Reset"]
    #[inline(always)]
    pub fn usbohci1_rst(&self) -> USBOHCI_RST_R {
        USBOHCI_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USBOTG0 Gating Clock"]
    #[inline(always)]
    pub fn usbotg0_gating(&self) -> USBOTG0_GATING_R {
        USBOTG0_GATING_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "USBEHCI Gating Clock"]
    #[inline(always)]
    pub unsafe fn usbehci_gating(&self, n: usize) -> USBEHCI_GATING_R {
        USBEHCI_GATING_R::new(((self.bits >> (n + 4)) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USBEHCI Gating Clock"]
    #[inline(always)]
    pub fn usbehci0_gating(&self) -> USBEHCI_GATING_R {
        USBEHCI_GATING_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USBEHCI Gating Clock"]
    #[inline(always)]
    pub fn usbehci1_gating(&self) -> USBEHCI_GATING_R {
        USBEHCI_GATING_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "USBOHCI Gating Clock"]
    #[inline(always)]
    pub unsafe fn usbohci_gating(&self, n: usize) -> USBOHCI_GATING_R {
        USBOHCI_GATING_R::new(((self.bits >> n) & 0x01) != 0)
    }
    #[doc = "Bit 0 - USBOHCI Gating Clock"]
    #[inline(always)]
    pub fn usbohci0_gating(&self) -> USBOHCI_GATING_R {
        USBOHCI_GATING_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USBOHCI Gating Clock"]
    #[inline(always)]
    pub fn usbohci1_gating(&self) -> USBOHCI_GATING_R {
        USBOHCI_GATING_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - USBOTG0 Reset"]
    #[inline(always)]
    pub fn usbotg0_rst(&mut self) -> USBOTG0_RST_W {
        USBOTG0_RST_W { w: self }
    }
    #[doc = "USBEHCI Reset"]
    #[inline(always)]
    pub unsafe fn usbehci_rst(&mut self, n: usize) -> USBEHCI_RST_W {
        USBEHCI_RST_W {
            w: self,
            offset: n + 20,
        }
    }
    #[doc = "Bit 20 - USBEHCI Reset"]
    #[inline(always)]
    pub fn usbehci0_rst(&mut self) -> USBEHCI_RST_CGW<20> {
        USBEHCI_RST_CGW { w: self }
    }
    #[doc = "Bit 21 - USBEHCI Reset"]
    #[inline(always)]
    pub fn usbehci1_rst(&mut self) -> USBEHCI_RST_CGW<21> {
        USBEHCI_RST_CGW { w: self }
    }
    #[doc = "USBOHCI Reset"]
    #[inline(always)]
    pub unsafe fn usbohci_rst(&mut self, n: usize) -> USBOHCI_RST_W {
        USBOHCI_RST_W {
            w: self,
            offset: n + 16,
        }
    }
    #[doc = "Bit 16 - USBOHCI Reset"]
    #[inline(always)]
    pub fn usbohci0_rst(&mut self) -> USBOHCI_RST_CGW<16> {
        USBOHCI_RST_CGW { w: self }
    }
    #[doc = "Bit 17 - USBOHCI Reset"]
    #[inline(always)]
    pub fn usbohci1_rst(&mut self) -> USBOHCI_RST_CGW<17> {
        USBOHCI_RST_CGW { w: self }
    }
    #[doc = "Bit 8 - USBOTG0 Gating Clock"]
    #[inline(always)]
    pub fn usbotg0_gating(&mut self) -> USBOTG0_GATING_W {
        USBOTG0_GATING_W { w: self }
    }
    #[doc = "USBEHCI Gating Clock"]
    #[inline(always)]
    pub unsafe fn usbehci_gating(&mut self, n: usize) -> USBEHCI_GATING_W {
        USBEHCI_GATING_W {
            w: self,
            offset: n + 4,
        }
    }
    #[doc = "Bit 4 - USBEHCI Gating Clock"]
    #[inline(always)]
    pub fn usbehci0_gating(&mut self) -> USBEHCI_GATING_CGW<4> {
        USBEHCI_GATING_CGW { w: self }
    }
    #[doc = "Bit 5 - USBEHCI Gating Clock"]
    #[inline(always)]
    pub fn usbehci1_gating(&mut self) -> USBEHCI_GATING_CGW<5> {
        USBEHCI_GATING_CGW { w: self }
    }
    #[doc = "USBOHCI Gating Clock"]
    #[inline(always)]
    pub unsafe fn usbohci_gating(&mut self, n: usize) -> USBOHCI_GATING_W {
        USBOHCI_GATING_W { w: self, offset: n }
    }
    #[doc = "Bit 0 - USBOHCI Gating Clock"]
    #[inline(always)]
    pub fn usbohci0_gating(&mut self) -> USBOHCI_GATING_CGW<0> {
        USBOHCI_GATING_CGW { w: self }
    }
    #[doc = "Bit 1 - USBOHCI Gating Clock"]
    #[inline(always)]
    pub fn usbohci1_gating(&mut self) -> USBOHCI_GATING_CGW<1> {
        USBOHCI_GATING_CGW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Bus Gating Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_bgr](index.html) module"]
pub struct USB_BGR_SPEC;
impl crate::RegisterSpec for USB_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_bgr::R](R) reader structure"]
impl crate::Readable for USB_BGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_bgr::W](W) writer structure"]
impl crate::Writable for USB_BGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_BGR to value 0"]
impl crate::Resettable for USB_BGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
