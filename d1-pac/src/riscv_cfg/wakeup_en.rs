#[doc = "Register `wakeup_en` reader"]
pub type R = crate::R<WAKEUP_EN_SPEC>;
#[doc = "Register `wakeup_en` writer"]
pub type W = crate::W<WAKEUP_EN_SPEC>;
#[doc = "Field `wp_en` reader - Wakeup Enable"]
pub type WP_EN_R = crate::BitReader;
#[doc = "Field `wp_en` writer - Wakeup Enable"]
pub type WP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup Enable"]
    #[inline(always)]
    pub fn wp_en(&self) -> WP_EN_R {
        WP_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wp_en(&mut self) -> WP_EN_W<WAKEUP_EN_SPEC> {
        WP_EN_W::new(self, 0)
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
#[doc = "Wakeup Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAKEUP_EN_SPEC;
impl crate::RegisterSpec for WAKEUP_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeup_en::R`](R) reader structure"]
impl crate::Readable for WAKEUP_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wakeup_en::W`](W) writer structure"]
impl crate::Writable for WAKEUP_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wakeup_en to value 0"]
impl crate::Resettable for WAKEUP_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
