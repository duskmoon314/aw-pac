#[doc = "Register `efuse_hv_pwrswt_ctrl` reader"]
pub type R = crate::R<EFUSE_HV_PWRSWT_CTRL_SPEC>;
#[doc = "Register `efuse_hv_pwrswt_ctrl` writer"]
pub type W = crate::W<EFUSE_HV_PWRSWT_CTRL_SPEC>;
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
    pub const fn variant(&self) -> EFUSE_1_8V_POWER_SWITCH_CONTROL_A {
        match self.bits {
            true => EFUSE_1_8V_POWER_SWITCH_CONTROL_A::OPEN,
            false => EFUSE_1_8V_POWER_SWITCH_CONTROL_A::CLOSE,
        }
    }
    #[doc = "Open power switch"]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == EFUSE_1_8V_POWER_SWITCH_CONTROL_A::OPEN
    }
    #[doc = "Close power switch"]
    #[inline(always)]
    pub fn is_close(&self) -> bool {
        *self == EFUSE_1_8V_POWER_SWITCH_CONTROL_A::CLOSE
    }
}
#[doc = "Field `efuse_1_8v_power_switch_control` writer - 1: Open power switch 0: Close power switch"]
pub type EFUSE_1_8V_POWER_SWITCH_CONTROL_W<'a, REG> =
    crate::BitWriter<'a, REG, EFUSE_1_8V_POWER_SWITCH_CONTROL_A>;
impl<'a, REG> EFUSE_1_8V_POWER_SWITCH_CONTROL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Open power switch"]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(EFUSE_1_8V_POWER_SWITCH_CONTROL_A::OPEN)
    }
    #[doc = "Close power switch"]
    #[inline(always)]
    pub fn close(self) -> &'a mut crate::W<REG> {
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
    pub fn efuse_1_8v_power_switch_control(
        &mut self,
    ) -> EFUSE_1_8V_POWER_SWITCH_CONTROL_W<EFUSE_HV_PWRSWT_CTRL_SPEC> {
        EFUSE_1_8V_POWER_SWITCH_CONTROL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Efuse High Voltage Power Switch Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse_hv_pwrswt_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuse_hv_pwrswt_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EFUSE_HV_PWRSWT_CTRL_SPEC;
impl crate::RegisterSpec for EFUSE_HV_PWRSWT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_hv_pwrswt_ctrl::R`](R) reader structure"]
impl crate::Readable for EFUSE_HV_PWRSWT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`efuse_hv_pwrswt_ctrl::W`](W) writer structure"]
impl crate::Writable for EFUSE_HV_PWRSWT_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets efuse_hv_pwrswt_ctrl to value 0"]
impl crate::Resettable for EFUSE_HV_PWRSWT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
