#[doc = "Register `LRADC_INTS` reader"]
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
#[doc = "Register `LRADC_INTS` writer"]
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
#[doc = "ADC0 Key Up Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ADC0_KEYUP_PENDING` reader - ADC0 Key Up Pending"]
pub struct ADC0_KEYUP_PENDING_R(crate::FieldReader<bool>);
impl ADC0_KEYUP_PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC0_KEYUP_PENDING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == ADC0_KEYUP_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == ADC0_KEYUP_PENDING_A::PENDING
    }
}
impl core::ops::Deref for ADC0_KEYUP_PENDING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0_KEYUP_PENDING` writer - ADC0 Key Up Pending"]
pub struct ADC0_KEYUP_PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_KEYUP_PENDING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0_KEYUP_PENDING_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
#[doc = "ADC0 Already Hold Key Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ADC0_ALRDY_HOLD_PENDING` reader - ADC0 Already Hold Key Pending"]
pub struct ADC0_ALRDY_HOLD_PENDING_R(crate::FieldReader<bool>);
impl ADC0_ALRDY_HOLD_PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC0_ALRDY_HOLD_PENDING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == ADC0_ALRDY_HOLD_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == ADC0_ALRDY_HOLD_PENDING_A::PENDING
    }
}
impl core::ops::Deref for ADC0_ALRDY_HOLD_PENDING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0_ALRDY_HOLD_PENDING` writer - ADC0 Already Hold Key Pending"]
pub struct ADC0_ALRDY_HOLD_PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_ALRDY_HOLD_PENDING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0_ALRDY_HOLD_PENDING_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
#[doc = "ADC0 Hold Key Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ADC0_HOLD_PENDING` reader - ADC0 Hold Key Pending"]
pub struct ADC0_HOLD_PENDING_R(crate::FieldReader<bool>);
impl ADC0_HOLD_PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC0_HOLD_PENDING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == ADC0_HOLD_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == ADC0_HOLD_PENDING_A::PENDING
    }
}
impl core::ops::Deref for ADC0_HOLD_PENDING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0_HOLD_PENDING` writer - ADC0 Hold Key Pending"]
pub struct ADC0_HOLD_PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_HOLD_PENDING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0_HOLD_PENDING_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "ADC0 Key Down Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ADC0_KEYDOWN_PENDING` reader - ADC0 Key Down Pending"]
pub struct ADC0_KEYDOWN_PENDING_R(crate::FieldReader<bool>);
impl ADC0_KEYDOWN_PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC0_KEYDOWN_PENDING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == ADC0_KEYDOWN_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == ADC0_KEYDOWN_PENDING_A::PENDING
    }
}
impl core::ops::Deref for ADC0_KEYDOWN_PENDING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0_KEYDOWN_PENDING` writer - ADC0 Key Down Pending"]
pub struct ADC0_KEYDOWN_PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_KEYDOWN_PENDING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0_KEYDOWN_PENDING_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
#[doc = "ADC0 Data Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ADC0_DATA_PENDING` reader - ADC0 Data Pending"]
pub struct ADC0_DATA_PENDING_R(crate::FieldReader<bool>);
impl ADC0_DATA_PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC0_DATA_PENDING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == ADC0_DATA_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == ADC0_DATA_PENDING_A::PENDING
    }
}
impl core::ops::Deref for ADC0_DATA_PENDING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0_DATA_PENDING` writer - ADC0 Data Pending"]
pub struct ADC0_DATA_PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_DATA_PENDING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0_DATA_PENDING_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = "Bit 4 - ADC0 Key Up Pending"]
    #[inline(always)]
    pub fn adc0_keyup_pending(&self) -> ADC0_KEYUP_PENDING_R {
        ADC0_KEYUP_PENDING_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC0 Already Hold Key Pending"]
    #[inline(always)]
    pub fn adc0_alrdy_hold_pending(&self) -> ADC0_ALRDY_HOLD_PENDING_R {
        ADC0_ALRDY_HOLD_PENDING_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC0 Hold Key Pending"]
    #[inline(always)]
    pub fn adc0_hold_pending(&self) -> ADC0_HOLD_PENDING_R {
        ADC0_HOLD_PENDING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - ADC0 Key Down Pending"]
    #[inline(always)]
    pub fn adc0_keydown_pending(&self) -> ADC0_KEYDOWN_PENDING_R {
        ADC0_KEYDOWN_PENDING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - ADC0 Data Pending"]
    #[inline(always)]
    pub fn adc0_data_pending(&self) -> ADC0_DATA_PENDING_R {
        ADC0_DATA_PENDING_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - ADC0 Key Up Pending"]
    #[inline(always)]
    pub fn adc0_keyup_pending(&mut self) -> ADC0_KEYUP_PENDING_W {
        ADC0_KEYUP_PENDING_W { w: self }
    }
    #[doc = "Bit 3 - ADC0 Already Hold Key Pending"]
    #[inline(always)]
    pub fn adc0_alrdy_hold_pending(&mut self) -> ADC0_ALRDY_HOLD_PENDING_W {
        ADC0_ALRDY_HOLD_PENDING_W { w: self }
    }
    #[doc = "Bit 2 - ADC0 Hold Key Pending"]
    #[inline(always)]
    pub fn adc0_hold_pending(&mut self) -> ADC0_HOLD_PENDING_W {
        ADC0_HOLD_PENDING_W { w: self }
    }
    #[doc = "Bit 1 - ADC0 Key Down Pending"]
    #[inline(always)]
    pub fn adc0_keydown_pending(&mut self) -> ADC0_KEYDOWN_PENDING_W {
        ADC0_KEYDOWN_PENDING_W { w: self }
    }
    #[doc = "Bit 0 - ADC0 Data Pending"]
    #[inline(always)]
    pub fn adc0_data_pending(&mut self) -> ADC0_DATA_PENDING_W {
        ADC0_DATA_PENDING_W { w: self }
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
}
#[doc = "`reset()` method sets LRADC_INTS to value 0"]
impl crate::Resettable for LRADC_INTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
