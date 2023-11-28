#[doc = "Register `rom_cfg` reader"]
pub type R = crate::R<ROM_CFG_SPEC>;
#[doc = "Register `rom_cfg` writer"]
pub type W = crate::W<ROM_CFG_SPEC>;
#[doc = "Field `rom_cfg` reader - ROM Configuration"]
pub type ROM_CFG_R = crate::FieldReader;
#[doc = "Field `rom_cfg` writer - ROM Configuration"]
pub type ROM_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ROM Configuration"]
    #[inline(always)]
    pub fn rom_cfg(&self) -> ROM_CFG_R {
        ROM_CFG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ROM Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn rom_cfg(&mut self) -> ROM_CFG_W<ROM_CFG_SPEC> {
        ROM_CFG_W::new(self, 0)
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
#[doc = "ROM Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROM_CFG_SPEC;
impl crate::RegisterSpec for ROM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_cfg::R`](R) reader structure"]
impl crate::Readable for ROM_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rom_cfg::W`](W) writer structure"]
impl crate::Writable for ROM_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rom_cfg to value 0"]
impl crate::Resettable for ROM_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
