#[doc = "Register `hc_control` reader"]
pub struct R(crate::R<HC_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hc_control` writer"]
pub struct W(crate::W<HC_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC_CONTROL_SPEC>;
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
impl From<crate::W<HC_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `control_bulk_service_ratio` reader - This specifies the service ratio between Control and Bulk EDs. Before processing any of the nonperiodic lists, HC must compare the ratio specified with its internal count on how many nonempty Control EDs have been processed, in determining whether to continue serving another Control ED or switching to Bulk EDs. The internal count will be retained when crossing the frame boundary. In case of reset, HCD is responsible for restoring this value."]
pub type CONTROL_BULK_SERVICE_RATIO_R = crate::FieldReader<u8, CONTROL_BULK_SERVICE_RATIO_A>;
#[doc = "This specifies the service ratio between Control and Bulk EDs. Before processing any of the nonperiodic lists, HC must compare the ratio specified with its internal count on how many nonempty Control EDs have been processed, in determining whether to continue serving another Control ED or switching to Bulk EDs. The internal count will be retained when crossing the frame boundary. In case of reset, HCD is responsible for restoring this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CONTROL_BULK_SERVICE_RATIO_A {
    #[doc = "0: 1:1"]
    R1 = 0,
    #[doc = "1: 2:1"]
    R2 = 1,
    #[doc = "2: 3:1"]
    R3 = 2,
    #[doc = "3: 4:1"]
    R4 = 3,
}
impl From<CONTROL_BULK_SERVICE_RATIO_A> for u8 {
    #[inline(always)]
    fn from(variant: CONTROL_BULK_SERVICE_RATIO_A) -> Self {
        variant as _
    }
}
impl CONTROL_BULK_SERVICE_RATIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONTROL_BULK_SERVICE_RATIO_A {
        match self.bits {
            0 => CONTROL_BULK_SERVICE_RATIO_A::R1,
            1 => CONTROL_BULK_SERVICE_RATIO_A::R2,
            2 => CONTROL_BULK_SERVICE_RATIO_A::R3,
            3 => CONTROL_BULK_SERVICE_RATIO_A::R4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `R1`"]
    #[inline(always)]
    pub fn is_r1(&self) -> bool {
        *self == CONTROL_BULK_SERVICE_RATIO_A::R1
    }
    #[doc = "Checks if the value of the field is `R2`"]
    #[inline(always)]
    pub fn is_r2(&self) -> bool {
        *self == CONTROL_BULK_SERVICE_RATIO_A::R2
    }
    #[doc = "Checks if the value of the field is `R3`"]
    #[inline(always)]
    pub fn is_r3(&self) -> bool {
        *self == CONTROL_BULK_SERVICE_RATIO_A::R3
    }
    #[doc = "Checks if the value of the field is `R4`"]
    #[inline(always)]
    pub fn is_r4(&self) -> bool {
        *self == CONTROL_BULK_SERVICE_RATIO_A::R4
    }
}
#[doc = "Field `control_bulk_service_ratio` writer - This specifies the service ratio between Control and Bulk EDs. Before processing any of the nonperiodic lists, HC must compare the ratio specified with its internal count on how many nonempty Control EDs have been processed, in determining whether to continue serving another Control ED or switching to Bulk EDs. The internal count will be retained when crossing the frame boundary. In case of reset, HCD is responsible for restoring this value."]
pub type CONTROL_BULK_SERVICE_RATIO_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, HC_CONTROL_SPEC, u8, CONTROL_BULK_SERVICE_RATIO_A, 2, O>;
impl<'a, const O: u8> CONTROL_BULK_SERVICE_RATIO_W<'a, O> {
    #[doc = "1:1"]
    #[inline(always)]
    pub fn r1(self) -> &'a mut W {
        self.variant(CONTROL_BULK_SERVICE_RATIO_A::R1)
    }
    #[doc = "2:1"]
    #[inline(always)]
    pub fn r2(self) -> &'a mut W {
        self.variant(CONTROL_BULK_SERVICE_RATIO_A::R2)
    }
    #[doc = "3:1"]
    #[inline(always)]
    pub fn r3(self) -> &'a mut W {
        self.variant(CONTROL_BULK_SERVICE_RATIO_A::R3)
    }
    #[doc = "4:1"]
    #[inline(always)]
    pub fn r4(self) -> &'a mut W {
        self.variant(CONTROL_BULK_SERVICE_RATIO_A::R4)
    }
}
#[doc = "Field `periodic_list_enable` reader - This bit is set to enable the processing of periodic list in the next Frame. If cleared by HCD, processing of the periodic list does not occur after the next SOF. HC must check this bit before it starts processing the list."]
pub type PERIODIC_LIST_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `periodic_list_enable` writer - This bit is set to enable the processing of periodic list in the next Frame. If cleared by HCD, processing of the periodic list does not occur after the next SOF. HC must check this bit before it starts processing the list."]
pub type PERIODIC_LIST_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_CONTROL_SPEC, bool, O>;
#[doc = "Field `control_list_enable` reader - This bit is set to enable the processing of the Control list in the next Frame. If cleared by HCD, the processing of the Control list does not occur after the next SOF. HC must check this bit whenever it determines to process the list. When disabled, HCD may modify the list. If HcControlCurrentED is pointing to an ED to be removed, HCD must advance the pointer by updating HcControlCurrentED before re-enabling processing of the list."]
pub type CONTROL_LIST_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `control_list_enable` writer - This bit is set to enable the processing of the Control list in the next Frame. If cleared by HCD, the processing of the Control list does not occur after the next SOF. HC must check this bit whenever it determines to process the list. When disabled, HCD may modify the list. If HcControlCurrentED is pointing to an ED to be removed, HCD must advance the pointer by updating HcControlCurrentED before re-enabling processing of the list."]
pub type CONTROL_LIST_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_CONTROL_SPEC, bool, O>;
#[doc = "Field `isochronous_enable` reader - This bit is used by HCD to enable/disable processing of isochronous EDs. While processing the periodic list in a Frame, HC checks the status of this bit when it finds an Isochronous ED (F=1). If set (enabled), HC continues processing the EDs. If cleared (disabled), HC halts processing of the periodic list (which now contains only isochronous EDs) and begins processing the Bulk/Control lists. Setting this bit is guaranteed to take effect in the next Frame (not the current Frame)."]
pub type ISOCHRONOUS_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `isochronous_enable` writer - This bit is used by HCD to enable/disable processing of isochronous EDs. While processing the periodic list in a Frame, HC checks the status of this bit when it finds an Isochronous ED (F=1). If set (enabled), HC continues processing the EDs. If cleared (disabled), HC halts processing of the periodic list (which now contains only isochronous EDs) and begins processing the Bulk/Control lists. Setting this bit is guaranteed to take effect in the next Frame (not the current Frame)."]
pub type ISOCHRONOUS_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_CONTROL_SPEC, bool, O>;
#[doc = "Field `bulk_list_enable` reader - This bit is set to enable the processing of the Bulk list in the next Frame. If cleared by HCD, the processing of the Bulk list does not occur after the next SOF. HC checks this bit whenever it determines to process the list. When disabled, HCD may modify the list. If HcBulkCurrentED is pointing to an ED to be removed, HCD must advance the pointer by updating HcBulkCurrentED before re-enabling processing of the list."]
pub type BULK_LIST_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `bulk_list_enable` writer - This bit is set to enable the processing of the Bulk list in the next Frame. If cleared by HCD, the processing of the Bulk list does not occur after the next SOF. HC checks this bit whenever it determines to process the list. When disabled, HCD may modify the list. If HcBulkCurrentED is pointing to an ED to be removed, HCD must advance the pointer by updating HcBulkCurrentED before re-enabling processing of the list."]
pub type BULK_LIST_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HC_CONTROL_SPEC, bool, O>;
#[doc = "Field `host_controller_functional_state_for_usb` reader - A transition to USBOperational from another state causes SOF generation to begin 1 ms later. HCD may determine whether HC has begun sending SOFs by reading the StartoFrame field of HcInterruptStatus.\n\nThis field may be changed by HC only when in the USBSUSPEND state. HC may move from the USBSUSPEND state to the USBRESUME state after detecting the resume signaling from a downstream port.\n\nHC enters USBSUSPEND after a software reset, whereas it enters USBRESET after a hardware reset. The latter also resets the Root Hub and asserts subsequent reset signaling to downstream ports."]
pub type HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_R =
    crate::FieldReader<u8, HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A>;
#[doc = "A transition to USBOperational from another state causes SOF generation to begin 1 ms later. HCD may determine whether HC has begun sending SOFs by reading the StartoFrame field of HcInterruptStatus.\n\nThis field may be changed by HC only when in the USBSUSPEND state. HC may move from the USBSUSPEND state to the USBRESUME state after detecting the resume signaling from a downstream port.\n\nHC enters USBSUSPEND after a software reset, whereas it enters USBRESET after a hardware reset. The latter also resets the Root Hub and asserts subsequent reset signaling to downstream ports.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A {
    #[doc = "0: USB Reset"]
    USB_RESET = 0,
    #[doc = "1: USB Resume"]
    USB_RESUME = 1,
    #[doc = "2: USB Operational"]
    USB_OPERATIONAL = 2,
    #[doc = "3: USB Suspend"]
    USB_SUSPEND = 3,
}
impl From<HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A> for u8 {
    #[inline(always)]
    fn from(variant: HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A) -> Self {
        variant as _
    }
}
impl HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A {
        match self.bits {
            0 => HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A::USB_RESET,
            1 => HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A::USB_RESUME,
            2 => HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A::USB_OPERATIONAL,
            3 => HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A::USB_SUSPEND,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USB_RESET`"]
    #[inline(always)]
    pub fn is_usb_reset(&self) -> bool {
        *self == HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A::USB_RESET
    }
    #[doc = "Checks if the value of the field is `USB_RESUME`"]
    #[inline(always)]
    pub fn is_usb_resume(&self) -> bool {
        *self == HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A::USB_RESUME
    }
    #[doc = "Checks if the value of the field is `USB_OPERATIONAL`"]
    #[inline(always)]
    pub fn is_usb_operational(&self) -> bool {
        *self == HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A::USB_OPERATIONAL
    }
    #[doc = "Checks if the value of the field is `USB_SUSPEND`"]
    #[inline(always)]
    pub fn is_usb_suspend(&self) -> bool {
        *self == HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A::USB_SUSPEND
    }
}
#[doc = "Field `host_controller_functional_state_for_usb` writer - A transition to USBOperational from another state causes SOF generation to begin 1 ms later. HCD may determine whether HC has begun sending SOFs by reading the StartoFrame field of HcInterruptStatus.\n\nThis field may be changed by HC only when in the USBSUSPEND state. HC may move from the USBSUSPEND state to the USBRESUME state after detecting the resume signaling from a downstream port.\n\nHC enters USBSUSPEND after a software reset, whereas it enters USBRESET after a hardware reset. The latter also resets the Root Hub and asserts subsequent reset signaling to downstream ports."]
pub type HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_W<'a, const O: u8> = crate::FieldWriterSafe<
    'a,
    u32,
    HC_CONTROL_SPEC,
    u8,
    HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A,
    2,
    O,
>;
impl<'a, const O: u8> HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_W<'a, O> {
    #[doc = "USB Reset"]
    #[inline(always)]
    pub fn usb_reset(self) -> &'a mut W {
        self.variant(HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A::USB_RESET)
    }
    #[doc = "USB Resume"]
    #[inline(always)]
    pub fn usb_resume(self) -> &'a mut W {
        self.variant(HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A::USB_RESUME)
    }
    #[doc = "USB Operational"]
    #[inline(always)]
    pub fn usb_operational(self) -> &'a mut W {
        self.variant(HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A::USB_OPERATIONAL)
    }
    #[doc = "USB Suspend"]
    #[inline(always)]
    pub fn usb_suspend(self) -> &'a mut W {
        self.variant(HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_A::USB_SUSPEND)
    }
}
#[doc = "Field `interrupt_routing` reader - InterruptRouting This bit determines the routing of interrupts generated by events registered in HcInterruptStatus. If clear, all interrupt are routed to the normal host bus interrupt mechanism. If set interrupts are routed to the System Management Interrupt. HCD clears this bit upon a hardware reset, but it does not alter this bit upon a software reset. HCD uses this bit as a tag to indicate the ownership of HC."]
pub type INTERRUPT_ROUTING_R = crate::BitReader<bool>;
#[doc = "Field `interrupt_routing` writer - InterruptRouting This bit determines the routing of interrupts generated by events registered in HcInterruptStatus. If clear, all interrupt are routed to the normal host bus interrupt mechanism. If set interrupts are routed to the System Management Interrupt. HCD clears this bit upon a hardware reset, but it does not alter this bit upon a software reset. HCD uses this bit as a tag to indicate the ownership of HC."]
pub type INTERRUPT_ROUTING_W<'a, const O: u8> = crate::BitWriter<'a, u32, HC_CONTROL_SPEC, bool, O>;
#[doc = "Field `remote_wakeup_connected` reader - This bit indicates whether HC supports remote wakeup signaling. If remote wakeup is supported and used by the system, it is the responsibility of system firmware to set this bit during POST. HC clear the bit upon a hardware reset but does not alter it upon a software reset. Remote wakeup signaling of the host system is host-bus-specific and is not described in this specification."]
pub type REMOTE_WAKEUP_CONNECTED_R = crate::BitReader<bool>;
#[doc = "Field `remote_wakeup_connected` writer - This bit indicates whether HC supports remote wakeup signaling. If remote wakeup is supported and used by the system, it is the responsibility of system firmware to set this bit during POST. HC clear the bit upon a hardware reset but does not alter it upon a software reset. Remote wakeup signaling of the host system is host-bus-specific and is not described in this specification."]
pub type REMOTE_WAKEUP_CONNECTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_CONTROL_SPEC, bool, O>;
#[doc = "Field `remote_wakeup_enable` reader - This bit is used by HCD to enable or disable the remote wakeup feature upon the detection of upstream resume signaling. When this bit is set and the ResumeDetected bit in HcInterruptStatus is set, a remote wakeup is signaled to the host system. Setting this bit has no impact on the generation of hardware interrupt."]
pub type REMOTE_WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `remote_wakeup_enable` writer - This bit is used by HCD to enable or disable the remote wakeup feature upon the detection of upstream resume signaling. When this bit is set and the ResumeDetected bit in HcInterruptStatus is set, a remote wakeup is signaled to the host system. Setting this bit has no impact on the generation of hardware interrupt."]
pub type REMOTE_WAKEUP_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - This specifies the service ratio between Control and Bulk EDs. Before processing any of the nonperiodic lists, HC must compare the ratio specified with its internal count on how many nonempty Control EDs have been processed, in determining whether to continue serving another Control ED or switching to Bulk EDs. The internal count will be retained when crossing the frame boundary. In case of reset, HCD is responsible for restoring this value."]
    #[inline(always)]
    pub fn control_bulk_service_ratio(&self) -> CONTROL_BULK_SERVICE_RATIO_R {
        CONTROL_BULK_SERVICE_RATIO_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This bit is set to enable the processing of periodic list in the next Frame. If cleared by HCD, processing of the periodic list does not occur after the next SOF. HC must check this bit before it starts processing the list."]
    #[inline(always)]
    pub fn periodic_list_enable(&self) -> PERIODIC_LIST_ENABLE_R {
        PERIODIC_LIST_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is set to enable the processing of the Control list in the next Frame. If cleared by HCD, the processing of the Control list does not occur after the next SOF. HC must check this bit whenever it determines to process the list. When disabled, HCD may modify the list. If HcControlCurrentED is pointing to an ED to be removed, HCD must advance the pointer by updating HcControlCurrentED before re-enabling processing of the list."]
    #[inline(always)]
    pub fn control_list_enable(&self) -> CONTROL_LIST_ENABLE_R {
        CONTROL_LIST_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is used by HCD to enable/disable processing of isochronous EDs. While processing the periodic list in a Frame, HC checks the status of this bit when it finds an Isochronous ED (F=1). If set (enabled), HC continues processing the EDs. If cleared (disabled), HC halts processing of the periodic list (which now contains only isochronous EDs) and begins processing the Bulk/Control lists. Setting this bit is guaranteed to take effect in the next Frame (not the current Frame)."]
    #[inline(always)]
    pub fn isochronous_enable(&self) -> ISOCHRONOUS_ENABLE_R {
        ISOCHRONOUS_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is set to enable the processing of the Bulk list in the next Frame. If cleared by HCD, the processing of the Bulk list does not occur after the next SOF. HC checks this bit whenever it determines to process the list. When disabled, HCD may modify the list. If HcBulkCurrentED is pointing to an ED to be removed, HCD must advance the pointer by updating HcBulkCurrentED before re-enabling processing of the list."]
    #[inline(always)]
    pub fn bulk_list_enable(&self) -> BULK_LIST_ENABLE_R {
        BULK_LIST_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - A transition to USBOperational from another state causes SOF generation to begin 1 ms later. HCD may determine whether HC has begun sending SOFs by reading the StartoFrame field of HcInterruptStatus.\n\nThis field may be changed by HC only when in the USBSUSPEND state. HC may move from the USBSUSPEND state to the USBRESUME state after detecting the resume signaling from a downstream port.\n\nHC enters USBSUSPEND after a software reset, whereas it enters USBRESET after a hardware reset. The latter also resets the Root Hub and asserts subsequent reset signaling to downstream ports."]
    #[inline(always)]
    pub fn host_controller_functional_state_for_usb(
        &self,
    ) -> HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_R {
        HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - InterruptRouting This bit determines the routing of interrupts generated by events registered in HcInterruptStatus. If clear, all interrupt are routed to the normal host bus interrupt mechanism. If set interrupts are routed to the System Management Interrupt. HCD clears this bit upon a hardware reset, but it does not alter this bit upon a software reset. HCD uses this bit as a tag to indicate the ownership of HC."]
    #[inline(always)]
    pub fn interrupt_routing(&self) -> INTERRUPT_ROUTING_R {
        INTERRUPT_ROUTING_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit indicates whether HC supports remote wakeup signaling. If remote wakeup is supported and used by the system, it is the responsibility of system firmware to set this bit during POST. HC clear the bit upon a hardware reset but does not alter it upon a software reset. Remote wakeup signaling of the host system is host-bus-specific and is not described in this specification."]
    #[inline(always)]
    pub fn remote_wakeup_connected(&self) -> REMOTE_WAKEUP_CONNECTED_R {
        REMOTE_WAKEUP_CONNECTED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit is used by HCD to enable or disable the remote wakeup feature upon the detection of upstream resume signaling. When this bit is set and the ResumeDetected bit in HcInterruptStatus is set, a remote wakeup is signaled to the host system. Setting this bit has no impact on the generation of hardware interrupt."]
    #[inline(always)]
    pub fn remote_wakeup_enable(&self) -> REMOTE_WAKEUP_ENABLE_R {
        REMOTE_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - This specifies the service ratio between Control and Bulk EDs. Before processing any of the nonperiodic lists, HC must compare the ratio specified with its internal count on how many nonempty Control EDs have been processed, in determining whether to continue serving another Control ED or switching to Bulk EDs. The internal count will be retained when crossing the frame boundary. In case of reset, HCD is responsible for restoring this value."]
    #[inline(always)]
    #[must_use]
    pub fn control_bulk_service_ratio(&mut self) -> CONTROL_BULK_SERVICE_RATIO_W<0> {
        CONTROL_BULK_SERVICE_RATIO_W::new(self)
    }
    #[doc = "Bit 2 - This bit is set to enable the processing of periodic list in the next Frame. If cleared by HCD, processing of the periodic list does not occur after the next SOF. HC must check this bit before it starts processing the list."]
    #[inline(always)]
    #[must_use]
    pub fn periodic_list_enable(&mut self) -> PERIODIC_LIST_ENABLE_W<2> {
        PERIODIC_LIST_ENABLE_W::new(self)
    }
    #[doc = "Bit 3 - This bit is set to enable the processing of the Control list in the next Frame. If cleared by HCD, the processing of the Control list does not occur after the next SOF. HC must check this bit whenever it determines to process the list. When disabled, HCD may modify the list. If HcControlCurrentED is pointing to an ED to be removed, HCD must advance the pointer by updating HcControlCurrentED before re-enabling processing of the list."]
    #[inline(always)]
    #[must_use]
    pub fn control_list_enable(&mut self) -> CONTROL_LIST_ENABLE_W<3> {
        CONTROL_LIST_ENABLE_W::new(self)
    }
    #[doc = "Bit 3 - This bit is used by HCD to enable/disable processing of isochronous EDs. While processing the periodic list in a Frame, HC checks the status of this bit when it finds an Isochronous ED (F=1). If set (enabled), HC continues processing the EDs. If cleared (disabled), HC halts processing of the periodic list (which now contains only isochronous EDs) and begins processing the Bulk/Control lists. Setting this bit is guaranteed to take effect in the next Frame (not the current Frame)."]
    #[inline(always)]
    #[must_use]
    pub fn isochronous_enable(&mut self) -> ISOCHRONOUS_ENABLE_W<3> {
        ISOCHRONOUS_ENABLE_W::new(self)
    }
    #[doc = "Bit 5 - This bit is set to enable the processing of the Bulk list in the next Frame. If cleared by HCD, the processing of the Bulk list does not occur after the next SOF. HC checks this bit whenever it determines to process the list. When disabled, HCD may modify the list. If HcBulkCurrentED is pointing to an ED to be removed, HCD must advance the pointer by updating HcBulkCurrentED before re-enabling processing of the list."]
    #[inline(always)]
    #[must_use]
    pub fn bulk_list_enable(&mut self) -> BULK_LIST_ENABLE_W<5> {
        BULK_LIST_ENABLE_W::new(self)
    }
    #[doc = "Bits 6:7 - A transition to USBOperational from another state causes SOF generation to begin 1 ms later. HCD may determine whether HC has begun sending SOFs by reading the StartoFrame field of HcInterruptStatus.\n\nThis field may be changed by HC only when in the USBSUSPEND state. HC may move from the USBSUSPEND state to the USBRESUME state after detecting the resume signaling from a downstream port.\n\nHC enters USBSUSPEND after a software reset, whereas it enters USBRESET after a hardware reset. The latter also resets the Root Hub and asserts subsequent reset signaling to downstream ports."]
    #[inline(always)]
    #[must_use]
    pub fn host_controller_functional_state_for_usb(
        &mut self,
    ) -> HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_W<6> {
        HOST_CONTROLLER_FUNCTIONAL_STATE_FOR_USB_W::new(self)
    }
    #[doc = "Bit 8 - InterruptRouting This bit determines the routing of interrupts generated by events registered in HcInterruptStatus. If clear, all interrupt are routed to the normal host bus interrupt mechanism. If set interrupts are routed to the System Management Interrupt. HCD clears this bit upon a hardware reset, but it does not alter this bit upon a software reset. HCD uses this bit as a tag to indicate the ownership of HC."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_routing(&mut self) -> INTERRUPT_ROUTING_W<8> {
        INTERRUPT_ROUTING_W::new(self)
    }
    #[doc = "Bit 9 - This bit indicates whether HC supports remote wakeup signaling. If remote wakeup is supported and used by the system, it is the responsibility of system firmware to set this bit during POST. HC clear the bit upon a hardware reset but does not alter it upon a software reset. Remote wakeup signaling of the host system is host-bus-specific and is not described in this specification."]
    #[inline(always)]
    #[must_use]
    pub fn remote_wakeup_connected(&mut self) -> REMOTE_WAKEUP_CONNECTED_W<9> {
        REMOTE_WAKEUP_CONNECTED_W::new(self)
    }
    #[doc = "Bit 10 - This bit is used by HCD to enable or disable the remote wakeup feature upon the detection of upstream resume signaling. When this bit is set and the ResumeDetected bit in HcInterruptStatus is set, a remote wakeup is signaled to the host system. Setting this bit has no impact on the generation of hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn remote_wakeup_enable(&mut self) -> REMOTE_WAKEUP_ENABLE_W<10> {
        REMOTE_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc_control](index.html) module"]
pub struct HC_CONTROL_SPEC;
impl crate::RegisterSpec for HC_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc_control::R](R) reader structure"]
impl crate::Readable for HC_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc_control::W](W) writer structure"]
impl crate::Writable for HC_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_control to value 0"]
impl crate::Resettable for HC_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
