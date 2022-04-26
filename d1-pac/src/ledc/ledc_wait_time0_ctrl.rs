#[doc = "Register `LEDC_WAIT_TIME0_CTRL` reader"]
pub struct R(crate::R<LEDC_WAIT_TIME0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEDC_WAIT_TIME0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEDC_WAIT_TIME0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEDC_WAIT_TIME0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LEDC_WAIT_TIME0_CTRL` writer"]
pub struct W(crate::W<LEDC_WAIT_TIME0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEDC_WAIT_TIME0_CTRL_SPEC>;
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
impl From<crate::W<LEDC_WAIT_TIME0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEDC_WAIT_TIME0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAIT_TIM0_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<WAIT_TIM0_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WAIT_TIM0_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAIT_TIM0_EN` reader - "]
pub struct WAIT_TIM0_EN_R(crate::FieldReader<bool>);
impl WAIT_TIM0_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAIT_TIM0_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAIT_TIM0_EN_A {
        match self.bits {
            false => WAIT_TIM0_EN_A::DISABLE,
            true => WAIT_TIM0_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WAIT_TIM0_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WAIT_TIM0_EN_A::ENABLE
    }
}
impl core::ops::Deref for WAIT_TIM0_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAIT_TIM0_EN` writer - "]
pub struct WAIT_TIM0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_TIM0_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAIT_TIM0_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WAIT_TIM0_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WAIT_TIM0_EN_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `TOTAL_WAIT_TIME0` reader - "]
pub struct TOTAL_WAIT_TIME0_R(crate::FieldReader<u8>);
impl TOTAL_WAIT_TIME0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOTAL_WAIT_TIME0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOTAL_WAIT_TIME0_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOTAL_WAIT_TIME0` writer - "]
pub struct TOTAL_WAIT_TIME0_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTAL_WAIT_TIME0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wait_tim0_en(&self) -> WAIT_TIM0_EN_R {
        WAIT_TIM0_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn total_wait_time0(&self) -> TOTAL_WAIT_TIME0_R {
        TOTAL_WAIT_TIME0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wait_tim0_en(&mut self) -> WAIT_TIM0_EN_W {
        WAIT_TIM0_EN_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn total_wait_time0(&mut self) -> TOTAL_WAIT_TIME0_W {
        TOTAL_WAIT_TIME0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC Wait Time0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_wait_time0_ctrl](index.html) module"]
pub struct LEDC_WAIT_TIME0_CTRL_SPEC;
impl crate::RegisterSpec for LEDC_WAIT_TIME0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ledc_wait_time0_ctrl::R](R) reader structure"]
impl crate::Readable for LEDC_WAIT_TIME0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ledc_wait_time0_ctrl::W](W) writer structure"]
impl crate::Writable for LEDC_WAIT_TIME0_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LEDC_WAIT_TIME0_CTRL to value 0"]
impl crate::Resettable for LEDC_WAIT_TIME0_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
