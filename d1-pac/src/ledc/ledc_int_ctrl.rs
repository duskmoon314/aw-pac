#[doc = "Register `LEDC_INT_CTRL` reader"]
pub struct R(crate::R<LEDC_INT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEDC_INT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEDC_INT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEDC_INT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LEDC_INT_CTRL` writer"]
pub struct W(crate::W<LEDC_INT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEDC_INT_CTRL_SPEC>;
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
impl From<crate::W<LEDC_INT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEDC_INT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GLOBAL_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<GLOBAL_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: GLOBAL_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GLOBAL_INT_EN` reader - "]
pub type GLOBAL_INT_EN_R = crate::BitReader<GLOBAL_INT_EN_A>;
impl GLOBAL_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GLOBAL_INT_EN_A {
        match self.bits {
            false => GLOBAL_INT_EN_A::DISABLE,
            true => GLOBAL_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GLOBAL_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GLOBAL_INT_EN_A::ENABLE
    }
}
#[doc = "Field `GLOBAL_INT_EN` writer - "]
pub type GLOBAL_INT_EN_W<'a> = crate::BitWriter<'a, u32, LEDC_INT_CTRL_SPEC, GLOBAL_INT_EN_A, 5>;
impl<'a> GLOBAL_INT_EN_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GLOBAL_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GLOBAL_INT_EN_A::ENABLE)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_OVERFLOW_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<FIFO_OVERFLOW_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_OVERFLOW_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_OVERFLOW_INT_EN` reader - "]
pub type FIFO_OVERFLOW_INT_EN_R = crate::BitReader<FIFO_OVERFLOW_INT_EN_A>;
impl FIFO_OVERFLOW_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_OVERFLOW_INT_EN_A {
        match self.bits {
            false => FIFO_OVERFLOW_INT_EN_A::DISABLE,
            true => FIFO_OVERFLOW_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FIFO_OVERFLOW_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FIFO_OVERFLOW_INT_EN_A::ENABLE
    }
}
#[doc = "Field `FIFO_OVERFLOW_INT_EN` writer - "]
pub type FIFO_OVERFLOW_INT_EN_W<'a> =
    crate::BitWriter<'a, u32, LEDC_INT_CTRL_SPEC, FIFO_OVERFLOW_INT_EN_A, 4>;
impl<'a> FIFO_OVERFLOW_INT_EN_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FIFO_OVERFLOW_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FIFO_OVERFLOW_INT_EN_A::ENABLE)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITDATA_TIMEOUT_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<WAITDATA_TIMEOUT_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WAITDATA_TIMEOUT_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITDATA_TIMEOUT_INT_EN` reader - "]
pub type WAITDATA_TIMEOUT_INT_EN_R = crate::BitReader<WAITDATA_TIMEOUT_INT_EN_A>;
impl WAITDATA_TIMEOUT_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITDATA_TIMEOUT_INT_EN_A {
        match self.bits {
            false => WAITDATA_TIMEOUT_INT_EN_A::DISABLE,
            true => WAITDATA_TIMEOUT_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WAITDATA_TIMEOUT_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WAITDATA_TIMEOUT_INT_EN_A::ENABLE
    }
}
#[doc = "Field `WAITDATA_TIMEOUT_INT_EN` writer - "]
pub type WAITDATA_TIMEOUT_INT_EN_W<'a> =
    crate::BitWriter<'a, u32, LEDC_INT_CTRL_SPEC, WAITDATA_TIMEOUT_INT_EN_A, 3>;
impl<'a> WAITDATA_TIMEOUT_INT_EN_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WAITDATA_TIMEOUT_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WAITDATA_TIMEOUT_INT_EN_A::ENABLE)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_CPUREQ_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<FIFO_CPUREQ_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_CPUREQ_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_CPUREQ_INT_EN` reader - "]
pub type FIFO_CPUREQ_INT_EN_R = crate::BitReader<FIFO_CPUREQ_INT_EN_A>;
impl FIFO_CPUREQ_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_CPUREQ_INT_EN_A {
        match self.bits {
            false => FIFO_CPUREQ_INT_EN_A::DISABLE,
            true => FIFO_CPUREQ_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FIFO_CPUREQ_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FIFO_CPUREQ_INT_EN_A::ENABLE
    }
}
#[doc = "Field `FIFO_CPUREQ_INT_EN` writer - "]
pub type FIFO_CPUREQ_INT_EN_W<'a> =
    crate::BitWriter<'a, u32, LEDC_INT_CTRL_SPEC, FIFO_CPUREQ_INT_EN_A, 1>;
impl<'a> FIFO_CPUREQ_INT_EN_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FIFO_CPUREQ_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FIFO_CPUREQ_INT_EN_A::ENABLE)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LED_TRANS_FINISH_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<LED_TRANS_FINISH_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LED_TRANS_FINISH_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LED_TRANS_FINISH_INT_EN` reader - "]
pub type LED_TRANS_FINISH_INT_EN_R = crate::BitReader<LED_TRANS_FINISH_INT_EN_A>;
impl LED_TRANS_FINISH_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LED_TRANS_FINISH_INT_EN_A {
        match self.bits {
            false => LED_TRANS_FINISH_INT_EN_A::DISABLE,
            true => LED_TRANS_FINISH_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LED_TRANS_FINISH_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LED_TRANS_FINISH_INT_EN_A::ENABLE
    }
}
#[doc = "Field `LED_TRANS_FINISH_INT_EN` writer - "]
pub type LED_TRANS_FINISH_INT_EN_W<'a> =
    crate::BitWriter<'a, u32, LEDC_INT_CTRL_SPEC, LED_TRANS_FINISH_INT_EN_A, 0>;
impl<'a> LED_TRANS_FINISH_INT_EN_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LED_TRANS_FINISH_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LED_TRANS_FINISH_INT_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn global_int_en(&self) -> GLOBAL_INT_EN_R {
        GLOBAL_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fifo_overflow_int_en(&self) -> FIFO_OVERFLOW_INT_EN_R {
        FIFO_OVERFLOW_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn waitdata_timeout_int_en(&self) -> WAITDATA_TIMEOUT_INT_EN_R {
        WAITDATA_TIMEOUT_INT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fifo_cpureq_int_en(&self) -> FIFO_CPUREQ_INT_EN_R {
        FIFO_CPUREQ_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn led_trans_finish_int_en(&self) -> LED_TRANS_FINISH_INT_EN_R {
        LED_TRANS_FINISH_INT_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn global_int_en(&mut self) -> GLOBAL_INT_EN_W {
        GLOBAL_INT_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fifo_overflow_int_en(&mut self) -> FIFO_OVERFLOW_INT_EN_W {
        FIFO_OVERFLOW_INT_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn waitdata_timeout_int_en(&mut self) -> WAITDATA_TIMEOUT_INT_EN_W {
        WAITDATA_TIMEOUT_INT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fifo_cpureq_int_en(&mut self) -> FIFO_CPUREQ_INT_EN_W {
        FIFO_CPUREQ_INT_EN_W::new(self)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn led_trans_finish_int_en(&mut self) -> LED_TRANS_FINISH_INT_EN_W {
        LED_TRANS_FINISH_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_int_ctrl](index.html) module"]
pub struct LEDC_INT_CTRL_SPEC;
impl crate::RegisterSpec for LEDC_INT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ledc_int_ctrl::R](R) reader structure"]
impl crate::Readable for LEDC_INT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ledc_int_ctrl::W](W) writer structure"]
impl crate::Writable for LEDC_INT_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LEDC_INT_CTRL to value 0"]
impl crate::Resettable for LEDC_INT_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
