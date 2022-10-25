#[doc = "Register `alarm_config` reader"]
pub struct R(crate::R<ALARM_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARM_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARM_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARM_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `alarm_config` writer"]
pub struct W(crate::W<ALARM_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARM_CONFIG_SPEC>;
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
impl From<crate::W<ALARM_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARM_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `alarm_wakeup` reader - Configuration of alarm wake up output."]
pub type ALARM_WAKEUP_R = crate::BitReader<ALARM_WAKEUP_A>;
#[doc = "Configuration of alarm wake up output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALARM_WAKEUP_A {
    #[doc = "0: Disable alarm wake up output"]
    DISABLE = 0,
    #[doc = "1: Enable alarm wake up output"]
    ENABLE = 1,
}
impl From<ALARM_WAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM_WAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
impl ALARM_WAKEUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM_WAKEUP_A {
        match self.bits {
            false => ALARM_WAKEUP_A::DISABLE,
            true => ALARM_WAKEUP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ALARM_WAKEUP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ALARM_WAKEUP_A::ENABLE
    }
}
#[doc = "Field `alarm_wakeup` writer - Configuration of alarm wake up output."]
pub type ALARM_WAKEUP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ALARM_CONFIG_SPEC, ALARM_WAKEUP_A, O>;
impl<'a, const O: u8> ALARM_WAKEUP_W<'a, O> {
    #[doc = "Disable alarm wake up output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ALARM_WAKEUP_A::DISABLE)
    }
    #[doc = "Enable alarm wake up output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ALARM_WAKEUP_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Configuration of alarm wake up output."]
    #[inline(always)]
    pub fn alarm_wakeup(&self) -> ALARM_WAKEUP_R {
        ALARM_WAKEUP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configuration of alarm wake up output."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_wakeup(&mut self) -> ALARM_WAKEUP_W<0> {
        ALARM_WAKEUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarm_config](index.html) module"]
pub struct ALARM_CONFIG_SPEC;
impl crate::RegisterSpec for ALARM_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarm_config::R](R) reader structure"]
impl crate::Readable for ALARM_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarm_config::W](W) writer structure"]
impl crate::Writable for ALARM_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets alarm_config to value 0"]
impl crate::Resettable for ALARM_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
