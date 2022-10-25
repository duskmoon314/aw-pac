#[doc = "Register `ledc_int_sts` reader"]
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
#[doc = "Register `ledc_int_sts` writer"]
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
#[doc = "Field `lec_trans_finish_int` reader - "]
pub type LEC_TRANS_FINISH_INT_R = crate::BitReader<LEC_TRANS_FINISH_INT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LEC_TRANS_FINISH_INT_R {
    #[doc = "Get enumerated values variant"]
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
        *self == LEC_TRANS_FINISH_INT_A::NOT_TRANS_COMPLETE
    }
    #[doc = "Checks if the value of the field is `TRANS_COMPLETE`"]
    #[inline(always)]
    pub fn is_trans_complete(&self) -> bool {
        *self == LEC_TRANS_FINISH_INT_A::TRANS_COMPLETE
    }
}
#[doc = "Field `lec_trans_finish_int` writer - "]
pub type LEC_TRANS_FINISH_INT_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, LEDC_INT_STS_SPEC, LEC_TRANS_FINISH_INT_A, O>;
impl<'a, const O: u8> LEC_TRANS_FINISH_INT_W<'a, O> {
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
}
#[doc = "Field `fifo_cpureq_int` reader - "]
pub type FIFO_CPUREQ_INT_R = crate::BitReader<FIFO_CPUREQ_INT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl FIFO_CPUREQ_INT_R {
    #[doc = "Get enumerated values variant"]
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
        *self == FIFO_CPUREQ_INT_A::NOT_REQUEST
    }
    #[doc = "Checks if the value of the field is `REQUEST`"]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == FIFO_CPUREQ_INT_A::REQUEST
    }
}
#[doc = "Field `fifo_cpureq_int` writer - "]
pub type FIFO_CPUREQ_INT_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, LEDC_INT_STS_SPEC, FIFO_CPUREQ_INT_A, O>;
impl<'a, const O: u8> FIFO_CPUREQ_INT_W<'a, O> {
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
}
#[doc = "Field `waitdata_timeout_int` reader - "]
pub type WAITDATA_TIMEOUT_INT_R = crate::BitReader<WAITDATA_TIMEOUT_INT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl WAITDATA_TIMEOUT_INT_R {
    #[doc = "Get enumerated values variant"]
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
        *self == WAITDATA_TIMEOUT_INT_A::NOT_TIMEOUT
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == WAITDATA_TIMEOUT_INT_A::TIMEOUT
    }
}
#[doc = "Field `waitdata_timeout_int` writer - "]
pub type WAITDATA_TIMEOUT_INT_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, LEDC_INT_STS_SPEC, WAITDATA_TIMEOUT_INT_A, O>;
impl<'a, const O: u8> WAITDATA_TIMEOUT_INT_W<'a, O> {
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
}
#[doc = "Field `fifo_overflow_int` reader - "]
pub type FIFO_OVERFLOW_INT_R = crate::BitReader<FIFO_OVERFLOW_INT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl FIFO_OVERFLOW_INT_R {
    #[doc = "Get enumerated values variant"]
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
        *self == FIFO_OVERFLOW_INT_A::NOT_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == FIFO_OVERFLOW_INT_A::OVERFLOW
    }
}
#[doc = "Field `fifo_overflow_int` writer - "]
pub type FIFO_OVERFLOW_INT_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, LEDC_INT_STS_SPEC, FIFO_OVERFLOW_INT_A, O>;
impl<'a, const O: u8> FIFO_OVERFLOW_INT_W<'a, O> {
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
}
#[doc = "Field `fifo_wlw` reader - "]
pub type FIFO_WLW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fifo_full` reader - "]
pub type FIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `fifo_empty` reader - "]
pub type FIFO_EMPTY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lec_trans_finish_int(&self) -> LEC_TRANS_FINISH_INT_R {
        LEC_TRANS_FINISH_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fifo_cpureq_int(&self) -> FIFO_CPUREQ_INT_R {
        FIFO_CPUREQ_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn waitdata_timeout_int(&self) -> WAITDATA_TIMEOUT_INT_R {
        WAITDATA_TIMEOUT_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fifo_overflow_int(&self) -> FIFO_OVERFLOW_INT_R {
        FIFO_OVERFLOW_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 10:15"]
    #[inline(always)]
    pub fn fifo_wlw(&self) -> FIFO_WLW_R {
        FIFO_WLW_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn lec_trans_finish_int(&mut self) -> LEC_TRANS_FINISH_INT_W<0> {
        LEC_TRANS_FINISH_INT_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_cpureq_int(&mut self) -> FIFO_CPUREQ_INT_W<1> {
        FIFO_CPUREQ_INT_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn waitdata_timeout_int(&mut self) -> WAITDATA_TIMEOUT_INT_W<3> {
        WAITDATA_TIMEOUT_INT_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overflow_int(&mut self) -> FIFO_OVERFLOW_INT_W<4> {
        FIFO_OVERFLOW_INT_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x1b;
}
#[doc = "`reset()` method sets ledc_int_sts to value 0"]
impl crate::Resettable for LEDC_INT_STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
