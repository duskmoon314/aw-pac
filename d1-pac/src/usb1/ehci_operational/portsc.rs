#[doc = "Register `portsc` reader"]
pub struct R(crate::R<PORTSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `portsc` writer"]
pub struct W(crate::W<PORTSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTSC_SPEC>;
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
impl From<crate::W<PORTSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `current_connect_status` reader - Current Connect Status\n\nDevice is present on port when the value of this field is a one, and no device is present on port when the value of this field is a zero. This value reflects the current state of the port, and may not correspond directly to the event that caused the Connect Status Change(Bit 1) to be set.\n\nThis field is zero if Port Power zero."]
pub type CURRENT_CONNECT_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `connect_status_change` reader - Connect Status Change\n\n1=Change in Current Connect Status\n\n0=No change\n\nIndicates a change has occurred in the current connect status of the port. The host controller sets this bit for all changes to the port device connect status, even if system software has not cleared an existing connect status change. For example, the insertion status changes twice before system software has cleared the changed condition, hub hardware will be 'setting' an already-set bit. Software sets this bit to 0 by writing a 1 to it.\n\nThis field is zero if Port Power is zero."]
pub type CONNECT_STATUS_CHANGE_R = crate::BitReader<bool>;
#[doc = "Field `connect_status_change` writer - Connect Status Change\n\n1=Change in Current Connect Status\n\n0=No change\n\nIndicates a change has occurred in the current connect status of the port. The host controller sets this bit for all changes to the port device connect status, even if system software has not cleared an existing connect status change. For example, the insertion status changes twice before system software has cleared the changed condition, hub hardware will be 'setting' an already-set bit. Software sets this bit to 0 by writing a 1 to it.\n\nThis field is zero if Port Power is zero."]
pub type CONNECT_STATUS_CHANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC_SPEC, bool, O>;
#[doc = "Field `port_enabled_disabled` reader - Port Enabled/Disabled\n\n1=Enable\n\n0=Disable\n\nPorts can only be enabled by the host controller as a part of the reset and enable. Software cannot enable a port by writing a one to this field. The host controller will only set this bit to a one when the reset sequence determines that the attached device is a high- speed device.\n\nPorts can be disabled by either a fault condition(disconnect event or other fault condition) or by host software. Note that the bit status does not change until the port state actually changes. There may be a delay in disabling or enabling a port due to other host controller and bus events.\n\nWhen the port is disabled, downstream propagation of data is blocked on this port except for reset.\n\nThe default value of this field is '0'.\n\nThis field is zero if Port Power is zero."]
pub type PORT_ENABLED_DISABLED_R = crate::BitReader<bool>;
#[doc = "Field `port_enabled_disabled` writer - Port Enabled/Disabled\n\n1=Enable\n\n0=Disable\n\nPorts can only be enabled by the host controller as a part of the reset and enable. Software cannot enable a port by writing a one to this field. The host controller will only set this bit to a one when the reset sequence determines that the attached device is a high- speed device.\n\nPorts can be disabled by either a fault condition(disconnect event or other fault condition) or by host software. Note that the bit status does not change until the port state actually changes. There may be a delay in disabling or enabling a port due to other host controller and bus events.\n\nWhen the port is disabled, downstream propagation of data is blocked on this port except for reset.\n\nThe default value of this field is '0'.\n\nThis field is zero if Port Power is zero."]
pub type PORT_ENABLED_DISABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC_SPEC, bool, O>;
#[doc = "Field `port_enable_disable_change` reader - Port Enable/Disable Change\n\n1 = Port enabled/disabled status has changed\n\n0 = No change\n\nFor the root hub, this bit gets set to a one only when a port is disabled due to the appropriate conditions existing at the EOF2 point (See Chapter 11 of the USB Specification for the definition of a Port Error). Software clears this bit by writing a 1 to it.\n\nThis field is zero if Port Power is zero."]
pub type PORT_ENABLE_DISABLE_CHANGE_R = crate::BitReader<bool>;
#[doc = "Field `port_enable_disable_change` writer - Port Enable/Disable Change\n\n1 = Port enabled/disabled status has changed\n\n0 = No change\n\nFor the root hub, this bit gets set to a one only when a port is disabled due to the appropriate conditions existing at the EOF2 point (See Chapter 11 of the USB Specification for the definition of a Port Error). Software clears this bit by writing a 1 to it.\n\nThis field is zero if Port Power is zero."]
pub type PORT_ENABLE_DISABLE_CHANGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PORTSC_SPEC, bool, O>;
#[doc = "Field `over_current_active` reader - Over-current Active\n\n0 = This port does not have an over-current condition \n\n1 = This port currently has an over-current condition\n\nThis bit will automatically transition from a one to a zero when the over current condition is removed.\n\nThe default value of this bit is '0'."]
pub type OVER_CURRENT_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `over_current_change` reader - Over-current Change\n\nThis bit gets set to a one when there is a change to Over-current Active. Software clears this bit by writing a one to this bit position."]
pub type OVER_CURRENT_CHANGE_R = crate::BitReader<bool>;
#[doc = "Field `over_current_change` writer - Over-current Change\n\nThis bit gets set to a one when there is a change to Over-current Active. Software clears this bit by writing a one to this bit position."]
pub type OVER_CURRENT_CHANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC_SPEC, bool, O>;
#[doc = "Field `force_port_resume` reader - Force Port Resume\n\n1 = Resume detected/driven on port. 0 = No resume (K-state) detected/driven on port. Default value = 0.\n\nThis functionality defined for manipulating this bit depends on the value of the Suspend bit. For example, if the port is not suspend and software transitions this bit to a one, then the effects on the bus are undefined.\n\nSoftware sets this bit to 1 to drive resume signaling. The Host Controller sets this bit to a 1 if a J-to-K transition is detected while the port is in the Suspend state. When this bit transitions to a one because a J-to-K transition is detected, the Port Change Detect bit in the USBSTS register is also set to a one. If software sets this bit to a one, the host controller must not set the Port Change Detect bit.\n\nNote that when the EHCI controller owns the port, the resume sequence follows the defined sequence documented in the USB Specification Revision 2.0. The resume signaling (Full-speed 'K') is driven on the port as long as this remains a one. Software must appropriately time the Resume and set this bit to a zero when the appropriate amount of time has elapsed. Writing a zero (from one) causes the port to return high-speed mode (forcing the bus below the port into a high-speed idle). This bit will remain a one until the port has switched to high-speed idle. The host controller must complete this transition within 2 milliseconds of software setting this bit to a zero.\n\nThis field is zero if Port Power is zero."]
pub type FORCE_PORT_RESUME_R = crate::BitReader<bool>;
#[doc = "Field `force_port_resume` writer - Force Port Resume\n\n1 = Resume detected/driven on port. 0 = No resume (K-state) detected/driven on port. Default value = 0.\n\nThis functionality defined for manipulating this bit depends on the value of the Suspend bit. For example, if the port is not suspend and software transitions this bit to a one, then the effects on the bus are undefined.\n\nSoftware sets this bit to 1 to drive resume signaling. The Host Controller sets this bit to a 1 if a J-to-K transition is detected while the port is in the Suspend state. When this bit transitions to a one because a J-to-K transition is detected, the Port Change Detect bit in the USBSTS register is also set to a one. If software sets this bit to a one, the host controller must not set the Port Change Detect bit.\n\nNote that when the EHCI controller owns the port, the resume sequence follows the defined sequence documented in the USB Specification Revision 2.0. The resume signaling (Full-speed 'K') is driven on the port as long as this remains a one. Software must appropriately time the Resume and set this bit to a zero when the appropriate amount of time has elapsed. Writing a zero (from one) causes the port to return high-speed mode (forcing the bus below the port into a high-speed idle). This bit will remain a one until the port has switched to high-speed idle. The host controller must complete this transition within 2 milliseconds of software setting this bit to a zero.\n\nThis field is zero if Port Power is zero."]
pub type FORCE_PORT_RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC_SPEC, bool, O>;
#[doc = "Field `suspend` reader - Suspend\n\nPort Enabled Bit and Suspend bit of this register define the port states\n\nWhen in suspend state, downstream propagation of data is blocked on this port, except for port reset. The blocking occurs at the end of the current transaction, if a transaction was in progress when this bit was written to 1. In the suspend state, the port is sensitive to resume detection. Not that the bit status does not change until the port is suspend and that there may be a delay in suspending a port if there is a transaction currently in progress on the USB.\n\nA write of zero to this bit is ignored by the host controller. The host controller will unconditionally set this bit to a zero when:\n\n1. Software sets the Force Port Resume bit to a zero(from a one).\n2. Software sets the Port Reset bit to a one(from a zero).\n\nIf host software sets this bit to a one when the port is not enabled(i.e. Port enabled bit is a zero), the results are undefined.\n\nThis field is zero if Port Power is zero.\n\nThe default value in this field is '0'."]
pub type SUSPEND_R = crate::BitReader<SUSPEND_A>;
#[doc = "Suspend\n\nPort Enabled Bit and Suspend bit of this register define the port states\n\nWhen in suspend state, downstream propagation of data is blocked on this port, except for port reset. The blocking occurs at the end of the current transaction, if a transaction was in progress when this bit was written to 1. In the suspend state, the port is sensitive to resume detection. Not that the bit status does not change until the port is suspend and that there may be a delay in suspending a port if there is a transaction currently in progress on the USB.\n\nA write of zero to this bit is ignored by the host controller. The host controller will unconditionally set this bit to a zero when:\n\n1. Software sets the Force Port Resume bit to a zero(from a one).\n2. Software sets the Port Reset bit to a one(from a zero).\n\nIf host software sets this bit to a one when the port is not enabled(i.e. Port enabled bit is a zero), the results are undefined.\n\nThis field is zero if Port Power is zero.\n\nThe default value in this field is '0'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPEND_A {
    #[doc = "0: Not suspend"]
    NOT_SUSPEND = 0,
    #[doc = "1: Suspend"]
    SUSPEND = 1,
}
impl From<SUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPEND_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSPEND_A {
        match self.bits {
            false => SUSPEND_A::NOT_SUSPEND,
            true => SUSPEND_A::SUSPEND,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_SUSPEND`"]
    #[inline(always)]
    pub fn is_not_suspend(&self) -> bool {
        *self == SUSPEND_A::NOT_SUSPEND
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == SUSPEND_A::SUSPEND
    }
}
#[doc = "Field `suspend` writer - Suspend\n\nPort Enabled Bit and Suspend bit of this register define the port states\n\nWhen in suspend state, downstream propagation of data is blocked on this port, except for port reset. The blocking occurs at the end of the current transaction, if a transaction was in progress when this bit was written to 1. In the suspend state, the port is sensitive to resume detection. Not that the bit status does not change until the port is suspend and that there may be a delay in suspending a port if there is a transaction currently in progress on the USB.\n\nA write of zero to this bit is ignored by the host controller. The host controller will unconditionally set this bit to a zero when:\n\n1. Software sets the Force Port Resume bit to a zero(from a one).\n2. Software sets the Port Reset bit to a one(from a zero).\n\nIf host software sets this bit to a one when the port is not enabled(i.e. Port enabled bit is a zero), the results are undefined.\n\nThis field is zero if Port Power is zero.\n\nThe default value in this field is '0'."]
pub type SUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC_SPEC, SUSPEND_A, O>;
impl<'a, const O: u8> SUSPEND_W<'a, O> {
    #[doc = "Not suspend"]
    #[inline(always)]
    pub fn not_suspend(self) -> &'a mut W {
        self.variant(SUSPEND_A::NOT_SUSPEND)
    }
    #[doc = "Suspend"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(SUSPEND_A::SUSPEND)
    }
}
#[doc = "Field `port_reset` reader - Port Reset\n\n1=Port is in Reset. 0=Port is not in Reset. Default value = 0.\n\nWhen software writes a one to this bit (from a zero), the bus reset sequence as defined in the USB Specification Revision 2.0 is started. Software writes a zero to this bit to terminate the bus reset sequence. Software must keep this bit at a one long enough to ensure the reset sequence, as specified in the USB Specification Revision 2.0, completes.\n\n**Note: When software writes this bit to a one, it must also write a zero to the Port Enable bit.**\n\nNote that when software writes a zero to this bit there may be a delay before the bit status changes to a zero. The bit status will not read as a zero until after the reset has completed. If the port is in high-speed mode after reset is complete, the host controller will automatically enable this port (e.g. set the Port Enable bit to a one). A host controller must terminate the reset and stabilize the state of the port within 2 milliseconds of software transitioning this bit from a one to a zero. For example: if the port detects that the attached device is high-speed during reset, then the host controller must have the port in the enabled state with 2ms of software writing this bit to a zero. The HC Halted bit in the USBSTS register should be a zero before software attempts to use this bit. The host controller may hold Port Reset asserted to a one when the HC Halted bit is a one. This field is zero if Port Power is zero."]
pub type PORT_RESET_R = crate::BitReader<bool>;
#[doc = "Field `port_reset` writer - Port Reset\n\n1=Port is in Reset. 0=Port is not in Reset. Default value = 0.\n\nWhen software writes a one to this bit (from a zero), the bus reset sequence as defined in the USB Specification Revision 2.0 is started. Software writes a zero to this bit to terminate the bus reset sequence. Software must keep this bit at a one long enough to ensure the reset sequence, as specified in the USB Specification Revision 2.0, completes.\n\n**Note: When software writes this bit to a one, it must also write a zero to the Port Enable bit.**\n\nNote that when software writes a zero to this bit there may be a delay before the bit status changes to a zero. The bit status will not read as a zero until after the reset has completed. If the port is in high-speed mode after reset is complete, the host controller will automatically enable this port (e.g. set the Port Enable bit to a one). A host controller must terminate the reset and stabilize the state of the port within 2 milliseconds of software transitioning this bit from a one to a zero. For example: if the port detects that the attached device is high-speed during reset, then the host controller must have the port in the enabled state with 2ms of software writing this bit to a zero. The HC Halted bit in the USBSTS register should be a zero before software attempts to use this bit. The host controller may hold Port Reset asserted to a one when the HC Halted bit is a one. This field is zero if Port Power is zero."]
pub type PORT_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC_SPEC, bool, O>;
#[doc = "Field `line_status` reader - Line Status\n\nThese bits relect the current logical levels of the D+ (bit11) and D- (bit10) signal lines. These bits are used for detection of low-speed USB devices prior to port reset and enable sequence. This read only field is valid only when the port enable bit is zero and the current connect status bit is set to a one."]
pub type LINE_STATUS_R = crate::FieldReader<u8, LINE_STATUS_A>;
#[doc = "Line Status\n\nThese bits relect the current logical levels of the D+ (bit11) and D- (bit10) signal lines. These bits are used for detection of low-speed USB devices prior to port reset and enable sequence. This read only field is valid only when the port enable bit is zero and the current connect status bit is set to a one.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LINE_STATUS_A {
    #[doc = "0: Not Low-speed device, perform EHCI reset."]
    SE0 = 0,
    #[doc = "2: Not Low-speed device, perform EHCI reset."]
    J_STATE = 2,
    #[doc = "1: Low-speed device, release ownership of port."]
    K_STATE = 1,
    #[doc = "3: Not Low-speed device, perform EHCI reset."]
    UNDEFINED = 3,
}
impl From<LINE_STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE_STATUS_A) -> Self {
        variant as _
    }
}
impl LINE_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE_STATUS_A {
        match self.bits {
            0 => LINE_STATUS_A::SE0,
            2 => LINE_STATUS_A::J_STATE,
            1 => LINE_STATUS_A::K_STATE,
            3 => LINE_STATUS_A::UNDEFINED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE0`"]
    #[inline(always)]
    pub fn is_se0(&self) -> bool {
        *self == LINE_STATUS_A::SE0
    }
    #[doc = "Checks if the value of the field is `J_STATE`"]
    #[inline(always)]
    pub fn is_j_state(&self) -> bool {
        *self == LINE_STATUS_A::J_STATE
    }
    #[doc = "Checks if the value of the field is `K_STATE`"]
    #[inline(always)]
    pub fn is_k_state(&self) -> bool {
        *self == LINE_STATUS_A::K_STATE
    }
    #[doc = "Checks if the value of the field is `UNDEFINED`"]
    #[inline(always)]
    pub fn is_undefined(&self) -> bool {
        *self == LINE_STATUS_A::UNDEFINED
    }
}
#[doc = "Field `port_owner` reader - Port Owner\n\nThis bit unconditionally goes to a 0b when the Configured bit in the CONFIGFLAG register makes a 0b to 1b transition. This bit unconditionally goes to 1b whenever the Configured bit is zero. System software uses this field to release ownership of the port to selected host controller (in the event that the attached device is not a high-speed device).Software writes a one to this bit when the attached device is not a high-speed device. A one in this bit means that a companion host controller owns and controls the port. Default Value = 1b."]
pub type PORT_OWNER_R = crate::BitReader<bool>;
#[doc = "Field `port_owner` writer - Port Owner\n\nThis bit unconditionally goes to a 0b when the Configured bit in the CONFIGFLAG register makes a 0b to 1b transition. This bit unconditionally goes to 1b whenever the Configured bit is zero. System software uses this field to release ownership of the port to selected host controller (in the event that the attached device is not a high-speed device).Software writes a one to this bit when the attached device is not a high-speed device. A one in this bit means that a companion host controller owns and controls the port. Default Value = 1b."]
pub type PORT_OWNER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC_SPEC, bool, O>;
#[doc = "Field `port_test_control` reader - Port Test Control\n\nThe value in this field specifies the test mode of the port."]
pub type PORT_TEST_CONTROL_R = crate::FieldReader<u8, PORT_TEST_CONTROL_A>;
#[doc = "Port Test Control\n\nThe value in this field specifies the test mode of the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PORT_TEST_CONTROL_A {
    #[doc = "0: The port is NOT operating in a test mode."]
    NOT_OPERATING = 0,
    #[doc = "1: Test J_STATE"]
    TEST_J_STATE = 1,
    #[doc = "2: Test K_STATE"]
    TEST_K_STATE = 2,
    #[doc = "3: Test SE0_NAK"]
    TEST_SE0_NAK = 3,
    #[doc = "4: Test Packet"]
    TEST_PACKET = 4,
    #[doc = "5: Test FORCE_ENABLE"]
    TEST_FORCE_ENABLE = 5,
}
impl From<PORT_TEST_CONTROL_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT_TEST_CONTROL_A) -> Self {
        variant as _
    }
}
impl PORT_TEST_CONTROL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PORT_TEST_CONTROL_A> {
        match self.bits {
            0 => Some(PORT_TEST_CONTROL_A::NOT_OPERATING),
            1 => Some(PORT_TEST_CONTROL_A::TEST_J_STATE),
            2 => Some(PORT_TEST_CONTROL_A::TEST_K_STATE),
            3 => Some(PORT_TEST_CONTROL_A::TEST_SE0_NAK),
            4 => Some(PORT_TEST_CONTROL_A::TEST_PACKET),
            5 => Some(PORT_TEST_CONTROL_A::TEST_FORCE_ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_OPERATING`"]
    #[inline(always)]
    pub fn is_not_operating(&self) -> bool {
        *self == PORT_TEST_CONTROL_A::NOT_OPERATING
    }
    #[doc = "Checks if the value of the field is `TEST_J_STATE`"]
    #[inline(always)]
    pub fn is_test_j_state(&self) -> bool {
        *self == PORT_TEST_CONTROL_A::TEST_J_STATE
    }
    #[doc = "Checks if the value of the field is `TEST_K_STATE`"]
    #[inline(always)]
    pub fn is_test_k_state(&self) -> bool {
        *self == PORT_TEST_CONTROL_A::TEST_K_STATE
    }
    #[doc = "Checks if the value of the field is `TEST_SE0_NAK`"]
    #[inline(always)]
    pub fn is_test_se0_nak(&self) -> bool {
        *self == PORT_TEST_CONTROL_A::TEST_SE0_NAK
    }
    #[doc = "Checks if the value of the field is `TEST_PACKET`"]
    #[inline(always)]
    pub fn is_test_packet(&self) -> bool {
        *self == PORT_TEST_CONTROL_A::TEST_PACKET
    }
    #[doc = "Checks if the value of the field is `TEST_FORCE_ENABLE`"]
    #[inline(always)]
    pub fn is_test_force_enable(&self) -> bool {
        *self == PORT_TEST_CONTROL_A::TEST_FORCE_ENABLE
    }
}
#[doc = "Field `port_test_control` writer - Port Test Control\n\nThe value in this field specifies the test mode of the port."]
pub type PORT_TEST_CONTROL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PORTSC_SPEC, u8, PORT_TEST_CONTROL_A, 4, O>;
impl<'a, const O: u8> PORT_TEST_CONTROL_W<'a, O> {
    #[doc = "The port is NOT operating in a test mode."]
    #[inline(always)]
    pub fn not_operating(self) -> &'a mut W {
        self.variant(PORT_TEST_CONTROL_A::NOT_OPERATING)
    }
    #[doc = "Test J_STATE"]
    #[inline(always)]
    pub fn test_j_state(self) -> &'a mut W {
        self.variant(PORT_TEST_CONTROL_A::TEST_J_STATE)
    }
    #[doc = "Test K_STATE"]
    #[inline(always)]
    pub fn test_k_state(self) -> &'a mut W {
        self.variant(PORT_TEST_CONTROL_A::TEST_K_STATE)
    }
    #[doc = "Test SE0_NAK"]
    #[inline(always)]
    pub fn test_se0_nak(self) -> &'a mut W {
        self.variant(PORT_TEST_CONTROL_A::TEST_SE0_NAK)
    }
    #[doc = "Test Packet"]
    #[inline(always)]
    pub fn test_packet(self) -> &'a mut W {
        self.variant(PORT_TEST_CONTROL_A::TEST_PACKET)
    }
    #[doc = "Test FORCE_ENABLE"]
    #[inline(always)]
    pub fn test_force_enable(self) -> &'a mut W {
        self.variant(PORT_TEST_CONTROL_A::TEST_FORCE_ENABLE)
    }
}
#[doc = "Field `wkcnnt_e` reader - Wake on Connect Enable (WKCNNT_E)\n\nWriting this bit to a one enable the port to be sensitive to device connects as wake-up events.\n\nThis field is zero if Port Power is zero.\n\nThe default value in this field is '0'."]
pub type WKCNNT_E_R = crate::BitReader<bool>;
#[doc = "Field `wkcnnt_e` writer - Wake on Connect Enable (WKCNNT_E)\n\nWriting this bit to a one enable the port to be sensitive to device connects as wake-up events.\n\nThis field is zero if Port Power is zero.\n\nThe default value in this field is '0'."]
pub type WKCNNT_E_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC_SPEC, bool, O>;
#[doc = "Field `wkdscnnt_e` reader - Wake on Disconnect Enable (WKDSCNNT_E)\n\nWriting this bit to a one enables the port to be sensitive to device disconnects as wake-up events.\n\nThis field is zero if Port Power is zero.\n\nThe default value in this field is '0'."]
pub type WKDSCNNT_E_R = crate::BitReader<bool>;
#[doc = "Field `wkdscnnt_e` writer - Wake on Disconnect Enable (WKDSCNNT_E)\n\nWriting this bit to a one enables the port to be sensitive to device disconnects as wake-up events.\n\nThis field is zero if Port Power is zero.\n\nThe default value in this field is '0'."]
pub type WKDSCNNT_E_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Current Connect Status\n\nDevice is present on port when the value of this field is a one, and no device is present on port when the value of this field is a zero. This value reflects the current state of the port, and may not correspond directly to the event that caused the Connect Status Change(Bit 1) to be set.\n\nThis field is zero if Port Power zero."]
    #[inline(always)]
    pub fn current_connect_status(&self) -> CURRENT_CONNECT_STATUS_R {
        CURRENT_CONNECT_STATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Connect Status Change\n\n1=Change in Current Connect Status\n\n0=No change\n\nIndicates a change has occurred in the current connect status of the port. The host controller sets this bit for all changes to the port device connect status, even if system software has not cleared an existing connect status change. For example, the insertion status changes twice before system software has cleared the changed condition, hub hardware will be 'setting' an already-set bit. Software sets this bit to 0 by writing a 1 to it.\n\nThis field is zero if Port Power is zero."]
    #[inline(always)]
    pub fn connect_status_change(&self) -> CONNECT_STATUS_CHANGE_R {
        CONNECT_STATUS_CHANGE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled\n\n1=Enable\n\n0=Disable\n\nPorts can only be enabled by the host controller as a part of the reset and enable. Software cannot enable a port by writing a one to this field. The host controller will only set this bit to a one when the reset sequence determines that the attached device is a high- speed device.\n\nPorts can be disabled by either a fault condition(disconnect event or other fault condition) or by host software. Note that the bit status does not change until the port state actually changes. There may be a delay in disabling or enabling a port due to other host controller and bus events.\n\nWhen the port is disabled, downstream propagation of data is blocked on this port except for reset.\n\nThe default value of this field is '0'.\n\nThis field is zero if Port Power is zero."]
    #[inline(always)]
    pub fn port_enabled_disabled(&self) -> PORT_ENABLED_DISABLED_R {
        PORT_ENABLED_DISABLED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change\n\n1 = Port enabled/disabled status has changed\n\n0 = No change\n\nFor the root hub, this bit gets set to a one only when a port is disabled due to the appropriate conditions existing at the EOF2 point (See Chapter 11 of the USB Specification for the definition of a Port Error). Software clears this bit by writing a 1 to it.\n\nThis field is zero if Port Power is zero."]
    #[inline(always)]
    pub fn port_enable_disable_change(&self) -> PORT_ENABLE_DISABLE_CHANGE_R {
        PORT_ENABLE_DISABLE_CHANGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Over-current Active\n\n0 = This port does not have an over-current condition \n\n1 = This port currently has an over-current condition\n\nThis bit will automatically transition from a one to a zero when the over current condition is removed.\n\nThe default value of this bit is '0'."]
    #[inline(always)]
    pub fn over_current_active(&self) -> OVER_CURRENT_ACTIVE_R {
        OVER_CURRENT_ACTIVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Over-current Change\n\nThis bit gets set to a one when there is a change to Over-current Active. Software clears this bit by writing a one to this bit position."]
    #[inline(always)]
    pub fn over_current_change(&self) -> OVER_CURRENT_CHANGE_R {
        OVER_CURRENT_CHANGE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Port Resume\n\n1 = Resume detected/driven on port. 0 = No resume (K-state) detected/driven on port. Default value = 0.\n\nThis functionality defined for manipulating this bit depends on the value of the Suspend bit. For example, if the port is not suspend and software transitions this bit to a one, then the effects on the bus are undefined.\n\nSoftware sets this bit to 1 to drive resume signaling. The Host Controller sets this bit to a 1 if a J-to-K transition is detected while the port is in the Suspend state. When this bit transitions to a one because a J-to-K transition is detected, the Port Change Detect bit in the USBSTS register is also set to a one. If software sets this bit to a one, the host controller must not set the Port Change Detect bit.\n\nNote that when the EHCI controller owns the port, the resume sequence follows the defined sequence documented in the USB Specification Revision 2.0. The resume signaling (Full-speed 'K') is driven on the port as long as this remains a one. Software must appropriately time the Resume and set this bit to a zero when the appropriate amount of time has elapsed. Writing a zero (from one) causes the port to return high-speed mode (forcing the bus below the port into a high-speed idle). This bit will remain a one until the port has switched to high-speed idle. The host controller must complete this transition within 2 milliseconds of software setting this bit to a zero.\n\nThis field is zero if Port Power is zero."]
    #[inline(always)]
    pub fn force_port_resume(&self) -> FORCE_PORT_RESUME_R {
        FORCE_PORT_RESUME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Suspend\n\nPort Enabled Bit and Suspend bit of this register define the port states\n\nWhen in suspend state, downstream propagation of data is blocked on this port, except for port reset. The blocking occurs at the end of the current transaction, if a transaction was in progress when this bit was written to 1. In the suspend state, the port is sensitive to resume detection. Not that the bit status does not change until the port is suspend and that there may be a delay in suspending a port if there is a transaction currently in progress on the USB.\n\nA write of zero to this bit is ignored by the host controller. The host controller will unconditionally set this bit to a zero when:\n\n1. Software sets the Force Port Resume bit to a zero(from a one).\n2. Software sets the Port Reset bit to a one(from a zero).\n\nIf host software sets this bit to a one when the port is not enabled(i.e. Port enabled bit is a zero), the results are undefined.\n\nThis field is zero if Port Power is zero.\n\nThe default value in this field is '0'."]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Reset\n\n1=Port is in Reset. 0=Port is not in Reset. Default value = 0.\n\nWhen software writes a one to this bit (from a zero), the bus reset sequence as defined in the USB Specification Revision 2.0 is started. Software writes a zero to this bit to terminate the bus reset sequence. Software must keep this bit at a one long enough to ensure the reset sequence, as specified in the USB Specification Revision 2.0, completes.\n\n**Note: When software writes this bit to a one, it must also write a zero to the Port Enable bit.**\n\nNote that when software writes a zero to this bit there may be a delay before the bit status changes to a zero. The bit status will not read as a zero until after the reset has completed. If the port is in high-speed mode after reset is complete, the host controller will automatically enable this port (e.g. set the Port Enable bit to a one). A host controller must terminate the reset and stabilize the state of the port within 2 milliseconds of software transitioning this bit from a one to a zero. For example: if the port detects that the attached device is high-speed during reset, then the host controller must have the port in the enabled state with 2ms of software writing this bit to a zero. The HC Halted bit in the USBSTS register should be a zero before software attempts to use this bit. The host controller may hold Port Reset asserted to a one when the HC Halted bit is a one. This field is zero if Port Power is zero."]
    #[inline(always)]
    pub fn port_reset(&self) -> PORT_RESET_R {
        PORT_RESET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Line Status\n\nThese bits relect the current logical levels of the D+ (bit11) and D- (bit10) signal lines. These bits are used for detection of low-speed USB devices prior to port reset and enable sequence. This read only field is valid only when the port enable bit is zero and the current connect status bit is set to a one."]
    #[inline(always)]
    pub fn line_status(&self) -> LINE_STATUS_R {
        LINE_STATUS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 13 - Port Owner\n\nThis bit unconditionally goes to a 0b when the Configured bit in the CONFIGFLAG register makes a 0b to 1b transition. This bit unconditionally goes to 1b whenever the Configured bit is zero. System software uses this field to release ownership of the port to selected host controller (in the event that the attached device is not a high-speed device).Software writes a one to this bit when the attached device is not a high-speed device. A one in this bit means that a companion host controller owns and controls the port. Default Value = 1b."]
    #[inline(always)]
    pub fn port_owner(&self) -> PORT_OWNER_R {
        PORT_OWNER_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Port Test Control\n\nThe value in this field specifies the test mode of the port."]
    #[inline(always)]
    pub fn port_test_control(&self) -> PORT_TEST_CONTROL_R {
        PORT_TEST_CONTROL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Wake on Connect Enable (WKCNNT_E)\n\nWriting this bit to a one enable the port to be sensitive to device connects as wake-up events.\n\nThis field is zero if Port Power is zero.\n\nThe default value in this field is '0'."]
    #[inline(always)]
    pub fn wkcnnt_e(&self) -> WKCNNT_E_R {
        WKCNNT_E_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Wake on Disconnect Enable (WKDSCNNT_E)\n\nWriting this bit to a one enables the port to be sensitive to device disconnects as wake-up events.\n\nThis field is zero if Port Power is zero.\n\nThe default value in this field is '0'."]
    #[inline(always)]
    pub fn wkdscnnt_e(&self) -> WKDSCNNT_E_R {
        WKDSCNNT_E_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Connect Status Change\n\n1=Change in Current Connect Status\n\n0=No change\n\nIndicates a change has occurred in the current connect status of the port. The host controller sets this bit for all changes to the port device connect status, even if system software has not cleared an existing connect status change. For example, the insertion status changes twice before system software has cleared the changed condition, hub hardware will be 'setting' an already-set bit. Software sets this bit to 0 by writing a 1 to it.\n\nThis field is zero if Port Power is zero."]
    #[inline(always)]
    #[must_use]
    pub fn connect_status_change(&mut self) -> CONNECT_STATUS_CHANGE_W<1> {
        CONNECT_STATUS_CHANGE_W::new(self)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled\n\n1=Enable\n\n0=Disable\n\nPorts can only be enabled by the host controller as a part of the reset and enable. Software cannot enable a port by writing a one to this field. The host controller will only set this bit to a one when the reset sequence determines that the attached device is a high- speed device.\n\nPorts can be disabled by either a fault condition(disconnect event or other fault condition) or by host software. Note that the bit status does not change until the port state actually changes. There may be a delay in disabling or enabling a port due to other host controller and bus events.\n\nWhen the port is disabled, downstream propagation of data is blocked on this port except for reset.\n\nThe default value of this field is '0'.\n\nThis field is zero if Port Power is zero."]
    #[inline(always)]
    #[must_use]
    pub fn port_enabled_disabled(&mut self) -> PORT_ENABLED_DISABLED_W<2> {
        PORT_ENABLED_DISABLED_W::new(self)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change\n\n1 = Port enabled/disabled status has changed\n\n0 = No change\n\nFor the root hub, this bit gets set to a one only when a port is disabled due to the appropriate conditions existing at the EOF2 point (See Chapter 11 of the USB Specification for the definition of a Port Error). Software clears this bit by writing a 1 to it.\n\nThis field is zero if Port Power is zero."]
    #[inline(always)]
    #[must_use]
    pub fn port_enable_disable_change(&mut self) -> PORT_ENABLE_DISABLE_CHANGE_W<3> {
        PORT_ENABLE_DISABLE_CHANGE_W::new(self)
    }
    #[doc = "Bit 5 - Over-current Change\n\nThis bit gets set to a one when there is a change to Over-current Active. Software clears this bit by writing a one to this bit position."]
    #[inline(always)]
    #[must_use]
    pub fn over_current_change(&mut self) -> OVER_CURRENT_CHANGE_W<5> {
        OVER_CURRENT_CHANGE_W::new(self)
    }
    #[doc = "Bit 6 - Force Port Resume\n\n1 = Resume detected/driven on port. 0 = No resume (K-state) detected/driven on port. Default value = 0.\n\nThis functionality defined for manipulating this bit depends on the value of the Suspend bit. For example, if the port is not suspend and software transitions this bit to a one, then the effects on the bus are undefined.\n\nSoftware sets this bit to 1 to drive resume signaling. The Host Controller sets this bit to a 1 if a J-to-K transition is detected while the port is in the Suspend state. When this bit transitions to a one because a J-to-K transition is detected, the Port Change Detect bit in the USBSTS register is also set to a one. If software sets this bit to a one, the host controller must not set the Port Change Detect bit.\n\nNote that when the EHCI controller owns the port, the resume sequence follows the defined sequence documented in the USB Specification Revision 2.0. The resume signaling (Full-speed 'K') is driven on the port as long as this remains a one. Software must appropriately time the Resume and set this bit to a zero when the appropriate amount of time has elapsed. Writing a zero (from one) causes the port to return high-speed mode (forcing the bus below the port into a high-speed idle). This bit will remain a one until the port has switched to high-speed idle. The host controller must complete this transition within 2 milliseconds of software setting this bit to a zero.\n\nThis field is zero if Port Power is zero."]
    #[inline(always)]
    #[must_use]
    pub fn force_port_resume(&mut self) -> FORCE_PORT_RESUME_W<6> {
        FORCE_PORT_RESUME_W::new(self)
    }
    #[doc = "Bit 7 - Suspend\n\nPort Enabled Bit and Suspend bit of this register define the port states\n\nWhen in suspend state, downstream propagation of data is blocked on this port, except for port reset. The blocking occurs at the end of the current transaction, if a transaction was in progress when this bit was written to 1. In the suspend state, the port is sensitive to resume detection. Not that the bit status does not change until the port is suspend and that there may be a delay in suspending a port if there is a transaction currently in progress on the USB.\n\nA write of zero to this bit is ignored by the host controller. The host controller will unconditionally set this bit to a zero when:\n\n1. Software sets the Force Port Resume bit to a zero(from a one).\n2. Software sets the Port Reset bit to a one(from a zero).\n\nIf host software sets this bit to a one when the port is not enabled(i.e. Port enabled bit is a zero), the results are undefined.\n\nThis field is zero if Port Power is zero.\n\nThe default value in this field is '0'."]
    #[inline(always)]
    #[must_use]
    pub fn suspend(&mut self) -> SUSPEND_W<7> {
        SUSPEND_W::new(self)
    }
    #[doc = "Bit 8 - Port Reset\n\n1=Port is in Reset. 0=Port is not in Reset. Default value = 0.\n\nWhen software writes a one to this bit (from a zero), the bus reset sequence as defined in the USB Specification Revision 2.0 is started. Software writes a zero to this bit to terminate the bus reset sequence. Software must keep this bit at a one long enough to ensure the reset sequence, as specified in the USB Specification Revision 2.0, completes.\n\n**Note: When software writes this bit to a one, it must also write a zero to the Port Enable bit.**\n\nNote that when software writes a zero to this bit there may be a delay before the bit status changes to a zero. The bit status will not read as a zero until after the reset has completed. If the port is in high-speed mode after reset is complete, the host controller will automatically enable this port (e.g. set the Port Enable bit to a one). A host controller must terminate the reset and stabilize the state of the port within 2 milliseconds of software transitioning this bit from a one to a zero. For example: if the port detects that the attached device is high-speed during reset, then the host controller must have the port in the enabled state with 2ms of software writing this bit to a zero. The HC Halted bit in the USBSTS register should be a zero before software attempts to use this bit. The host controller may hold Port Reset asserted to a one when the HC Halted bit is a one. This field is zero if Port Power is zero."]
    #[inline(always)]
    #[must_use]
    pub fn port_reset(&mut self) -> PORT_RESET_W<8> {
        PORT_RESET_W::new(self)
    }
    #[doc = "Bit 13 - Port Owner\n\nThis bit unconditionally goes to a 0b when the Configured bit in the CONFIGFLAG register makes a 0b to 1b transition. This bit unconditionally goes to 1b whenever the Configured bit is zero. System software uses this field to release ownership of the port to selected host controller (in the event that the attached device is not a high-speed device).Software writes a one to this bit when the attached device is not a high-speed device. A one in this bit means that a companion host controller owns and controls the port. Default Value = 1b."]
    #[inline(always)]
    #[must_use]
    pub fn port_owner(&mut self) -> PORT_OWNER_W<13> {
        PORT_OWNER_W::new(self)
    }
    #[doc = "Bits 16:19 - Port Test Control\n\nThe value in this field specifies the test mode of the port."]
    #[inline(always)]
    #[must_use]
    pub fn port_test_control(&mut self) -> PORT_TEST_CONTROL_W<16> {
        PORT_TEST_CONTROL_W::new(self)
    }
    #[doc = "Bit 20 - Wake on Connect Enable (WKCNNT_E)\n\nWriting this bit to a one enable the port to be sensitive to device connects as wake-up events.\n\nThis field is zero if Port Power is zero.\n\nThe default value in this field is '0'."]
    #[inline(always)]
    #[must_use]
    pub fn wkcnnt_e(&mut self) -> WKCNNT_E_W<20> {
        WKCNNT_E_W::new(self)
    }
    #[doc = "Bit 21 - Wake on Disconnect Enable (WKDSCNNT_E)\n\nWriting this bit to a one enables the port to be sensitive to device disconnects as wake-up events.\n\nThis field is zero if Port Power is zero.\n\nThe default value in this field is '0'."]
    #[inline(always)]
    #[must_use]
    pub fn wkdscnnt_e(&mut self) -> WKDSCNNT_E_W<21> {
        WKDSCNNT_E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EHCI Port Status/Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portsc](index.html) module"]
pub struct PORTSC_SPEC;
impl crate::RegisterSpec for PORTSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [portsc::R](R) reader structure"]
impl crate::Readable for PORTSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portsc::W](W) writer structure"]
impl crate::Writable for PORTSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets portsc to value 0x2000"]
impl crate::Resettable for PORTSC_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
