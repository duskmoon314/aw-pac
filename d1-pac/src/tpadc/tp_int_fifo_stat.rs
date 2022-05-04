#[doc = "Register `TP_INT_FIFO_STAT` reader"]
pub struct R(crate::R<TP_INT_FIFO_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TP_INT_FIFO_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TP_INT_FIFO_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TP_INT_FIFO_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TP_INT_FIFO_STAT` writer"]
pub struct W(crate::W<TP_INT_FIFO_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TP_INT_FIFO_STAT_SPEC>;
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
impl From<crate::W<TP_INT_FIFO_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TP_INT_FIFO_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TP FIFO Overrun Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_OVERRUN_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<FIFO_OVERRUN_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_OVERRUN_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_OVERRUN_PENDING` reader - TP FIFO Overrun Pending"]
pub struct FIFO_OVERRUN_PENDING_R(crate::FieldReader<bool>);
impl FIFO_OVERRUN_PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_OVERRUN_PENDING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_OVERRUN_PENDING_A {
        match self.bits {
            false => FIFO_OVERRUN_PENDING_A::NO_PENDING,
            true => FIFO_OVERRUN_PENDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        **self == FIFO_OVERRUN_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == FIFO_OVERRUN_PENDING_A::PENDING
    }
}
impl core::ops::Deref for FIFO_OVERRUN_PENDING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_OVERRUN_PENDING` writer - TP FIFO Overrun Pending"]
pub struct FIFO_OVERRUN_PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_OVERRUN_PENDING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_OVERRUN_PENDING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(FIFO_OVERRUN_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(FIFO_OVERRUN_PENDING_A::PENDING)
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
#[doc = "TP FIFO Data Available Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_DATA_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<FIFO_DATA_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_DATA_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_DATA_PENDING` reader - TP FIFO Data Available Pending"]
pub struct FIFO_DATA_PENDING_R(crate::FieldReader<bool>);
impl FIFO_DATA_PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_DATA_PENDING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_DATA_PENDING_A {
        match self.bits {
            false => FIFO_DATA_PENDING_A::NO_PENDING,
            true => FIFO_DATA_PENDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        **self == FIFO_DATA_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == FIFO_DATA_PENDING_A::PENDING
    }
}
impl core::ops::Deref for FIFO_DATA_PENDING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_DATA_PENDING` writer - TP FIFO Data Available Pending"]
pub struct FIFO_DATA_PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_DATA_PENDING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_DATA_PENDING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(FIFO_DATA_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(FIFO_DATA_PENDING_A::PENDING)
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
#[doc = "Field `RXA_CNT` reader - TP FIFO Available Sample Word Count"]
pub struct RXA_CNT_R(crate::FieldReader<u8>);
impl RXA_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXA_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXA_CNT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "TP Idle Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP_IDLE_FLG_A {
    #[doc = "0: `0`"]
    IDLE = 0,
    #[doc = "1: `1`"]
    NOT_IDLE = 1,
}
impl From<TP_IDLE_FLG_A> for bool {
    #[inline(always)]
    fn from(variant: TP_IDLE_FLG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP_IDLE_FLG` reader - TP Idle Flag"]
pub struct TP_IDLE_FLG_R(crate::FieldReader<bool>);
impl TP_IDLE_FLG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP_IDLE_FLG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_IDLE_FLG_A {
        match self.bits {
            false => TP_IDLE_FLG_A::IDLE,
            true => TP_IDLE_FLG_A::NOT_IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == TP_IDLE_FLG_A::IDLE
    }
    #[doc = "Checks if the value of the field is `NOT_IDLE`"]
    #[inline(always)]
    pub fn is_not_idle(&self) -> bool {
        **self == TP_IDLE_FLG_A::NOT_IDLE
    }
}
impl core::ops::Deref for TP_IDLE_FLG_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "TP Last Touch (Stylus UP) Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP_UP_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<TP_UP_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: TP_UP_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP_UP_PENDING` reader - TP Last Touch (Stylus UP) Pending"]
pub struct TP_UP_PENDING_R(crate::FieldReader<bool>);
impl TP_UP_PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP_UP_PENDING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_UP_PENDING_A {
        match self.bits {
            false => TP_UP_PENDING_A::NO_PENDING,
            true => TP_UP_PENDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        **self == TP_UP_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == TP_UP_PENDING_A::PENDING
    }
}
impl core::ops::Deref for TP_UP_PENDING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP_UP_PENDING` writer - TP Last Touch (Stylus UP) Pending"]
pub struct TP_UP_PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> TP_UP_PENDING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP_UP_PENDING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(TP_UP_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(TP_UP_PENDING_A::PENDING)
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
#[doc = "TP First Touch (Stylus DOWN) Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP_DOWN_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<TP_DOWN_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: TP_DOWN_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP_DOWN_PENDING` reader - TP First Touch (Stylus DOWN) Pending"]
pub struct TP_DOWN_PENDING_R(crate::FieldReader<bool>);
impl TP_DOWN_PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP_DOWN_PENDING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_DOWN_PENDING_A {
        match self.bits {
            false => TP_DOWN_PENDING_A::NO_PENDING,
            true => TP_DOWN_PENDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        **self == TP_DOWN_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == TP_DOWN_PENDING_A::PENDING
    }
}
impl core::ops::Deref for TP_DOWN_PENDING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP_DOWN_PENDING` writer - TP First Touch (Stylus DOWN) Pending"]
pub struct TP_DOWN_PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> TP_DOWN_PENDING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP_DOWN_PENDING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(TP_DOWN_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(TP_DOWN_PENDING_A::PENDING)
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
    #[doc = "Bit 17 - TP FIFO Overrun Pending"]
    #[inline(always)]
    pub fn fifo_overrun_pending(&self) -> FIFO_OVERRUN_PENDING_R {
        FIFO_OVERRUN_PENDING_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - TP FIFO Data Available Pending"]
    #[inline(always)]
    pub fn fifo_data_pending(&self) -> FIFO_DATA_PENDING_R {
        FIFO_DATA_PENDING_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 8:13 - TP FIFO Available Sample Word Count"]
    #[inline(always)]
    pub fn rxa_cnt(&self) -> RXA_CNT_R {
        RXA_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 2 - TP Idle Flag"]
    #[inline(always)]
    pub fn tp_idle_flg(&self) -> TP_IDLE_FLG_R {
        TP_IDLE_FLG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - TP Last Touch (Stylus UP) Pending"]
    #[inline(always)]
    pub fn tp_up_pending(&self) -> TP_UP_PENDING_R {
        TP_UP_PENDING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - TP First Touch (Stylus DOWN) Pending"]
    #[inline(always)]
    pub fn tp_down_pending(&self) -> TP_DOWN_PENDING_R {
        TP_DOWN_PENDING_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - TP FIFO Overrun Pending"]
    #[inline(always)]
    pub fn fifo_overrun_pending(&mut self) -> FIFO_OVERRUN_PENDING_W {
        FIFO_OVERRUN_PENDING_W { w: self }
    }
    #[doc = "Bit 16 - TP FIFO Data Available Pending"]
    #[inline(always)]
    pub fn fifo_data_pending(&mut self) -> FIFO_DATA_PENDING_W {
        FIFO_DATA_PENDING_W { w: self }
    }
    #[doc = "Bit 1 - TP Last Touch (Stylus UP) Pending"]
    #[inline(always)]
    pub fn tp_up_pending(&mut self) -> TP_UP_PENDING_W {
        TP_UP_PENDING_W { w: self }
    }
    #[doc = "Bit 0 - TP First Touch (Stylus DOWN) Pending"]
    #[inline(always)]
    pub fn tp_down_pending(&mut self) -> TP_DOWN_PENDING_W {
        TP_DOWN_PENDING_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TP Interrupt FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tp_int_fifo_stat](index.html) module"]
pub struct TP_INT_FIFO_STAT_SPEC;
impl crate::RegisterSpec for TP_INT_FIFO_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tp_int_fifo_stat::R](R) reader structure"]
impl crate::Readable for TP_INT_FIFO_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tp_int_fifo_stat::W](W) writer structure"]
impl crate::Writable for TP_INT_FIFO_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TP_INT_FIFO_STAT to value 0"]
impl crate::Resettable for TP_INT_FIFO_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
