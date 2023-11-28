#[doc = "Register `hc_interrupt_disable` reader"]
pub type R = crate::R<HC_INTERRUPT_DISABLE_SPEC>;
#[doc = "Register `hc_interrupt_disable` writer"]
pub type W = crate::W<HC_INTERRUPT_DISABLE_SPEC>;
#[doc = "Field `scheduling_overrun` reader - SchedulingOverrun Interrupt Disable"]
pub type SCHEDULING_OVERRUN_R = crate::BitReader<SCHEDULING_OVERRUN_A>;
#[doc = "SchedulingOverrun Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCHEDULING_OVERRUN_A {
    #[doc = "0: Ignore"]
    IGNORE = 0,
    #[doc = "1: Disable interrupt generation due to Scheduling Overrun"]
    DISABLE = 1,
}
impl From<SCHEDULING_OVERRUN_A> for bool {
    #[inline(always)]
    fn from(variant: SCHEDULING_OVERRUN_A) -> Self {
        variant as u8 != 0
    }
}
impl SCHEDULING_OVERRUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCHEDULING_OVERRUN_A {
        match self.bits {
            false => SCHEDULING_OVERRUN_A::IGNORE,
            true => SCHEDULING_OVERRUN_A::DISABLE,
        }
    }
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == SCHEDULING_OVERRUN_A::IGNORE
    }
    #[doc = "Disable interrupt generation due to Scheduling Overrun"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCHEDULING_OVERRUN_A::DISABLE
    }
}
#[doc = "Field `scheduling_overrun` writer - SchedulingOverrun Interrupt Disable"]
pub type SCHEDULING_OVERRUN_W<'a, REG> = crate::BitWriter<'a, REG, SCHEDULING_OVERRUN_A>;
impl<'a, REG> SCHEDULING_OVERRUN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut crate::W<REG> {
        self.variant(SCHEDULING_OVERRUN_A::IGNORE)
    }
    #[doc = "Disable interrupt generation due to Scheduling Overrun"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SCHEDULING_OVERRUN_A::DISABLE)
    }
}
#[doc = "Field `writeback_done_head` reader - WritebackDoneHead Interrupt Disable"]
pub type WRITEBACK_DONE_HEAD_R = crate::BitReader<WRITEBACK_DONE_HEAD_A>;
#[doc = "WritebackDoneHead Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRITEBACK_DONE_HEAD_A {
    #[doc = "0: Ignore"]
    IGNORE = 0,
    #[doc = "1: Disable interrupt generation due to Writeback Done Head"]
    DISABLE = 1,
}
impl From<WRITEBACK_DONE_HEAD_A> for bool {
    #[inline(always)]
    fn from(variant: WRITEBACK_DONE_HEAD_A) -> Self {
        variant as u8 != 0
    }
}
impl WRITEBACK_DONE_HEAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WRITEBACK_DONE_HEAD_A {
        match self.bits {
            false => WRITEBACK_DONE_HEAD_A::IGNORE,
            true => WRITEBACK_DONE_HEAD_A::DISABLE,
        }
    }
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == WRITEBACK_DONE_HEAD_A::IGNORE
    }
    #[doc = "Disable interrupt generation due to Writeback Done Head"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WRITEBACK_DONE_HEAD_A::DISABLE
    }
}
#[doc = "Field `writeback_done_head` writer - WritebackDoneHead Interrupt Disable"]
pub type WRITEBACK_DONE_HEAD_W<'a, REG> = crate::BitWriter<'a, REG, WRITEBACK_DONE_HEAD_A>;
impl<'a, REG> WRITEBACK_DONE_HEAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut crate::W<REG> {
        self.variant(WRITEBACK_DONE_HEAD_A::IGNORE)
    }
    #[doc = "Disable interrupt generation due to Writeback Done Head"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WRITEBACK_DONE_HEAD_A::DISABLE)
    }
}
#[doc = "Field `start_of_frame` reader - StartofFrame Interrupt Disable"]
pub type START_OF_FRAME_R = crate::BitReader<START_OF_FRAME_A>;
#[doc = "StartofFrame Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_OF_FRAME_A {
    #[doc = "0: Ignore"]
    IGNORE = 0,
    #[doc = "1: Disable interrupt generation due to Start of Frame"]
    DISABLE = 1,
}
impl From<START_OF_FRAME_A> for bool {
    #[inline(always)]
    fn from(variant: START_OF_FRAME_A) -> Self {
        variant as u8 != 0
    }
}
impl START_OF_FRAME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> START_OF_FRAME_A {
        match self.bits {
            false => START_OF_FRAME_A::IGNORE,
            true => START_OF_FRAME_A::DISABLE,
        }
    }
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == START_OF_FRAME_A::IGNORE
    }
    #[doc = "Disable interrupt generation due to Start of Frame"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == START_OF_FRAME_A::DISABLE
    }
}
#[doc = "Field `start_of_frame` writer - StartofFrame Interrupt Disable"]
pub type START_OF_FRAME_W<'a, REG> = crate::BitWriter<'a, REG, START_OF_FRAME_A>;
impl<'a, REG> START_OF_FRAME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut crate::W<REG> {
        self.variant(START_OF_FRAME_A::IGNORE)
    }
    #[doc = "Disable interrupt generation due to Start of Frame"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(START_OF_FRAME_A::DISABLE)
    }
}
#[doc = "Field `resume_detected` reader - ResumeDetected Interrupt Disable"]
pub type RESUME_DETECTED_R = crate::BitReader<RESUME_DETECTED_A>;
#[doc = "ResumeDetected Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESUME_DETECTED_A {
    #[doc = "0: Ignore"]
    IGNORE = 0,
    #[doc = "1: Disable interrupt generation due to Resume Detected"]
    DISABLE = 1,
}
impl From<RESUME_DETECTED_A> for bool {
    #[inline(always)]
    fn from(variant: RESUME_DETECTED_A) -> Self {
        variant as u8 != 0
    }
}
impl RESUME_DETECTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESUME_DETECTED_A {
        match self.bits {
            false => RESUME_DETECTED_A::IGNORE,
            true => RESUME_DETECTED_A::DISABLE,
        }
    }
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == RESUME_DETECTED_A::IGNORE
    }
    #[doc = "Disable interrupt generation due to Resume Detected"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RESUME_DETECTED_A::DISABLE
    }
}
#[doc = "Field `resume_detected` writer - ResumeDetected Interrupt Disable"]
pub type RESUME_DETECTED_W<'a, REG> = crate::BitWriter<'a, REG, RESUME_DETECTED_A>;
impl<'a, REG> RESUME_DETECTED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut crate::W<REG> {
        self.variant(RESUME_DETECTED_A::IGNORE)
    }
    #[doc = "Disable interrupt generation due to Resume Detected"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RESUME_DETECTED_A::DISABLE)
    }
}
#[doc = "Field `unrecoverable_error` reader - UnrecoverableError Interrupt Disable"]
pub type UNRECOVERABLE_ERROR_R = crate::BitReader<UNRECOVERABLE_ERROR_A>;
#[doc = "UnrecoverableError Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNRECOVERABLE_ERROR_A {
    #[doc = "0: Ignore"]
    IGNORE = 0,
    #[doc = "1: Disable interrupt generation due to Unrecoverable Error"]
    DISABLE = 1,
}
impl From<UNRECOVERABLE_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: UNRECOVERABLE_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl UNRECOVERABLE_ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UNRECOVERABLE_ERROR_A {
        match self.bits {
            false => UNRECOVERABLE_ERROR_A::IGNORE,
            true => UNRECOVERABLE_ERROR_A::DISABLE,
        }
    }
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == UNRECOVERABLE_ERROR_A::IGNORE
    }
    #[doc = "Disable interrupt generation due to Unrecoverable Error"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UNRECOVERABLE_ERROR_A::DISABLE
    }
}
#[doc = "Field `unrecoverable_error` writer - UnrecoverableError Interrupt Disable"]
pub type UNRECOVERABLE_ERROR_W<'a, REG> = crate::BitWriter<'a, REG, UNRECOVERABLE_ERROR_A>;
impl<'a, REG> UNRECOVERABLE_ERROR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut crate::W<REG> {
        self.variant(UNRECOVERABLE_ERROR_A::IGNORE)
    }
    #[doc = "Disable interrupt generation due to Unrecoverable Error"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(UNRECOVERABLE_ERROR_A::DISABLE)
    }
}
#[doc = "Field `frame_number_overflow` reader - FrameNumberOverflow Interrupt Disable"]
pub type FRAME_NUMBER_OVERFLOW_R = crate::BitReader<FRAME_NUMBER_OVERFLOW_A>;
#[doc = "FrameNumberOverflow Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAME_NUMBER_OVERFLOW_A {
    #[doc = "0: Ignore"]
    IGNORE = 0,
    #[doc = "1: Disable interrupt generation due to Frame Number Overflow"]
    DISABLE = 1,
}
impl From<FRAME_NUMBER_OVERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: FRAME_NUMBER_OVERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl FRAME_NUMBER_OVERFLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FRAME_NUMBER_OVERFLOW_A {
        match self.bits {
            false => FRAME_NUMBER_OVERFLOW_A::IGNORE,
            true => FRAME_NUMBER_OVERFLOW_A::DISABLE,
        }
    }
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == FRAME_NUMBER_OVERFLOW_A::IGNORE
    }
    #[doc = "Disable interrupt generation due to Frame Number Overflow"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FRAME_NUMBER_OVERFLOW_A::DISABLE
    }
}
#[doc = "Field `frame_number_overflow` writer - FrameNumberOverflow Interrupt Disable"]
pub type FRAME_NUMBER_OVERFLOW_W<'a, REG> = crate::BitWriter<'a, REG, FRAME_NUMBER_OVERFLOW_A>;
impl<'a, REG> FRAME_NUMBER_OVERFLOW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut crate::W<REG> {
        self.variant(FRAME_NUMBER_OVERFLOW_A::IGNORE)
    }
    #[doc = "Disable interrupt generation due to Frame Number Overflow"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FRAME_NUMBER_OVERFLOW_A::DISABLE)
    }
}
#[doc = "Field `root_hub_status_change` reader - RootHubStatusChange Interrupt Disable"]
pub type ROOT_HUB_STATUS_CHANGE_R = crate::BitReader<ROOT_HUB_STATUS_CHANGE_A>;
#[doc = "RootHubStatusChange Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROOT_HUB_STATUS_CHANGE_A {
    #[doc = "0: Ignore"]
    IGNORE = 0,
    #[doc = "1: Disable interrupt generation due to Root Hub Status Change"]
    DISABLE = 1,
}
impl From<ROOT_HUB_STATUS_CHANGE_A> for bool {
    #[inline(always)]
    fn from(variant: ROOT_HUB_STATUS_CHANGE_A) -> Self {
        variant as u8 != 0
    }
}
impl ROOT_HUB_STATUS_CHANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ROOT_HUB_STATUS_CHANGE_A {
        match self.bits {
            false => ROOT_HUB_STATUS_CHANGE_A::IGNORE,
            true => ROOT_HUB_STATUS_CHANGE_A::DISABLE,
        }
    }
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == ROOT_HUB_STATUS_CHANGE_A::IGNORE
    }
    #[doc = "Disable interrupt generation due to Root Hub Status Change"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ROOT_HUB_STATUS_CHANGE_A::DISABLE
    }
}
#[doc = "Field `root_hub_status_change` writer - RootHubStatusChange Interrupt Disable"]
pub type ROOT_HUB_STATUS_CHANGE_W<'a, REG> = crate::BitWriter<'a, REG, ROOT_HUB_STATUS_CHANGE_A>;
impl<'a, REG> ROOT_HUB_STATUS_CHANGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_HUB_STATUS_CHANGE_A::IGNORE)
    }
    #[doc = "Disable interrupt generation due to Root Hub Status Change"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_HUB_STATUS_CHANGE_A::DISABLE)
    }
}
#[doc = "Field `master_interrupt_disable` reader - MasterInterruptEnable\n\nA '0' writtern to this field is ignored by HC. A '1' written to this field disables interrupt generation due to events specified in the other bits of this register. This field is set after a hardware or software reset."]
pub type MASTER_INTERRUPT_DISABLE_R = crate::BitReader;
#[doc = "Field `master_interrupt_disable` writer - MasterInterruptEnable\n\nA '0' writtern to this field is ignored by HC. A '1' written to this field disables interrupt generation due to events specified in the other bits of this register. This field is set after a hardware or software reset."]
pub type MASTER_INTERRUPT_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SchedulingOverrun Interrupt Disable"]
    #[inline(always)]
    pub fn scheduling_overrun(&self) -> SCHEDULING_OVERRUN_R {
        SCHEDULING_OVERRUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WritebackDoneHead Interrupt Disable"]
    #[inline(always)]
    pub fn writeback_done_head(&self) -> WRITEBACK_DONE_HEAD_R {
        WRITEBACK_DONE_HEAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - StartofFrame Interrupt Disable"]
    #[inline(always)]
    pub fn start_of_frame(&self) -> START_OF_FRAME_R {
        START_OF_FRAME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ResumeDetected Interrupt Disable"]
    #[inline(always)]
    pub fn resume_detected(&self) -> RESUME_DETECTED_R {
        RESUME_DETECTED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UnrecoverableError Interrupt Disable"]
    #[inline(always)]
    pub fn unrecoverable_error(&self) -> UNRECOVERABLE_ERROR_R {
        UNRECOVERABLE_ERROR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FrameNumberOverflow Interrupt Disable"]
    #[inline(always)]
    pub fn frame_number_overflow(&self) -> FRAME_NUMBER_OVERFLOW_R {
        FRAME_NUMBER_OVERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RootHubStatusChange Interrupt Disable"]
    #[inline(always)]
    pub fn root_hub_status_change(&self) -> ROOT_HUB_STATUS_CHANGE_R {
        ROOT_HUB_STATUS_CHANGE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 31 - MasterInterruptEnable\n\nA '0' writtern to this field is ignored by HC. A '1' written to this field disables interrupt generation due to events specified in the other bits of this register. This field is set after a hardware or software reset."]
    #[inline(always)]
    pub fn master_interrupt_disable(&self) -> MASTER_INTERRUPT_DISABLE_R {
        MASTER_INTERRUPT_DISABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SchedulingOverrun Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn scheduling_overrun(&mut self) -> SCHEDULING_OVERRUN_W<HC_INTERRUPT_DISABLE_SPEC> {
        SCHEDULING_OVERRUN_W::new(self, 0)
    }
    #[doc = "Bit 1 - WritebackDoneHead Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn writeback_done_head(&mut self) -> WRITEBACK_DONE_HEAD_W<HC_INTERRUPT_DISABLE_SPEC> {
        WRITEBACK_DONE_HEAD_W::new(self, 1)
    }
    #[doc = "Bit 2 - StartofFrame Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn start_of_frame(&mut self) -> START_OF_FRAME_W<HC_INTERRUPT_DISABLE_SPEC> {
        START_OF_FRAME_W::new(self, 2)
    }
    #[doc = "Bit 3 - ResumeDetected Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn resume_detected(&mut self) -> RESUME_DETECTED_W<HC_INTERRUPT_DISABLE_SPEC> {
        RESUME_DETECTED_W::new(self, 3)
    }
    #[doc = "Bit 4 - UnrecoverableError Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn unrecoverable_error(&mut self) -> UNRECOVERABLE_ERROR_W<HC_INTERRUPT_DISABLE_SPEC> {
        UNRECOVERABLE_ERROR_W::new(self, 4)
    }
    #[doc = "Bit 5 - FrameNumberOverflow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn frame_number_overflow(&mut self) -> FRAME_NUMBER_OVERFLOW_W<HC_INTERRUPT_DISABLE_SPEC> {
        FRAME_NUMBER_OVERFLOW_W::new(self, 5)
    }
    #[doc = "Bit 6 - RootHubStatusChange Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn root_hub_status_change(
        &mut self,
    ) -> ROOT_HUB_STATUS_CHANGE_W<HC_INTERRUPT_DISABLE_SPEC> {
        ROOT_HUB_STATUS_CHANGE_W::new(self, 6)
    }
    #[doc = "Bit 31 - MasterInterruptEnable\n\nA '0' writtern to this field is ignored by HC. A '1' written to this field disables interrupt generation due to events specified in the other bits of this register. This field is set after a hardware or software reset."]
    #[inline(always)]
    #[must_use]
    pub fn master_interrupt_disable(
        &mut self,
    ) -> MASTER_INTERRUPT_DISABLE_W<HC_INTERRUPT_DISABLE_SPEC> {
        MASTER_INTERRUPT_DISABLE_W::new(self, 31)
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
#[doc = "OHCI Interrupt Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_interrupt_disable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_interrupt_disable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HC_INTERRUPT_DISABLE_SPEC;
impl crate::RegisterSpec for HC_INTERRUPT_DISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc_interrupt_disable::R`](R) reader structure"]
impl crate::Readable for HC_INTERRUPT_DISABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hc_interrupt_disable::W`](W) writer structure"]
impl crate::Writable for HC_INTERRUPT_DISABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_interrupt_disable to value 0"]
impl crate::Resettable for HC_INTERRUPT_DISABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
