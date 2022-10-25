#[doc = "Register `tve_vsync_number` reader"]
pub struct R(crate::R<TVE_VSYNC_NUMBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_VSYNC_NUMBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_VSYNC_NUMBER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_VSYNC_NUMBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_vsync_number` writer"]
pub struct W(crate::W<TVE_VSYNC_NUMBER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_VSYNC_NUMBER_SPEC>;
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
impl From<crate::W<TVE_VSYNC_NUMBER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_VSYNC_NUMBER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `vsync5` reader - VSync5 Number of equalization pulse selection\n\nThis bit selects whether the number of equalization pulses is 5 or 6. This parameter is applicable only for interlaced video."]
pub type VSYNC5_R = crate::BitReader<VSYNC5_A>;
#[doc = "VSync5 Number of equalization pulse selection\n\nThis bit selects whether the number of equalization pulses is 5 or 6. This parameter is applicable only for interlaced video.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSYNC5_A {
    #[doc = "0: 5 equalization pulse(default)"]
    _5 = 0,
    #[doc = "1: 6 equalization pulses"]
    _6 = 1,
}
impl From<VSYNC5_A> for bool {
    #[inline(always)]
    fn from(variant: VSYNC5_A) -> Self {
        variant as u8 != 0
    }
}
impl VSYNC5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSYNC5_A {
        match self.bits {
            false => VSYNC5_A::_5,
            true => VSYNC5_A::_6,
        }
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == VSYNC5_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == VSYNC5_A::_6
    }
}
#[doc = "Field `vsync5` writer - VSync5 Number of equalization pulse selection\n\nThis bit selects whether the number of equalization pulses is 5 or 6. This parameter is applicable only for interlaced video."]
pub type VSYNC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, TVE_VSYNC_NUMBER_SPEC, VSYNC5_A, O>;
impl<'a, const O: u8> VSYNC5_W<'a, O> {
    #[doc = "5 equalization pulse(default)"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(VSYNC5_A::_5)
    }
    #[doc = "6 equalization pulses"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(VSYNC5_A::_6)
    }
}
impl R {
    #[doc = "Bit 0 - VSync5 Number of equalization pulse selection\n\nThis bit selects whether the number of equalization pulses is 5 or 6. This parameter is applicable only for interlaced video."]
    #[inline(always)]
    pub fn vsync5(&self) -> VSYNC5_R {
        VSYNC5_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VSync5 Number of equalization pulse selection\n\nThis bit selects whether the number of equalization pulses is 5 or 6. This parameter is applicable only for interlaced video."]
    #[inline(always)]
    #[must_use]
    pub fn vsync5(&mut self) -> VSYNC5_W<0> {
        VSYNC5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder VSYNC Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_vsync_number](index.html) module"]
pub struct TVE_VSYNC_NUMBER_SPEC;
impl crate::RegisterSpec for TVE_VSYNC_NUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_vsync_number::R](R) reader structure"]
impl crate::Readable for TVE_VSYNC_NUMBER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_vsync_number::W](W) writer structure"]
impl crate::Writable for TVE_VSYNC_NUMBER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_vsync_number to value 0"]
impl crate::Resettable for TVE_VSYNC_NUMBER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
