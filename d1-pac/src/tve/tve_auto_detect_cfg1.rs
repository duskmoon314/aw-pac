#[doc = "Register `tve_auto_detect_cfg1` reader"]
pub type R = crate::R<TVE_AUTO_DETECT_CFG1_SPEC>;
#[doc = "Register `tve_auto_detect_cfg1` writer"]
pub type W = crate::W<TVE_AUTO_DETECT_CFG1_SPEC>;
#[doc = "Field `detect_pulse_start` reader - Detect signal start time"]
pub type DETECT_PULSE_START_R = crate::FieldReader<u16>;
#[doc = "Field `detect_pulse_start` writer - Detect signal start time"]
pub type DETECT_PULSE_START_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `detect_pulse_periods` reader - Use 32K clock"]
pub type DETECT_PULSE_PERIODS_R = crate::FieldReader<u16>;
#[doc = "Field `detect_pulse_periods` writer - Use 32K clock"]
pub type DETECT_PULSE_PERIODS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Detect signal start time"]
    #[inline(always)]
    pub fn detect_pulse_start(&self) -> DETECT_PULSE_START_R {
        DETECT_PULSE_START_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Use 32K clock"]
    #[inline(always)]
    pub fn detect_pulse_periods(&self) -> DETECT_PULSE_PERIODS_R {
        DETECT_PULSE_PERIODS_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Detect signal start time"]
    #[inline(always)]
    #[must_use]
    pub fn detect_pulse_start(&mut self) -> DETECT_PULSE_START_W<TVE_AUTO_DETECT_CFG1_SPEC> {
        DETECT_PULSE_START_W::new(self, 0)
    }
    #[doc = "Bits 16:30 - Use 32K clock"]
    #[inline(always)]
    #[must_use]
    pub fn detect_pulse_periods(&mut self) -> DETECT_PULSE_PERIODS_W<TVE_AUTO_DETECT_CFG1_SPEC> {
        DETECT_PULSE_PERIODS_W::new(self, 16)
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
#[doc = "TV Encoder Auto Detect Configuration Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_auto_detect_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_auto_detect_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_AUTO_DETECT_CFG1_SPEC;
impl crate::RegisterSpec for TVE_AUTO_DETECT_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_auto_detect_cfg1::R`](R) reader structure"]
impl crate::Readable for TVE_AUTO_DETECT_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_auto_detect_cfg1::W`](W) writer structure"]
impl crate::Writable for TVE_AUTO_DETECT_CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_auto_detect_cfg1 to value 0"]
impl crate::Resettable for TVE_AUTO_DETECT_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
