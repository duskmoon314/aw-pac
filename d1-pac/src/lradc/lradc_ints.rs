#[doc = "Register `lradc_ints` reader"]
pub struct R(crate::R<LRADC_INTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LRADC_INTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LRADC_INTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LRADC_INTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lradc_ints` writer"]
pub struct W(crate::W<LRADC_INTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LRADC_INTS_SPEC>;
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
impl From<crate::W<LRADC_INTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LRADC_INTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc0_data_pending` reader - ADC0 Data Pending"]
pub type ADC0_DATA_PENDING_R = crate::BitReader<ADC0_DATA_PENDING_A>;
#[doc = "ADC0 Data Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_DATA_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<ADC0_DATA_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_DATA_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_DATA_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_DATA_PENDING_A {
        match self.bits {
            false => ADC0_DATA_PENDING_A::NO_PENDING,
            true => ADC0_DATA_PENDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == ADC0_DATA_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADC0_DATA_PENDING_A::PENDING
    }
}
#[doc = "Field `adc0_data_pending` writer - ADC0 Data Pending"]
pub type ADC0_DATA_PENDING_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, LRADC_INTS_SPEC, ADC0_DATA_PENDING_A, O>;
impl<'a, const O: u8> ADC0_DATA_PENDING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(ADC0_DATA_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ADC0_DATA_PENDING_A::PENDING)
    }
}
#[doc = "Field `adc0_keydown_pending` reader - ADC0 Key Down Pending"]
pub type ADC0_KEYDOWN_PENDING_R = crate::BitReader<ADC0_KEYDOWN_PENDING_A>;
#[doc = "ADC0 Key Down Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_KEYDOWN_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<ADC0_KEYDOWN_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_KEYDOWN_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_KEYDOWN_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_KEYDOWN_PENDING_A {
        match self.bits {
            false => ADC0_KEYDOWN_PENDING_A::NO_PENDING,
            true => ADC0_KEYDOWN_PENDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == ADC0_KEYDOWN_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADC0_KEYDOWN_PENDING_A::PENDING
    }
}
#[doc = "Field `adc0_keydown_pending` writer - ADC0 Key Down Pending"]
pub type ADC0_KEYDOWN_PENDING_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, LRADC_INTS_SPEC, ADC0_KEYDOWN_PENDING_A, O>;
impl<'a, const O: u8> ADC0_KEYDOWN_PENDING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(ADC0_KEYDOWN_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ADC0_KEYDOWN_PENDING_A::PENDING)
    }
}
#[doc = "Field `adc0_hold_pending` reader - ADC0 Hold Key Pending"]
pub type ADC0_HOLD_PENDING_R = crate::BitReader<ADC0_HOLD_PENDING_A>;
#[doc = "ADC0 Hold Key Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_HOLD_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<ADC0_HOLD_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_HOLD_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_HOLD_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_HOLD_PENDING_A {
        match self.bits {
            false => ADC0_HOLD_PENDING_A::NO_PENDING,
            true => ADC0_HOLD_PENDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == ADC0_HOLD_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADC0_HOLD_PENDING_A::PENDING
    }
}
#[doc = "Field `adc0_hold_pending` writer - ADC0 Hold Key Pending"]
pub type ADC0_HOLD_PENDING_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, LRADC_INTS_SPEC, ADC0_HOLD_PENDING_A, O>;
impl<'a, const O: u8> ADC0_HOLD_PENDING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(ADC0_HOLD_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ADC0_HOLD_PENDING_A::PENDING)
    }
}
#[doc = "Field `adc0_alrdy_hold_pending` reader - ADC0 Already Hold Key Pending"]
pub type ADC0_ALRDY_HOLD_PENDING_R = crate::BitReader<ADC0_ALRDY_HOLD_PENDING_A>;
#[doc = "ADC0 Already Hold Key Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_ALRDY_HOLD_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<ADC0_ALRDY_HOLD_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_ALRDY_HOLD_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_ALRDY_HOLD_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_ALRDY_HOLD_PENDING_A {
        match self.bits {
            false => ADC0_ALRDY_HOLD_PENDING_A::NO_PENDING,
            true => ADC0_ALRDY_HOLD_PENDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == ADC0_ALRDY_HOLD_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADC0_ALRDY_HOLD_PENDING_A::PENDING
    }
}
#[doc = "Field `adc0_alrdy_hold_pending` writer - ADC0 Already Hold Key Pending"]
pub type ADC0_ALRDY_HOLD_PENDING_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, LRADC_INTS_SPEC, ADC0_ALRDY_HOLD_PENDING_A, O>;
impl<'a, const O: u8> ADC0_ALRDY_HOLD_PENDING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(ADC0_ALRDY_HOLD_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ADC0_ALRDY_HOLD_PENDING_A::PENDING)
    }
}
#[doc = "Field `adc0_keyup_pending` reader - ADC0 Key Up Pending"]
pub type ADC0_KEYUP_PENDING_R = crate::BitReader<ADC0_KEYUP_PENDING_A>;
#[doc = "ADC0 Key Up Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_KEYUP_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<ADC0_KEYUP_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_KEYUP_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_KEYUP_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_KEYUP_PENDING_A {
        match self.bits {
            false => ADC0_KEYUP_PENDING_A::NO_PENDING,
            true => ADC0_KEYUP_PENDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == ADC0_KEYUP_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADC0_KEYUP_PENDING_A::PENDING
    }
}
#[doc = "Field `adc0_keyup_pending` writer - ADC0 Key Up Pending"]
pub type ADC0_KEYUP_PENDING_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, LRADC_INTS_SPEC, ADC0_KEYUP_PENDING_A, O>;
impl<'a, const O: u8> ADC0_KEYUP_PENDING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(ADC0_KEYUP_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ADC0_KEYUP_PENDING_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 0 - ADC0 Data Pending"]
    #[inline(always)]
    pub fn adc0_data_pending(&self) -> ADC0_DATA_PENDING_R {
        ADC0_DATA_PENDING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC0 Key Down Pending"]
    #[inline(always)]
    pub fn adc0_keydown_pending(&self) -> ADC0_KEYDOWN_PENDING_R {
        ADC0_KEYDOWN_PENDING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC0 Hold Key Pending"]
    #[inline(always)]
    pub fn adc0_hold_pending(&self) -> ADC0_HOLD_PENDING_R {
        ADC0_HOLD_PENDING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC0 Already Hold Key Pending"]
    #[inline(always)]
    pub fn adc0_alrdy_hold_pending(&self) -> ADC0_ALRDY_HOLD_PENDING_R {
        ADC0_ALRDY_HOLD_PENDING_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC0 Key Up Pending"]
    #[inline(always)]
    pub fn adc0_keyup_pending(&self) -> ADC0_KEYUP_PENDING_R {
        ADC0_KEYUP_PENDING_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC0 Data Pending"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_data_pending(&mut self) -> ADC0_DATA_PENDING_W<0> {
        ADC0_DATA_PENDING_W::new(self)
    }
    #[doc = "Bit 1 - ADC0 Key Down Pending"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_keydown_pending(&mut self) -> ADC0_KEYDOWN_PENDING_W<1> {
        ADC0_KEYDOWN_PENDING_W::new(self)
    }
    #[doc = "Bit 2 - ADC0 Hold Key Pending"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_hold_pending(&mut self) -> ADC0_HOLD_PENDING_W<2> {
        ADC0_HOLD_PENDING_W::new(self)
    }
    #[doc = "Bit 3 - ADC0 Already Hold Key Pending"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_alrdy_hold_pending(&mut self) -> ADC0_ALRDY_HOLD_PENDING_W<3> {
        ADC0_ALRDY_HOLD_PENDING_W::new(self)
    }
    #[doc = "Bit 4 - ADC0 Key Up Pending"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_keyup_pending(&mut self) -> ADC0_KEYUP_PENDING_W<4> {
        ADC0_KEYUP_PENDING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LRADC Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lradc_ints](index.html) module"]
pub struct LRADC_INTS_SPEC;
impl crate::RegisterSpec for LRADC_INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lradc_ints::R](R) reader structure"]
impl crate::Readable for LRADC_INTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lradc_ints::W](W) writer structure"]
impl crate::Writable for LRADC_INTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x1f;
}
#[doc = "`reset()` method sets lradc_ints to value 0"]
impl crate::Resettable for LRADC_INTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
