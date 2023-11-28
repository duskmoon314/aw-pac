#[doc = "Register `wdog_output_cfg` reader"]
pub type R = crate::R<WDOG_OUTPUT_CFG_SPEC>;
#[doc = "Register `wdog_output_cfg` writer"]
pub type W = crate::W<WDOG_OUTPUT_CFG_SPEC>;
#[doc = "Field `wdog_output_config` reader - Configure the valid time for the watchdog reset signal."]
pub type WDOG_OUTPUT_CONFIG_R = crate::FieldReader<u16>;
#[doc = "Field `wdog_output_config` writer - Configure the valid time for the watchdog reset signal."]
pub type WDOG_OUTPUT_CONFIG_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Configure the valid time for the watchdog reset signal."]
    #[inline(always)]
    pub fn wdog_output_config(&self) -> WDOG_OUTPUT_CONFIG_R {
        WDOG_OUTPUT_CONFIG_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Configure the valid time for the watchdog reset signal."]
    #[inline(always)]
    #[must_use]
    pub fn wdog_output_config(&mut self) -> WDOG_OUTPUT_CONFIG_W<WDOG_OUTPUT_CFG_SPEC> {
        WDOG_OUTPUT_CONFIG_W::new(self, 0)
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
#[doc = "Watchdog Output Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdog_output_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdog_output_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDOG_OUTPUT_CFG_SPEC;
impl crate::RegisterSpec for WDOG_OUTPUT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdog_output_cfg::R`](R) reader structure"]
impl crate::Readable for WDOG_OUTPUT_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdog_output_cfg::W`](W) writer structure"]
impl crate::Writable for WDOG_OUTPUT_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wdog_output_cfg to value 0"]
impl crate::Resettable for WDOG_OUTPUT_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
