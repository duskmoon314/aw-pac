#[doc = "Register `TP_INT_FIFO_CTRL` reader"]
pub struct R(crate::R<TP_INT_FIFO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TP_INT_FIFO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TP_INT_FIFO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TP_INT_FIFO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TP_INT_FIFO_CTRL` writer"]
pub struct W(crate::W<TP_INT_FIFO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TP_INT_FIFO_CTRL_SPEC>;
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
impl From<crate::W<TP_INT_FIFO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TP_INT_FIFO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TP FIFO Overrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP_OVERRUN_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TP_OVERRUN_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TP_OVERRUN_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP_OVERRUN_IRQ_EN` reader - TP FIFO Overrun Interrupt Enable"]
pub struct TP_OVERRUN_IRQ_EN_R(crate::FieldReader<bool>);
impl TP_OVERRUN_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP_OVERRUN_IRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_OVERRUN_IRQ_EN_A {
        match self.bits {
            false => TP_OVERRUN_IRQ_EN_A::DISABLE,
            true => TP_OVERRUN_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TP_OVERRUN_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TP_OVERRUN_IRQ_EN_A::ENABLE
    }
}
impl core::ops::Deref for TP_OVERRUN_IRQ_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP_OVERRUN_IRQ_EN` writer - TP FIFO Overrun Interrupt Enable"]
pub struct TP_OVERRUN_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TP_OVERRUN_IRQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP_OVERRUN_IRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TP_OVERRUN_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TP_OVERRUN_IRQ_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "TP FIFO Data Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP_DATA_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TP_DATA_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TP_DATA_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP_DATA_IRQ_EN` reader - TP FIFO Data Interrupt Enable"]
pub struct TP_DATA_IRQ_EN_R(crate::FieldReader<bool>);
impl TP_DATA_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP_DATA_IRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_DATA_IRQ_EN_A {
        match self.bits {
            false => TP_DATA_IRQ_EN_A::DISABLE,
            true => TP_DATA_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TP_DATA_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TP_DATA_IRQ_EN_A::ENABLE
    }
}
impl core::ops::Deref for TP_DATA_IRQ_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP_DATA_IRQ_EN` writer - TP FIFO Data Interrupt Enable"]
pub struct TP_DATA_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TP_DATA_IRQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP_DATA_IRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TP_DATA_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TP_DATA_IRQ_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "TP FIFO XY Data Interchange Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP_DATA_XY_CHANGE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TP_DATA_XY_CHANGE_A> for bool {
    #[inline(always)]
    fn from(variant: TP_DATA_XY_CHANGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP_DATA_XY_CHANGE` reader - TP FIFO XY Data Interchange Function Select"]
pub struct TP_DATA_XY_CHANGE_R(crate::FieldReader<bool>);
impl TP_DATA_XY_CHANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP_DATA_XY_CHANGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_DATA_XY_CHANGE_A {
        match self.bits {
            false => TP_DATA_XY_CHANGE_A::DISABLE,
            true => TP_DATA_XY_CHANGE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TP_DATA_XY_CHANGE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TP_DATA_XY_CHANGE_A::ENABLE
    }
}
impl core::ops::Deref for TP_DATA_XY_CHANGE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP_DATA_XY_CHANGE` writer - TP FIFO XY Data Interchange Function Select"]
pub struct TP_DATA_XY_CHANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TP_DATA_XY_CHANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP_DATA_XY_CHANGE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TP_DATA_XY_CHANGE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TP_DATA_XY_CHANGE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `TP_FIFO_TRIG_LEVEL` reader - TP FIFO Data Available Trigger Level"]
pub struct TP_FIFO_TRIG_LEVEL_R(crate::FieldReader<u8>);
impl TP_FIFO_TRIG_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TP_FIFO_TRIG_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TP_FIFO_TRIG_LEVEL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP_FIFO_TRIG_LEVEL` writer - TP FIFO Data Available Trigger Level"]
pub struct TP_FIFO_TRIG_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TP_FIFO_TRIG_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "TP FIFO Data Available DRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP_DATA_ERQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TP_DATA_ERQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TP_DATA_ERQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP_DATA_ERQ_EN` reader - TP FIFO Data Available DRQ Enable"]
pub struct TP_DATA_ERQ_EN_R(crate::FieldReader<bool>);
impl TP_DATA_ERQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP_DATA_ERQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_DATA_ERQ_EN_A {
        match self.bits {
            false => TP_DATA_ERQ_EN_A::DISABLE,
            true => TP_DATA_ERQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TP_DATA_ERQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TP_DATA_ERQ_EN_A::ENABLE
    }
}
impl core::ops::Deref for TP_DATA_ERQ_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP_DATA_ERQ_EN` writer - TP FIFO Data Available DRQ Enable"]
pub struct TP_DATA_ERQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TP_DATA_ERQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP_DATA_ERQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TP_DATA_ERQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TP_DATA_ERQ_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "TP FIFO Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP_FIFO_FLUSH_A {
    #[doc = "1: `1`"]
    FLUSH = 1,
}
impl From<TP_FIFO_FLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: TP_FIFO_FLUSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP_FIFO_FLUSH` reader - TP FIFO Flush"]
pub struct TP_FIFO_FLUSH_R(crate::FieldReader<bool>);
impl TP_FIFO_FLUSH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP_FIFO_FLUSH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TP_FIFO_FLUSH_A> {
        match self.bits {
            true => Some(TP_FIFO_FLUSH_A::FLUSH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        **self == TP_FIFO_FLUSH_A::FLUSH
    }
}
impl core::ops::Deref for TP_FIFO_FLUSH_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP_FIFO_FLUSH` writer - TP FIFO Flush"]
pub struct TP_FIFO_FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TP_FIFO_FLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP_FIFO_FLUSH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(TP_FIFO_FLUSH_A::FLUSH)
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "TP Last Touch (Stylus UP) IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP_UP_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TP_UP_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TP_UP_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP_UP_IRQ_EN` reader - TP Last Touch (Stylus UP) IRQ Enable"]
pub struct TP_UP_IRQ_EN_R(crate::FieldReader<bool>);
impl TP_UP_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP_UP_IRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_UP_IRQ_EN_A {
        match self.bits {
            false => TP_UP_IRQ_EN_A::DISABLE,
            true => TP_UP_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TP_UP_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TP_UP_IRQ_EN_A::ENABLE
    }
}
impl core::ops::Deref for TP_UP_IRQ_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP_UP_IRQ_EN` writer - TP Last Touch (Stylus UP) IRQ Enable"]
pub struct TP_UP_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TP_UP_IRQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP_UP_IRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TP_UP_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TP_UP_IRQ_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "TP First Touch (Stylus DOWN) IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP_DOWN_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TP_DOWN_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TP_DOWN_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP_DOWN_IRQ_EN` reader - TP First Touch (Stylus DOWN) IRQ Enable"]
pub struct TP_DOWN_IRQ_EN_R(crate::FieldReader<bool>);
impl TP_DOWN_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP_DOWN_IRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_DOWN_IRQ_EN_A {
        match self.bits {
            false => TP_DOWN_IRQ_EN_A::DISABLE,
            true => TP_DOWN_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TP_DOWN_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TP_DOWN_IRQ_EN_A::ENABLE
    }
}
impl core::ops::Deref for TP_DOWN_IRQ_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP_DOWN_IRQ_EN` writer - TP First Touch (Stylus DOWN) IRQ Enable"]
pub struct TP_DOWN_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TP_DOWN_IRQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP_DOWN_IRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TP_DOWN_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TP_DOWN_IRQ_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 17 - TP FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn tp_overrun_irq_en(&self) -> TP_OVERRUN_IRQ_EN_R {
        TP_OVERRUN_IRQ_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - TP FIFO Data Interrupt Enable"]
    #[inline(always)]
    pub fn tp_data_irq_en(&self) -> TP_DATA_IRQ_EN_R {
        TP_DATA_IRQ_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 13 - TP FIFO XY Data Interchange Function Select"]
    #[inline(always)]
    pub fn tp_data_xy_change(&self) -> TP_DATA_XY_CHANGE_R {
        TP_DATA_XY_CHANGE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 8:12 - TP FIFO Data Available Trigger Level"]
    #[inline(always)]
    pub fn tp_fifo_trig_level(&self) -> TP_FIFO_TRIG_LEVEL_R {
        TP_FIFO_TRIG_LEVEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - TP FIFO Data Available DRQ Enable"]
    #[inline(always)]
    pub fn tp_data_erq_en(&self) -> TP_DATA_ERQ_EN_R {
        TP_DATA_ERQ_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 4 - TP FIFO Flush"]
    #[inline(always)]
    pub fn tp_fifo_flush(&self) -> TP_FIFO_FLUSH_R {
        TP_FIFO_FLUSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 1 - TP Last Touch (Stylus UP) IRQ Enable"]
    #[inline(always)]
    pub fn tp_up_irq_en(&self) -> TP_UP_IRQ_EN_R {
        TP_UP_IRQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - TP First Touch (Stylus DOWN) IRQ Enable"]
    #[inline(always)]
    pub fn tp_down_irq_en(&self) -> TP_DOWN_IRQ_EN_R {
        TP_DOWN_IRQ_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - TP FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn tp_overrun_irq_en(&mut self) -> TP_OVERRUN_IRQ_EN_W {
        TP_OVERRUN_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 16 - TP FIFO Data Interrupt Enable"]
    #[inline(always)]
    pub fn tp_data_irq_en(&mut self) -> TP_DATA_IRQ_EN_W {
        TP_DATA_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 13 - TP FIFO XY Data Interchange Function Select"]
    #[inline(always)]
    pub fn tp_data_xy_change(&mut self) -> TP_DATA_XY_CHANGE_W {
        TP_DATA_XY_CHANGE_W { w: self }
    }
    #[doc = "Bits 8:12 - TP FIFO Data Available Trigger Level"]
    #[inline(always)]
    pub fn tp_fifo_trig_level(&mut self) -> TP_FIFO_TRIG_LEVEL_W {
        TP_FIFO_TRIG_LEVEL_W { w: self }
    }
    #[doc = "Bit 7 - TP FIFO Data Available DRQ Enable"]
    #[inline(always)]
    pub fn tp_data_erq_en(&mut self) -> TP_DATA_ERQ_EN_W {
        TP_DATA_ERQ_EN_W { w: self }
    }
    #[doc = "Bit 4 - TP FIFO Flush"]
    #[inline(always)]
    pub fn tp_fifo_flush(&mut self) -> TP_FIFO_FLUSH_W {
        TP_FIFO_FLUSH_W { w: self }
    }
    #[doc = "Bit 1 - TP Last Touch (Stylus UP) IRQ Enable"]
    #[inline(always)]
    pub fn tp_up_irq_en(&mut self) -> TP_UP_IRQ_EN_W {
        TP_UP_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 0 - TP First Touch (Stylus DOWN) IRQ Enable"]
    #[inline(always)]
    pub fn tp_down_irq_en(&mut self) -> TP_DOWN_IRQ_EN_W {
        TP_DOWN_IRQ_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TP Interrupt FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tp_int_fifo_ctrl](index.html) module"]
pub struct TP_INT_FIFO_CTRL_SPEC;
impl crate::RegisterSpec for TP_INT_FIFO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tp_int_fifo_ctrl::R](R) reader structure"]
impl crate::Readable for TP_INT_FIFO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tp_int_fifo_ctrl::W](W) writer structure"]
impl crate::Writable for TP_INT_FIFO_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TP_INT_FIFO_CTRL to value 0"]
impl crate::Resettable for TP_INT_FIFO_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
