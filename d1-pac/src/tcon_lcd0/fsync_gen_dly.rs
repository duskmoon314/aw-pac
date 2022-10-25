#[doc = "Register `fsync_gen_dly` reader"]
pub struct R(crate::R<FSYNC_GEN_DLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSYNC_GEN_DLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSYNC_GEN_DLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSYNC_GEN_DLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fsync_gen_dly` writer"]
pub struct W(crate::W<FSYNC_GEN_DLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSYNC_GEN_DLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FSYNC_GEN_DLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSYNC_GEN_DLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sensor_act1_time` reader - Delay 0-4095 Pixel clk Period\n\nThe actual delay is sensor_act1_time+1."]
pub type SENSOR_ACT1_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `sensor_act1_time` writer - Delay 0-4095 Pixel clk Period\n\nThe actual delay is sensor_act1_time+1."]
pub type SENSOR_ACT1_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSYNC_GEN_DLY_SPEC, u16, u16, 12, O>;
#[doc = "Field `sensor_act0_time` reader - Delay 0-4095 Pixel clk Period\n\nThe actual delay is sensor_act0_time+1."]
pub type SENSOR_ACT0_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `sensor_act0_time` writer - Delay 0-4095 Pixel clk Period\n\nThe actual delay is sensor_act0_time+1."]
pub type SENSOR_ACT0_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSYNC_GEN_DLY_SPEC, u16, u16, 12, O>;
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
    pub fn sensor_act1_time(&mut self) -> SENSOR_ACT1_TIME_W<0> {
        SENSOR_ACT1_TIME_W::new(self)
    }
    #[doc = "Bits 16:27 - Delay 0-4095 Pixel clk Period\n\nThe actual delay is sensor_act0_time+1."]
    #[inline(always)]
    #[must_use]
    pub fn sensor_act0_time(&mut self) -> SENSOR_ACT0_TIME_W<16> {
        SENSOR_ACT0_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FSYNC_GEN_DLY\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsync_gen_dly](index.html) module"]
pub struct FSYNC_GEN_DLY_SPEC;
impl crate::RegisterSpec for FSYNC_GEN_DLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsync_gen_dly::R](R) reader structure"]
impl crate::Readable for FSYNC_GEN_DLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsync_gen_dly::W](W) writer structure"]
impl crate::Writable for FSYNC_GEN_DLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fsync_gen_dly to value 0"]
impl crate::Resettable for FSYNC_GEN_DLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
