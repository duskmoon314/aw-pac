#[doc = "Register `smhc_a12a` reader"]
pub type R = crate::R<SMHC_A12A_SPEC>;
#[doc = "Register `smhc_a12a` writer"]
pub type W = crate::W<SMHC_A12A_SPEC>;
#[doc = "Field `sd_a12a` reader - "]
pub type SD_A12A_R = crate::FieldReader<u16>;
#[doc = "Field `sd_a12a` writer - "]
pub type SD_A12A_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sd_a12a(&self) -> SD_A12A_R {
        SD_A12A_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn sd_a12a(&mut self) -> SD_A12A_W<SMHC_A12A_SPEC> {
        SD_A12A_W::new(self, 0)
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
#[doc = "Auto Command 12 Argument Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_a12a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_a12a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_A12A_SPEC;
impl crate::RegisterSpec for SMHC_A12A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_a12a::R`](R) reader structure"]
impl crate::Readable for SMHC_A12A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_a12a::W`](W) writer structure"]
impl crate::Writable for SMHC_A12A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_a12a to value 0"]
impl crate::Resettable for SMHC_A12A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
