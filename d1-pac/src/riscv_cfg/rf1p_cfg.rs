#[doc = "Register `rf1p_cfg` reader"]
pub type R = crate::R<RF1P_CFG_SPEC>;
#[doc = "Register `rf1p_cfg` writer"]
pub type W = crate::W<RF1P_CFG_SPEC>;
#[doc = "Field `rf1p_cfg` reader - RF1P Configuration"]
pub type RF1P_CFG_R = crate::FieldReader;
#[doc = "Field `rf1p_cfg` writer - RF1P Configuration"]
pub type RF1P_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RF1P Configuration"]
    #[inline(always)]
    pub fn rf1p_cfg(&self) -> RF1P_CFG_R {
        RF1P_CFG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RF1P Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn rf1p_cfg(&mut self) -> RF1P_CFG_W<RF1P_CFG_SPEC> {
        RF1P_CFG_W::new(self, 0)
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
#[doc = "RF1P Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rf1p_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rf1p_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RF1P_CFG_SPEC;
impl crate::RegisterSpec for RF1P_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf1p_cfg::R`](R) reader structure"]
impl crate::Readable for RF1P_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rf1p_cfg::W`](W) writer structure"]
impl crate::Writable for RF1P_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf1p_cfg to value 0"]
impl crate::Resettable for RF1P_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
