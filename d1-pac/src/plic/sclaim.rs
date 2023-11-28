#[doc = "Register `sclaim` reader"]
pub type R = crate::R<SCLAIM_SPEC>;
#[doc = "Register `sclaim` writer"]
pub type W = crate::W<SCLAIM_SPEC>;
#[doc = "Field `sclaim` reader - "]
pub type SCLAIM_R = crate::FieldReader<u16>;
#[doc = "Field `sclaim` writer - "]
pub type SCLAIM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn sclaim(&self) -> SCLAIM_R {
        SCLAIM_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn sclaim(&mut self) -> SCLAIM_W<SCLAIM_SPEC> {
        SCLAIM_W::new(self, 0)
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
#[doc = "Supervisor Mode Claim/Complete Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sclaim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sclaim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCLAIM_SPEC;
impl crate::RegisterSpec for SCLAIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sclaim::R`](R) reader structure"]
impl crate::Readable for SCLAIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sclaim::W`](W) writer structure"]
impl crate::Writable for SCLAIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sclaim to value 0"]
impl crate::Resettable for SCLAIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
