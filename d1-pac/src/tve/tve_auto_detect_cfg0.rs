#[doc = "Register `tve_auto_detect_cfg0` reader"]
pub type R = crate::R<TVE_AUTO_DETECT_CFG0_SPEC>;
#[doc = "Register `tve_auto_detect_cfg0` writer"]
pub type W = crate::W<TVE_AUTO_DETECT_CFG0_SPEC>;
#[doc = "Field `detect_pulse_value` reader - Use for DAC data input at auto detect pluse. Set the pulse amplitude."]
pub type DETECT_PULSE_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `detect_pulse_value` writer - Use for DAC data input at auto detect pluse. Set the pulse amplitude."]
pub type DETECT_PULSE_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Use for DAC data input at auto detect pluse. Set the pulse amplitude."]
    #[inline(always)]
    pub fn detect_pulse_value(&self) -> DETECT_PULSE_VALUE_R {
        DETECT_PULSE_VALUE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Use for DAC data input at auto detect pluse. Set the pulse amplitude."]
    #[inline(always)]
    #[must_use]
    pub fn detect_pulse_value(&mut self) -> DETECT_PULSE_VALUE_W<TVE_AUTO_DETECT_CFG0_SPEC> {
        DETECT_PULSE_VALUE_W::new(self, 0)
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
#[doc = "TV Encoder Auto Detect Configuration Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_auto_detect_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_auto_detect_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_AUTO_DETECT_CFG0_SPEC;
impl crate::RegisterSpec for TVE_AUTO_DETECT_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_auto_detect_cfg0::R`](R) reader structure"]
impl crate::Readable for TVE_AUTO_DETECT_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_auto_detect_cfg0::W`](W) writer structure"]
impl crate::Writable for TVE_AUTO_DETECT_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_auto_detect_cfg0 to value 0"]
impl crate::Resettable for TVE_AUTO_DETECT_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
