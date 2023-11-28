#[doc = "Register `fsync_gen_dly` reader"]
pub type R = crate::R<FSYNC_GEN_DLY_SPEC>;
#[doc = "Register `fsync_gen_dly` writer"]
pub type W = crate::W<FSYNC_GEN_DLY_SPEC>;
#[doc = "Field `sensor_act1_time` reader - Delay 0-4095 Pixel clk Period\n\nThe actual delay is sensor_act1_time+1."]
pub type SENSOR_ACT1_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `sensor_act1_time` writer - Delay 0-4095 Pixel clk Period\n\nThe actual delay is sensor_act1_time+1."]
pub type SENSOR_ACT1_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `sensor_act0_time` reader - Delay 0-4095 Pixel clk Period\n\nThe actual delay is sensor_act0_time+1."]
pub type SENSOR_ACT0_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `sensor_act0_time` writer - Delay 0-4095 Pixel clk Period\n\nThe actual delay is sensor_act0_time+1."]
pub type SENSOR_ACT0_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Delay 0-4095 Pixel clk Period\n\nThe actual delay is sensor_act1_time+1."]
    #[inline(always)]
    pub fn sensor_act1_time(&self) -> SENSOR_ACT1_TIME_R {
        SENSOR_ACT1_TIME_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Delay 0-4095 Pixel clk Period\n\nThe actual delay is sensor_act0_time+1."]
    #[inline(always)]
    pub fn sensor_act0_time(&self) -> SENSOR_ACT0_TIME_R {
        SENSOR_ACT0_TIME_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Delay 0-4095 Pixel clk Period\n\nThe actual delay is sensor_act1_time+1."]
    #[inline(always)]
    #[must_use]
    pub fn sensor_act1_time(&mut self) -> SENSOR_ACT1_TIME_W<FSYNC_GEN_DLY_SPEC> {
        SENSOR_ACT1_TIME_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Delay 0-4095 Pixel clk Period\n\nThe actual delay is sensor_act0_time+1."]
    #[inline(always)]
    #[must_use]
    pub fn sensor_act0_time(&mut self) -> SENSOR_ACT0_TIME_W<FSYNC_GEN_DLY_SPEC> {
        SENSOR_ACT0_TIME_W::new(self, 16)
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
#[doc = "FSYNC_GEN_DLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsync_gen_dly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsync_gen_dly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSYNC_GEN_DLY_SPEC;
impl crate::RegisterSpec for FSYNC_GEN_DLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsync_gen_dly::R`](R) reader structure"]
impl crate::Readable for FSYNC_GEN_DLY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fsync_gen_dly::W`](W) writer structure"]
impl crate::Writable for FSYNC_GEN_DLY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fsync_gen_dly to value 0"]
impl crate::Resettable for FSYNC_GEN_DLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
