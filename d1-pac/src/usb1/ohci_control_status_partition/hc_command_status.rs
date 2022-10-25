#[doc = "Register `hc_command_status` reader"]
pub struct R(crate::R<HC_COMMAND_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC_COMMAND_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC_COMMAND_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC_COMMAND_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hc_command_status` writer"]
pub struct W(crate::W<HC_COMMAND_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC_COMMAND_STATUS_SPEC>;
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
impl From<crate::W<HC_COMMAND_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC_COMMAND_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `host_controller_reset` reader - HostControllerReset\n\nThis bit is by HCD to initiate a software reset of HC. Regardless of the functional state of HC, it moves to the USBSuspend state in which most of the operational registers are reset except those stated otherwise; e.g, the InteruptRouting field of HcControl, and no Host bus accesses are allowed. This bit is cleared by HC upon the completion of the reset operation. The reset operation must be completed within 10 ms. This bit, when set, should not cause a reset to the Root Hub and no subsequent reset signaling should be asserted to its downstream ports."]
pub type HOST_CONTROLLER_RESET_R = crate::BitReader<bool>;
#[doc = "Field `host_controller_reset` writer - HostControllerReset\n\nThis bit is by HCD to initiate a software reset of HC. Regardless of the functional state of HC, it moves to the USBSuspend state in which most of the operational registers are reset except those stated otherwise; e.g, the InteruptRouting field of HcControl, and no Host bus accesses are allowed. This bit is cleared by HC upon the completion of the reset operation. The reset operation must be completed within 10 ms. This bit, when set, should not cause a reset to the Root Hub and no subsequent reset signaling should be asserted to its downstream ports."]
pub type HOST_CONTROLLER_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_COMMAND_STATUS_SPEC, bool, O>;
#[doc = "Field `control_list_filled` reader - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list. It is set by HCD whenever it adds a TD to an ED in the Control list.\n\nWhen HC begins to process the head of the Control list, it checks CLF. As long as ControlListFilled is 0, HC will not start processing the Control list. If CF is 1, HC will start processing the Control list and will set ControlListFilled to 0. If HC finds a TD on the list, then HC will set ControlListFilled to 1 causing the Control list processing to continue. If no TD is found on the Control list, and if the HCD does not set ControlListFilled , then ControlListFilled will still be 0 when HC completes processing the Control list and Control list processing will stop."]
pub type CONTROL_LIST_FILLED_R = crate::BitReader<bool>;
#[doc = "Field `control_list_filled` writer - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list. It is set by HCD whenever it adds a TD to an ED in the Control list.\n\nWhen HC begins to process the head of the Control list, it checks CLF. As long as ControlListFilled is 0, HC will not start processing the Control list. If CF is 1, HC will start processing the Control list and will set ControlListFilled to 0. If HC finds a TD on the list, then HC will set ControlListFilled to 1 causing the Control list processing to continue. If no TD is found on the Control list, and if the HCD does not set ControlListFilled , then ControlListFilled will still be 0 when HC completes processing the Control list and Control list processing will stop."]
pub type CONTROL_LIST_FILLED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_COMMAND_STATUS_SPEC, bool, O>;
#[doc = "Field `bulkl_list_filled` reader - BulklListFilled\n\nThis bit is used to indicate whether there are any TDs on the Bulk list. It is set by HCD whenever it adds a TD to an ED in the Bulk list. When HC begins to process the head of the Bulk list, it checks BLF. As long as BulkListFilled is 0, HC will not start processing the Bulk list. If BulkListFilled is 1, HC will start processing the Bulk list and will set BF to 0. If HC finds a TD on the list, then HC will set BulkListFilled to 1 causing the Bulk list processing to continue. If no TD is found on the Bulk list, and if HCD does not set BulkListFilled , then BulkListFilled will still be 0 when HC completes processing the Bulk list and Bulk list processing will stop."]
pub type BULKL_LIST_FILLED_R = crate::BitReader<bool>;
#[doc = "Field `bulkl_list_filled` writer - BulklListFilled\n\nThis bit is used to indicate whether there are any TDs on the Bulk list. It is set by HCD whenever it adds a TD to an ED in the Bulk list. When HC begins to process the head of the Bulk list, it checks BLF. As long as BulkListFilled is 0, HC will not start processing the Bulk list. If BulkListFilled is 1, HC will start processing the Bulk list and will set BF to 0. If HC finds a TD on the list, then HC will set BulkListFilled to 1 causing the Bulk list processing to continue. If no TD is found on the Bulk list, and if HCD does not set BulkListFilled , then BulkListFilled will still be 0 when HC completes processing the Bulk list and Bulk list processing will stop."]
pub type BULKL_LIST_FILLED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_COMMAND_STATUS_SPEC, bool, O>;
#[doc = "Field `owership_change_request` reader - OwershipChangeRequest\n\nThis bit is set by an OS HCD to request a change of control of the HC. When set HC will set the OwnershipChange field in. After the changeover, this bit is cleared and remains so until the next request from OS HCD."]
pub type OWERSHIP_CHANGE_REQUEST_R = crate::BitReader<bool>;
#[doc = "Field `owership_change_request` writer - OwershipChangeRequest\n\nThis bit is set by an OS HCD to request a change of control of the HC. When set HC will set the OwnershipChange field in. After the changeover, this bit is cleared and remains so until the next request from OS HCD."]
pub type OWERSHIP_CHANGE_REQUEST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_COMMAND_STATUS_SPEC, bool, O>;
#[doc = "Field `scheduling_overrun_count` reader - SchedulingOverrunCoun\n\nThese bits are incremented on each scheduling overrun error. It is initialized to 00b and wraps around at 11b. This will be incremented when a scheduling overrun is detected even if SchedulingOverrun in has already been set. This is used by HCD to monitor any persistent scheduling problem."]
pub type SCHEDULING_OVERRUN_COUNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - HostControllerReset\n\nThis bit is by HCD to initiate a software reset of HC. Regardless of the functional state of HC, it moves to the USBSuspend state in which most of the operational registers are reset except those stated otherwise; e.g, the InteruptRouting field of HcControl, and no Host bus accesses are allowed. This bit is cleared by HC upon the completion of the reset operation. The reset operation must be completed within 10 ms. This bit, when set, should not cause a reset to the Root Hub and no subsequent reset signaling should be asserted to its downstream ports."]
    #[inline(always)]
    pub fn host_controller_reset(&self) -> HOST_CONTROLLER_RESET_R {
        HOST_CONTROLLER_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list. It is set by HCD whenever it adds a TD to an ED in the Control list.\n\nWhen HC begins to process the head of the Control list, it checks CLF. As long as ControlListFilled is 0, HC will not start processing the Control list. If CF is 1, HC will start processing the Control list and will set ControlListFilled to 0. If HC finds a TD on the list, then HC will set ControlListFilled to 1 causing the Control list processing to continue. If no TD is found on the Control list, and if the HCD does not set ControlListFilled , then ControlListFilled will still be 0 when HC completes processing the Control list and Control list processing will stop."]
    #[inline(always)]
    pub fn control_list_filled(&self) -> CONTROL_LIST_FILLED_R {
        CONTROL_LIST_FILLED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BulklListFilled\n\nThis bit is used to indicate whether there are any TDs on the Bulk list. It is set by HCD whenever it adds a TD to an ED in the Bulk list. When HC begins to process the head of the Bulk list, it checks BLF. As long as BulkListFilled is 0, HC will not start processing the Bulk list. If BulkListFilled is 1, HC will start processing the Bulk list and will set BF to 0. If HC finds a TD on the list, then HC will set BulkListFilled to 1 causing the Bulk list processing to continue. If no TD is found on the Bulk list, and if HCD does not set BulkListFilled , then BulkListFilled will still be 0 when HC completes processing the Bulk list and Bulk list processing will stop."]
    #[inline(always)]
    pub fn bulkl_list_filled(&self) -> BULKL_LIST_FILLED_R {
        BULKL_LIST_FILLED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OwershipChangeRequest\n\nThis bit is set by an OS HCD to request a change of control of the HC. When set HC will set the OwnershipChange field in. After the changeover, this bit is cleared and remains so until the next request from OS HCD."]
    #[inline(always)]
    pub fn owership_change_request(&self) -> OWERSHIP_CHANGE_REQUEST_R {
        OWERSHIP_CHANGE_REQUEST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:17 - SchedulingOverrunCoun\n\nThese bits are incremented on each scheduling overrun error. It is initialized to 00b and wraps around at 11b. This will be incremented when a scheduling overrun is detected even if SchedulingOverrun in has already been set. This is used by HCD to monitor any persistent scheduling problem."]
    #[inline(always)]
    pub fn scheduling_overrun_count(&self) -> SCHEDULING_OVERRUN_COUNT_R {
        SCHEDULING_OVERRUN_COUNT_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - HostControllerReset\n\nThis bit is by HCD to initiate a software reset of HC. Regardless of the functional state of HC, it moves to the USBSuspend state in which most of the operational registers are reset except those stated otherwise; e.g, the InteruptRouting field of HcControl, and no Host bus accesses are allowed. This bit is cleared by HC upon the completion of the reset operation. The reset operation must be completed within 10 ms. This bit, when set, should not cause a reset to the Root Hub and no subsequent reset signaling should be asserted to its downstream ports."]
    #[inline(always)]
    #[must_use]
    pub fn host_controller_reset(&mut self) -> HOST_CONTROLLER_RESET_W<0> {
        HOST_CONTROLLER_RESET_W::new(self)
    }
    #[doc = "Bit 1 - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list. It is set by HCD whenever it adds a TD to an ED in the Control list.\n\nWhen HC begins to process the head of the Control list, it checks CLF. As long as ControlListFilled is 0, HC will not start processing the Control list. If CF is 1, HC will start processing the Control list and will set ControlListFilled to 0. If HC finds a TD on the list, then HC will set ControlListFilled to 1 causing the Control list processing to continue. If no TD is found on the Control list, and if the HCD does not set ControlListFilled , then ControlListFilled will still be 0 when HC completes processing the Control list and Control list processing will stop."]
    #[inline(always)]
    #[must_use]
    pub fn control_list_filled(&mut self) -> CONTROL_LIST_FILLED_W<1> {
        CONTROL_LIST_FILLED_W::new(self)
    }
    #[doc = "Bit 2 - BulklListFilled\n\nThis bit is used to indicate whether there are any TDs on the Bulk list. It is set by HCD whenever it adds a TD to an ED in the Bulk list. When HC begins to process the head of the Bulk list, it checks BLF. As long as BulkListFilled is 0, HC will not start processing the Bulk list. If BulkListFilled is 1, HC will start processing the Bulk list and will set BF to 0. If HC finds a TD on the list, then HC will set BulkListFilled to 1 causing the Bulk list processing to continue. If no TD is found on the Bulk list, and if HCD does not set BulkListFilled , then BulkListFilled will still be 0 when HC completes processing the Bulk list and Bulk list processing will stop."]
    #[inline(always)]
    #[must_use]
    pub fn bulkl_list_filled(&mut self) -> BULKL_LIST_FILLED_W<2> {
        BULKL_LIST_FILLED_W::new(self)
    }
    #[doc = "Bit 3 - OwershipChangeRequest\n\nThis bit is set by an OS HCD to request a change of control of the HC. When set HC will set the OwnershipChange field in. After the changeover, this bit is cleared and remains so until the next request from OS HCD."]
    #[inline(always)]
    #[must_use]
    pub fn owership_change_request(&mut self) -> OWERSHIP_CHANGE_REQUEST_W<3> {
        OWERSHIP_CHANGE_REQUEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI Command Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc_command_status](index.html) module"]
pub struct HC_COMMAND_STATUS_SPEC;
impl crate::RegisterSpec for HC_COMMAND_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc_command_status::R](R) reader structure"]
impl crate::Readable for HC_COMMAND_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc_command_status::W](W) writer structure"]
impl crate::Writable for HC_COMMAND_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_command_status to value 0"]
impl crate::Resettable for HC_COMMAND_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
