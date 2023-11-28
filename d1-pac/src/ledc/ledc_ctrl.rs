#[doc = "Register `ledc_ctrl` reader"]
pub type R = crate::R<LEDC_CTRL_SPEC>;
#[doc = "Register `ledc_ctrl` writer"]
pub type W = crate::W<LEDC_CTRL_SPEC>;
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
    pub const fn variant(&self) -> LEDC_EN_A {
        match self.bits {
            false => LEDC_EN_A::DISABLE,
            true => LEDC_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LEDC_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LEDC_EN_A::ENABLE
    }
}
#[doc = "Field `ledc_en` writer - "]
pub type LEDC_EN_W<'a, REG> = crate::BitWriter<'a, REG, LEDC_EN_A>;
impl<'a, REG> LEDC_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LEDC_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LEDC_EN_A::ENABLE)
    }
}
#[doc = "Field `ledc_soft_reset` reader - "]
pub type LEDC_SOFT_RESET_R = crate::BitReader;
#[doc = "Field `ledc_soft_reset` writer - "]
pub type LEDC_SOFT_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub const fn variant(&self) -> LED_MSB__A {
        match self.bits {
            false => LED_MSB__A::LSB,
            true => LED_MSB__A::MSB,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == LED_MSB__A::LSB
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == LED_MSB__A::MSB
    }
}
#[doc = "Field `led_msb_[B,R,G,TOP]` writer - "]
pub type LED_MSB__W<'a, REG> = crate::BitWriter<'a, REG, LED_MSB__A>;
impl<'a, REG> LED_MSB__W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(LED_MSB__A::LSB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(LED_MSB__A::MSB)
    }
}
#[doc = "Field `led_rgb_mode` reader - "]
pub type LED_RGB_MODE_R = crate::FieldReader<LED_RGB_MODE_A>;
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
impl crate::FieldSpec for LED_RGB_MODE_A {
    type Ux = u8;
}
impl LED_RGB_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LED_RGB_MODE_A> {
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
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_grb(&self) -> bool {
        *self == LED_RGB_MODE_A::GRB
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_gbr(&self) -> bool {
        *self == LED_RGB_MODE_A::GBR
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_rgb(&self) -> bool {
        *self == LED_RGB_MODE_A::RGB
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_rbg(&self) -> bool {
        *self == LED_RGB_MODE_A::RBG
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_bgr(&self) -> bool {
        *self == LED_RGB_MODE_A::BGR
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_brg(&self) -> bool {
        *self == LED_RGB_MODE_A::BRG
    }
}
#[doc = "Field `led_rgb_mode` writer - "]
pub type LED_RGB_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LED_RGB_MODE_A>;
impl<'a, REG> LED_RGB_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn grb(self) -> &'a mut crate::W<REG> {
        self.variant(LED_RGB_MODE_A::GRB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn gbr(self) -> &'a mut crate::W<REG> {
        self.variant(LED_RGB_MODE_A::GBR)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rgb(self) -> &'a mut crate::W<REG> {
        self.variant(LED_RGB_MODE_A::RGB)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rbg(self) -> &'a mut crate::W<REG> {
        self.variant(LED_RGB_MODE_A::RBG)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn bgr(self) -> &'a mut crate::W<REG> {
        self.variant(LED_RGB_MODE_A::BGR)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn brg(self) -> &'a mut crate::W<REG> {
        self.variant(LED_RGB_MODE_A::BRG)
    }
}
#[doc = "Field `reset_led_en` reader - "]
pub type RESET_LED_EN_R = crate::BitReader;
#[doc = "Field `reset_led_en` writer - "]
pub type RESET_LED_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `total_data_length` reader - "]
pub type TOTAL_DATA_LENGTH_R = crate::FieldReader<u16>;
#[doc = "Field `total_data_length` writer - "]
pub type TOTAL_DATA_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
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
    #[doc = "\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `led_msb_B` field"]
    #[inline(always)]
    pub fn led_msb_(&self, n: u8) -> LED_MSB__R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        LED_MSB__R::new(((self.bits >> (n + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - led_msb_B"]
    #[inline(always)]
    pub fn led_msb_b(&self) -> LED_MSB__R {
        LED_MSB__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - led_msb_R"]
    #[inline(always)]
    pub fn led_msb_r(&self) -> LED_MSB__R {
        LED_MSB__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - led_msb_G"]
    #[inline(always)]
    pub fn led_msb_g(&self) -> LED_MSB__R {
        LED_MSB__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - led_msb_TOP"]
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
    pub fn ledc_en(&mut self) -> LEDC_EN_W<LEDC_CTRL_SPEC> {
        LEDC_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_soft_reset(&mut self) -> LEDC_SOFT_RESET_W<LEDC_CTRL_SPEC> {
        LEDC_SOFT_RESET_W::new(self, 1)
    }
    #[doc = "\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `led_msb_B` field"]
    #[inline(always)]
    #[must_use]
    pub fn led_msb_(&mut self, n: u8) -> LED_MSB__W<LEDC_CTRL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        LED_MSB__W::new(self, n + 2)
    }
    #[doc = "Bit 2 - led_msb_B"]
    #[inline(always)]
    #[must_use]
    pub fn led_msb_b(&mut self) -> LED_MSB__W<LEDC_CTRL_SPEC> {
        LED_MSB__W::new(self, 2)
    }
    #[doc = "Bit 3 - led_msb_R"]
    #[inline(always)]
    #[must_use]
    pub fn led_msb_r(&mut self) -> LED_MSB__W<LEDC_CTRL_SPEC> {
        LED_MSB__W::new(self, 3)
    }
    #[doc = "Bit 4 - led_msb_G"]
    #[inline(always)]
    #[must_use]
    pub fn led_msb_g(&mut self) -> LED_MSB__W<LEDC_CTRL_SPEC> {
        LED_MSB__W::new(self, 4)
    }
    #[doc = "Bit 5 - led_msb_TOP"]
    #[inline(always)]
    #[must_use]
    pub fn led_msb_top(&mut self) -> LED_MSB__W<LEDC_CTRL_SPEC> {
        LED_MSB__W::new(self, 5)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    #[must_use]
    pub fn led_rgb_mode(&mut self) -> LED_RGB_MODE_W<LEDC_CTRL_SPEC> {
        LED_RGB_MODE_W::new(self, 6)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn reset_led_en(&mut self) -> RESET_LED_EN_W<LEDC_CTRL_SPEC> {
        RESET_LED_EN_W::new(self, 10)
    }
    #[doc = "Bits 16:28"]
    #[inline(always)]
    #[must_use]
    pub fn total_data_length(&mut self) -> TOTAL_DATA_LENGTH_W<LEDC_CTRL_SPEC> {
        TOTAL_DATA_LENGTH_W::new(self, 16)
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
#[doc = "LEDC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LEDC_CTRL_SPEC;
impl crate::RegisterSpec for LEDC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ledc_ctrl::R`](R) reader structure"]
impl crate::Readable for LEDC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ledc_ctrl::W`](W) writer structure"]
impl crate::Writable for LEDC_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ledc_ctrl to value 0"]
impl crate::Resettable for LEDC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
