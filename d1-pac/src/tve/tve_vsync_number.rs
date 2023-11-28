#[doc = "Register `tve_vsync_number` reader"]
pub type R = crate::R<TVE_VSYNC_NUMBER_SPEC>;
#[doc = "Register `tve_vsync_number` writer"]
pub type W = crate::W<TVE_VSYNC_NUMBER_SPEC>;
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
    pub const fn variant(&self) -> VSYNC5_A {
        match self.bits {
            false => VSYNC5_A::_5,
            true => VSYNC5_A::_6,
        }
    }
    #[doc = "5 equalization pulse(default)"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == VSYNC5_A::_5
    }
    #[doc = "6 equalization pulses"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == VSYNC5_A::_6
    }
}
#[doc = "Field `vsync5` writer - VSync5 Number of equalization pulse selection\n\nThis bit selects whether the number of equalization pulses is 5 or 6. This parameter is applicable only for interlaced video."]
pub type VSYNC5_W<'a, REG> = crate::BitWriter<'a, REG, VSYNC5_A>;
impl<'a, REG> VSYNC5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "5 equalization pulse(default)"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut crate::W<REG> {
        self.variant(VSYNC5_A::_5)
    }
    #[doc = "6 equalization pulses"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut crate::W<REG> {
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
    pub fn vsync5(&mut self) -> VSYNC5_W<TVE_VSYNC_NUMBER_SPEC> {
        VSYNC5_W::new(self, 0)
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
#[doc = "TV Encoder VSYNC Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_vsync_number::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_vsync_number::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_VSYNC_NUMBER_SPEC;
impl crate::RegisterSpec for TVE_VSYNC_NUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_vsync_number::R`](R) reader structure"]
impl crate::Readable for TVE_VSYNC_NUMBER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_vsync_number::W`](W) writer structure"]
impl crate::Writable for TVE_VSYNC_NUMBER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_vsync_number to value 0"]
impl crate::Resettable for TVE_VSYNC_NUMBER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
