#[doc = "Register `hs_tmr%s_ctrl` reader"]
pub struct R(crate::R<HS_TMR_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HS_TMR_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HS_TMR_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HS_TMR_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hs_tmr%s_ctrl` writer"]
pub struct W(crate::W<HS_TMR_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HS_TMR_CTRL_SPEC>;
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
impl From<crate::W<HS_TMR_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HS_TMR_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hs_tmr_en` reader - HSTimer Enable"]
pub type HS_TMR_EN_R = crate::BitReader<HS_TMR_EN_A>;
#[doc = "HSTimer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HS_TMR_EN_A {
    #[doc = "0: `0`"]
    STOP_PAUSE = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<HS_TMR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HS_TMR_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HS_TMR_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_TMR_EN_A {
        match self.bits {
            false => HS_TMR_EN_A::STOP_PAUSE,
            true => HS_TMR_EN_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_PAUSE`"]
    #[inline(always)]
    pub fn is_stop_pause(&self) -> bool {
        *self == HS_TMR_EN_A::STOP_PAUSE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == HS_TMR_EN_A::START
    }
}
#[doc = "Field `hs_tmr_en` writer - HSTimer Enable"]
pub type HS_TMR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HS_TMR_CTRL_SPEC, HS_TMR_EN_A, O>;
impl<'a, const O: u8> HS_TMR_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stop_pause(self) -> &'a mut W {
        self.variant(HS_TMR_EN_A::STOP_PAUSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(HS_TMR_EN_A::START)
    }
}
#[doc = "Field `hs_tmr_reload` reader - HSTimer Reload"]
pub type HS_TMR_RELOAD_R = crate::BitReader<HS_TMR_RELOAD_A>;
#[doc = "HSTimer Reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HS_TMR_RELOAD_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    RELOAD = 1,
}
impl From<HS_TMR_RELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: HS_TMR_RELOAD_A) -> Self {
        variant as u8 != 0
    }
}
impl HS_TMR_RELOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_TMR_RELOAD_A {
        match self.bits {
            false => HS_TMR_RELOAD_A::NO_EFFECT,
            true => HS_TMR_RELOAD_A::RELOAD,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == HS_TMR_RELOAD_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `RELOAD`"]
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == HS_TMR_RELOAD_A::RELOAD
    }
}
#[doc = "Field `hs_tmr_reload` writer - HSTimer Reload"]
pub type HS_TMR_RELOAD_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, HS_TMR_CTRL_SPEC, HS_TMR_RELOAD_A, O>;
impl<'a, const O: u8> HS_TMR_RELOAD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(HS_TMR_RELOAD_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(HS_TMR_RELOAD_A::RELOAD)
    }
}
#[doc = "Field `hs_tmr_clk` reader - Select the pre-scale for the HSTimer clock sources"]
pub type HS_TMR_CLK_R = crate::FieldReader<u8, HS_TMR_CLK_A>;
#[doc = "Select the pre-scale for the HSTimer clock sources\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HS_TMR_CLK_A {
    #[doc = "0: `0`"]
    P1 = 0,
    #[doc = "1: `1`"]
    P2 = 1,
    #[doc = "2: `10`"]
    P4 = 2,
    #[doc = "3: `11`"]
    P8 = 3,
    #[doc = "4: `100`"]
    P16 = 4,
}
impl From<HS_TMR_CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: HS_TMR_CLK_A) -> Self {
        variant as _
    }
}
impl HS_TMR_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HS_TMR_CLK_A> {
        match self.bits {
            0 => Some(HS_TMR_CLK_A::P1),
            1 => Some(HS_TMR_CLK_A::P2),
            2 => Some(HS_TMR_CLK_A::P4),
            3 => Some(HS_TMR_CLK_A::P8),
            4 => Some(HS_TMR_CLK_A::P16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P1`"]
    #[inline(always)]
    pub fn is_p1(&self) -> bool {
        *self == HS_TMR_CLK_A::P1
    }
    #[doc = "Checks if the value of the field is `P2`"]
    #[inline(always)]
    pub fn is_p2(&self) -> bool {
        *self == HS_TMR_CLK_A::P2
    }
    #[doc = "Checks if the value of the field is `P4`"]
    #[inline(always)]
    pub fn is_p4(&self) -> bool {
        *self == HS_TMR_CLK_A::P4
    }
    #[doc = "Checks if the value of the field is `P8`"]
    #[inline(always)]
    pub fn is_p8(&self) -> bool {
        *self == HS_TMR_CLK_A::P8
    }
    #[doc = "Checks if the value of the field is `P16`"]
    #[inline(always)]
    pub fn is_p16(&self) -> bool {
        *self == HS_TMR_CLK_A::P16
    }
}
#[doc = "Field `hs_tmr_clk` writer - Select the pre-scale for the HSTimer clock sources"]
pub type HS_TMR_CLK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HS_TMR_CTRL_SPEC, u8, HS_TMR_CLK_A, 3, O>;
impl<'a, const O: u8> HS_TMR_CLK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn p1(self) -> &'a mut W {
        self.variant(HS_TMR_CLK_A::P1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn p2(self) -> &'a mut W {
        self.variant(HS_TMR_CLK_A::P2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn p4(self) -> &'a mut W {
        self.variant(HS_TMR_CLK_A::P4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn p8(self) -> &'a mut W {
        self.variant(HS_TMR_CLK_A::P8)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn p16(self) -> &'a mut W {
        self.variant(HS_TMR_CLK_A::P16)
    }
}
#[doc = "Field `hs_tmr_mode` reader - Select the timing mode for HSTimer"]
pub type HS_TMR_MODE_R = crate::BitReader<HS_TMR_MODE_A>;
#[doc = "Select the timing mode for HSTimer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HS_TMR_MODE_A {
    #[doc = "0: `0`"]
    PERIODIC = 0,
    #[doc = "1: `1`"]
    ONE_SHOT = 1,
}
impl From<HS_TMR_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: HS_TMR_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl HS_TMR_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_TMR_MODE_A {
        match self.bits {
            false => HS_TMR_MODE_A::PERIODIC,
            true => HS_TMR_MODE_A::ONE_SHOT,
        }
    }
    #[doc = "Checks if the value of the field is `PERIODIC`"]
    #[inline(always)]
    pub fn is_periodic(&self) -> bool {
        *self == HS_TMR_MODE_A::PERIODIC
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT`"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == HS_TMR_MODE_A::ONE_SHOT
    }
}
#[doc = "Field `hs_tmr_mode` writer - Select the timing mode for HSTimer"]
pub type HS_TMR_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HS_TMR_CTRL_SPEC, HS_TMR_MODE_A, O>;
impl<'a, const O: u8> HS_TMR_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn periodic(self) -> &'a mut W {
        self.variant(HS_TMR_MODE_A::PERIODIC)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(HS_TMR_MODE_A::ONE_SHOT)
    }
}
#[doc = "Field `hs_tmr_test` reader - Select the operating mode for HSTimer"]
pub type HS_TMR_TEST_R = crate::BitReader<HS_TMR_TEST_A>;
#[doc = "Select the operating mode for HSTimer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HS_TMR_TEST_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    TEST = 1,
}
impl From<HS_TMR_TEST_A> for bool {
    #[inline(always)]
    fn from(variant: HS_TMR_TEST_A) -> Self {
        variant as u8 != 0
    }
}
impl HS_TMR_TEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_TMR_TEST_A {
        match self.bits {
            false => HS_TMR_TEST_A::NORMAL,
            true => HS_TMR_TEST_A::TEST,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == HS_TMR_TEST_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `TEST`"]
    #[inline(always)]
    pub fn is_test(&self) -> bool {
        *self == HS_TMR_TEST_A::TEST
    }
}
#[doc = "Field `hs_tmr_test` writer - Select the operating mode for HSTimer"]
pub type HS_TMR_TEST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HS_TMR_CTRL_SPEC, HS_TMR_TEST_A, O>;
impl<'a, const O: u8> HS_TMR_TEST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(HS_TMR_TEST_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn test(self) -> &'a mut W {
        self.variant(HS_TMR_TEST_A::TEST)
    }
}
impl R {
    #[doc = "Bit 0 - HSTimer Enable"]
    #[inline(always)]
    pub fn hs_tmr_en(&self) -> HS_TMR_EN_R {
        HS_TMR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSTimer Reload"]
    #[inline(always)]
    pub fn hs_tmr_reload(&self) -> HS_TMR_RELOAD_R {
        HS_TMR_RELOAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Select the pre-scale for the HSTimer clock sources"]
    #[inline(always)]
    pub fn hs_tmr_clk(&self) -> HS_TMR_CLK_R {
        HS_TMR_CLK_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Select the timing mode for HSTimer"]
    #[inline(always)]
    pub fn hs_tmr_mode(&self) -> HS_TMR_MODE_R {
        HS_TMR_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - Select the operating mode for HSTimer"]
    #[inline(always)]
    pub fn hs_tmr_test(&self) -> HS_TMR_TEST_R {
        HS_TMR_TEST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSTimer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tmr_en(&mut self) -> HS_TMR_EN_W<0> {
        HS_TMR_EN_W::new(self)
    }
    #[doc = "Bit 1 - HSTimer Reload"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tmr_reload(&mut self) -> HS_TMR_RELOAD_W<1> {
        HS_TMR_RELOAD_W::new(self)
    }
    #[doc = "Bits 4:6 - Select the pre-scale for the HSTimer clock sources"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tmr_clk(&mut self) -> HS_TMR_CLK_W<4> {
        HS_TMR_CLK_W::new(self)
    }
    #[doc = "Bit 7 - Select the timing mode for HSTimer"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tmr_mode(&mut self) -> HS_TMR_MODE_W<7> {
        HS_TMR_MODE_W::new(self)
    }
    #[doc = "Bit 31 - Select the operating mode for HSTimer"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tmr_test(&mut self) -> HS_TMR_TEST_W<31> {
        HS_TMR_TEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HS Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hs_tmr_ctrl](index.html) module"]
pub struct HS_TMR_CTRL_SPEC;
impl crate::RegisterSpec for HS_TMR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hs_tmr_ctrl::R](R) reader structure"]
impl crate::Readable for HS_TMR_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hs_tmr_ctrl::W](W) writer structure"]
impl crate::Writable for HS_TMR_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x02;
}
#[doc = "`reset()` method sets hs_tmr%s_ctrl to value 0"]
impl crate::Resettable for HS_TMR_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
