#[doc = "Register `tve_low_pass_gain_control` reader"]
pub type R = crate::R<TVE_LOW_PASS_GAIN_CONTROL_SPEC>;
#[doc = "Register `tve_low_pass_gain_control` writer"]
pub type W = crate::W<TVE_LOW_PASS_GAIN_CONTROL_SPEC>;
#[doc = "Field `beta` reader - Gain control: large gain limitation."]
pub type BETA_R = crate::FieldReader;
#[doc = "Field `beta` writer - Gain control: large gain limitation."]
pub type BETA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `dif_up` reader - Gain control: limitation threshold."]
pub type DIF_UP_R = crate::FieldReader;
#[doc = "Field `dif_up` writer - Gain control: limitation threshold."]
pub type DIF_UP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4 - Gain control: large gain limitation."]
    #[inline(always)]
    pub fn beta(&self) -> BETA_R {
        BETA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Gain control: limitation threshold."]
    #[inline(always)]
    pub fn dif_up(&self) -> DIF_UP_R {
        DIF_UP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Gain control: large gain limitation."]
    #[inline(always)]
    #[must_use]
    pub fn beta(&mut self) -> BETA_W<TVE_LOW_PASS_GAIN_CONTROL_SPEC> {
        BETA_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Gain control: limitation threshold."]
    #[inline(always)]
    #[must_use]
    pub fn dif_up(&mut self) -> DIF_UP_W<TVE_LOW_PASS_GAIN_CONTROL_SPEC> {
        DIF_UP_W::new(self, 16)
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
#[doc = "TV Encoder Low Pass Gain Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_low_pass_gain_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_low_pass_gain_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_LOW_PASS_GAIN_CONTROL_SPEC;
impl crate::RegisterSpec for TVE_LOW_PASS_GAIN_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_low_pass_gain_control::R`](R) reader structure"]
impl crate::Readable for TVE_LOW_PASS_GAIN_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_low_pass_gain_control::W`](W) writer structure"]
impl crate::Writable for TVE_LOW_PASS_GAIN_CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_low_pass_gain_control to value 0"]
impl crate::Resettable for TVE_LOW_PASS_GAIN_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
