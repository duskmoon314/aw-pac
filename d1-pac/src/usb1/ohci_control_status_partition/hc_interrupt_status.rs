#[doc = "Register `hc_interrupt_status` reader"]
pub type R = crate::R<HC_INTERRUPT_STATUS_SPEC>;
#[doc = "Register `hc_interrupt_status` writer"]
pub type W = crate::W<HC_INTERRUPT_STATUS_SPEC>;
#[doc = "Field `scheduling_overrun` reader - SchedulingOverrun\n\nThis bit is set when the USB schedule for the current Frame overruns and after the update of. A scheduling overrun will also cause the SchedulingOverrunCount of to be incremented."]
pub type SCHEDULING_OVERRUN_R = crate::BitReader;
#[doc = "Field `scheduling_overrun` writer - SchedulingOverrun\n\nThis bit is set when the USB schedule for the current Frame overruns and after the update of. A scheduling overrun will also cause the SchedulingOverrunCount of to be incremented."]
pub type SCHEDULING_OVERRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `writeback_done_head` reader - WritebackDoneHead\n\nThis bit is set immediately after HC has written to. Further updates of the will not occur until this bit has been cleared. HCD should only clear this bit after it has saved the content of HccaDoneHead."]
pub type WRITEBACK_DONE_HEAD_R = crate::BitReader;
#[doc = "Field `writeback_done_head` writer - WritebackDoneHead\n\nThis bit is set immediately after HC has written to. Further updates of the will not occur until this bit has been cleared. HCD should only clear this bit after it has saved the content of HccaDoneHead."]
pub type WRITEBACK_DONE_HEAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `start_of_frame` reader - StartofFrame\n\nThis bit is set by HC at each start of frame and after the update of HccaFrameNumber. HC also generates a SOF token at the same time."]
pub type START_OF_FRAME_R = crate::BitReader;
#[doc = "Field `start_of_frame` writer - StartofFrame\n\nThis bit is set by HC at each start of frame and after the update of HccaFrameNumber. HC also generates a SOF token at the same time."]
pub type START_OF_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `resume_detected` reader - ResumeDetected\n\nThis bit is set when HC detects that a device on the USB is asserting resume signaling. It is the transition from no resume signaling to resume signaling causing this bit to be set. This bit is not set when HCD sets the USBRseume state."]
pub type RESUME_DETECTED_R = crate::BitReader;
#[doc = "Field `resume_detected` writer - ResumeDetected\n\nThis bit is set when HC detects that a device on the USB is asserting resume signaling. It is the transition from no resume signaling to resume signaling causing this bit to be set. This bit is not set when HCD sets the USBRseume state."]
pub type RESUME_DETECTED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `unrecoverable_error` reader - UnrecoverableError\n\nThis bit is set when HC detects a system error not related to USB. HC should not proceed with any processing nor signaling before the system error has been corrected. HCD clears this bit after HC has been reset."]
pub type UNRECOVERABLE_ERROR_R = crate::BitReader;
#[doc = "Field `unrecoverable_error` writer - UnrecoverableError\n\nThis bit is set when HC detects a system error not related to USB. HC should not proceed with any processing nor signaling before the system error has been corrected. HCD clears this bit after HC has been reset."]
pub type UNRECOVERABLE_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `frame_number_overflow` reader - FrameNumberOverflow\n\nThis bit is set when the MSb of (bit 15) changes value, from 0 to 1 or from 1 to 0, and after has been updated."]
pub type FRAME_NUMBER_OVERFLOW_R = crate::BitReader;
#[doc = "Field `frame_number_overflow` writer - FrameNumberOverflow\n\nThis bit is set when the MSb of (bit 15) changes value, from 0 to 1 or from 1 to 0, and after has been updated."]
pub type FRAME_NUMBER_OVERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `root_hub_status_change` reader - RootHubStatusChange\n\nThis bit is set when the content of or the content of any of \\[ NumberofDownstreamPort \\] has changed."]
pub type ROOT_HUB_STATUS_CHANGE_R = crate::BitReader;
#[doc = "Field `root_hub_status_change` writer - RootHubStatusChange\n\nThis bit is set when the content of or the content of any of \\[ NumberofDownstreamPort \\] has changed."]
pub type ROOT_HUB_STATUS_CHANGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SchedulingOverrun\n\nThis bit is set when the USB schedule for the current Frame overruns and after the update of. A scheduling overrun will also cause the SchedulingOverrunCount of to be incremented."]
    #[inline(always)]
    pub fn scheduling_overrun(&self) -> SCHEDULING_OVERRUN_R {
        SCHEDULING_OVERRUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WritebackDoneHead\n\nThis bit is set immediately after HC has written to. Further updates of the will not occur until this bit has been cleared. HCD should only clear this bit after it has saved the content of HccaDoneHead."]
    #[inline(always)]
    pub fn writeback_done_head(&self) -> WRITEBACK_DONE_HEAD_R {
        WRITEBACK_DONE_HEAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - StartofFrame\n\nThis bit is set by HC at each start of frame and after the update of HccaFrameNumber. HC also generates a SOF token at the same time."]
    #[inline(always)]
    pub fn start_of_frame(&self) -> START_OF_FRAME_R {
        START_OF_FRAME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ResumeDetected\n\nThis bit is set when HC detects that a device on the USB is asserting resume signaling. It is the transition from no resume signaling to resume signaling causing this bit to be set. This bit is not set when HCD sets the USBRseume state."]
    #[inline(always)]
    pub fn resume_detected(&self) -> RESUME_DETECTED_R {
        RESUME_DETECTED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UnrecoverableError\n\nThis bit is set when HC detects a system error not related to USB. HC should not proceed with any processing nor signaling before the system error has been corrected. HCD clears this bit after HC has been reset."]
    #[inline(always)]
    pub fn unrecoverable_error(&self) -> UNRECOVERABLE_ERROR_R {
        UNRECOVERABLE_ERROR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FrameNumberOverflow\n\nThis bit is set when the MSb of (bit 15) changes value, from 0 to 1 or from 1 to 0, and after has been updated."]
    #[inline(always)]
    pub fn frame_number_overflow(&self) -> FRAME_NUMBER_OVERFLOW_R {
        FRAME_NUMBER_OVERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RootHubStatusChange\n\nThis bit is set when the content of or the content of any of \\[ NumberofDownstreamPort \\] has changed."]
    #[inline(always)]
    pub fn root_hub_status_change(&self) -> ROOT_HUB_STATUS_CHANGE_R {
        ROOT_HUB_STATUS_CHANGE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SchedulingOverrun\n\nThis bit is set when the USB schedule for the current Frame overruns and after the update of. A scheduling overrun will also cause the SchedulingOverrunCount of to be incremented."]
    #[inline(always)]
    #[must_use]
    pub fn scheduling_overrun(&mut self) -> SCHEDULING_OVERRUN_W<HC_INTERRUPT_STATUS_SPEC> {
        SCHEDULING_OVERRUN_W::new(self, 0)
    }
    #[doc = "Bit 1 - WritebackDoneHead\n\nThis bit is set immediately after HC has written to. Further updates of the will not occur until this bit has been cleared. HCD should only clear this bit after it has saved the content of HccaDoneHead."]
    #[inline(always)]
    #[must_use]
    pub fn writeback_done_head(&mut self) -> WRITEBACK_DONE_HEAD_W<HC_INTERRUPT_STATUS_SPEC> {
        WRITEBACK_DONE_HEAD_W::new(self, 1)
    }
    #[doc = "Bit 2 - StartofFrame\n\nThis bit is set by HC at each start of frame and after the update of HccaFrameNumber. HC also generates a SOF token at the same time."]
    #[inline(always)]
    #[must_use]
    pub fn start_of_frame(&mut self) -> START_OF_FRAME_W<HC_INTERRUPT_STATUS_SPEC> {
        START_OF_FRAME_W::new(self, 2)
    }
    #[doc = "Bit 3 - ResumeDetected\n\nThis bit is set when HC detects that a device on the USB is asserting resume signaling. It is the transition from no resume signaling to resume signaling causing this bit to be set. This bit is not set when HCD sets the USBRseume state."]
    #[inline(always)]
    #[must_use]
    pub fn resume_detected(&mut self) -> RESUME_DETECTED_W<HC_INTERRUPT_STATUS_SPEC> {
        RESUME_DETECTED_W::new(self, 3)
    }
    #[doc = "Bit 4 - UnrecoverableError\n\nThis bit is set when HC detects a system error not related to USB. HC should not proceed with any processing nor signaling before the system error has been corrected. HCD clears this bit after HC has been reset."]
    #[inline(always)]
    #[must_use]
    pub fn unrecoverable_error(&mut self) -> UNRECOVERABLE_ERROR_W<HC_INTERRUPT_STATUS_SPEC> {
        UNRECOVERABLE_ERROR_W::new(self, 4)
    }
    #[doc = "Bit 5 - FrameNumberOverflow\n\nThis bit is set when the MSb of (bit 15) changes value, from 0 to 1 or from 1 to 0, and after has been updated."]
    #[inline(always)]
    #[must_use]
    pub fn frame_number_overflow(&mut self) -> FRAME_NUMBER_OVERFLOW_W<HC_INTERRUPT_STATUS_SPEC> {
        FRAME_NUMBER_OVERFLOW_W::new(self, 5)
    }
    #[doc = "Bit 6 - RootHubStatusChange\n\nThis bit is set when the content of or the content of any of \\[ NumberofDownstreamPort \\] has changed."]
    #[inline(always)]
    #[must_use]
    pub fn root_hub_status_change(&mut self) -> ROOT_HUB_STATUS_CHANGE_W<HC_INTERRUPT_STATUS_SPEC> {
        ROOT_HUB_STATUS_CHANGE_W::new(self, 6)
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
#[doc = "OHCI Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_interrupt_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_interrupt_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HC_INTERRUPT_STATUS_SPEC;
impl crate::RegisterSpec for HC_INTERRUPT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc_interrupt_status::R`](R) reader structure"]
impl crate::Readable for HC_INTERRUPT_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hc_interrupt_status::W`](W) writer structure"]
impl crate::Writable for HC_INTERRUPT_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_interrupt_status to value 0"]
impl crate::Resettable for HC_INTERRUPT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
