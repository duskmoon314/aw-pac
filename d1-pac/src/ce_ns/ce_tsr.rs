#[doc = "Register `ce_tsr` reader"]
pub type R = crate::R<CE_TSR_SPEC>;
#[doc = "Register `ce_tsr` writer"]
pub type W = crate::W<CE_TSR_SPEC>;
#[doc = "Field `running_channel_number` reader - Running Channel Number"]
pub type RUNNING_CHANNEL_NUMBER_R = crate::FieldReader<RUNNING_CHANNEL_NUMBER_A>;
#[doc = "Running Channel Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RUNNING_CHANNEL_NUMBER_A {
    #[doc = "0: Task channel 0"]
    T0 = 0,
    #[doc = "1: Task channel 1"]
    T1 = 1,
    #[doc = "2: Task channel 2"]
    T2 = 2,
    #[doc = "3: Task channel 3"]
    T3 = 3,
}
impl From<RUNNING_CHANNEL_NUMBER_A> for u8 {
    #[inline(always)]
    fn from(variant: RUNNING_CHANNEL_NUMBER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RUNNING_CHANNEL_NUMBER_A {
    type Ux = u8;
}
impl RUNNING_CHANNEL_NUMBER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RUNNING_CHANNEL_NUMBER_A {
        match self.bits {
            0 => RUNNING_CHANNEL_NUMBER_A::T0,
            1 => RUNNING_CHANNEL_NUMBER_A::T1,
            2 => RUNNING_CHANNEL_NUMBER_A::T2,
            3 => RUNNING_CHANNEL_NUMBER_A::T3,
            _ => unreachable!(),
        }
    }
    #[doc = "Task channel 0"]
    #[inline(always)]
    pub fn is_t0(&self) -> bool {
        *self == RUNNING_CHANNEL_NUMBER_A::T0
    }
    #[doc = "Task channel 1"]
    #[inline(always)]
    pub fn is_t1(&self) -> bool {
        *self == RUNNING_CHANNEL_NUMBER_A::T1
    }
    #[doc = "Task channel 2"]
    #[inline(always)]
    pub fn is_t2(&self) -> bool {
        *self == RUNNING_CHANNEL_NUMBER_A::T2
    }
    #[doc = "Task channel 3"]
    #[inline(always)]
    pub fn is_t3(&self) -> bool {
        *self == RUNNING_CHANNEL_NUMBER_A::T3
    }
}
impl R {
    #[doc = "Bits 0:1 - Running Channel Number"]
    #[inline(always)]
    pub fn running_channel_number(&self) -> RUNNING_CHANNEL_NUMBER_R {
        RUNNING_CHANNEL_NUMBER_R::new((self.bits & 3) as u8)
    }
}
impl W {
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
#[doc = "Task Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_tsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_tsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CE_TSR_SPEC;
impl crate::RegisterSpec for CE_TSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ce_tsr::R`](R) reader structure"]
impl crate::Readable for CE_TSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ce_tsr::W`](W) writer structure"]
impl crate::Writable for CE_TSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ce_tsr to value 0"]
impl crate::Resettable for CE_TSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
