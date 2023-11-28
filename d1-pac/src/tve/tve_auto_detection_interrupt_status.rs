#[doc = "Register `tve_auto_detection_interrupt_status` reader"]
pub type R = crate::R<TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC>;
#[doc = "Register `tve_auto_detection_interrupt_status` writer"]
pub type W = crate::W<TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC>;
#[doc = "Field `dac0_auto_detect_interrupt_active_flag` reader - Write 1 to inactive DAC0 auto detection interrupt"]
pub type DAC0_AUTO_DETECT_INTERRUPT_ACTIVE_FLAG_R = crate::BitReader;
#[doc = "Field `dac0_auto_detect_interrupt_active_flag` writer - Write 1 to inactive DAC0 auto detection interrupt"]
pub type DAC0_AUTO_DETECT_INTERRUPT_ACTIVE_FLAG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to inactive DAC0 auto detection interrupt"]
    #[inline(always)]
    pub fn dac0_auto_detect_interrupt_active_flag(
        &self,
    ) -> DAC0_AUTO_DETECT_INTERRUPT_ACTIVE_FLAG_R {
        DAC0_AUTO_DETECT_INTERRUPT_ACTIVE_FLAG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to inactive DAC0 auto detection interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dac0_auto_detect_interrupt_active_flag(
        &mut self,
    ) -> DAC0_AUTO_DETECT_INTERRUPT_ACTIVE_FLAG_W<TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC> {
        DAC0_AUTO_DETECT_INTERRUPT_ACTIVE_FLAG_W::new(self, 0)
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
#[doc = "TV Encoder Auto Detection Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_auto_detection_interrupt_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_auto_detection_interrupt_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC;
impl crate::RegisterSpec for TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_auto_detection_interrupt_status::R`](R) reader structure"]
impl crate::Readable for TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_auto_detection_interrupt_status::W`](W) writer structure"]
impl crate::Writable for TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets tve_auto_detection_interrupt_status to value 0"]
impl crate::Resettable for TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
