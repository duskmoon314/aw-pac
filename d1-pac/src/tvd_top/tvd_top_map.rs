#[doc = "Register `tvd_top_map` reader"]
pub type R = crate::R<TVD_TOP_MAP_SPEC>;
#[doc = "Register `tvd_top_map` writer"]
pub type W = crate::W<TVD_TOP_MAP_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TVD_TOP_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
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
#[doc = "TVD TOP MAP Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_top_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_top_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVD_TOP_MAP_SPEC;
impl crate::RegisterSpec for TVD_TOP_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tvd_top_map::R`](R) reader structure"]
impl crate::Readable for TVD_TOP_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tvd_top_map::W`](W) writer structure"]
impl crate::Writable for TVD_TOP_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tvd_top_map to value 0"]
impl crate::Resettable for TVD_TOP_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
