#[doc = "Register `LEDC_DATA_FINISH_CNT` reader"]
pub struct R(crate::R<LEDC_DATA_FINISH_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEDC_DATA_FINISH_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEDC_DATA_FINISH_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEDC_DATA_FINISH_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LEDC_DATA_FINISH_CNT` writer"]
pub struct W(crate::W<LEDC_DATA_FINISH_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEDC_DATA_FINISH_CNT_SPEC>;
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
impl From<crate::W<LEDC_DATA_FINISH_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEDC_DATA_FINISH_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LED_WAIT_DATA_TIME` reader - "]
pub struct LED_WAIT_DATA_TIME_R(crate::FieldReader<u16, u16>);
impl LED_WAIT_DATA_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LED_WAIT_DATA_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LED_WAIT_DATA_TIME_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LED_WAIT_DATA_TIME` writer - "]
pub struct LED_WAIT_DATA_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_WAIT_DATA_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | ((value as u32 & 0x3fff) << 16);
        self.w
    }
}
#[doc = "Field `LED_DATA_FINISH_CNT` reader - "]
pub struct LED_DATA_FINISH_CNT_R(crate::FieldReader<u16, u16>);
impl LED_DATA_FINISH_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LED_DATA_FINISH_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LED_DATA_FINISH_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:29"]
    #[inline(always)]
    pub fn led_wait_data_time(&self) -> LED_WAIT_DATA_TIME_R {
        LED_WAIT_DATA_TIME_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn led_data_finish_cnt(&self) -> LED_DATA_FINISH_CNT_R {
        LED_DATA_FINISH_CNT_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:29"]
    #[inline(always)]
    pub fn led_wait_data_time(&mut self) -> LED_WAIT_DATA_TIME_W {
        LED_WAIT_DATA_TIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC Data Finish Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_data_finish_cnt](index.html) module"]
pub struct LEDC_DATA_FINISH_CNT_SPEC;
impl crate::RegisterSpec for LEDC_DATA_FINISH_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ledc_data_finish_cnt::R](R) reader structure"]
impl crate::Readable for LEDC_DATA_FINISH_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ledc_data_finish_cnt::W](W) writer structure"]
impl crate::Writable for LEDC_DATA_FINISH_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LEDC_DATA_FINISH_CNT to value 0"]
impl crate::Resettable for LEDC_DATA_FINISH_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
