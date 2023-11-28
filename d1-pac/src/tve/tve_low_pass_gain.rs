#[doc = "Register `tve_low_pass_gain` reader"]
pub type R = crate::R<TVE_LOW_PASS_GAIN_SPEC>;
#[doc = "Register `tve_low_pass_gain` writer"]
pub type W = crate::W<TVE_LOW_PASS_GAIN_SPEC>;
#[doc = "Field `gain` reader - Peaking gain setting."]
pub type GAIN_R = crate::FieldReader;
#[doc = "Field `gain` writer - Peaking gain setting."]
pub type GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Peaking gain setting."]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Peaking gain setting."]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GAIN_W<TVE_LOW_PASS_GAIN_SPEC> {
        GAIN_W::new(self, 0)
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
#[doc = "TV Encoder Low Pass Gain Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_low_pass_gain::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_low_pass_gain::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_LOW_PASS_GAIN_SPEC;
impl crate::RegisterSpec for TVE_LOW_PASS_GAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_low_pass_gain::R`](R) reader structure"]
impl crate::Readable for TVE_LOW_PASS_GAIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_low_pass_gain::W`](W) writer structure"]
impl crate::Writable for TVE_LOW_PASS_GAIN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_low_pass_gain to value 0"]
impl crate::Resettable for TVE_LOW_PASS_GAIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
