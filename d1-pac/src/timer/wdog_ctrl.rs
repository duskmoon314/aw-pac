#[doc = "Register `wdog_ctrl` reader"]
pub struct R(crate::R<WDOG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wdog_ctrl` writer"]
pub struct W(crate::W<WDOG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOG_CTRL_SPEC>;
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
impl From<crate::W<WDOG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDOG_KEY_FIELD` writer - Watchdog Key Field"]
pub struct WDOG_KEY_FIELD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_KEY_FIELD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 1)) | ((value as u32 & 0x0fff) << 1);
        self.w
    }
}
#[doc = "Watchdog Restart\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG_RESTART_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    RESTART = 1,
}
impl From<WDOG_RESTART_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG_RESTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDOG_RESTART` reader - Watchdog Restart"]
pub struct WDOG_RESTART_R(crate::FieldReader<bool, WDOG_RESTART_A>);
impl WDOG_RESTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDOG_RESTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG_RESTART_A {
        match self.bits {
            false => WDOG_RESTART_A::NO_EFFECT,
            true => WDOG_RESTART_A::RESTART,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == WDOG_RESTART_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `RESTART`"]
    #[inline(always)]
    pub fn is_restart(&self) -> bool {
        **self == WDOG_RESTART_A::RESTART
    }
}
impl core::ops::Deref for WDOG_RESTART_R {
    type Target = crate::FieldReader<bool, WDOG_RESTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOG_RESTART` writer - Watchdog Restart"]
pub struct WDOG_RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_RESTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDOG_RESTART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(WDOG_RESTART_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn restart(self) -> &'a mut W {
        self.variant(WDOG_RESTART_A::RESTART)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog Restart"]
    #[inline(always)]
    pub fn wdog_restart(&self) -> WDOG_RESTART_R {
        WDOG_RESTART_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:12 - Watchdog Key Field"]
    #[inline(always)]
    pub fn wdog_key_field(&mut self) -> WDOG_KEY_FIELD_W {
        WDOG_KEY_FIELD_W { w: self }
    }
    #[doc = "Bit 0 - Watchdog Restart"]
    #[inline(always)]
    pub fn wdog_restart(&mut self) -> WDOG_RESTART_W {
        WDOG_RESTART_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdog_ctrl](index.html) module"]
pub struct WDOG_CTRL_SPEC;
impl crate::RegisterSpec for WDOG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdog_ctrl::R](R) reader structure"]
impl crate::Readable for WDOG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdog_ctrl::W](W) writer structure"]
impl crate::Writable for WDOG_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets wdog_ctrl to value 0"]
impl crate::Resettable for WDOG_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
