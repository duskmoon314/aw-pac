#[doc = "Register `hc_rh_status` reader"]
pub struct R(crate::R<HC_RH_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC_RH_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC_RH_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC_RH_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hc_rh_status` writer"]
pub struct W(crate::W<HC_RH_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC_RH_STATUS_SPEC>;
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
impl From<crate::W<HC_RH_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC_RH_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `r_local_power_status_w_clear_global_power` reader - (Read)LocalPowerStatus\n\nWhen read, this bit returns the LocalPowerStatus of the Root Hub. The Root Hub does not support the local power status feature; thus, this bit is always read as '0'.\n\n(Write)ClearGlobalPower\n\nWhen write, this bit is operated as the ClearGlobalPower. In global power mode ( PowerSwitchingMode =0), This bit is written to '1' to turn off power to all ports (clear PortPowerStatus ). In per-port power mode, it clears PortPowerStatus only on ports whose PortPowerControlMask bit is not set. Writing a '0' has no effect."]
pub type R_LOCAL_POWER_STATUS_W_CLEAR_GLOBAL_POWER_R = crate::BitReader<bool>;
#[doc = "Field `r_local_power_status_w_clear_global_power` writer - (Read)LocalPowerStatus\n\nWhen read, this bit returns the LocalPowerStatus of the Root Hub. The Root Hub does not support the local power status feature; thus, this bit is always read as '0'.\n\n(Write)ClearGlobalPower\n\nWhen write, this bit is operated as the ClearGlobalPower. In global power mode ( PowerSwitchingMode =0), This bit is written to '1' to turn off power to all ports (clear PortPowerStatus ). In per-port power mode, it clears PortPowerStatus only on ports whose PortPowerControlMask bit is not set. Writing a '0' has no effect."]
pub type R_LOCAL_POWER_STATUS_W_CLEAR_GLOBAL_POWER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_STATUS_SPEC, bool, O>;
#[doc = "Field `over_current_indicator` reader - This bit reports overcurrent conditions when the global reporting is implemented. When set, an overcurrent condition exists. When cleared, all power operations are normal. If per-port overcurrent protection is implemented this bit is always '0'"]
pub type OVER_CURRENT_INDICATOR_R = crate::BitReader<bool>;
#[doc = "Field `over_current_indicator` writer - This bit reports overcurrent conditions when the global reporting is implemented. When set, an overcurrent condition exists. When cleared, all power operations are normal. If per-port overcurrent protection is implemented this bit is always '0'"]
pub type OVER_CURRENT_INDICATOR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_STATUS_SPEC, bool, O>;
#[doc = "Field `r_device_remote_wakeup_enable_w_set_remote_wakeup_enable` reader - (read)DeviceRemoteWakeupEnable\n\nThis bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt.\n\n0 ConnectStatusChange is not a remote wakeup event.\n\n1 ConnectStatusChange is a remote wakeup event.\n\n(write)SetRemoteWakeupEnable\n\nWriting a '1' sets DeviceRemoveWakeupEnable. Writing a '0' has no effect."]
pub type R_DEVICE_REMOTE_WAKEUP_ENABLE_W_SET_REMOTE_WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `r_device_remote_wakeup_enable_w_set_remote_wakeup_enable` writer - (read)DeviceRemoteWakeupEnable\n\nThis bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt.\n\n0 ConnectStatusChange is not a remote wakeup event.\n\n1 ConnectStatusChange is a remote wakeup event.\n\n(write)SetRemoteWakeupEnable\n\nWriting a '1' sets DeviceRemoveWakeupEnable. Writing a '0' has no effect."]
pub type R_DEVICE_REMOTE_WAKEUP_ENABLE_W_SET_REMOTE_WAKEUP_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_STATUS_SPEC, bool, O>;
#[doc = "Field `r_local_power_status_w_set_global_power` reader - (read)LocalPowerStatusChange\n\nThe Root Hub does not support the local power status features, thus, this bit is always read as '0'.\n\n(write)SetGlobalPower\n\nIn global power mode ( PowerSwitchingMode =0), This bit is written to '1' to turn on power to all ports (clear PortPowerStatus ). In per-port power mode, it sets PortPowerStatus only on ports whose PortPowerControlMask bit is not set. Writing a '0' has no effect."]
pub type R_LOCAL_POWER_STATUS_W_SET_GLOBAL_POWER_R = crate::BitReader<bool>;
#[doc = "Field `r_local_power_status_w_set_global_power` writer - (read)LocalPowerStatusChange\n\nThe Root Hub does not support the local power status features, thus, this bit is always read as '0'.\n\n(write)SetGlobalPower\n\nIn global power mode ( PowerSwitchingMode =0), This bit is written to '1' to turn on power to all ports (clear PortPowerStatus ). In per-port power mode, it sets PortPowerStatus only on ports whose PortPowerControlMask bit is not set. Writing a '0' has no effect."]
pub type R_LOCAL_POWER_STATUS_W_SET_GLOBAL_POWER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_STATUS_SPEC, bool, O>;
#[doc = "Field `over_current_indicator_change` reader - This bit is set by hardware when a change has occurred to the OverCurrentIndicator field of this register. The HCD clears this bit by writing a '1'. Writing a '0' has no effect."]
pub type OVER_CURRENT_INDICATOR_CHANGE_R = crate::BitReader<bool>;
#[doc = "Field `over_current_indicator_change` writer - This bit is set by hardware when a change has occurred to the OverCurrentIndicator field of this register. The HCD clears this bit by writing a '1'. Writing a '0' has no effect."]
pub type OVER_CURRENT_INDICATOR_CHANGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_STATUS_SPEC, bool, O>;
#[doc = "Field `clear_remote_eakeup_enable` reader - (write)ClearRemoteWakeupEnable\n\nWrite a '1' clears DeviceRemoteWakeupEnable. Writing a '0' has no effect."]
pub type CLEAR_REMOTE_EAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `clear_remote_eakeup_enable` writer - (write)ClearRemoteWakeupEnable\n\nWrite a '1' clears DeviceRemoteWakeupEnable. Writing a '0' has no effect."]
pub type CLEAR_REMOTE_EAKEUP_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - (Read)LocalPowerStatus\n\nWhen read, this bit returns the LocalPowerStatus of the Root Hub. The Root Hub does not support the local power status feature; thus, this bit is always read as '0'.\n\n(Write)ClearGlobalPower\n\nWhen write, this bit is operated as the ClearGlobalPower. In global power mode ( PowerSwitchingMode =0), This bit is written to '1' to turn off power to all ports (clear PortPowerStatus ). In per-port power mode, it clears PortPowerStatus only on ports whose PortPowerControlMask bit is not set. Writing a '0' has no effect."]
    #[inline(always)]
    pub fn r_local_power_status_w_clear_global_power(
        &self,
    ) -> R_LOCAL_POWER_STATUS_W_CLEAR_GLOBAL_POWER_R {
        R_LOCAL_POWER_STATUS_W_CLEAR_GLOBAL_POWER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit reports overcurrent conditions when the global reporting is implemented. When set, an overcurrent condition exists. When cleared, all power operations are normal. If per-port overcurrent protection is implemented this bit is always '0'"]
    #[inline(always)]
    pub fn over_current_indicator(&self) -> OVER_CURRENT_INDICATOR_R {
        OVER_CURRENT_INDICATOR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 15 - (read)DeviceRemoteWakeupEnable\n\nThis bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt.\n\n0 ConnectStatusChange is not a remote wakeup event.\n\n1 ConnectStatusChange is a remote wakeup event.\n\n(write)SetRemoteWakeupEnable\n\nWriting a '1' sets DeviceRemoveWakeupEnable. Writing a '0' has no effect."]
    #[inline(always)]
    pub fn r_device_remote_wakeup_enable_w_set_remote_wakeup_enable(
        &self,
    ) -> R_DEVICE_REMOTE_WAKEUP_ENABLE_W_SET_REMOTE_WAKEUP_ENABLE_R {
        R_DEVICE_REMOTE_WAKEUP_ENABLE_W_SET_REMOTE_WAKEUP_ENABLE_R::new(
            ((self.bits >> 15) & 1) != 0,
        )
    }
    #[doc = "Bit 16 - (read)LocalPowerStatusChange\n\nThe Root Hub does not support the local power status features, thus, this bit is always read as '0'.\n\n(write)SetGlobalPower\n\nIn global power mode ( PowerSwitchingMode =0), This bit is written to '1' to turn on power to all ports (clear PortPowerStatus ). In per-port power mode, it sets PortPowerStatus only on ports whose PortPowerControlMask bit is not set. Writing a '0' has no effect."]
    #[inline(always)]
    pub fn r_local_power_status_w_set_global_power(
        &self,
    ) -> R_LOCAL_POWER_STATUS_W_SET_GLOBAL_POWER_R {
        R_LOCAL_POWER_STATUS_W_SET_GLOBAL_POWER_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit is set by hardware when a change has occurred to the OverCurrentIndicator field of this register. The HCD clears this bit by writing a '1'. Writing a '0' has no effect."]
    #[inline(always)]
    pub fn over_current_indicator_change(&self) -> OVER_CURRENT_INDICATOR_CHANGE_R {
        OVER_CURRENT_INDICATOR_CHANGE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - (write)ClearRemoteWakeupEnable\n\nWrite a '1' clears DeviceRemoteWakeupEnable. Writing a '0' has no effect."]
    #[inline(always)]
    pub fn clear_remote_eakeup_enable(&self) -> CLEAR_REMOTE_EAKEUP_ENABLE_R {
        CLEAR_REMOTE_EAKEUP_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (Read)LocalPowerStatus\n\nWhen read, this bit returns the LocalPowerStatus of the Root Hub. The Root Hub does not support the local power status feature; thus, this bit is always read as '0'.\n\n(Write)ClearGlobalPower\n\nWhen write, this bit is operated as the ClearGlobalPower. In global power mode ( PowerSwitchingMode =0), This bit is written to '1' to turn off power to all ports (clear PortPowerStatus ). In per-port power mode, it clears PortPowerStatus only on ports whose PortPowerControlMask bit is not set. Writing a '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn r_local_power_status_w_clear_global_power(
        &mut self,
    ) -> R_LOCAL_POWER_STATUS_W_CLEAR_GLOBAL_POWER_W<0> {
        R_LOCAL_POWER_STATUS_W_CLEAR_GLOBAL_POWER_W::new(self)
    }
    #[doc = "Bit 1 - This bit reports overcurrent conditions when the global reporting is implemented. When set, an overcurrent condition exists. When cleared, all power operations are normal. If per-port overcurrent protection is implemented this bit is always '0'"]
    #[inline(always)]
    #[must_use]
    pub fn over_current_indicator(&mut self) -> OVER_CURRENT_INDICATOR_W<1> {
        OVER_CURRENT_INDICATOR_W::new(self)
    }
    #[doc = "Bit 15 - (read)DeviceRemoteWakeupEnable\n\nThis bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt.\n\n0 ConnectStatusChange is not a remote wakeup event.\n\n1 ConnectStatusChange is a remote wakeup event.\n\n(write)SetRemoteWakeupEnable\n\nWriting a '1' sets DeviceRemoveWakeupEnable. Writing a '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn r_device_remote_wakeup_enable_w_set_remote_wakeup_enable(
        &mut self,
    ) -> R_DEVICE_REMOTE_WAKEUP_ENABLE_W_SET_REMOTE_WAKEUP_ENABLE_W<15> {
        R_DEVICE_REMOTE_WAKEUP_ENABLE_W_SET_REMOTE_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 16 - (read)LocalPowerStatusChange\n\nThe Root Hub does not support the local power status features, thus, this bit is always read as '0'.\n\n(write)SetGlobalPower\n\nIn global power mode ( PowerSwitchingMode =0), This bit is written to '1' to turn on power to all ports (clear PortPowerStatus ). In per-port power mode, it sets PortPowerStatus only on ports whose PortPowerControlMask bit is not set. Writing a '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn r_local_power_status_w_set_global_power(
        &mut self,
    ) -> R_LOCAL_POWER_STATUS_W_SET_GLOBAL_POWER_W<16> {
        R_LOCAL_POWER_STATUS_W_SET_GLOBAL_POWER_W::new(self)
    }
    #[doc = "Bit 17 - This bit is set by hardware when a change has occurred to the OverCurrentIndicator field of this register. The HCD clears this bit by writing a '1'. Writing a '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn over_current_indicator_change(&mut self) -> OVER_CURRENT_INDICATOR_CHANGE_W<17> {
        OVER_CURRENT_INDICATOR_CHANGE_W::new(self)
    }
    #[doc = "Bit 31 - (write)ClearRemoteWakeupEnable\n\nWrite a '1' clears DeviceRemoteWakeupEnable. Writing a '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn clear_remote_eakeup_enable(&mut self) -> CLEAR_REMOTE_EAKEUP_ENABLE_W<31> {
        CLEAR_REMOTE_EAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI Root Hub Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc_rh_status](index.html) module"]
pub struct HC_RH_STATUS_SPEC;
impl crate::RegisterSpec for HC_RH_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc_rh_status::R](R) reader structure"]
impl crate::Readable for HC_RH_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc_rh_status::W](W) writer structure"]
impl crate::Writable for HC_RH_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_rh_status to value 0"]
impl crate::Resettable for HC_RH_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
