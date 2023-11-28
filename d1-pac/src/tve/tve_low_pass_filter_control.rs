#[doc = "Register `tve_low_pass_filter_control` reader"]
pub type R = crate::R<TVE_LOW_PASS_FILTER_CONTROL_SPEC>;
#[doc = "Register `tve_low_pass_filter_control` writer"]
pub type W = crate::W<TVE_LOW_PASS_FILTER_CONTROL_SPEC>;
#[doc = "Field `bp1_ratio` reader - Default band-pass filter1 ratio\n\nIn two complement, the range is from -31 to 31."]
pub type BP1_RATIO_R = crate::FieldReader;
#[doc = "Field `bp1_ratio` writer - Default band-pass filter1 ratio\n\nIn two complement, the range is from -31 to 31."]
pub type BP1_RATIO_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `bp0_ratio` reader - Default band-pass filter0 ratio\n\nIn two complement, the range is from -31 to 31."]
pub type BP0_RATIO_R = crate::FieldReader;
#[doc = "Field `bp0_ratio` writer - Default band-pass filter0 ratio\n\nIn two complement, the range is from -31 to 31."]
pub type BP0_RATIO_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `hp_ratio` reader - Default high-pass filter ratio\n\nIn two complement, the range is from -31 to 31."]
pub type HP_RATIO_R = crate::FieldReader;
#[doc = "Field `hp_ratio` writer - Default high-pass filter ratio\n\nIn two complement, the range is from -31 to 31."]
pub type HP_RATIO_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Default band-pass filter1 ratio\n\nIn two complement, the range is from -31 to 31."]
    #[inline(always)]
    pub fn bp1_ratio(&self) -> BP1_RATIO_R {
        BP1_RATIO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Default band-pass filter0 ratio\n\nIn two complement, the range is from -31 to 31."]
    #[inline(always)]
    pub fn bp0_ratio(&self) -> BP0_RATIO_R {
        BP0_RATIO_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Default high-pass filter ratio\n\nIn two complement, the range is from -31 to 31."]
    #[inline(always)]
    pub fn hp_ratio(&self) -> HP_RATIO_R {
        HP_RATIO_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Default band-pass filter1 ratio\n\nIn two complement, the range is from -31 to 31."]
    #[inline(always)]
    #[must_use]
    pub fn bp1_ratio(&mut self) -> BP1_RATIO_W<TVE_LOW_PASS_FILTER_CONTROL_SPEC> {
        BP1_RATIO_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Default band-pass filter0 ratio\n\nIn two complement, the range is from -31 to 31."]
    #[inline(always)]
    #[must_use]
    pub fn bp0_ratio(&mut self) -> BP0_RATIO_W<TVE_LOW_PASS_FILTER_CONTROL_SPEC> {
        BP0_RATIO_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Default high-pass filter ratio\n\nIn two complement, the range is from -31 to 31."]
    #[inline(always)]
    #[must_use]
    pub fn hp_ratio(&mut self) -> HP_RATIO_W<TVE_LOW_PASS_FILTER_CONTROL_SPEC> {
        HP_RATIO_W::new(self, 16)
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
#[doc = "TV Encoder Low Pass Filter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_low_pass_filter_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_low_pass_filter_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_LOW_PASS_FILTER_CONTROL_SPEC;
impl crate::RegisterSpec for TVE_LOW_PASS_FILTER_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_low_pass_filter_control::R`](R) reader structure"]
impl crate::Readable for TVE_LOW_PASS_FILTER_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_low_pass_filter_control::W`](W) writer structure"]
impl crate::Writable for TVE_LOW_PASS_FILTER_CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_low_pass_filter_control to value 0"]
impl crate::Resettable for TVE_LOW_PASS_FILTER_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
