#[doc = "Register `usbcmd` reader"]
pub type R = crate::R<USBCMD_SPEC>;
#[doc = "Register `usbcmd` writer"]
pub type W = crate::W<USBCMD_SPEC>;
#[doc = "Field `run_stop` reader - Run/Stop\n\nWhen set to a 1, the Host Controller proceeds with execution of the schedule. When set to 0, the Host Controller completes the current and any actively pipelined transactions on the USB and then halts. The Host Controller must halt within 16 micro-frames after software clears this bit. The HC Halted bit indicates when the Host Controller has finished its pending pipelined transactions and has entered the stopped state.\n\nSoftware must not write a one to this field unless the Host Controller is in the Halt State. The default value is 0x0."]
pub type RUN_STOP_R = crate::BitReader;
#[doc = "Field `run_stop` writer - Run/Stop\n\nWhen set to a 1, the Host Controller proceeds with execution of the schedule. When set to 0, the Host Controller completes the current and any actively pipelined transactions on the USB and then halts. The Host Controller must halt within 16 micro-frames after software clears this bit. The HC Halted bit indicates when the Host Controller has finished its pending pipelined transactions and has entered the stopped state.\n\nSoftware must not write a one to this field unless the Host Controller is in the Halt State. The default value is 0x0."]
pub type RUN_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `host_controller_reset` reader - Host Controller Reset\n\nThis control bit is used by software to reset the host controller. The effects of this on Root Hub registers are similar to a Chip Hardware Reset.\n\nWhen software writes a one to this bit, the Host Controller resets its internal pipelines, timers, counters, state machines, etc. to their initial value. Any transaction currently in progress on USB is immediately terminated. A USB reset is not driven on downstream ports.\n\nAll operational registers, including port registers and port state machines are set to their initial values. Port ownership reverts to the companion host controller(s). Software must reinitialize the host controller as described in Section 4.1 of the CHEI Specification in order to return the host controller to an operational state. This bit is set to zero by the Host Controller when the reset process is complete. Software cannot terminate the reset process early by writing a zero to this register.\n\nSoftware should not set this bit to a one when the HC Halted bit in the USBSTS register is a zero. Attempting to reset an actively running host controller will result in undefined behavior."]
pub type HOST_CONTROLLER_RESET_R = crate::BitReader;
#[doc = "Field `host_controller_reset` writer - Host Controller Reset\n\nThis control bit is used by software to reset the host controller. The effects of this on Root Hub registers are similar to a Chip Hardware Reset.\n\nWhen software writes a one to this bit, the Host Controller resets its internal pipelines, timers, counters, state machines, etc. to their initial value. Any transaction currently in progress on USB is immediately terminated. A USB reset is not driven on downstream ports.\n\nAll operational registers, including port registers and port state machines are set to their initial values. Port ownership reverts to the companion host controller(s). Software must reinitialize the host controller as described in Section 4.1 of the CHEI Specification in order to return the host controller to an operational state. This bit is set to zero by the Host Controller when the reset process is complete. Software cannot terminate the reset process early by writing a zero to this register.\n\nSoftware should not set this bit to a one when the HC Halted bit in the USBSTS register is a zero. Attempting to reset an actively running host controller will result in undefined behavior."]
pub type HOST_CONTROLLER_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `frame_list_size` reader - This field is R/W only if Programmable Frame List Flag in the HCCPARAMS register is set to one. This field specifies the size of the Frame list."]
pub type FRAME_LIST_SIZE_R = crate::FieldReader<FRAME_LIST_SIZE_A>;
#[doc = "This field is R/W only if Programmable Frame List Flag in the HCCPARAMS register is set to one. This field specifies the size of the Frame list.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRAME_LIST_SIZE_A {
    #[doc = "0: 1024 frames"]
    F1024 = 0,
    #[doc = "1: 512 frames"]
    F512 = 1,
    #[doc = "2: 256 frames"]
    F256 = 2,
}
impl From<FRAME_LIST_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: FRAME_LIST_SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FRAME_LIST_SIZE_A {
    type Ux = u8;
}
impl FRAME_LIST_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FRAME_LIST_SIZE_A> {
        match self.bits {
            0 => Some(FRAME_LIST_SIZE_A::F1024),
            1 => Some(FRAME_LIST_SIZE_A::F512),
            2 => Some(FRAME_LIST_SIZE_A::F256),
            _ => None,
        }
    }
    #[doc = "1024 frames"]
    #[inline(always)]
    pub fn is_f1024(&self) -> bool {
        *self == FRAME_LIST_SIZE_A::F1024
    }
    #[doc = "512 frames"]
    #[inline(always)]
    pub fn is_f512(&self) -> bool {
        *self == FRAME_LIST_SIZE_A::F512
    }
    #[doc = "256 frames"]
    #[inline(always)]
    pub fn is_f256(&self) -> bool {
        *self == FRAME_LIST_SIZE_A::F256
    }
}
#[doc = "Field `frame_list_size` writer - This field is R/W only if Programmable Frame List Flag in the HCCPARAMS register is set to one. This field specifies the size of the Frame list."]
pub type FRAME_LIST_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FRAME_LIST_SIZE_A>;
impl<'a, REG> FRAME_LIST_SIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1024 frames"]
    #[inline(always)]
    pub fn f1024(self) -> &'a mut crate::W<REG> {
        self.variant(FRAME_LIST_SIZE_A::F1024)
    }
    #[doc = "512 frames"]
    #[inline(always)]
    pub fn f512(self) -> &'a mut crate::W<REG> {
        self.variant(FRAME_LIST_SIZE_A::F512)
    }
    #[doc = "256 frames"]
    #[inline(always)]
    pub fn f256(self) -> &'a mut crate::W<REG> {
        self.variant(FRAME_LIST_SIZE_A::F256)
    }
}
#[doc = "Field `periodic_schedule_enable` reader - Periodic Schedule Enable\n\nThis bit controls whether the host controller skips processing the Periodic Schedule."]
pub type PERIODIC_SCHEDULE_ENABLE_R = crate::BitReader<PERIODIC_SCHEDULE_ENABLE_A>;
#[doc = "Periodic Schedule Enable\n\nThis bit controls whether the host controller skips processing the Periodic Schedule.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERIODIC_SCHEDULE_ENABLE_A {
    #[doc = "0: Do not process the Periodic Schedule"]
    DISABLE = 0,
    #[doc = "1: Use the PERIODICLISTBASE register to access the Periodic Schedule"]
    ENABLE = 1,
}
impl From<PERIODIC_SCHEDULE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PERIODIC_SCHEDULE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl PERIODIC_SCHEDULE_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PERIODIC_SCHEDULE_ENABLE_A {
        match self.bits {
            false => PERIODIC_SCHEDULE_ENABLE_A::DISABLE,
            true => PERIODIC_SCHEDULE_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Do not process the Periodic Schedule"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PERIODIC_SCHEDULE_ENABLE_A::DISABLE
    }
    #[doc = "Use the PERIODICLISTBASE register to access the Periodic Schedule"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PERIODIC_SCHEDULE_ENABLE_A::ENABLE
    }
}
#[doc = "Field `periodic_schedule_enable` writer - Periodic Schedule Enable\n\nThis bit controls whether the host controller skips processing the Periodic Schedule."]
pub type PERIODIC_SCHEDULE_ENABLE_W<'a, REG> =
    crate::BitWriter<'a, REG, PERIODIC_SCHEDULE_ENABLE_A>;
impl<'a, REG> PERIODIC_SCHEDULE_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not process the Periodic Schedule"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PERIODIC_SCHEDULE_ENABLE_A::DISABLE)
    }
    #[doc = "Use the PERIODICLISTBASE register to access the Periodic Schedule"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PERIODIC_SCHEDULE_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `asynchronous_schedule_enable` reader - Asynchronous Schedule Enable\n\nThis bit controls whether the host controller skips processing the Asynchronous Schedule."]
pub type ASYNCHRONOUS_SCHEDULE_ENABLE_R = crate::BitReader<ASYNCHRONOUS_SCHEDULE_ENABLE_A>;
#[doc = "Asynchronous Schedule Enable\n\nThis bit controls whether the host controller skips processing the Asynchronous Schedule.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASYNCHRONOUS_SCHEDULE_ENABLE_A {
    #[doc = "0: Do not process the Asynchronous Schedule"]
    DISABLE = 0,
    #[doc = "1: Use the ASYNLISTADDR register to access the Asynchronous Schedule"]
    ENABLE = 1,
}
impl From<ASYNCHRONOUS_SCHEDULE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ASYNCHRONOUS_SCHEDULE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ASYNCHRONOUS_SCHEDULE_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASYNCHRONOUS_SCHEDULE_ENABLE_A {
        match self.bits {
            false => ASYNCHRONOUS_SCHEDULE_ENABLE_A::DISABLE,
            true => ASYNCHRONOUS_SCHEDULE_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Do not process the Asynchronous Schedule"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ASYNCHRONOUS_SCHEDULE_ENABLE_A::DISABLE
    }
    #[doc = "Use the ASYNLISTADDR register to access the Asynchronous Schedule"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ASYNCHRONOUS_SCHEDULE_ENABLE_A::ENABLE
    }
}
#[doc = "Field `asynchronous_schedule_enable` writer - Asynchronous Schedule Enable\n\nThis bit controls whether the host controller skips processing the Asynchronous Schedule."]
pub type ASYNCHRONOUS_SCHEDULE_ENABLE_W<'a, REG> =
    crate::BitWriter<'a, REG, ASYNCHRONOUS_SCHEDULE_ENABLE_A>;
impl<'a, REG> ASYNCHRONOUS_SCHEDULE_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not process the Asynchronous Schedule"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCHRONOUS_SCHEDULE_ENABLE_A::DISABLE)
    }
    #[doc = "Use the ASYNLISTADDR register to access the Asynchronous Schedule"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCHRONOUS_SCHEDULE_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `interrupt_on_async_advance_doorbell` reader - Interrupt on Async Advance Doorbell\n\nThis bit is used as a doorbell by software to tell the host controller to issue an interrupt the next time it advances asynchronous schedule. Software must write a 1 to this bit to ring the doorbell. When the host controller has evicted all appropriate cached schedule state, it sets the Interrupt on Async Advance status bit in the USBSTS. if the Interrupt on Async Advance Enable bit in the USBINTR register is a one then the host controller will assert an interrupt at the next interrupt threshold.\n\nThe host controller sets this bit to a zero after it has set the Interrupt on Async Advance status bit in the USBSTS register to a one.\n\nSoftware should not write a one to this bit when the asynchronous schedule is disabled. Doing so will yield undefined results."]
pub type INTERRUPT_ON_ASYNC_ADVANCE_DOORBELL_R = crate::BitReader;
#[doc = "Field `interrupt_on_async_advance_doorbell` writer - Interrupt on Async Advance Doorbell\n\nThis bit is used as a doorbell by software to tell the host controller to issue an interrupt the next time it advances asynchronous schedule. Software must write a 1 to this bit to ring the doorbell. When the host controller has evicted all appropriate cached schedule state, it sets the Interrupt on Async Advance status bit in the USBSTS. if the Interrupt on Async Advance Enable bit in the USBINTR register is a one then the host controller will assert an interrupt at the next interrupt threshold.\n\nThe host controller sets this bit to a zero after it has set the Interrupt on Async Advance status bit in the USBSTS register to a one.\n\nSoftware should not write a one to this bit when the asynchronous schedule is disabled. Doing so will yield undefined results."]
pub type INTERRUPT_ON_ASYNC_ADVANCE_DOORBELL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `light_host_controller_reset` reader - Light Host Controller Reset (OPTIONAL)\n\nThis control bit is not required. If implemented, it allows the driver to reset the EHCI controller without affecting the state of the ports or relationship to the companion host controllers. For example, the PORSTC registers should not be reset to their default values and the CF bit setting should not go to zero (retaining port ownership relationships). A host software read of this bit as zero indicates the Light Host Controller Reset has completed and it si safe for software to re- initialize the host controller. A host software read of this bit as a one indicates the Light Host"]
pub type LIGHT_HOST_CONTROLLER_RESET_R = crate::BitReader;
#[doc = "Field `light_host_controller_reset` writer - Light Host Controller Reset (OPTIONAL)\n\nThis control bit is not required. If implemented, it allows the driver to reset the EHCI controller without affecting the state of the ports or relationship to the companion host controllers. For example, the PORSTC registers should not be reset to their default values and the CF bit setting should not go to zero (retaining port ownership relationships). A host software read of this bit as zero indicates the Light Host Controller Reset has completed and it si safe for software to re- initialize the host controller. A host software read of this bit as a one indicates the Light Host"]
pub type LIGHT_HOST_CONTROLLER_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `asynchronous_schedule_park_mode_count` reader - Asynchronous Schedule Park Mode Count (OPTIONAL)\n\nAsynchronous Park Capability bit in the HCCPARAMS register is a one, then this field defaults to 0x3 and is W/R. Otherwise it defaults to zero and is R. It contains a count of the number of successive transactions the host controller is allowed to execute from a high-speed queue head on the Asynchronous schedule before continuing traversal of the Asynchronous schedule. Valid value are 0x1 to 0x3.Software must not write a zero to this bit when Park Mode Enable is a one as it will result in undefined behavior."]
pub type ASYNCHRONOUS_SCHEDULE_PARK_MODE_COUNT_R = crate::FieldReader;
#[doc = "Field `asynchronous_schedule_park_mode_enable` reader - Asynchronous Schedule Park Mode Enable (OPTIONAL)\n\nIf the Asynchronous Park Capability bit in the HCCPARAMS register is a one, then this bit defaults to a 1 and is R/W. Otherwise the bit must be a zero and is Read Only. Software uses this bit to enable or disable Park mode. When this bit is one, Park mode is enabled. When this bit is zero, Park mode is disabled."]
pub type ASYNCHRONOUS_SCHEDULE_PARK_MODE_ENABLE_R = crate::BitReader;
#[doc = "Field `interrupt_threshold_control` reader - Interrupt Threshold Control\n\nThe value in this field is used by system software to select the maximum rate at which the host controller will issue interrupts."]
pub type INTERRUPT_THRESHOLD_CONTROL_R = crate::FieldReader<INTERRUPT_THRESHOLD_CONTROL_A>;
#[doc = "Interrupt Threshold Control\n\nThe value in this field is used by system software to select the maximum rate at which the host controller will issue interrupts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTERRUPT_THRESHOLD_CONTROL_A {
    #[doc = "1: 1 micro-frame"]
    MF1 = 1,
    #[doc = "2: 2 micro-frames"]
    MF2 = 2,
    #[doc = "4: 4 micro-frames"]
    MF4 = 4,
    #[doc = "8: 8 micro-frames (default, equates to 1ms)"]
    MF8 = 8,
    #[doc = "16: 16 micro-frames (2ms)"]
    MF16 = 16,
    #[doc = "32: 32 micro-frames (4ms)"]
    MF32 = 32,
    #[doc = "64: 64 micro-frames (8ms)"]
    MF64 = 64,
}
impl From<INTERRUPT_THRESHOLD_CONTROL_A> for u8 {
    #[inline(always)]
    fn from(variant: INTERRUPT_THRESHOLD_CONTROL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INTERRUPT_THRESHOLD_CONTROL_A {
    type Ux = u8;
}
impl INTERRUPT_THRESHOLD_CONTROL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INTERRUPT_THRESHOLD_CONTROL_A> {
        match self.bits {
            1 => Some(INTERRUPT_THRESHOLD_CONTROL_A::MF1),
            2 => Some(INTERRUPT_THRESHOLD_CONTROL_A::MF2),
            4 => Some(INTERRUPT_THRESHOLD_CONTROL_A::MF4),
            8 => Some(INTERRUPT_THRESHOLD_CONTROL_A::MF8),
            16 => Some(INTERRUPT_THRESHOLD_CONTROL_A::MF16),
            32 => Some(INTERRUPT_THRESHOLD_CONTROL_A::MF32),
            64 => Some(INTERRUPT_THRESHOLD_CONTROL_A::MF64),
            _ => None,
        }
    }
    #[doc = "1 micro-frame"]
    #[inline(always)]
    pub fn is_mf1(&self) -> bool {
        *self == INTERRUPT_THRESHOLD_CONTROL_A::MF1
    }
    #[doc = "2 micro-frames"]
    #[inline(always)]
    pub fn is_mf2(&self) -> bool {
        *self == INTERRUPT_THRESHOLD_CONTROL_A::MF2
    }
    #[doc = "4 micro-frames"]
    #[inline(always)]
    pub fn is_mf4(&self) -> bool {
        *self == INTERRUPT_THRESHOLD_CONTROL_A::MF4
    }
    #[doc = "8 micro-frames (default, equates to 1ms)"]
    #[inline(always)]
    pub fn is_mf8(&self) -> bool {
        *self == INTERRUPT_THRESHOLD_CONTROL_A::MF8
    }
    #[doc = "16 micro-frames (2ms)"]
    #[inline(always)]
    pub fn is_mf16(&self) -> bool {
        *self == INTERRUPT_THRESHOLD_CONTROL_A::MF16
    }
    #[doc = "32 micro-frames (4ms)"]
    #[inline(always)]
    pub fn is_mf32(&self) -> bool {
        *self == INTERRUPT_THRESHOLD_CONTROL_A::MF32
    }
    #[doc = "64 micro-frames (8ms)"]
    #[inline(always)]
    pub fn is_mf64(&self) -> bool {
        *self == INTERRUPT_THRESHOLD_CONTROL_A::MF64
    }
}
#[doc = "Field `interrupt_threshold_control` writer - Interrupt Threshold Control\n\nThe value in this field is used by system software to select the maximum rate at which the host controller will issue interrupts."]
pub type INTERRUPT_THRESHOLD_CONTROL_W<'a, REG> =
    crate::FieldWriter<'a, REG, 8, INTERRUPT_THRESHOLD_CONTROL_A>;
impl<'a, REG> INTERRUPT_THRESHOLD_CONTROL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 micro-frame"]
    #[inline(always)]
    pub fn mf1(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_THRESHOLD_CONTROL_A::MF1)
    }
    #[doc = "2 micro-frames"]
    #[inline(always)]
    pub fn mf2(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_THRESHOLD_CONTROL_A::MF2)
    }
    #[doc = "4 micro-frames"]
    #[inline(always)]
    pub fn mf4(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_THRESHOLD_CONTROL_A::MF4)
    }
    #[doc = "8 micro-frames (default, equates to 1ms)"]
    #[inline(always)]
    pub fn mf8(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_THRESHOLD_CONTROL_A::MF8)
    }
    #[doc = "16 micro-frames (2ms)"]
    #[inline(always)]
    pub fn mf16(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_THRESHOLD_CONTROL_A::MF16)
    }
    #[doc = "32 micro-frames (4ms)"]
    #[inline(always)]
    pub fn mf32(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_THRESHOLD_CONTROL_A::MF32)
    }
    #[doc = "64 micro-frames (8ms)"]
    #[inline(always)]
    pub fn mf64(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_THRESHOLD_CONTROL_A::MF64)
    }
}
impl R {
    #[doc = "Bit 0 - Run/Stop\n\nWhen set to a 1, the Host Controller proceeds with execution of the schedule. When set to 0, the Host Controller completes the current and any actively pipelined transactions on the USB and then halts. The Host Controller must halt within 16 micro-frames after software clears this bit. The HC Halted bit indicates when the Host Controller has finished its pending pipelined transactions and has entered the stopped state.\n\nSoftware must not write a one to this field unless the Host Controller is in the Halt State. The default value is 0x0."]
    #[inline(always)]
    pub fn run_stop(&self) -> RUN_STOP_R {
        RUN_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host Controller Reset\n\nThis control bit is used by software to reset the host controller. The effects of this on Root Hub registers are similar to a Chip Hardware Reset.\n\nWhen software writes a one to this bit, the Host Controller resets its internal pipelines, timers, counters, state machines, etc. to their initial value. Any transaction currently in progress on USB is immediately terminated. A USB reset is not driven on downstream ports.\n\nAll operational registers, including port registers and port state machines are set to their initial values. Port ownership reverts to the companion host controller(s). Software must reinitialize the host controller as described in Section 4.1 of the CHEI Specification in order to return the host controller to an operational state. This bit is set to zero by the Host Controller when the reset process is complete. Software cannot terminate the reset process early by writing a zero to this register.\n\nSoftware should not set this bit to a one when the HC Halted bit in the USBSTS register is a zero. Attempting to reset an actively running host controller will result in undefined behavior."]
    #[inline(always)]
    pub fn host_controller_reset(&self) -> HOST_CONTROLLER_RESET_R {
        HOST_CONTROLLER_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - This field is R/W only if Programmable Frame List Flag in the HCCPARAMS register is set to one. This field specifies the size of the Frame list."]
    #[inline(always)]
    pub fn frame_list_size(&self) -> FRAME_LIST_SIZE_R {
        FRAME_LIST_SIZE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Periodic Schedule Enable\n\nThis bit controls whether the host controller skips processing the Periodic Schedule."]
    #[inline(always)]
    pub fn periodic_schedule_enable(&self) -> PERIODIC_SCHEDULE_ENABLE_R {
        PERIODIC_SCHEDULE_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable\n\nThis bit controls whether the host controller skips processing the Asynchronous Schedule."]
    #[inline(always)]
    pub fn asynchronous_schedule_enable(&self) -> ASYNCHRONOUS_SCHEDULE_ENABLE_R {
        ASYNCHRONOUS_SCHEDULE_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt on Async Advance Doorbell\n\nThis bit is used as a doorbell by software to tell the host controller to issue an interrupt the next time it advances asynchronous schedule. Software must write a 1 to this bit to ring the doorbell. When the host controller has evicted all appropriate cached schedule state, it sets the Interrupt on Async Advance status bit in the USBSTS. if the Interrupt on Async Advance Enable bit in the USBINTR register is a one then the host controller will assert an interrupt at the next interrupt threshold.\n\nThe host controller sets this bit to a zero after it has set the Interrupt on Async Advance status bit in the USBSTS register to a one.\n\nSoftware should not write a one to this bit when the asynchronous schedule is disabled. Doing so will yield undefined results."]
    #[inline(always)]
    pub fn interrupt_on_async_advance_doorbell(&self) -> INTERRUPT_ON_ASYNC_ADVANCE_DOORBELL_R {
        INTERRUPT_ON_ASYNC_ADVANCE_DOORBELL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Light Host Controller Reset (OPTIONAL)\n\nThis control bit is not required. If implemented, it allows the driver to reset the EHCI controller without affecting the state of the ports or relationship to the companion host controllers. For example, the PORSTC registers should not be reset to their default values and the CF bit setting should not go to zero (retaining port ownership relationships). A host software read of this bit as zero indicates the Light Host Controller Reset has completed and it si safe for software to re- initialize the host controller. A host software read of this bit as a one indicates the Light Host"]
    #[inline(always)]
    pub fn light_host_controller_reset(&self) -> LIGHT_HOST_CONTROLLER_RESET_R {
        LIGHT_HOST_CONTROLLER_RESET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Asynchronous Schedule Park Mode Count (OPTIONAL)\n\nAsynchronous Park Capability bit in the HCCPARAMS register is a one, then this field defaults to 0x3 and is W/R. Otherwise it defaults to zero and is R. It contains a count of the number of successive transactions the host controller is allowed to execute from a high-speed queue head on the Asynchronous schedule before continuing traversal of the Asynchronous schedule. Valid value are 0x1 to 0x3.Software must not write a zero to this bit when Park Mode Enable is a one as it will result in undefined behavior."]
    #[inline(always)]
    pub fn asynchronous_schedule_park_mode_count(&self) -> ASYNCHRONOUS_SCHEDULE_PARK_MODE_COUNT_R {
        ASYNCHRONOUS_SCHEDULE_PARK_MODE_COUNT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Asynchronous Schedule Park Mode Enable (OPTIONAL)\n\nIf the Asynchronous Park Capability bit in the HCCPARAMS register is a one, then this bit defaults to a 1 and is R/W. Otherwise the bit must be a zero and is Read Only. Software uses this bit to enable or disable Park mode. When this bit is one, Park mode is enabled. When this bit is zero, Park mode is disabled."]
    #[inline(always)]
    pub fn asynchronous_schedule_park_mode_enable(
        &self,
    ) -> ASYNCHRONOUS_SCHEDULE_PARK_MODE_ENABLE_R {
        ASYNCHRONOUS_SCHEDULE_PARK_MODE_ENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control\n\nThe value in this field is used by system software to select the maximum rate at which the host controller will issue interrupts."]
    #[inline(always)]
    pub fn interrupt_threshold_control(&self) -> INTERRUPT_THRESHOLD_CONTROL_R {
        INTERRUPT_THRESHOLD_CONTROL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Run/Stop\n\nWhen set to a 1, the Host Controller proceeds with execution of the schedule. When set to 0, the Host Controller completes the current and any actively pipelined transactions on the USB and then halts. The Host Controller must halt within 16 micro-frames after software clears this bit. The HC Halted bit indicates when the Host Controller has finished its pending pipelined transactions and has entered the stopped state.\n\nSoftware must not write a one to this field unless the Host Controller is in the Halt State. The default value is 0x0."]
    #[inline(always)]
    #[must_use]
    pub fn run_stop(&mut self) -> RUN_STOP_W<USBCMD_SPEC> {
        RUN_STOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Host Controller Reset\n\nThis control bit is used by software to reset the host controller. The effects of this on Root Hub registers are similar to a Chip Hardware Reset.\n\nWhen software writes a one to this bit, the Host Controller resets its internal pipelines, timers, counters, state machines, etc. to their initial value. Any transaction currently in progress on USB is immediately terminated. A USB reset is not driven on downstream ports.\n\nAll operational registers, including port registers and port state machines are set to their initial values. Port ownership reverts to the companion host controller(s). Software must reinitialize the host controller as described in Section 4.1 of the CHEI Specification in order to return the host controller to an operational state. This bit is set to zero by the Host Controller when the reset process is complete. Software cannot terminate the reset process early by writing a zero to this register.\n\nSoftware should not set this bit to a one when the HC Halted bit in the USBSTS register is a zero. Attempting to reset an actively running host controller will result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn host_controller_reset(&mut self) -> HOST_CONTROLLER_RESET_W<USBCMD_SPEC> {
        HOST_CONTROLLER_RESET_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - This field is R/W only if Programmable Frame List Flag in the HCCPARAMS register is set to one. This field specifies the size of the Frame list."]
    #[inline(always)]
    #[must_use]
    pub fn frame_list_size(&mut self) -> FRAME_LIST_SIZE_W<USBCMD_SPEC> {
        FRAME_LIST_SIZE_W::new(self, 2)
    }
    #[doc = "Bit 4 - Periodic Schedule Enable\n\nThis bit controls whether the host controller skips processing the Periodic Schedule."]
    #[inline(always)]
    #[must_use]
    pub fn periodic_schedule_enable(&mut self) -> PERIODIC_SCHEDULE_ENABLE_W<USBCMD_SPEC> {
        PERIODIC_SCHEDULE_ENABLE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable\n\nThis bit controls whether the host controller skips processing the Asynchronous Schedule."]
    #[inline(always)]
    #[must_use]
    pub fn asynchronous_schedule_enable(&mut self) -> ASYNCHRONOUS_SCHEDULE_ENABLE_W<USBCMD_SPEC> {
        ASYNCHRONOUS_SCHEDULE_ENABLE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt on Async Advance Doorbell\n\nThis bit is used as a doorbell by software to tell the host controller to issue an interrupt the next time it advances asynchronous schedule. Software must write a 1 to this bit to ring the doorbell. When the host controller has evicted all appropriate cached schedule state, it sets the Interrupt on Async Advance status bit in the USBSTS. if the Interrupt on Async Advance Enable bit in the USBINTR register is a one then the host controller will assert an interrupt at the next interrupt threshold.\n\nThe host controller sets this bit to a zero after it has set the Interrupt on Async Advance status bit in the USBSTS register to a one.\n\nSoftware should not write a one to this bit when the asynchronous schedule is disabled. Doing so will yield undefined results."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_on_async_advance_doorbell(
        &mut self,
    ) -> INTERRUPT_ON_ASYNC_ADVANCE_DOORBELL_W<USBCMD_SPEC> {
        INTERRUPT_ON_ASYNC_ADVANCE_DOORBELL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Light Host Controller Reset (OPTIONAL)\n\nThis control bit is not required. If implemented, it allows the driver to reset the EHCI controller without affecting the state of the ports or relationship to the companion host controllers. For example, the PORSTC registers should not be reset to their default values and the CF bit setting should not go to zero (retaining port ownership relationships). A host software read of this bit as zero indicates the Light Host Controller Reset has completed and it si safe for software to re- initialize the host controller. A host software read of this bit as a one indicates the Light Host"]
    #[inline(always)]
    #[must_use]
    pub fn light_host_controller_reset(&mut self) -> LIGHT_HOST_CONTROLLER_RESET_W<USBCMD_SPEC> {
        LIGHT_HOST_CONTROLLER_RESET_W::new(self, 7)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control\n\nThe value in this field is used by system software to select the maximum rate at which the host controller will issue interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_threshold_control(&mut self) -> INTERRUPT_THRESHOLD_CONTROL_W<USBCMD_SPEC> {
        INTERRUPT_THRESHOLD_CONTROL_W::new(self, 16)
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
#[doc = "EHCI USB Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbcmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbcmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBCMD_SPEC;
impl crate::RegisterSpec for USBCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbcmd::R`](R) reader structure"]
impl crate::Readable for USBCMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbcmd::W`](W) writer structure"]
impl crate::Writable for USBCMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets usbcmd to value 0"]
impl crate::Resettable for USBCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
