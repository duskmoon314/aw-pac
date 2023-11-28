#[doc = "Register `smhc_ext_cmd` reader"]
pub type R = crate::R<SMHC_EXT_CMD_SPEC>;
#[doc = "Register `smhc_ext_cmd` writer"]
pub type W = crate::W<SMHC_EXT_CMD_SPEC>;
#[doc = "Field `auto_cmd23_en` reader - Send CMD23 Automatically"]
pub type AUTO_CMD23_EN_R = crate::BitReader;
#[doc = "Field `auto_cmd23_en` writer - Send CMD23 Automatically"]
pub type AUTO_CMD23_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Send CMD23 Automatically"]
    #[inline(always)]
    pub fn auto_cmd23_en(&self) -> AUTO_CMD23_EN_R {
        AUTO_CMD23_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send CMD23 Automatically"]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd23_en(&mut self) -> AUTO_CMD23_EN_W<SMHC_EXT_CMD_SPEC> {
        AUTO_CMD23_EN_W::new(self, 0)
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
#[doc = "Extended Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_ext_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_ext_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_EXT_CMD_SPEC;
impl crate::RegisterSpec for SMHC_EXT_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_ext_cmd::R`](R) reader structure"]
impl crate::Readable for SMHC_EXT_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_ext_cmd::W`](W) writer structure"]
impl crate::Writable for SMHC_EXT_CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_ext_cmd to value 0"]
impl crate::Resettable for SMHC_EXT_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
