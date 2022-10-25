#[doc = "Register `ce_tsr` reader"]
pub struct R(crate::R<CE_TSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CE_TSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CE_TSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CE_TSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ce_tsr` writer"]
pub struct W(crate::W<CE_TSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CE_TSR_SPEC>;
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
impl From<crate::W<CE_TSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CE_TSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `running_channel_number` reader - Running Channel Number"]
pub type RUNNING_CHANNEL_NUMBER_R = crate::FieldReader<u8, RUNNING_CHANNEL_NUMBER_A>;
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
impl RUNNING_CHANNEL_NUMBER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUNNING_CHANNEL_NUMBER_A {
        match self.bits {
            0 => RUNNING_CHANNEL_NUMBER_A::T0,
            1 => RUNNING_CHANNEL_NUMBER_A::T1,
            2 => RUNNING_CHANNEL_NUMBER_A::T2,
            3 => RUNNING_CHANNEL_NUMBER_A::T3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `T0`"]
    #[inline(always)]
    pub fn is_t0(&self) -> bool {
        *self == RUNNING_CHANNEL_NUMBER_A::T0
    }
    #[doc = "Checks if the value of the field is `T1`"]
    #[inline(always)]
    pub fn is_t1(&self) -> bool {
        *self == RUNNING_CHANNEL_NUMBER_A::T1
    }
    #[doc = "Checks if the value of the field is `T2`"]
    #[inline(always)]
    pub fn is_t2(&self) -> bool {
        *self == RUNNING_CHANNEL_NUMBER_A::T2
    }
    #[doc = "Checks if the value of the field is `T3`"]
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Task Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce_tsr](index.html) module"]
pub struct CE_TSR_SPEC;
impl crate::RegisterSpec for CE_TSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ce_tsr::R](R) reader structure"]
impl crate::Readable for CE_TSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ce_tsr::W](W) writer structure"]
impl crate::Writable for CE_TSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ce_tsr to value 0"]
impl crate::Resettable for CE_TSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
