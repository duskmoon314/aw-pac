#[doc = "Register `ledc_ctrl` reader"]
pub struct R(crate::R<LEDC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEDC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEDC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEDC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ledc_ctrl` writer"]
pub struct W(crate::W<LEDC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEDC_CTRL_SPEC>;
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
impl From<crate::W<LEDC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEDC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ledc_en` reader - "]
pub type LEDC_EN_R = crate::BitReader<LEDC_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEDC_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<LEDC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LEDC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LEDC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEDC_EN_A {
        match self.bits {
            false => LEDC_EN_A::DISABLE,
            true => LEDC_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LEDC_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LEDC_EN_A::ENABLE
    }
}
#[doc = "Field `ledc_en` writer - "]
pub type LEDC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LEDC_CTRL_SPEC, LEDC_EN_A, O>;
impl<'a, const O: u8> LEDC_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LEDC_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LEDC_EN_A::ENABLE)
    }
}
#[doc = "Field `ledc_soft_reset` reader - "]
pub type LEDC_SOFT_RESET_R = crate::BitReader<bool>;
#[doc = "Field `ledc_soft_reset` writer - "]
pub type LEDC_SOFT_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, LEDC_CTRL_SPEC, bool, O>;
#[doc = "Field `led_msb_[B,R,G,TOP]` reader - "]
pub type LED_MSB__R = crate::BitReader<LED_MSB__A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LED_MSB__A {
    #[doc = "0: `0`"]
    LSB = 0,
    #[doc = "1: `1`"]
    MSB = 1,
}
impl From<LED_MSB__A> for bool {
    #[inline(always)]
    fn from(variant: LED_MSB__A) -> Self {
        variant as u8 != 0
    }
}
impl LED_MSB__R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LED_MSB__A {
        match self.bits {
            false => LED_MSB__A::LSB,
            true => LED_MSB__A::MSB,
        }
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == LED_MSB__A::LSB
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == LED_MSB__A::MSB
    }
}
#[doc = "Field `led_msb_[B,R,G,TOP]` writer - "]
pub type LED_MSB__W<'a, const O: u8> = crate::BitWriter<'a, u32, LEDC_CTRL_SPEC, LED_MSB__A, O>;
impl<'a, const O: u8> LED_MSB__W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(LED_MSB__A::LSB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(LED_MSB__A::MSB)
    }
}
#[doc = "Field `led_rgb_mode` reader - "]
pub type LED_RGB_MODE_R = crate::FieldReader<u8, LED_RGB_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LED_RGB_MODE_A {
    #[doc = "0: `0`"]
    GRB = 0,
    #[doc = "1: `1`"]
    GBR = 1,
    #[doc = "2: `10`"]
    RGB = 2,
    #[doc = "3: `11`"]
    RBG = 3,
    #[doc = "4: `100`"]
    BGR = 4,
    #[doc = "5: `101`"]
    BRG = 5,
}
impl From<LED_RGB_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LED_RGB_MODE_A) -> Self {
        variant as _
    }
}
impl LED_RGB_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LED_RGB_MODE_A> {
        match self.bits {
            0 => Some(LED_RGB_MODE_A::GRB),
            1 => Some(LED_RGB_MODE_A::GBR),
            2 => Some(LED_RGB_MODE_A::RGB),
            3 => Some(LED_RGB_MODE_A::RBG),
            4 => Some(LED_RGB_MODE_A::BGR),
            5 => Some(LED_RGB_MODE_A::BRG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GRB`"]
    #[inline(always)]
    pub fn is_grb(&self) -> bool {
        *self == LED_RGB_MODE_A::GRB
    }
    #[doc = "Checks if the value of the field is `GBR`"]
    #[inline(always)]
    pub fn is_gbr(&self) -> bool {
        *self == LED_RGB_MODE_A::GBR
    }
    #[doc = "Checks if the value of the field is `RGB`"]
    #[inline(always)]
    pub fn is_rgb(&self) -> bool {
        *self == LED_RGB_MODE_A::RGB
    }
    #[doc = "Checks if the value of the field is `RBG`"]
    #[inline(always)]
    pub fn is_rbg(&self) -> bool {
        *self == LED_RGB_MODE_A::RBG
    }
    #[doc = "Checks if the value of the field is `BGR`"]
    #[inline(always)]
    pub fn is_bgr(&self) -> bool {
        *self == LED_RGB_MODE_A::BGR
    }
    #[doc = "Checks if the value of the field is `BRG`"]
    #[inline(always)]
    pub fn is_brg(&self) -> bool {
        *self == LED_RGB_MODE_A::BRG
    }
}
#[doc = "Field `led_rgb_mode` writer - "]
pub type LED_RGB_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LEDC_CTRL_SPEC, u8, LED_RGB_MODE_A, 3, O>;
impl<'a, const O: u8> LED_RGB_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn grb(self) -> &'a mut W {
        self.variant(LED_RGB_MODE_A::GRB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn gbr(self) -> &'a mut W {
        self.variant(LED_RGB_MODE_A::GBR)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rgb(self) -> &'a mut W {
        self.variant(LED_RGB_MODE_A::RGB)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rbg(self) -> &'a mut W {
        self.variant(LED_RGB_MODE_A::RBG)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn bgr(self) -> &'a mut W {
        self.variant(LED_RGB_MODE_A::BGR)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn brg(self) -> &'a mut W {
        self.variant(LED_RGB_MODE_A::BRG)
    }
}
#[doc = "Field `reset_led_en` reader - "]
pub type RESET_LED_EN_R = crate::BitReader<bool>;
#[doc = "Field `reset_led_en` writer - "]
pub type RESET_LED_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LEDC_CTRL_SPEC, bool, O>;
#[doc = "Field `total_data_length` reader - "]
pub type TOTAL_DATA_LENGTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `total_data_length` writer - "]
pub type TOTAL_DATA_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LEDC_CTRL_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ledc_en(&self) -> LEDC_EN_R {
        LEDC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ledc_soft_reset(&self) -> LEDC_SOFT_RESET_R {
        LEDC_SOFT_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn led_msb_b(&self) -> LED_MSB__R {
        LED_MSB__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn led_msb_r(&self) -> LED_MSB__R {
        LED_MSB__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn led_msb_g(&self) -> LED_MSB__R {
        LED_MSB__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn led_msb_top(&self) -> LED_MSB__R {
        LED_MSB__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn led_rgb_mode(&self) -> LED_RGB_MODE_R {
        LED_RGB_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reset_led_en(&self) -> RESET_LED_EN_R {
        RESET_LED_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn total_data_length(&self) -> TOTAL_DATA_LENGTH_R {
        TOTAL_DATA_LENGTH_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_en(&mut self) -> LEDC_EN_W<0> {
        LEDC_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_soft_reset(&mut self) -> LEDC_SOFT_RESET_W<1> {
        LEDC_SOFT_RESET_W::new(self)
    }
    #[doc = ""]
    #[inline(always)]
    #[must_use]
    pub unsafe fn led_msb_<const O: u8>(&mut self) -> LED_MSB__W<O> {
        LED_MSB__W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn led_msb_b(&mut self) -> LED_MSB__W<2> {
        LED_MSB__W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn led_msb_r(&mut self) -> LED_MSB__W<3> {
        LED_MSB__W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn led_msb_g(&mut self) -> LED_MSB__W<4> {
        LED_MSB__W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn led_msb_top(&mut self) -> LED_MSB__W<5> {
        LED_MSB__W::new(self)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    #[must_use]
    pub fn led_rgb_mode(&mut self) -> LED_RGB_MODE_W<6> {
        LED_RGB_MODE_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn reset_led_en(&mut self) -> RESET_LED_EN_W<10> {
        RESET_LED_EN_W::new(self)
    }
    #[doc = "Bits 16:28"]
    #[inline(always)]
    #[must_use]
    pub fn total_data_length(&mut self) -> TOTAL_DATA_LENGTH_W<16> {
        TOTAL_DATA_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_ctrl](index.html) module"]
pub struct LEDC_CTRL_SPEC;
impl crate::RegisterSpec for LEDC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ledc_ctrl::R](R) reader structure"]
impl crate::Readable for LEDC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ledc_ctrl::W](W) writer structure"]
impl crate::Writable for LEDC_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ledc_ctrl to value 0"]
impl crate::Resettable for LEDC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
