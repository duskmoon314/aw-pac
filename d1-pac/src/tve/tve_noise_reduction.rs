#[doc = "Register `tve_noise_reduction` reader"]
pub type R = crate::R<TVE_NOISE_REDUCTION_SPEC>;
#[doc = "Register `tve_noise_reduction` writer"]
pub type W = crate::W<TVE_NOISE_REDUCTION_SPEC>;
#[doc = "Field `en` reader - "]
pub type EN_R = crate::BitReader;
#[doc = "Field `en` writer - "]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `t_value` reader - "]
pub type T_VALUE_R = crate::FieldReader;
#[doc = "Field `t_value` writer - "]
pub type T_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn t_value(&self) -> T_VALUE_R {
        T_VALUE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<TVE_NOISE_REDUCTION_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn t_value(&mut self) -> T_VALUE_W<TVE_NOISE_REDUCTION_SPEC> {
        T_VALUE_W::new(self, 16)
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
#[doc = "TV Encoder Noise Reduction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_noise_reduction::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_noise_reduction::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_NOISE_REDUCTION_SPEC;
impl crate::RegisterSpec for TVE_NOISE_REDUCTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_noise_reduction::R`](R) reader structure"]
impl crate::Readable for TVE_NOISE_REDUCTION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_noise_reduction::W`](W) writer structure"]
impl crate::Writable for TVE_NOISE_REDUCTION_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_noise_reduction to value 0"]
impl crate::Resettable for TVE_NOISE_REDUCTION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
