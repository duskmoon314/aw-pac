#[doc = "Register `efuse_hv_pwrswt_ctrl` reader"]
pub struct R(crate::R<EFUSE_HV_PWRSWT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSE_HV_PWRSWT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSE_HV_PWRSWT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSE_HV_PWRSWT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `efuse_hv_pwrswt_ctrl` writer"]
pub struct W(crate::W<EFUSE_HV_PWRSWT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSE_HV_PWRSWT_CTRL_SPEC>;
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
impl From<crate::W<EFUSE_HV_PWRSWT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSE_HV_PWRSWT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `efuse_1_8v_power_switch_control` reader - 1: Open power switch 0: Close power switch"]
pub type EFUSE_1_8V_POWER_SWITCH_CONTROL_R = crate::BitReader<EFUSE_1_8V_POWER_SWITCH_CONTROL_A>;
#[doc = "1: Open power switch 0: Close power switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EFUSE_1_8V_POWER_SWITCH_CONTROL_A {
    #[doc = "1: Open power switch"]
    OPEN = 1,
    #[doc = "0: Close power switch"]
    CLOSE = 0,
}
impl From<EFUSE_1_8V_POWER_SWITCH_CONTROL_A> for bool {
    #[inline(always)]
    fn from(variant: EFUSE_1_8V_POWER_SWITCH_CONTROL_A) -> Self {
        variant as u8 != 0
    }
}
impl EFUSE_1_8V_POWER_SWITCH_CONTROL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EFUSE_1_8V_POWER_SWITCH_CONTROL_A {
        match self.bits {
            true => EFUSE_1_8V_POWER_SWITCH_CONTROL_A::OPEN,
            false => EFUSE_1_8V_POWER_SWITCH_CONTROL_A::CLOSE,
        }
    }
    #[doc = "Checks if the value of the field is `OPEN`"]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == EFUSE_1_8V_POWER_SWITCH_CONTROL_A::OPEN
    }
    #[doc = "Checks if the value of the field is `CLOSE`"]
    #[inline(always)]
    pub fn is_close(&self) -> bool {
        *self == EFUSE_1_8V_POWER_SWITCH_CONTROL_A::CLOSE
    }
}
#[doc = "Field `efuse_1_8v_power_switch_control` writer - 1: Open power switch 0: Close power switch"]
pub type EFUSE_1_8V_POWER_SWITCH_CONTROL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EFUSE_HV_PWRSWT_CTRL_SPEC, EFUSE_1_8V_POWER_SWITCH_CONTROL_A, O>;
impl<'a, const O: u8> EFUSE_1_8V_POWER_SWITCH_CONTROL_W<'a, O> {
    #[doc = "Open power switch"]
    #[inline(always)]
    pub fn open(self) -> &'a mut W {
        self.variant(EFUSE_1_8V_POWER_SWITCH_CONTROL_A::OPEN)
    }
    #[doc = "Close power switch"]
    #[inline(always)]
    pub fn close(self) -> &'a mut W {
        self.variant(EFUSE_1_8V_POWER_SWITCH_CONTROL_A::CLOSE)
    }
}
impl R {
    #[doc = "Bit 0 - 1: Open power switch 0: Close power switch"]
    #[inline(always)]
    pub fn efuse_1_8v_power_switch_control(&self) -> EFUSE_1_8V_POWER_SWITCH_CONTROL_R {
        EFUSE_1_8V_POWER_SWITCH_CONTROL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Open power switch 0: Close power switch"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_1_8v_power_switch_control(&mut self) -> EFUSE_1_8V_POWER_SWITCH_CONTROL_W<0> {
        EFUSE_1_8V_POWER_SWITCH_CONTROL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Efuse High Voltage Power Switch Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_hv_pwrswt_ctrl](index.html) module"]
pub struct EFUSE_HV_PWRSWT_CTRL_SPEC;
impl crate::RegisterSpec for EFUSE_HV_PWRSWT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuse_hv_pwrswt_ctrl::R](R) reader structure"]
impl crate::Readable for EFUSE_HV_PWRSWT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuse_hv_pwrswt_ctrl::W](W) writer structure"]
impl crate::Writable for EFUSE_HV_PWRSWT_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets efuse_hv_pwrswt_ctrl to value 0"]
impl crate::Resettable for EFUSE_HV_PWRSWT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
