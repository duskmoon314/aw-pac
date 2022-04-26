#[doc = "Register `LEDC_INT_STS` reader"]
pub struct R(crate::R<LEDC_INT_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEDC_INT_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEDC_INT_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEDC_INT_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LEDC_INT_STS` writer"]
pub struct W(crate::W<LEDC_INT_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEDC_INT_STS_SPEC>;
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
impl From<crate::W<LEDC_INT_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEDC_INT_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO_EMPTY` reader - "]
pub struct FIFO_EMPTY_R(crate::FieldReader<bool>);
impl FIFO_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_EMPTY_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_FULL` reader - "]
pub struct FIFO_FULL_R(crate::FieldReader<bool>);
impl FIFO_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_FULL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_WLW` reader - "]
pub struct FIFO_WLW_R(crate::FieldReader<u8>);
impl FIFO_WLW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIFO_WLW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_WLW_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_OVERFLOW_INT_A {
    #[doc = "0: `0`"]
    NOT_OVERFLOW = 0,
    #[doc = "1: `1`"]
    OVERFLOW = 1,
}
impl From<FIFO_OVERFLOW_INT_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_OVERFLOW_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_OVERFLOW_INT` reader - "]
pub struct FIFO_OVERFLOW_INT_R(crate::FieldReader<bool>);
impl FIFO_OVERFLOW_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_OVERFLOW_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_OVERFLOW_INT_A {
        match self.bits {
            false => FIFO_OVERFLOW_INT_A::NOT_OVERFLOW,
            true => FIFO_OVERFLOW_INT_A::OVERFLOW,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_OVERFLOW`"]
    #[inline(always)]
    pub fn is_not_overflow(&self) -> bool {
        **self == FIFO_OVERFLOW_INT_A::NOT_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        **self == FIFO_OVERFLOW_INT_A::OVERFLOW
    }
}
impl core::ops::Deref for FIFO_OVERFLOW_INT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_OVERFLOW_INT` writer - "]
pub struct FIFO_OVERFLOW_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_OVERFLOW_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_OVERFLOW_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_overflow(self) -> &'a mut W {
        self.variant(FIFO_OVERFLOW_INT_A::NOT_OVERFLOW)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut W {
        self.variant(FIFO_OVERFLOW_INT_A::OVERFLOW)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITDATA_TIMEOUT_INT_A {
    #[doc = "0: `0`"]
    NOT_TIMEOUT = 0,
    #[doc = "1: `1`"]
    TIMEOUT = 1,
}
impl From<WAITDATA_TIMEOUT_INT_A> for bool {
    #[inline(always)]
    fn from(variant: WAITDATA_TIMEOUT_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITDATA_TIMEOUT_INT` reader - "]
pub struct WAITDATA_TIMEOUT_INT_R(crate::FieldReader<bool>);
impl WAITDATA_TIMEOUT_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAITDATA_TIMEOUT_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITDATA_TIMEOUT_INT_A {
        match self.bits {
            false => WAITDATA_TIMEOUT_INT_A::NOT_TIMEOUT,
            true => WAITDATA_TIMEOUT_INT_A::TIMEOUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_TIMEOUT`"]
    #[inline(always)]
    pub fn is_not_timeout(&self) -> bool {
        **self == WAITDATA_TIMEOUT_INT_A::NOT_TIMEOUT
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        **self == WAITDATA_TIMEOUT_INT_A::TIMEOUT
    }
}
impl core::ops::Deref for WAITDATA_TIMEOUT_INT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITDATA_TIMEOUT_INT` writer - "]
pub struct WAITDATA_TIMEOUT_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITDATA_TIMEOUT_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAITDATA_TIMEOUT_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_timeout(self) -> &'a mut W {
        self.variant(WAITDATA_TIMEOUT_INT_A::NOT_TIMEOUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut W {
        self.variant(WAITDATA_TIMEOUT_INT_A::TIMEOUT)
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_CPUREQ_INT_A {
    #[doc = "0: `0`"]
    NOT_REQUEST = 0,
    #[doc = "1: `1`"]
    REQUEST = 1,
}
impl From<FIFO_CPUREQ_INT_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_CPUREQ_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_CPUREQ_INT` reader - "]
pub struct FIFO_CPUREQ_INT_R(crate::FieldReader<bool>);
impl FIFO_CPUREQ_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_CPUREQ_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_CPUREQ_INT_A {
        match self.bits {
            false => FIFO_CPUREQ_INT_A::NOT_REQUEST,
            true => FIFO_CPUREQ_INT_A::REQUEST,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_REQUEST`"]
    #[inline(always)]
    pub fn is_not_request(&self) -> bool {
        **self == FIFO_CPUREQ_INT_A::NOT_REQUEST
    }
    #[doc = "Checks if the value of the field is `REQUEST`"]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        **self == FIFO_CPUREQ_INT_A::REQUEST
    }
}
impl core::ops::Deref for FIFO_CPUREQ_INT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_CPUREQ_INT` writer - "]
pub struct FIFO_CPUREQ_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_CPUREQ_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_CPUREQ_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_request(self) -> &'a mut W {
        self.variant(FIFO_CPUREQ_INT_A::NOT_REQUEST)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn request(self) -> &'a mut W {
        self.variant(FIFO_CPUREQ_INT_A::REQUEST)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEC_TRANS_FINISH_INT_A {
    #[doc = "0: `0`"]
    NOT_TRANS_COMPLETE = 0,
    #[doc = "1: `1`"]
    TRANS_COMPLETE = 1,
}
impl From<LEC_TRANS_FINISH_INT_A> for bool {
    #[inline(always)]
    fn from(variant: LEC_TRANS_FINISH_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEC_TRANS_FINISH_INT` reader - "]
pub struct LEC_TRANS_FINISH_INT_R(crate::FieldReader<bool>);
impl LEC_TRANS_FINISH_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LEC_TRANS_FINISH_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEC_TRANS_FINISH_INT_A {
        match self.bits {
            false => LEC_TRANS_FINISH_INT_A::NOT_TRANS_COMPLETE,
            true => LEC_TRANS_FINISH_INT_A::TRANS_COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_TRANS_COMPLETE`"]
    #[inline(always)]
    pub fn is_not_trans_complete(&self) -> bool {
        **self == LEC_TRANS_FINISH_INT_A::NOT_TRANS_COMPLETE
    }
    #[doc = "Checks if the value of the field is `TRANS_COMPLETE`"]
    #[inline(always)]
    pub fn is_trans_complete(&self) -> bool {
        **self == LEC_TRANS_FINISH_INT_A::TRANS_COMPLETE
    }
}
impl core::ops::Deref for LEC_TRANS_FINISH_INT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEC_TRANS_FINISH_INT` writer - "]
pub struct LEC_TRANS_FINISH_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> LEC_TRANS_FINISH_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEC_TRANS_FINISH_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_trans_complete(self) -> &'a mut W {
        self.variant(LEC_TRANS_FINISH_INT_A::NOT_TRANS_COMPLETE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn trans_complete(self) -> &'a mut W {
        self.variant(LEC_TRANS_FINISH_INT_A::TRANS_COMPLETE)
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
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 10:15"]
    #[inline(always)]
    pub fn fifo_wlw(&self) -> FIFO_WLW_R {
        FIFO_WLW_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fifo_overflow_int(&self) -> FIFO_OVERFLOW_INT_R {
        FIFO_OVERFLOW_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn waitdata_timeout_int(&self) -> WAITDATA_TIMEOUT_INT_R {
        WAITDATA_TIMEOUT_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fifo_cpureq_int(&self) -> FIFO_CPUREQ_INT_R {
        FIFO_CPUREQ_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lec_trans_finish_int(&self) -> LEC_TRANS_FINISH_INT_R {
        LEC_TRANS_FINISH_INT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fifo_overflow_int(&mut self) -> FIFO_OVERFLOW_INT_W {
        FIFO_OVERFLOW_INT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn waitdata_timeout_int(&mut self) -> WAITDATA_TIMEOUT_INT_W {
        WAITDATA_TIMEOUT_INT_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fifo_cpureq_int(&mut self) -> FIFO_CPUREQ_INT_W {
        FIFO_CPUREQ_INT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lec_trans_finish_int(&mut self) -> LEC_TRANS_FINISH_INT_W {
        LEC_TRANS_FINISH_INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_int_sts](index.html) module"]
pub struct LEDC_INT_STS_SPEC;
impl crate::RegisterSpec for LEDC_INT_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ledc_int_sts::R](R) reader structure"]
impl crate::Readable for LEDC_INT_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ledc_int_sts::W](W) writer structure"]
impl crate::Writable for LEDC_INT_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LEDC_INT_STS to value 0"]
impl crate::Resettable for LEDC_INT_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
