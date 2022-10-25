#[doc = "Register `alarm0_day_set` reader"]
pub struct R(crate::R<ALARM0_DAY_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARM0_DAY_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARM0_DAY_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARM0_DAY_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `alarm0_day_set` writer"]
pub struct W(crate::W<ALARM0_DAY_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARM0_DAY_SET_SPEC>;
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
impl From<crate::W<ALARM0_DAY_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARM0_DAY_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `alarm0_counter` reader - Alarm 0 Counter is based on Day."]
pub type ALARM0_COUNTER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `alarm0_counter` writer - Alarm 0 Counter is based on Day."]
pub type ALARM0_COUNTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ALARM0_DAY_SET_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Alarm 0 Counter is based on Day."]
    #[inline(always)]
    pub fn alarm0_counter(&self) -> ALARM0_COUNTER_R {
        ALARM0_COUNTER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Alarm 0 Counter is based on Day."]
    #[inline(always)]
    #[must_use]
    pub fn alarm0_counter(&mut self) -> ALARM0_COUNTER_W<0> {
        ALARM0_COUNTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm 0 Day Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarm0_day_set](index.html) module"]
pub struct ALARM0_DAY_SET_SPEC;
impl crate::RegisterSpec for ALARM0_DAY_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarm0_day_set::R](R) reader structure"]
impl crate::Readable for ALARM0_DAY_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarm0_day_set::W](W) writer structure"]
impl crate::Writable for ALARM0_DAY_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets alarm0_day_set to value 0"]
impl crate::Resettable for ALARM0_DAY_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
