#[doc = "Register `mclaim` reader"]
pub type R = crate::R<MCLAIM_SPEC>;
#[doc = "Register `mclaim` writer"]
pub type W = crate::W<MCLAIM_SPEC>;
#[doc = "Field `mclaim` reader - "]
pub type MCLAIM_R = crate::FieldReader<u16>;
#[doc = "Field `mclaim` writer - "]
pub type MCLAIM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn mclaim(&self) -> MCLAIM_R {
        MCLAIM_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn mclaim(&mut self) -> MCLAIM_W<MCLAIM_SPEC> {
        MCLAIM_W::new(self, 0)
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
#[doc = "Machine Mode Claim/Complete Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclaim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclaim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCLAIM_SPEC;
impl crate::RegisterSpec for MCLAIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mclaim::R`](R) reader structure"]
impl crate::Readable for MCLAIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mclaim::W`](W) writer structure"]
impl crate::Writable for MCLAIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mclaim to value 0"]
impl crate::Resettable for MCLAIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
