#[doc = "Register `configflag` reader"]
pub type R = crate::R<CONFIGFLAG_SPEC>;
#[doc = "Register `configflag` writer"]
pub type W = crate::W<CONFIGFLAG_SPEC>;
#[doc = "Field `cf` reader - Configure Flag (CF)\n\nHost software sets this bit as the last action in its process of configuring the Host Controller."]
pub type CF_R = crate::BitReader;
#[doc = "Field `cf` writer - Configure Flag (CF)\n\nHost software sets this bit as the last action in its process of configuring the Host Controller."]
pub type CF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configure Flag (CF)\n\nHost software sets this bit as the last action in its process of configuring the Host Controller."]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure Flag (CF)\n\nHost software sets this bit as the last action in its process of configuring the Host Controller."]
    #[inline(always)]
    #[must_use]
    pub fn cf(&mut self) -> CF_W<CONFIGFLAG_SPEC> {
        CF_W::new(self, 0)
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
#[doc = "EHCI Configure Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`configflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`configflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIGFLAG_SPEC;
impl crate::RegisterSpec for CONFIGFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`configflag::R`](R) reader structure"]
impl crate::Readable for CONFIGFLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`configflag::W`](W) writer structure"]
impl crate::Writable for CONFIGFLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets configflag to value 0"]
impl crate::Resettable for CONFIGFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
