#[doc = "Register `GP_DATAL_INTS` reader"]
pub struct R(crate::R<GP_DATAL_INTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_DATAL_INTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_DATAL_INTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_DATAL_INTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP_DATAL_INTS` writer"]
pub struct W(crate::W<GP_DATAL_INTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_DATAL_INTS_SPEC>;
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
impl From<crate::W<GP_DATAL_INTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_DATAL_INTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel Voltage Low Available Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH_LOW_PENGDING_A {
    #[doc = "0: NO Pending IRQ"]
    NO_PENDING = 0,
    #[doc = "1: Channel Voltage Low Available Pending IRQ"]
    PENDING = 1,
}
impl From<CH_LOW_PENGDING_A> for bool {
    #[inline(always)]
    fn from(variant: CH_LOW_PENGDING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `CH(0-1)_LOW_PENGDING` reader - Channel Voltage Low Available Interrupt Status"]
pub struct CH_LOW_PENGDING_R(crate::FieldReader<bool>);
impl CH_LOW_PENGDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH_LOW_PENGDING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH_LOW_PENGDING_A {
        match self.bits {
            false => CH_LOW_PENGDING_A::NO_PENDING,
            true => CH_LOW_PENGDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        **self == CH_LOW_PENGDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == CH_LOW_PENGDING_A::PENDING
    }
}
impl core::ops::Deref for CH_LOW_PENGDING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `CH(0-1)_LOW_PENGDING` const generic writer - Channel Voltage Low Available Interrupt Status"]
pub struct CH_LOW_PENGDING_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> CH_LOW_PENGDING_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH_LOW_PENGDING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "NO Pending IRQ"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(CH_LOW_PENGDING_A::NO_PENDING)
    }
    #[doc = "Channel Voltage Low Available Pending IRQ"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(CH_LOW_PENGDING_A::PENDING)
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
        self.w.bits = (self.w.bits & !(1 << O)) | ((value as u32 & 1) << O);
        self.w
    }
}
impl R {
    #[doc = "Channel Voltage Low Available Interrupt Status"]
    #[inline(always)]
    pub unsafe fn ch_low_pengding(&self, n: usize) -> CH_LOW_PENGDING_R {
        CH_LOW_PENGDING_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Channel Voltage Low Available Interrupt Status"]
    #[inline(always)]
    pub fn ch0_low_pengding(&self) -> CH_LOW_PENGDING_R {
        CH_LOW_PENGDING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Voltage Low Available Interrupt Status"]
    #[inline(always)]
    pub fn ch1_low_pengding(&self) -> CH_LOW_PENGDING_R {
        CH_LOW_PENGDING_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Channel Voltage Low Available Interrupt Status"]
    #[inline(always)]
    pub unsafe fn ch_low_pengding<const O: usize>(&mut self) -> CH_LOW_PENGDING_W<O> {
        CH_LOW_PENGDING_W { w: self }
    }
    #[doc = "Bit 0 - Channel Voltage Low Available Interrupt Status"]
    #[inline(always)]
    pub fn ch0_low_pengding(&mut self) -> CH_LOW_PENGDING_W<0> {
        CH_LOW_PENGDING_W { w: self }
    }
    #[doc = "Bit 1 - Channel Voltage Low Available Interrupt Status"]
    #[inline(always)]
    pub fn ch1_low_pengding(&mut self) -> CH_LOW_PENGDING_W<1> {
        CH_LOW_PENGDING_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPADC Data Low Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_datal_ints](index.html) module"]
pub struct GP_DATAL_INTS_SPEC;
impl crate::RegisterSpec for GP_DATAL_INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp_datal_ints::R](R) reader structure"]
impl crate::Readable for GP_DATAL_INTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_datal_ints::W](W) writer structure"]
impl crate::Writable for GP_DATAL_INTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GP_DATAL_INTS to value 0"]
impl crate::Resettable for GP_DATAL_INTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
