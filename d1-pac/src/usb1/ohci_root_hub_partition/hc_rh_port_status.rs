#[doc = "Register `hc_rh_port_status` reader"]
pub struct R(crate::R<HC_RH_PORT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC_RH_PORT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC_RH_PORT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC_RH_PORT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hc_rh_port_status` writer"]
pub struct W(crate::W<HC_RH_PORT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC_RH_PORT_STATUS_SPEC>;
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
impl From<crate::W<HC_RH_PORT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC_RH_PORT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `r_current_connect_status_w_clear_port_enable` reader - (read) CurrentConnectStatus\n\nThis bit reflects the current state of the downstream port.\n\n0 No device connected\n\n1 Device connected\n\n(write) ClearPortEnable\n\nThe HCD writes a '1' to clear the PortEnableStatus bit. Writing '0' to this bit has no effect. The CurrentConnectStatus is not affected by any write. Note: This bit is always read '1' when the attached device is nonremovalble (DviceRemoveable\\[NumberDownstreamPort\\])."]
pub type R_CURRENT_CONNECT_STATUS_W_CLEAR_PORT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `r_current_connect_status_w_clear_port_enable` writer - (read) CurrentConnectStatus\n\nThis bit reflects the current state of the downstream port.\n\n0 No device connected\n\n1 Device connected\n\n(write) ClearPortEnable\n\nThe HCD writes a '1' to clear the PortEnableStatus bit. Writing '0' to this bit has no effect. The CurrentConnectStatus is not affected by any write. Note: This bit is always read '1' when the attached device is nonremovalble (DviceRemoveable\\[NumberDownstreamPort\\])."]
pub type R_CURRENT_CONNECT_STATUS_W_CLEAR_PORT_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_PORT_STATUS_SPEC, bool, O>;
#[doc = "Field `r_port_enable_status_w_set_port_enable` reader - (read)PortEnableStatus\n\nThis bit indicates whether the port is enabled or disabled. The Root Hub may clear this bit when an overcurrent condition, disconnect event, switched-off power, or operational bus error such as babble is detected. This change also causes PortEnabledStatusChange to be set. HCD sets this bit by writing SetPortEnable and clears it by writing ClearPortEnable. This bit cannot be set when CurrentConnectStatus is cleared. This bit is also set, if not already, at the completion of a port reset when ResetStatusChange is set or port suspend when SuspendStatusChange is set.\n\n0 port is disabled\n\n1 port is enabled\n\n(write)SetPortEnable\n\nThe HCD sets PortEnableStatus by writing a ‘1’. Writing a ‘0’ has no effect. If CurrentConnectStatus is cleared, this write does not set PortEnableStatus, but instead sets ConnectStatusChange. This informs the driver that it attempted to enable a disconnected Port."]
pub type R_PORT_ENABLE_STATUS_W_SET_PORT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `r_port_enable_status_w_set_port_enable` writer - (read)PortEnableStatus\n\nThis bit indicates whether the port is enabled or disabled. The Root Hub may clear this bit when an overcurrent condition, disconnect event, switched-off power, or operational bus error such as babble is detected. This change also causes PortEnabledStatusChange to be set. HCD sets this bit by writing SetPortEnable and clears it by writing ClearPortEnable. This bit cannot be set when CurrentConnectStatus is cleared. This bit is also set, if not already, at the completion of a port reset when ResetStatusChange is set or port suspend when SuspendStatusChange is set.\n\n0 port is disabled\n\n1 port is enabled\n\n(write)SetPortEnable\n\nThe HCD sets PortEnableStatus by writing a ‘1’. Writing a ‘0’ has no effect. If CurrentConnectStatus is cleared, this write does not set PortEnableStatus, but instead sets ConnectStatusChange. This informs the driver that it attempted to enable a disconnected Port."]
pub type R_PORT_ENABLE_STATUS_W_SET_PORT_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_PORT_STATUS_SPEC, bool, O>;
#[doc = "Field `r_port_suspend_status_w_set_port_suspend` reader - (read) PortSuspendStatus\n\nThis bit indicates the port is suspended or in the resume sequence. It is set by a SetSuspendState write and cleared when PortSuspendStatusChange is set at the end of the resume interval. This bit cannot be set if CurrentConnectStatus is cleared. This bit is also cleared when PortResetStatusChange is set at the end of the port reset or when the HC is placed in the USBRESUME state. If an upstream resume is in progress, it should propagate to the HC.\n\n0 port is not suspended\n\n1 port is suspended\n\n(write) SetPortSuspend\n\nThe HCD sets the PortSuspendStatus bit by writing a '1' to this bit. Writing a '0' has no effect. If CurrentConnectStatus is cleared, this write does not set PortSuspendStatus ; instead it sets ConnectStatusChange. This informs the driver that it attempted to suspend a disconnected port."]
pub type R_PORT_SUSPEND_STATUS_W_SET_PORT_SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `r_port_suspend_status_w_set_port_suspend` writer - (read) PortSuspendStatus\n\nThis bit indicates the port is suspended or in the resume sequence. It is set by a SetSuspendState write and cleared when PortSuspendStatusChange is set at the end of the resume interval. This bit cannot be set if CurrentConnectStatus is cleared. This bit is also cleared when PortResetStatusChange is set at the end of the port reset or when the HC is placed in the USBRESUME state. If an upstream resume is in progress, it should propagate to the HC.\n\n0 port is not suspended\n\n1 port is suspended\n\n(write) SetPortSuspend\n\nThe HCD sets the PortSuspendStatus bit by writing a '1' to this bit. Writing a '0' has no effect. If CurrentConnectStatus is cleared, this write does not set PortSuspendStatus ; instead it sets ConnectStatusChange. This informs the driver that it attempted to suspend a disconnected port."]
pub type R_PORT_SUSPEND_STATUS_W_SET_PORT_SUSPEND_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_PORT_STATUS_SPEC, bool, O>;
#[doc = "Field `r_port_over_current_indicator_w_clear_suspend_status` reader - (read) PortOverCurrentIndicator \n\nThis bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis. If per-port overcurrent reporting is not supported, this bit is set to 0. If cleared, all power operations are normal for this port. If set, an overcurrent condition exists on this port. This bit always reflects the overcurrent input signal. 0 no overcurrent condition. 1 overcurrent condition detected.\n\n(write) ClearSuspendStatus\n\nThe HCD writes a '1' to initiate a resume. Writing a '0' has no effect. A resume is initiated only if PortSuspendStatus is set."]
pub type R_PORT_OVER_CURRENT_INDICATOR_W_CLEAR_SUSPEND_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `r_port_over_current_indicator_w_clear_suspend_status` writer - (read) PortOverCurrentIndicator \n\nThis bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis. If per-port overcurrent reporting is not supported, this bit is set to 0. If cleared, all power operations are normal for this port. If set, an overcurrent condition exists on this port. This bit always reflects the overcurrent input signal. 0 no overcurrent condition. 1 overcurrent condition detected.\n\n(write) ClearSuspendStatus\n\nThe HCD writes a '1' to initiate a resume. Writing a '0' has no effect. A resume is initiated only if PortSuspendStatus is set."]
pub type R_PORT_OVER_CURRENT_INDICATOR_W_CLEAR_SUSPEND_STATUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_PORT_STATUS_SPEC, bool, O>;
#[doc = "Field `r_port_reset_status_w_set_port_reset` reader - (read) PortResetStatus\n\nWhen this bit is set by a write to SetPortReset , port reset signaling is asserted. When reset is completed, this bit is cleared when PortResetStatusChange is set. This bit cannot be set if CurrentConnectStatus is cleared.\n\n0 port reset signal is not active\n\n1 port reset signal is active\n\n(write) SetPortReset\n\nThe HCD sets the port reset signaling by writing a '1' to this bit. Writing a '0' has no effect. If CurrentConnectStatus is cleared, this write does not set PortResetStatus , but instead sets ConnectStatusChange. This informs the driver that it attempted to reset a disconnected port."]
pub type R_PORT_RESET_STATUS_W_SET_PORT_RESET_R = crate::BitReader<bool>;
#[doc = "Field `r_port_reset_status_w_set_port_reset` writer - (read) PortResetStatus\n\nWhen this bit is set by a write to SetPortReset , port reset signaling is asserted. When reset is completed, this bit is cleared when PortResetStatusChange is set. This bit cannot be set if CurrentConnectStatus is cleared.\n\n0 port reset signal is not active\n\n1 port reset signal is active\n\n(write) SetPortReset\n\nThe HCD sets the port reset signaling by writing a '1' to this bit. Writing a '0' has no effect. If CurrentConnectStatus is cleared, this write does not set PortResetStatus , but instead sets ConnectStatusChange. This informs the driver that it attempted to reset a disconnected port."]
pub type R_PORT_RESET_STATUS_W_SET_PORT_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_PORT_STATUS_SPEC, bool, O>;
#[doc = "Field `r_port_power_status_w_set_port_power` reader - (read) PortPowerStatus\n\nThis bit reflects the port’s power status, regardless of the type of power switching implemented. This bit is cleared if an overcurrent condition is detected. HCD sets this bit by writing SetPortPower or SetGlobalPower. HCD clears this bit by writing ClearPortPower or ClearGlobalPower. Which power control switches are enabled is determined by PowerSwitchingMode and PortPortControlMask\\[NumberDownstreamPort\\]. In global switching mode(PowerSwitchingMode=0), Set/ClearGlobalPower controls this bit. In per-port power switching (PowerSwitchingMode=1), if the PortPowerControlMask\\[NDP\\] bit for the port is set, only Set/ClearPortPower commands are enabled. If the mask is not set, only Set/ClearGlobalPower commands are enabled. When port power is disabled, CurrentConnectStatus, PortEnableStatus, PortSuspendStatus, and PortResetStatus should be reset.\n\n0 port power is off\n\n1 port power is on\n\n(write) SetPortPower\n\nThe HCD writes a ‘1’ to set the PortPowerStatus bit. Writing a ‘0’ has no effect.\n\nNote: This bit is always reads ‘1b’ if power switching is not supported."]
pub type R_PORT_POWER_STATUS_W_SET_PORT_POWER_R = crate::BitReader<bool>;
#[doc = "Field `r_port_power_status_w_set_port_power` writer - (read) PortPowerStatus\n\nThis bit reflects the port’s power status, regardless of the type of power switching implemented. This bit is cleared if an overcurrent condition is detected. HCD sets this bit by writing SetPortPower or SetGlobalPower. HCD clears this bit by writing ClearPortPower or ClearGlobalPower. Which power control switches are enabled is determined by PowerSwitchingMode and PortPortControlMask\\[NumberDownstreamPort\\]. In global switching mode(PowerSwitchingMode=0), Set/ClearGlobalPower controls this bit. In per-port power switching (PowerSwitchingMode=1), if the PortPowerControlMask\\[NDP\\] bit for the port is set, only Set/ClearPortPower commands are enabled. If the mask is not set, only Set/ClearGlobalPower commands are enabled. When port power is disabled, CurrentConnectStatus, PortEnableStatus, PortSuspendStatus, and PortResetStatus should be reset.\n\n0 port power is off\n\n1 port power is on\n\n(write) SetPortPower\n\nThe HCD writes a ‘1’ to set the PortPowerStatus bit. Writing a ‘0’ has no effect.\n\nNote: This bit is always reads ‘1b’ if power switching is not supported."]
pub type R_PORT_POWER_STATUS_W_SET_PORT_POWER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_PORT_STATUS_SPEC, bool, O>;
#[doc = "Field `r_low_speed_device_attached_w_clear_port_power` reader - (read) LowSpeedDeviceAttached\n\nThis bit indicates the speed of the device attached to this port. When set, a Low Speed device is attached to this port. When clear, a Full Speed device is attached to this port. This field is valid only when the CurrentConnectStatus is set.\n\n0 full speed device attached\n\n1 low speed device attached\n\n(write) ClearPortPower\n\nThe HCD clears the PortPowerStatus bit by writing a '1' to this bit. Writing a '0' has no effect."]
pub type R_LOW_SPEED_DEVICE_ATTACHED_W_CLEAR_PORT_POWER_R = crate::BitReader<bool>;
#[doc = "Field `r_low_speed_device_attached_w_clear_port_power` writer - (read) LowSpeedDeviceAttached\n\nThis bit indicates the speed of the device attached to this port. When set, a Low Speed device is attached to this port. When clear, a Full Speed device is attached to this port. This field is valid only when the CurrentConnectStatus is set.\n\n0 full speed device attached\n\n1 low speed device attached\n\n(write) ClearPortPower\n\nThe HCD clears the PortPowerStatus bit by writing a '1' to this bit. Writing a '0' has no effect."]
pub type R_LOW_SPEED_DEVICE_ATTACHED_W_CLEAR_PORT_POWER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_PORT_STATUS_SPEC, bool, O>;
#[doc = "Field `connect_status_change` reader - This bit is set whenever a connect or disconnect event occurs. The HCD writes a '1' to clear this bit. Writing a '0' has no effect. If CurrentConnectStatus is cleared when a SetPortReset,SetPortEnable , or SetPortSuspend write occurs, this bit is set to force the driver to re-evaluate the connection status since these writes should not occur if the port is disconnected.\n\nNote: If the DeviceRemovable\\[NDP\\] bit is set, this bit is set only after a Root Hub reset to inform the system that the device is attached."]
pub type CONNECT_STATUS_CHANGE_R = crate::BitReader<CONNECT_STATUS_CHANGE_A>;
#[doc = "This bit is set whenever a connect or disconnect event occurs. The HCD writes a '1' to clear this bit. Writing a '0' has no effect. If CurrentConnectStatus is cleared when a SetPortReset,SetPortEnable , or SetPortSuspend write occurs, this bit is set to force the driver to re-evaluate the connection status since these writes should not occur if the port is disconnected.\n\nNote: If the DeviceRemovable\\[NDP\\] bit is set, this bit is set only after a Root Hub reset to inform the system that the device is attached.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONNECT_STATUS_CHANGE_A {
    #[doc = "0: no change in CurrentConnectStatus"]
    NO_CHANGE = 0,
    #[doc = "1: CurrentConnectStatus has changed"]
    CHANGE = 1,
}
impl From<CONNECT_STATUS_CHANGE_A> for bool {
    #[inline(always)]
    fn from(variant: CONNECT_STATUS_CHANGE_A) -> Self {
        variant as u8 != 0
    }
}
impl CONNECT_STATUS_CHANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONNECT_STATUS_CHANGE_A {
        match self.bits {
            false => CONNECT_STATUS_CHANGE_A::NO_CHANGE,
            true => CONNECT_STATUS_CHANGE_A::CHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == CONNECT_STATUS_CHANGE_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `CHANGE`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == CONNECT_STATUS_CHANGE_A::CHANGE
    }
}
#[doc = "Field `connect_status_change` writer - This bit is set whenever a connect or disconnect event occurs. The HCD writes a '1' to clear this bit. Writing a '0' has no effect. If CurrentConnectStatus is cleared when a SetPortReset,SetPortEnable , or SetPortSuspend write occurs, this bit is set to force the driver to re-evaluate the connection status since these writes should not occur if the port is disconnected.\n\nNote: If the DeviceRemovable\\[NDP\\] bit is set, this bit is set only after a Root Hub reset to inform the system that the device is attached."]
pub type CONNECT_STATUS_CHANGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_PORT_STATUS_SPEC, CONNECT_STATUS_CHANGE_A, O>;
impl<'a, const O: u8> CONNECT_STATUS_CHANGE_W<'a, O> {
    #[doc = "no change in CurrentConnectStatus"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(CONNECT_STATUS_CHANGE_A::NO_CHANGE)
    }
    #[doc = "CurrentConnectStatus has changed"]
    #[inline(always)]
    pub fn change(self) -> &'a mut W {
        self.variant(CONNECT_STATUS_CHANGE_A::CHANGE)
    }
}
#[doc = "Field `port_enable_status_change` reader - This bit is set when hardware events cause the PortEnableStatus bit to be cleared. Changes from HCD writes do not set this bit. The HCD writes a '1' to clear this bit. Writing a '0' has no effect."]
pub type PORT_ENABLE_STATUS_CHANGE_R = crate::BitReader<PORT_ENABLE_STATUS_CHANGE_A>;
#[doc = "This bit is set when hardware events cause the PortEnableStatus bit to be cleared. Changes from HCD writes do not set this bit. The HCD writes a '1' to clear this bit. Writing a '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT_ENABLE_STATUS_CHANGE_A {
    #[doc = "0: no change in PortEnableStatus"]
    NO_CHANGE = 0,
    #[doc = "1: PortEnableStatus has changed"]
    CHANGE = 1,
}
impl From<PORT_ENABLE_STATUS_CHANGE_A> for bool {
    #[inline(always)]
    fn from(variant: PORT_ENABLE_STATUS_CHANGE_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT_ENABLE_STATUS_CHANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT_ENABLE_STATUS_CHANGE_A {
        match self.bits {
            false => PORT_ENABLE_STATUS_CHANGE_A::NO_CHANGE,
            true => PORT_ENABLE_STATUS_CHANGE_A::CHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == PORT_ENABLE_STATUS_CHANGE_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `CHANGE`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == PORT_ENABLE_STATUS_CHANGE_A::CHANGE
    }
}
#[doc = "Field `port_enable_status_change` writer - This bit is set when hardware events cause the PortEnableStatus bit to be cleared. Changes from HCD writes do not set this bit. The HCD writes a '1' to clear this bit. Writing a '0' has no effect."]
pub type PORT_ENABLE_STATUS_CHANGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_PORT_STATUS_SPEC, PORT_ENABLE_STATUS_CHANGE_A, O>;
impl<'a, const O: u8> PORT_ENABLE_STATUS_CHANGE_W<'a, O> {
    #[doc = "no change in PortEnableStatus"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(PORT_ENABLE_STATUS_CHANGE_A::NO_CHANGE)
    }
    #[doc = "PortEnableStatus has changed"]
    #[inline(always)]
    pub fn change(self) -> &'a mut W {
        self.variant(PORT_ENABLE_STATUS_CHANGE_A::CHANGE)
    }
}
#[doc = "Field `port_suspend_status_change` reader - This bit is set when the full resume sequence has been completed. This sequence includes the 20-s resume pulse, LS EOP, and 3-ms resychronization delay. The HCD writes a '1' to clear this bit. Writing a '0' has no effect. This bit is also cleared when ResetStatusChange is set."]
pub type PORT_SUSPEND_STATUS_CHANGE_R = crate::BitReader<PORT_SUSPEND_STATUS_CHANGE_A>;
#[doc = "This bit is set when the full resume sequence has been completed. This sequence includes the 20-s resume pulse, LS EOP, and 3-ms resychronization delay. The HCD writes a '1' to clear this bit. Writing a '0' has no effect. This bit is also cleared when ResetStatusChange is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT_SUSPEND_STATUS_CHANGE_A {
    #[doc = "0: resume is not completed"]
    NOT_COMPLETE = 0,
    #[doc = "1: resume is completed"]
    COMPLETE = 1,
}
impl From<PORT_SUSPEND_STATUS_CHANGE_A> for bool {
    #[inline(always)]
    fn from(variant: PORT_SUSPEND_STATUS_CHANGE_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT_SUSPEND_STATUS_CHANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT_SUSPEND_STATUS_CHANGE_A {
        match self.bits {
            false => PORT_SUSPEND_STATUS_CHANGE_A::NOT_COMPLETE,
            true => PORT_SUSPEND_STATUS_CHANGE_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_COMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == PORT_SUSPEND_STATUS_CHANGE_A::NOT_COMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == PORT_SUSPEND_STATUS_CHANGE_A::COMPLETE
    }
}
#[doc = "Field `port_suspend_status_change` writer - This bit is set when the full resume sequence has been completed. This sequence includes the 20-s resume pulse, LS EOP, and 3-ms resychronization delay. The HCD writes a '1' to clear this bit. Writing a '0' has no effect. This bit is also cleared when ResetStatusChange is set."]
pub type PORT_SUSPEND_STATUS_CHANGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_PORT_STATUS_SPEC, PORT_SUSPEND_STATUS_CHANGE_A, O>;
impl<'a, const O: u8> PORT_SUSPEND_STATUS_CHANGE_W<'a, O> {
    #[doc = "resume is not completed"]
    #[inline(always)]
    pub fn not_complete(self) -> &'a mut W {
        self.variant(PORT_SUSPEND_STATUS_CHANGE_A::NOT_COMPLETE)
    }
    #[doc = "resume is completed"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(PORT_SUSPEND_STATUS_CHANGE_A::COMPLETE)
    }
}
#[doc = "Field `port_over_current_indicator_change` reader - This bit is valid only if overcurrent conditions are reported on a per-port basis. This bit is set when Root Hub changes the PortOverCurrentIndicator bit. The HCD writes a ‘1’ to clear this bit. Writing a ‘0’ has no effect."]
pub type PORT_OVER_CURRENT_INDICATOR_CHANGE_R =
    crate::BitReader<PORT_OVER_CURRENT_INDICATOR_CHANGE_A>;
#[doc = "This bit is valid only if overcurrent conditions are reported on a per-port basis. This bit is set when Root Hub changes the PortOverCurrentIndicator bit. The HCD writes a ‘1’ to clear this bit. Writing a ‘0’ has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT_OVER_CURRENT_INDICATOR_CHANGE_A {
    #[doc = "0: no change in PortOverCurrentIndicator"]
    NO_CHANGE = 0,
    #[doc = "1: PortOverCurrentIndicator has changed"]
    CHANGE = 1,
}
impl From<PORT_OVER_CURRENT_INDICATOR_CHANGE_A> for bool {
    #[inline(always)]
    fn from(variant: PORT_OVER_CURRENT_INDICATOR_CHANGE_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT_OVER_CURRENT_INDICATOR_CHANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT_OVER_CURRENT_INDICATOR_CHANGE_A {
        match self.bits {
            false => PORT_OVER_CURRENT_INDICATOR_CHANGE_A::NO_CHANGE,
            true => PORT_OVER_CURRENT_INDICATOR_CHANGE_A::CHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == PORT_OVER_CURRENT_INDICATOR_CHANGE_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `CHANGE`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == PORT_OVER_CURRENT_INDICATOR_CHANGE_A::CHANGE
    }
}
#[doc = "Field `port_over_current_indicator_change` writer - This bit is valid only if overcurrent conditions are reported on a per-port basis. This bit is set when Root Hub changes the PortOverCurrentIndicator bit. The HCD writes a ‘1’ to clear this bit. Writing a ‘0’ has no effect."]
pub type PORT_OVER_CURRENT_INDICATOR_CHANGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_PORT_STATUS_SPEC, PORT_OVER_CURRENT_INDICATOR_CHANGE_A, O>;
impl<'a, const O: u8> PORT_OVER_CURRENT_INDICATOR_CHANGE_W<'a, O> {
    #[doc = "no change in PortOverCurrentIndicator"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(PORT_OVER_CURRENT_INDICATOR_CHANGE_A::NO_CHANGE)
    }
    #[doc = "PortOverCurrentIndicator has changed"]
    #[inline(always)]
    pub fn change(self) -> &'a mut W {
        self.variant(PORT_OVER_CURRENT_INDICATOR_CHANGE_A::CHANGE)
    }
}
#[doc = "Field `port_reset_status_change` reader - This bit is set at the end of the 10ms port reset signal. The HCD writes a '1' to clear this bit. Writing a '0' has no effect."]
pub type PORT_RESET_STATUS_CHANGE_R = crate::BitReader<PORT_RESET_STATUS_CHANGE_A>;
#[doc = "This bit is set at the end of the 10ms port reset signal. The HCD writes a '1' to clear this bit. Writing a '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT_RESET_STATUS_CHANGE_A {
    #[doc = "0: port reset is not complete"]
    NOC_COMPLETE = 0,
    #[doc = "1: port reset is complete"]
    COMPLETE = 1,
}
impl From<PORT_RESET_STATUS_CHANGE_A> for bool {
    #[inline(always)]
    fn from(variant: PORT_RESET_STATUS_CHANGE_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT_RESET_STATUS_CHANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT_RESET_STATUS_CHANGE_A {
        match self.bits {
            false => PORT_RESET_STATUS_CHANGE_A::NOC_COMPLETE,
            true => PORT_RESET_STATUS_CHANGE_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOC_COMPLETE`"]
    #[inline(always)]
    pub fn is_noc_complete(&self) -> bool {
        *self == PORT_RESET_STATUS_CHANGE_A::NOC_COMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == PORT_RESET_STATUS_CHANGE_A::COMPLETE
    }
}
#[doc = "Field `port_reset_status_change` writer - This bit is set at the end of the 10ms port reset signal. The HCD writes a '1' to clear this bit. Writing a '0' has no effect."]
pub type PORT_RESET_STATUS_CHANGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_PORT_STATUS_SPEC, PORT_RESET_STATUS_CHANGE_A, O>;
impl<'a, const O: u8> PORT_RESET_STATUS_CHANGE_W<'a, O> {
    #[doc = "port reset is not complete"]
    #[inline(always)]
    pub fn noc_complete(self) -> &'a mut W {
        self.variant(PORT_RESET_STATUS_CHANGE_A::NOC_COMPLETE)
    }
    #[doc = "port reset is complete"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(PORT_RESET_STATUS_CHANGE_A::COMPLETE)
    }
}
impl R {
    #[doc = "Bit 0 - (read) CurrentConnectStatus\n\nThis bit reflects the current state of the downstream port.\n\n0 No device connected\n\n1 Device connected\n\n(write) ClearPortEnable\n\nThe HCD writes a '1' to clear the PortEnableStatus bit. Writing '0' to this bit has no effect. The CurrentConnectStatus is not affected by any write. Note: This bit is always read '1' when the attached device is nonremovalble (DviceRemoveable\\[NumberDownstreamPort\\])."]
    #[inline(always)]
    pub fn r_current_connect_status_w_clear_port_enable(
        &self,
    ) -> R_CURRENT_CONNECT_STATUS_W_CLEAR_PORT_ENABLE_R {
        R_CURRENT_CONNECT_STATUS_W_CLEAR_PORT_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (read)PortEnableStatus\n\nThis bit indicates whether the port is enabled or disabled. The Root Hub may clear this bit when an overcurrent condition, disconnect event, switched-off power, or operational bus error such as babble is detected. This change also causes PortEnabledStatusChange to be set. HCD sets this bit by writing SetPortEnable and clears it by writing ClearPortEnable. This bit cannot be set when CurrentConnectStatus is cleared. This bit is also set, if not already, at the completion of a port reset when ResetStatusChange is set or port suspend when SuspendStatusChange is set.\n\n0 port is disabled\n\n1 port is enabled\n\n(write)SetPortEnable\n\nThe HCD sets PortEnableStatus by writing a ‘1’. Writing a ‘0’ has no effect. If CurrentConnectStatus is cleared, this write does not set PortEnableStatus, but instead sets ConnectStatusChange. This informs the driver that it attempted to enable a disconnected Port."]
    #[inline(always)]
    pub fn r_port_enable_status_w_set_port_enable(
        &self,
    ) -> R_PORT_ENABLE_STATUS_W_SET_PORT_ENABLE_R {
        R_PORT_ENABLE_STATUS_W_SET_PORT_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - (read) PortSuspendStatus\n\nThis bit indicates the port is suspended or in the resume sequence. It is set by a SetSuspendState write and cleared when PortSuspendStatusChange is set at the end of the resume interval. This bit cannot be set if CurrentConnectStatus is cleared. This bit is also cleared when PortResetStatusChange is set at the end of the port reset or when the HC is placed in the USBRESUME state. If an upstream resume is in progress, it should propagate to the HC.\n\n0 port is not suspended\n\n1 port is suspended\n\n(write) SetPortSuspend\n\nThe HCD sets the PortSuspendStatus bit by writing a '1' to this bit. Writing a '0' has no effect. If CurrentConnectStatus is cleared, this write does not set PortSuspendStatus ; instead it sets ConnectStatusChange. This informs the driver that it attempted to suspend a disconnected port."]
    #[inline(always)]
    pub fn r_port_suspend_status_w_set_port_suspend(
        &self,
    ) -> R_PORT_SUSPEND_STATUS_W_SET_PORT_SUSPEND_R {
        R_PORT_SUSPEND_STATUS_W_SET_PORT_SUSPEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - (read) PortOverCurrentIndicator \n\nThis bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis. If per-port overcurrent reporting is not supported, this bit is set to 0. If cleared, all power operations are normal for this port. If set, an overcurrent condition exists on this port. This bit always reflects the overcurrent input signal. 0 no overcurrent condition. 1 overcurrent condition detected.\n\n(write) ClearSuspendStatus\n\nThe HCD writes a '1' to initiate a resume. Writing a '0' has no effect. A resume is initiated only if PortSuspendStatus is set."]
    #[inline(always)]
    pub fn r_port_over_current_indicator_w_clear_suspend_status(
        &self,
    ) -> R_PORT_OVER_CURRENT_INDICATOR_W_CLEAR_SUSPEND_STATUS_R {
        R_PORT_OVER_CURRENT_INDICATOR_W_CLEAR_SUSPEND_STATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - (read) PortResetStatus\n\nWhen this bit is set by a write to SetPortReset , port reset signaling is asserted. When reset is completed, this bit is cleared when PortResetStatusChange is set. This bit cannot be set if CurrentConnectStatus is cleared.\n\n0 port reset signal is not active\n\n1 port reset signal is active\n\n(write) SetPortReset\n\nThe HCD sets the port reset signaling by writing a '1' to this bit. Writing a '0' has no effect. If CurrentConnectStatus is cleared, this write does not set PortResetStatus , but instead sets ConnectStatusChange. This informs the driver that it attempted to reset a disconnected port."]
    #[inline(always)]
    pub fn r_port_reset_status_w_set_port_reset(&self) -> R_PORT_RESET_STATUS_W_SET_PORT_RESET_R {
        R_PORT_RESET_STATUS_W_SET_PORT_RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - (read) PortPowerStatus\n\nThis bit reflects the port’s power status, regardless of the type of power switching implemented. This bit is cleared if an overcurrent condition is detected. HCD sets this bit by writing SetPortPower or SetGlobalPower. HCD clears this bit by writing ClearPortPower or ClearGlobalPower. Which power control switches are enabled is determined by PowerSwitchingMode and PortPortControlMask\\[NumberDownstreamPort\\]. In global switching mode(PowerSwitchingMode=0), Set/ClearGlobalPower controls this bit. In per-port power switching (PowerSwitchingMode=1), if the PortPowerControlMask\\[NDP\\] bit for the port is set, only Set/ClearPortPower commands are enabled. If the mask is not set, only Set/ClearGlobalPower commands are enabled. When port power is disabled, CurrentConnectStatus, PortEnableStatus, PortSuspendStatus, and PortResetStatus should be reset.\n\n0 port power is off\n\n1 port power is on\n\n(write) SetPortPower\n\nThe HCD writes a ‘1’ to set the PortPowerStatus bit. Writing a ‘0’ has no effect.\n\nNote: This bit is always reads ‘1b’ if power switching is not supported."]
    #[inline(always)]
    pub fn r_port_power_status_w_set_port_power(&self) -> R_PORT_POWER_STATUS_W_SET_PORT_POWER_R {
        R_PORT_POWER_STATUS_W_SET_PORT_POWER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - (read) LowSpeedDeviceAttached\n\nThis bit indicates the speed of the device attached to this port. When set, a Low Speed device is attached to this port. When clear, a Full Speed device is attached to this port. This field is valid only when the CurrentConnectStatus is set.\n\n0 full speed device attached\n\n1 low speed device attached\n\n(write) ClearPortPower\n\nThe HCD clears the PortPowerStatus bit by writing a '1' to this bit. Writing a '0' has no effect."]
    #[inline(always)]
    pub fn r_low_speed_device_attached_w_clear_port_power(
        &self,
    ) -> R_LOW_SPEED_DEVICE_ATTACHED_W_CLEAR_PORT_POWER_R {
        R_LOW_SPEED_DEVICE_ATTACHED_W_CLEAR_PORT_POWER_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit is set whenever a connect or disconnect event occurs. The HCD writes a '1' to clear this bit. Writing a '0' has no effect. If CurrentConnectStatus is cleared when a SetPortReset,SetPortEnable , or SetPortSuspend write occurs, this bit is set to force the driver to re-evaluate the connection status since these writes should not occur if the port is disconnected.\n\nNote: If the DeviceRemovable\\[NDP\\] bit is set, this bit is set only after a Root Hub reset to inform the system that the device is attached."]
    #[inline(always)]
    pub fn connect_status_change(&self) -> CONNECT_STATUS_CHANGE_R {
        CONNECT_STATUS_CHANGE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit is set when hardware events cause the PortEnableStatus bit to be cleared. Changes from HCD writes do not set this bit. The HCD writes a '1' to clear this bit. Writing a '0' has no effect."]
    #[inline(always)]
    pub fn port_enable_status_change(&self) -> PORT_ENABLE_STATUS_CHANGE_R {
        PORT_ENABLE_STATUS_CHANGE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This bit is set when the full resume sequence has been completed. This sequence includes the 20-s resume pulse, LS EOP, and 3-ms resychronization delay. The HCD writes a '1' to clear this bit. Writing a '0' has no effect. This bit is also cleared when ResetStatusChange is set."]
    #[inline(always)]
    pub fn port_suspend_status_change(&self) -> PORT_SUSPEND_STATUS_CHANGE_R {
        PORT_SUSPEND_STATUS_CHANGE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This bit is valid only if overcurrent conditions are reported on a per-port basis. This bit is set when Root Hub changes the PortOverCurrentIndicator bit. The HCD writes a ‘1’ to clear this bit. Writing a ‘0’ has no effect."]
    #[inline(always)]
    pub fn port_over_current_indicator_change(&self) -> PORT_OVER_CURRENT_INDICATOR_CHANGE_R {
        PORT_OVER_CURRENT_INDICATOR_CHANGE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This bit is set at the end of the 10ms port reset signal. The HCD writes a '1' to clear this bit. Writing a '0' has no effect."]
    #[inline(always)]
    pub fn port_reset_status_change(&self) -> PORT_RESET_STATUS_CHANGE_R {
        PORT_RESET_STATUS_CHANGE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (read) CurrentConnectStatus\n\nThis bit reflects the current state of the downstream port.\n\n0 No device connected\n\n1 Device connected\n\n(write) ClearPortEnable\n\nThe HCD writes a '1' to clear the PortEnableStatus bit. Writing '0' to this bit has no effect. The CurrentConnectStatus is not affected by any write. Note: This bit is always read '1' when the attached device is nonremovalble (DviceRemoveable\\[NumberDownstreamPort\\])."]
    #[inline(always)]
    #[must_use]
    pub fn r_current_connect_status_w_clear_port_enable(
        &mut self,
    ) -> R_CURRENT_CONNECT_STATUS_W_CLEAR_PORT_ENABLE_W<0> {
        R_CURRENT_CONNECT_STATUS_W_CLEAR_PORT_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - (read)PortEnableStatus\n\nThis bit indicates whether the port is enabled or disabled. The Root Hub may clear this bit when an overcurrent condition, disconnect event, switched-off power, or operational bus error such as babble is detected. This change also causes PortEnabledStatusChange to be set. HCD sets this bit by writing SetPortEnable and clears it by writing ClearPortEnable. This bit cannot be set when CurrentConnectStatus is cleared. This bit is also set, if not already, at the completion of a port reset when ResetStatusChange is set or port suspend when SuspendStatusChange is set.\n\n0 port is disabled\n\n1 port is enabled\n\n(write)SetPortEnable\n\nThe HCD sets PortEnableStatus by writing a ‘1’. Writing a ‘0’ has no effect. If CurrentConnectStatus is cleared, this write does not set PortEnableStatus, but instead sets ConnectStatusChange. This informs the driver that it attempted to enable a disconnected Port."]
    #[inline(always)]
    #[must_use]
    pub fn r_port_enable_status_w_set_port_enable(
        &mut self,
    ) -> R_PORT_ENABLE_STATUS_W_SET_PORT_ENABLE_W<1> {
        R_PORT_ENABLE_STATUS_W_SET_PORT_ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - (read) PortSuspendStatus\n\nThis bit indicates the port is suspended or in the resume sequence. It is set by a SetSuspendState write and cleared when PortSuspendStatusChange is set at the end of the resume interval. This bit cannot be set if CurrentConnectStatus is cleared. This bit is also cleared when PortResetStatusChange is set at the end of the port reset or when the HC is placed in the USBRESUME state. If an upstream resume is in progress, it should propagate to the HC.\n\n0 port is not suspended\n\n1 port is suspended\n\n(write) SetPortSuspend\n\nThe HCD sets the PortSuspendStatus bit by writing a '1' to this bit. Writing a '0' has no effect. If CurrentConnectStatus is cleared, this write does not set PortSuspendStatus ; instead it sets ConnectStatusChange. This informs the driver that it attempted to suspend a disconnected port."]
    #[inline(always)]
    #[must_use]
    pub fn r_port_suspend_status_w_set_port_suspend(
        &mut self,
    ) -> R_PORT_SUSPEND_STATUS_W_SET_PORT_SUSPEND_W<2> {
        R_PORT_SUSPEND_STATUS_W_SET_PORT_SUSPEND_W::new(self)
    }
    #[doc = "Bit 3 - (read) PortOverCurrentIndicator \n\nThis bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis. If per-port overcurrent reporting is not supported, this bit is set to 0. If cleared, all power operations are normal for this port. If set, an overcurrent condition exists on this port. This bit always reflects the overcurrent input signal. 0 no overcurrent condition. 1 overcurrent condition detected.\n\n(write) ClearSuspendStatus\n\nThe HCD writes a '1' to initiate a resume. Writing a '0' has no effect. A resume is initiated only if PortSuspendStatus is set."]
    #[inline(always)]
    #[must_use]
    pub fn r_port_over_current_indicator_w_clear_suspend_status(
        &mut self,
    ) -> R_PORT_OVER_CURRENT_INDICATOR_W_CLEAR_SUSPEND_STATUS_W<3> {
        R_PORT_OVER_CURRENT_INDICATOR_W_CLEAR_SUSPEND_STATUS_W::new(self)
    }
    #[doc = "Bit 4 - (read) PortResetStatus\n\nWhen this bit is set by a write to SetPortReset , port reset signaling is asserted. When reset is completed, this bit is cleared when PortResetStatusChange is set. This bit cannot be set if CurrentConnectStatus is cleared.\n\n0 port reset signal is not active\n\n1 port reset signal is active\n\n(write) SetPortReset\n\nThe HCD sets the port reset signaling by writing a '1' to this bit. Writing a '0' has no effect. If CurrentConnectStatus is cleared, this write does not set PortResetStatus , but instead sets ConnectStatusChange. This informs the driver that it attempted to reset a disconnected port."]
    #[inline(always)]
    #[must_use]
    pub fn r_port_reset_status_w_set_port_reset(
        &mut self,
    ) -> R_PORT_RESET_STATUS_W_SET_PORT_RESET_W<4> {
        R_PORT_RESET_STATUS_W_SET_PORT_RESET_W::new(self)
    }
    #[doc = "Bit 8 - (read) PortPowerStatus\n\nThis bit reflects the port’s power status, regardless of the type of power switching implemented. This bit is cleared if an overcurrent condition is detected. HCD sets this bit by writing SetPortPower or SetGlobalPower. HCD clears this bit by writing ClearPortPower or ClearGlobalPower. Which power control switches are enabled is determined by PowerSwitchingMode and PortPortControlMask\\[NumberDownstreamPort\\]. In global switching mode(PowerSwitchingMode=0), Set/ClearGlobalPower controls this bit. In per-port power switching (PowerSwitchingMode=1), if the PortPowerControlMask\\[NDP\\] bit for the port is set, only Set/ClearPortPower commands are enabled. If the mask is not set, only Set/ClearGlobalPower commands are enabled. When port power is disabled, CurrentConnectStatus, PortEnableStatus, PortSuspendStatus, and PortResetStatus should be reset.\n\n0 port power is off\n\n1 port power is on\n\n(write) SetPortPower\n\nThe HCD writes a ‘1’ to set the PortPowerStatus bit. Writing a ‘0’ has no effect.\n\nNote: This bit is always reads ‘1b’ if power switching is not supported."]
    #[inline(always)]
    #[must_use]
    pub fn r_port_power_status_w_set_port_power(
        &mut self,
    ) -> R_PORT_POWER_STATUS_W_SET_PORT_POWER_W<8> {
        R_PORT_POWER_STATUS_W_SET_PORT_POWER_W::new(self)
    }
    #[doc = "Bit 9 - (read) LowSpeedDeviceAttached\n\nThis bit indicates the speed of the device attached to this port. When set, a Low Speed device is attached to this port. When clear, a Full Speed device is attached to this port. This field is valid only when the CurrentConnectStatus is set.\n\n0 full speed device attached\n\n1 low speed device attached\n\n(write) ClearPortPower\n\nThe HCD clears the PortPowerStatus bit by writing a '1' to this bit. Writing a '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn r_low_speed_device_attached_w_clear_port_power(
        &mut self,
    ) -> R_LOW_SPEED_DEVICE_ATTACHED_W_CLEAR_PORT_POWER_W<9> {
        R_LOW_SPEED_DEVICE_ATTACHED_W_CLEAR_PORT_POWER_W::new(self)
    }
    #[doc = "Bit 16 - This bit is set whenever a connect or disconnect event occurs. The HCD writes a '1' to clear this bit. Writing a '0' has no effect. If CurrentConnectStatus is cleared when a SetPortReset,SetPortEnable , or SetPortSuspend write occurs, this bit is set to force the driver to re-evaluate the connection status since these writes should not occur if the port is disconnected.\n\nNote: If the DeviceRemovable\\[NDP\\] bit is set, this bit is set only after a Root Hub reset to inform the system that the device is attached."]
    #[inline(always)]
    #[must_use]
    pub fn connect_status_change(&mut self) -> CONNECT_STATUS_CHANGE_W<16> {
        CONNECT_STATUS_CHANGE_W::new(self)
    }
    #[doc = "Bit 17 - This bit is set when hardware events cause the PortEnableStatus bit to be cleared. Changes from HCD writes do not set this bit. The HCD writes a '1' to clear this bit. Writing a '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn port_enable_status_change(&mut self) -> PORT_ENABLE_STATUS_CHANGE_W<17> {
        PORT_ENABLE_STATUS_CHANGE_W::new(self)
    }
    #[doc = "Bit 18 - This bit is set when the full resume sequence has been completed. This sequence includes the 20-s resume pulse, LS EOP, and 3-ms resychronization delay. The HCD writes a '1' to clear this bit. Writing a '0' has no effect. This bit is also cleared when ResetStatusChange is set."]
    #[inline(always)]
    #[must_use]
    pub fn port_suspend_status_change(&mut self) -> PORT_SUSPEND_STATUS_CHANGE_W<18> {
        PORT_SUSPEND_STATUS_CHANGE_W::new(self)
    }
    #[doc = "Bit 19 - This bit is valid only if overcurrent conditions are reported on a per-port basis. This bit is set when Root Hub changes the PortOverCurrentIndicator bit. The HCD writes a ‘1’ to clear this bit. Writing a ‘0’ has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn port_over_current_indicator_change(
        &mut self,
    ) -> PORT_OVER_CURRENT_INDICATOR_CHANGE_W<19> {
        PORT_OVER_CURRENT_INDICATOR_CHANGE_W::new(self)
    }
    #[doc = "Bit 20 - This bit is set at the end of the 10ms port reset signal. The HCD writes a '1' to clear this bit. Writing a '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn port_reset_status_change(&mut self) -> PORT_RESET_STATUS_CHANGE_W<20> {
        PORT_RESET_STATUS_CHANGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI Root Hub Port Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc_rh_port_status](index.html) module"]
pub struct HC_RH_PORT_STATUS_SPEC;
impl crate::RegisterSpec for HC_RH_PORT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc_rh_port_status::R](R) reader structure"]
impl crate::Readable for HC_RH_PORT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc_rh_port_status::W](W) writer structure"]
impl crate::Writable for HC_RH_PORT_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_rh_port_status to value 0"]
impl crate::Resettable for HC_RH_PORT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
