#[doc = "Register `led_reset_timing_ctrl` reader"]
pub type R = crate::R<LED_RESET_TIMING_CTRL_SPEC>;
#[doc = "Register `led_reset_timing_ctrl` writer"]
pub type W = crate::W<LED_RESET_TIMING_CTRL_SPEC>;
#[doc = "Field `led_num` reader - "]
pub type LED_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `led_num` writer - "]
pub type LED_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tr_time` reader - "]
pub type TR_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `tr_time` writer - "]
pub type TR_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn led_num(&self) -> LED_NUM_R {
        LED_NUM_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn tr_time(&self) -> TR_TIME_R {
        TR_TIME_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn led_num(&mut self) -> LED_NUM_W<LED_RESET_TIMING_CTRL_SPEC> {
        LED_NUM_W::new(self, 0)
    }
    #[doc = "Bits 16:28"]
    #[inline(always)]
    #[must_use]
    pub fn tr_time(&mut self) -> TR_TIME_W<LED_RESET_TIMING_CTRL_SPEC> {
        TR_TIME_W::new(self, 16)
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
#[doc = "LEDC Reset Timing Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`led_reset_timing_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`led_reset_timing_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LED_RESET_TIMING_CTRL_SPEC;
impl crate::RegisterSpec for LED_RESET_TIMING_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`led_reset_timing_ctrl::R`](R) reader structure"]
impl crate::Readable for LED_RESET_TIMING_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`led_reset_timing_ctrl::W`](W) writer structure"]
impl crate::Writable for LED_RESET_TIMING_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets led_reset_timing_ctrl to value 0"]
impl crate::Resettable for LED_RESET_TIMING_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
