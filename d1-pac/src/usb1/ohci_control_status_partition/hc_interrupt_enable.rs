#[doc = "Register `hc_interrupt_enable` reader"]
pub struct R(crate::R<HC_INTERRUPT_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC_INTERRUPT_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC_INTERRUPT_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC_INTERRUPT_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hc_interrupt_enable` writer"]
pub struct W(crate::W<HC_INTERRUPT_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC_INTERRUPT_ENABLE_SPEC>;
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
impl From<crate::W<HC_INTERRUPT_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC_INTERRUPT_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `scheduling_overrun` reader - SchedulingOverrun Interrupt Enable"]
pub type SCHEDULING_OVERRUN_R = crate::BitReader<SCHEDULING_OVERRUN_A>;
#[doc = "SchedulingOverrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCHEDULING_OVERRUN_A {
    #[doc = "0: Ignore"]
    IGNORE = 0,
    #[doc = "1: Enable interrupt generation due to Scheduling Overrun"]
    ENABLE = 1,
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
    pub fn variant(&self) -> SCHEDULING_OVERRUN_A {
        match self.bits {
            false => SCHEDULING_OVERRUN_A::IGNORE,
            true => SCHEDULING_OVERRUN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == SCHEDULING_OVERRUN_A::IGNORE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SCHEDULING_OVERRUN_A::ENABLE
    }
}
#[doc = "Field `scheduling_overrun` writer - SchedulingOverrun Interrupt Enable"]
pub type SCHEDULING_OVERRUN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_INTERRUPT_ENABLE_SPEC, SCHEDULING_OVERRUN_A, O>;
impl<'a, const O: u8> SCHEDULING_OVERRUN_W<'a, O> {
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(SCHEDULING_OVERRUN_A::IGNORE)
    }
    #[doc = "Enable interrupt generation due to Scheduling Overrun"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCHEDULING_OVERRUN_A::ENABLE)
    }
}
#[doc = "Field `writeback_done_head` reader - WritebackDoneHead Interrupt Enable"]
pub type WRITEBACK_DONE_HEAD_R = crate::BitReader<WRITEBACK_DONE_HEAD_A>;
#[doc = "WritebackDoneHead Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRITEBACK_DONE_HEAD_A {
    #[doc = "0: Ignore"]
    IGNORE = 0,
    #[doc = "1: Enable interrupt generation due to Writeback Done Head"]
    ENABLE = 1,
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
    pub fn variant(&self) -> WRITEBACK_DONE_HEAD_A {
        match self.bits {
            false => WRITEBACK_DONE_HEAD_A::IGNORE,
            true => WRITEBACK_DONE_HEAD_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == WRITEBACK_DONE_HEAD_A::IGNORE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WRITEBACK_DONE_HEAD_A::ENABLE
    }
}
#[doc = "Field `writeback_done_head` writer - WritebackDoneHead Interrupt Enable"]
pub type WRITEBACK_DONE_HEAD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_INTERRUPT_ENABLE_SPEC, WRITEBACK_DONE_HEAD_A, O>;
impl<'a, const O: u8> WRITEBACK_DONE_HEAD_W<'a, O> {
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(WRITEBACK_DONE_HEAD_A::IGNORE)
    }
    #[doc = "Enable interrupt generation due to Writeback Done Head"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WRITEBACK_DONE_HEAD_A::ENABLE)
    }
}
#[doc = "Field `start_of_frame` reader - StartofFrame Interrupt Enable"]
pub type START_OF_FRAME_R = crate::BitReader<START_OF_FRAME_A>;
#[doc = "StartofFrame Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_OF_FRAME_A {
    #[doc = "0: Ignore"]
    IGNORE = 0,
    #[doc = "1: Enable interrupt generation due to Start of Frame"]
    ENABLE = 1,
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
    pub fn variant(&self) -> START_OF_FRAME_A {
        match self.bits {
            false => START_OF_FRAME_A::IGNORE,
            true => START_OF_FRAME_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == START_OF_FRAME_A::IGNORE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == START_OF_FRAME_A::ENABLE
    }
}
#[doc = "Field `start_of_frame` writer - StartofFrame Interrupt Enable"]
pub type START_OF_FRAME_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_INTERRUPT_ENABLE_SPEC, START_OF_FRAME_A, O>;
impl<'a, const O: u8> START_OF_FRAME_W<'a, O> {
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(START_OF_FRAME_A::IGNORE)
    }
    #[doc = "Enable interrupt generation due to Start of Frame"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(START_OF_FRAME_A::ENABLE)
    }
}
#[doc = "Field `resume_detected` reader - ResumeDetected Interrupt Enable"]
pub type RESUME_DETECTED_R = crate::BitReader<RESUME_DETECTED_A>;
#[doc = "ResumeDetected Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESUME_DETECTED_A {
    #[doc = "0: Ignore"]
    IGNORE = 0,
    #[doc = "1: Enable interrupt generation due to Resume Detected"]
    ENABLE = 1,
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
    pub fn variant(&self) -> RESUME_DETECTED_A {
        match self.bits {
            false => RESUME_DETECTED_A::IGNORE,
            true => RESUME_DETECTED_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == RESUME_DETECTED_A::IGNORE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RESUME_DETECTED_A::ENABLE
    }
}
#[doc = "Field `resume_detected` writer - ResumeDetected Interrupt Enable"]
pub type RESUME_DETECTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_INTERRUPT_ENABLE_SPEC, RESUME_DETECTED_A, O>;
impl<'a, const O: u8> RESUME_DETECTED_W<'a, O> {
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(RESUME_DETECTED_A::IGNORE)
    }
    #[doc = "Enable interrupt generation due to Resume Detected"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RESUME_DETECTED_A::ENABLE)
    }
}
#[doc = "Field `unrecoverable_error` reader - UnrecoverableError Interrupt Enable"]
pub type UNRECOVERABLE_ERROR_R = crate::BitReader<UNRECOVERABLE_ERROR_A>;
#[doc = "UnrecoverableError Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNRECOVERABLE_ERROR_A {
    #[doc = "0: Ignore"]
    IGNORE = 0,
    #[doc = "1: Enable interrupt generation due to Unrecoverable Error"]
    ENABLE = 1,
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
    pub fn variant(&self) -> UNRECOVERABLE_ERROR_A {
        match self.bits {
            false => UNRECOVERABLE_ERROR_A::IGNORE,
            true => UNRECOVERABLE_ERROR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == UNRECOVERABLE_ERROR_A::IGNORE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UNRECOVERABLE_ERROR_A::ENABLE
    }
}
#[doc = "Field `unrecoverable_error` writer - UnrecoverableError Interrupt Enable"]
pub type UNRECOVERABLE_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_INTERRUPT_ENABLE_SPEC, UNRECOVERABLE_ERROR_A, O>;
impl<'a, const O: u8> UNRECOVERABLE_ERROR_W<'a, O> {
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(UNRECOVERABLE_ERROR_A::IGNORE)
    }
    #[doc = "Enable interrupt generation due to Unrecoverable Error"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UNRECOVERABLE_ERROR_A::ENABLE)
    }
}
#[doc = "Field `frame_number_overflow` reader - FrameNumberOverflow Interrupt Enable"]
pub type FRAME_NUMBER_OVERFLOW_R = crate::BitReader<FRAME_NUMBER_OVERFLOW_A>;
#[doc = "FrameNumberOverflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAME_NUMBER_OVERFLOW_A {
    #[doc = "0: Ignore"]
    IGNORE = 0,
    #[doc = "1: Enable interrupt generation due to Frame Number Overflow"]
    ENABLE = 1,
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
    pub fn variant(&self) -> FRAME_NUMBER_OVERFLOW_A {
        match self.bits {
            false => FRAME_NUMBER_OVERFLOW_A::IGNORE,
            true => FRAME_NUMBER_OVERFLOW_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == FRAME_NUMBER_OVERFLOW_A::IGNORE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FRAME_NUMBER_OVERFLOW_A::ENABLE
    }
}
#[doc = "Field `frame_number_overflow` writer - FrameNumberOverflow Interrupt Enable"]
pub type FRAME_NUMBER_OVERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_INTERRUPT_ENABLE_SPEC, FRAME_NUMBER_OVERFLOW_A, O>;
impl<'a, const O: u8> FRAME_NUMBER_OVERFLOW_W<'a, O> {
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(FRAME_NUMBER_OVERFLOW_A::IGNORE)
    }
    #[doc = "Enable interrupt generation due to Frame Number Overflow"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRAME_NUMBER_OVERFLOW_A::ENABLE)
    }
}
#[doc = "Field `root_hub_status_change` reader - RootHubStatusChange Interrupt Enable"]
pub type ROOT_HUB_STATUS_CHANGE_R = crate::BitReader<ROOT_HUB_STATUS_CHANGE_A>;
#[doc = "RootHubStatusChange Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROOT_HUB_STATUS_CHANGE_A {
    #[doc = "0: Ignore"]
    IGNORE = 0,
    #[doc = "1: Enable interrupt generation due to Root Hub Status Change"]
    ENABLE = 1,
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
    pub fn variant(&self) -> ROOT_HUB_STATUS_CHANGE_A {
        match self.bits {
            false => ROOT_HUB_STATUS_CHANGE_A::IGNORE,
            true => ROOT_HUB_STATUS_CHANGE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == ROOT_HUB_STATUS_CHANGE_A::IGNORE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ROOT_HUB_STATUS_CHANGE_A::ENABLE
    }
}
#[doc = "Field `root_hub_status_change` writer - RootHubStatusChange Interrupt Enable"]
pub type ROOT_HUB_STATUS_CHANGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_INTERRUPT_ENABLE_SPEC, ROOT_HUB_STATUS_CHANGE_A, O>;
impl<'a, const O: u8> ROOT_HUB_STATUS_CHANGE_W<'a, O> {
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(ROOT_HUB_STATUS_CHANGE_A::IGNORE)
    }
    #[doc = "Enable interrupt generation due to Root Hub Status Change"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ROOT_HUB_STATUS_CHANGE_A::ENABLE)
    }
}
#[doc = "Field `master_interrupt_enable` reader - MasterInterruptEnable\n\nA '0' writtern to this field is ignored by HC. A '1' written to this field enables interrupt generation due to events specified in the other bits of this register. This is used by HCD as Master Interrupt Enable."]
pub type MASTER_INTERRUPT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `master_interrupt_enable` writer - MasterInterruptEnable\n\nA '0' writtern to this field is ignored by HC. A '1' written to this field enables interrupt generation due to events specified in the other bits of this register. This is used by HCD as Master Interrupt Enable."]
pub type MASTER_INTERRUPT_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_INTERRUPT_ENABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SchedulingOverrun Interrupt Enable"]
    #[inline(always)]
    pub fn scheduling_overrun(&self) -> SCHEDULING_OVERRUN_R {
        SCHEDULING_OVERRUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WritebackDoneHead Interrupt Enable"]
    #[inline(always)]
    pub fn writeback_done_head(&self) -> WRITEBACK_DONE_HEAD_R {
        WRITEBACK_DONE_HEAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - StartofFrame Interrupt Enable"]
    #[inline(always)]
    pub fn start_of_frame(&self) -> START_OF_FRAME_R {
        START_OF_FRAME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ResumeDetected Interrupt Enable"]
    #[inline(always)]
    pub fn resume_detected(&self) -> RESUME_DETECTED_R {
        RESUME_DETECTED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UnrecoverableError Interrupt Enable"]
    #[inline(always)]
    pub fn unrecoverable_error(&self) -> UNRECOVERABLE_ERROR_R {
        UNRECOVERABLE_ERROR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FrameNumberOverflow Interrupt Enable"]
    #[inline(always)]
    pub fn frame_number_overflow(&self) -> FRAME_NUMBER_OVERFLOW_R {
        FRAME_NUMBER_OVERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RootHubStatusChange Interrupt Enable"]
    #[inline(always)]
    pub fn root_hub_status_change(&self) -> ROOT_HUB_STATUS_CHANGE_R {
        ROOT_HUB_STATUS_CHANGE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 31 - MasterInterruptEnable\n\nA '0' writtern to this field is ignored by HC. A '1' written to this field enables interrupt generation due to events specified in the other bits of this register. This is used by HCD as Master Interrupt Enable."]
    #[inline(always)]
    pub fn master_interrupt_enable(&self) -> MASTER_INTERRUPT_ENABLE_R {
        MASTER_INTERRUPT_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SchedulingOverrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scheduling_overrun(&mut self) -> SCHEDULING_OVERRUN_W<0> {
        SCHEDULING_OVERRUN_W::new(self)
    }
    #[doc = "Bit 1 - WritebackDoneHead Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn writeback_done_head(&mut self) -> WRITEBACK_DONE_HEAD_W<1> {
        WRITEBACK_DONE_HEAD_W::new(self)
    }
    #[doc = "Bit 2 - StartofFrame Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn start_of_frame(&mut self) -> START_OF_FRAME_W<2> {
        START_OF_FRAME_W::new(self)
    }
    #[doc = "Bit 3 - ResumeDetected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn resume_detected(&mut self) -> RESUME_DETECTED_W<3> {
        RESUME_DETECTED_W::new(self)
    }
    #[doc = "Bit 4 - UnrecoverableError Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn unrecoverable_error(&mut self) -> UNRECOVERABLE_ERROR_W<4> {
        UNRECOVERABLE_ERROR_W::new(self)
    }
    #[doc = "Bit 5 - FrameNumberOverflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frame_number_overflow(&mut self) -> FRAME_NUMBER_OVERFLOW_W<5> {
        FRAME_NUMBER_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 6 - RootHubStatusChange Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn root_hub_status_change(&mut self) -> ROOT_HUB_STATUS_CHANGE_W<6> {
        ROOT_HUB_STATUS_CHANGE_W::new(self)
    }
    #[doc = "Bit 31 - MasterInterruptEnable\n\nA '0' writtern to this field is ignored by HC. A '1' written to this field enables interrupt generation due to events specified in the other bits of this register. This is used by HCD as Master Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn master_interrupt_enable(&mut self) -> MASTER_INTERRUPT_ENABLE_W<31> {
        MASTER_INTERRUPT_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc_interrupt_enable](index.html) module"]
pub struct HC_INTERRUPT_ENABLE_SPEC;
impl crate::RegisterSpec for HC_INTERRUPT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc_interrupt_enable::R](R) reader structure"]
impl crate::Readable for HC_INTERRUPT_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc_interrupt_enable::W](W) writer structure"]
impl crate::Writable for HC_INTERRUPT_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_interrupt_enable to value 0"]
impl crate::Resettable for HC_INTERRUPT_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
