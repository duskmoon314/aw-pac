#[doc = "Register `led_t01_timing_ctrl` reader"]
pub type R = crate::R<LED_T01_TIMING_CTRL_SPEC>;
#[doc = "Register `led_t01_timing_ctrl` writer"]
pub type W = crate::W<LED_T01_TIMING_CTRL_SPEC>;
#[doc = "Field `t0l_time` reader - "]
pub type T0L_TIME_R = crate::FieldReader;
#[doc = "Field `t0l_time` writer - "]
pub type T0L_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `t0h_time` reader - "]
pub type T0H_TIME_R = crate::FieldReader;
#[doc = "Field `t0h_time` writer - "]
pub type T0H_TIME_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `t1l_time` reader - "]
pub type T1L_TIME_R = crate::FieldReader;
#[doc = "Field `t1l_time` writer - "]
pub type T1L_TIME_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `t1h_time` reader - "]
pub type T1H_TIME_R = crate::FieldReader;
#[doc = "Field `t1h_time` writer - "]
pub type T1H_TIME_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn t0l_time(&self) -> T0L_TIME_R {
        T0L_TIME_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn t0h_time(&self) -> T0H_TIME_R {
        T0H_TIME_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn t1l_time(&self) -> T1L_TIME_R {
        T1L_TIME_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:26"]
    #[inline(always)]
    pub fn t1h_time(&self) -> T1H_TIME_R {
        T1H_TIME_R::new(((self.bits >> 21) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn t0l_time(&mut self) -> T0L_TIME_W<LED_T01_TIMING_CTRL_SPEC> {
        T0L_TIME_W::new(self, 0)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    #[must_use]
    pub fn t0h_time(&mut self) -> T0H_TIME_W<LED_T01_TIMING_CTRL_SPEC> {
        T0H_TIME_W::new(self, 6)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn t1l_time(&mut self) -> T1L_TIME_W<LED_T01_TIMING_CTRL_SPEC> {
        T1L_TIME_W::new(self, 16)
    }
    #[doc = "Bits 21:26"]
    #[inline(always)]
    #[must_use]
    pub fn t1h_time(&mut self) -> T1H_TIME_W<LED_T01_TIMING_CTRL_SPEC> {
        T1H_TIME_W::new(self, 21)
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
#[doc = "LEDC T0 T1 Timing Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`led_t01_timing_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`led_t01_timing_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LED_T01_TIMING_CTRL_SPEC;
impl crate::RegisterSpec for LED_T01_TIMING_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`led_t01_timing_ctrl::R`](R) reader structure"]
impl crate::Readable for LED_T01_TIMING_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`led_t01_timing_ctrl::W`](W) writer structure"]
impl crate::Writable for LED_T01_TIMING_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets led_t01_timing_ctrl to value 0"]
impl crate::Resettable for LED_T01_TIMING_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
