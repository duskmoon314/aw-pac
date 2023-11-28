#[doc = "Register `ledc_data_finish_cnt` reader"]
pub type R = crate::R<LEDC_DATA_FINISH_CNT_SPEC>;
#[doc = "Register `ledc_data_finish_cnt` writer"]
pub type W = crate::W<LEDC_DATA_FINISH_CNT_SPEC>;
#[doc = "Field `led_data_finish_cnt` reader - "]
pub type LED_DATA_FINISH_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `led_wait_data_time` reader - "]
pub type LED_WAIT_DATA_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `led_wait_data_time` writer - "]
pub type LED_WAIT_DATA_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn led_data_finish_cnt(&self) -> LED_DATA_FINISH_CNT_R {
        LED_DATA_FINISH_CNT_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:29"]
    #[inline(always)]
    pub fn led_wait_data_time(&self) -> LED_WAIT_DATA_TIME_R {
        LED_WAIT_DATA_TIME_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:29"]
    #[inline(always)]
    #[must_use]
    pub fn led_wait_data_time(&mut self) -> LED_WAIT_DATA_TIME_W<LEDC_DATA_FINISH_CNT_SPEC> {
        LED_WAIT_DATA_TIME_W::new(self, 16)
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
#[doc = "LEDC Data Finish Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_data_finish_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_data_finish_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LEDC_DATA_FINISH_CNT_SPEC;
impl crate::RegisterSpec for LEDC_DATA_FINISH_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ledc_data_finish_cnt::R`](R) reader structure"]
impl crate::Readable for LEDC_DATA_FINISH_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ledc_data_finish_cnt::W`](W) writer structure"]
impl crate::Writable for LEDC_DATA_FINISH_CNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ledc_data_finish_cnt to value 0"]
impl crate::Resettable for LEDC_DATA_FINISH_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
