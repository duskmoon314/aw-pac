#[doc = "Register `ledc_wait_time0_ctrl` reader"]
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
#[doc = "Register `ledc_wait_time0_ctrl` writer"]
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
#[doc = "Field `total_wait_time0` reader - "]
pub type TOTAL_WAIT_TIME0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `total_wait_time0` writer - "]
pub type TOTAL_WAIT_TIME0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LEDC_WAIT_TIME0_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `wait_tim0_en` reader - "]
pub type WAIT_TIM0_EN_R = crate::BitReader<WAIT_TIM0_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl WAIT_TIM0_EN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == WAIT_TIM0_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WAIT_TIM0_EN_A::ENABLE
    }
}
#[doc = "Field `wait_tim0_en` writer - "]
pub type WAIT_TIM0_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LEDC_WAIT_TIME0_CTRL_SPEC, WAIT_TIM0_EN_A, O>;
impl<'a, const O: u8> WAIT_TIM0_EN_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn total_wait_time0(&self) -> TOTAL_WAIT_TIME0_R {
        TOTAL_WAIT_TIME0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wait_tim0_en(&self) -> WAIT_TIM0_EN_R {
        WAIT_TIM0_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn total_wait_time0(&mut self) -> TOTAL_WAIT_TIME0_W<0> {
        TOTAL_WAIT_TIME0_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn wait_tim0_en(&mut self) -> WAIT_TIM0_EN_W<8> {
        WAIT_TIM0_EN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ledc_wait_time0_ctrl to value 0"]
impl crate::Resettable for LEDC_WAIT_TIME0_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
