#[doc = "Register `smhc_clkdiv` reader"]
pub struct R(crate::R<SMHC_CLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_CLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_CLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_CLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `smhc_clkdiv` writer"]
pub struct W(crate::W<SMHC_CLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_CLKDIV_SPEC>;
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
impl From<crate::W<SMHC_CLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_CLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cclk_div` reader - Card Clock Divider"]
pub type CCLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cclk_div` writer - Card Clock Divider"]
pub type CCLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMHC_CLKDIV_SPEC, u8, u8, 8, O>;
#[doc = "Field `cclk_enb` reader - Card Clock Enable"]
pub type CCLK_ENB_R = crate::BitReader<CCLK_ENB_A>;
#[doc = "Card Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLK_ENB_A {
    #[doc = "0: Card Clock is off"]
    OFF = 0,
    #[doc = "1: Card Clock is on"]
    ON = 1,
}
impl From<CCLK_ENB_A> for bool {
    #[inline(always)]
    fn from(variant: CCLK_ENB_A) -> Self {
        variant as u8 != 0
    }
}
impl CCLK_ENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCLK_ENB_A {
        match self.bits {
            false => CCLK_ENB_A::OFF,
            true => CCLK_ENB_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CCLK_ENB_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CCLK_ENB_A::ON
    }
}
#[doc = "Field `cclk_enb` writer - Card Clock Enable"]
pub type CCLK_ENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_CLKDIV_SPEC, CCLK_ENB_A, O>;
impl<'a, const O: u8> CCLK_ENB_W<'a, O> {
    #[doc = "Card Clock is off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CCLK_ENB_A::OFF)
    }
    #[doc = "Card Clock is on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CCLK_ENB_A::ON)
    }
}
#[doc = "Field `cclk_ctrl` reader - Card Clock Output Control"]
pub type CCLK_CTRL_R = crate::BitReader<CCLK_CTRL_A>;
#[doc = "Card Clock Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLK_CTRL_A {
    #[doc = "0: Card clock is always on"]
    ON = 0,
    #[doc = "1: Turn off card clock when FSM is in IDLE state"]
    OFF_IDLE = 1,
}
impl From<CCLK_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: CCLK_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl CCLK_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCLK_CTRL_A {
        match self.bits {
            false => CCLK_CTRL_A::ON,
            true => CCLK_CTRL_A::OFF_IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CCLK_CTRL_A::ON
    }
    #[doc = "Checks if the value of the field is `OFF_IDLE`"]
    #[inline(always)]
    pub fn is_off_idle(&self) -> bool {
        *self == CCLK_CTRL_A::OFF_IDLE
    }
}
#[doc = "Field `cclk_ctrl` writer - Card Clock Output Control"]
pub type CCLK_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_CLKDIV_SPEC, CCLK_CTRL_A, O>;
impl<'a, const O: u8> CCLK_CTRL_W<'a, O> {
    #[doc = "Card clock is always on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CCLK_CTRL_A::ON)
    }
    #[doc = "Turn off card clock when FSM is in IDLE state"]
    #[inline(always)]
    pub fn off_idle(self) -> &'a mut W {
        self.variant(CCLK_CTRL_A::OFF_IDLE)
    }
}
#[doc = "Field `mask_data0` reader - "]
pub type MASK_DATA0_R = crate::BitReader<MASK_DATA0_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASK_DATA0_A {
    #[doc = "0: Do not mask data0 when update clock"]
    NOT_MASK = 0,
    #[doc = "1: Mask data0 when update clock"]
    MASK = 1,
}
impl From<MASK_DATA0_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_DATA0_A) -> Self {
        variant as u8 != 0
    }
}
impl MASK_DATA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_DATA0_A {
        match self.bits {
            false => MASK_DATA0_A::NOT_MASK,
            true => MASK_DATA0_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_MASK`"]
    #[inline(always)]
    pub fn is_not_mask(&self) -> bool {
        *self == MASK_DATA0_A::NOT_MASK
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == MASK_DATA0_A::MASK
    }
}
#[doc = "Field `mask_data0` writer - "]
pub type MASK_DATA0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SMHC_CLKDIV_SPEC, MASK_DATA0_A, O>;
impl<'a, const O: u8> MASK_DATA0_W<'a, O> {
    #[doc = "Do not mask data0 when update clock"]
    #[inline(always)]
    pub fn not_mask(self) -> &'a mut W {
        self.variant(MASK_DATA0_A::NOT_MASK)
    }
    #[doc = "Mask data0 when update clock"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(MASK_DATA0_A::MASK)
    }
}
impl R {
    #[doc = "Bits 0:7 - Card Clock Divider"]
    #[inline(always)]
    pub fn cclk_div(&self) -> CCLK_DIV_R {
        CCLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Card Clock Enable"]
    #[inline(always)]
    pub fn cclk_enb(&self) -> CCLK_ENB_R {
        CCLK_ENB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Card Clock Output Control"]
    #[inline(always)]
    pub fn cclk_ctrl(&self) -> CCLK_CTRL_R {
        CCLK_CTRL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn mask_data0(&self) -> MASK_DATA0_R {
        MASK_DATA0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Card Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn cclk_div(&mut self) -> CCLK_DIV_W<0> {
        CCLK_DIV_W::new(self)
    }
    #[doc = "Bit 16 - Card Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cclk_enb(&mut self) -> CCLK_ENB_W<16> {
        CCLK_ENB_W::new(self)
    }
    #[doc = "Bit 17 - Card Clock Output Control"]
    #[inline(always)]
    #[must_use]
    pub fn cclk_ctrl(&mut self) -> CCLK_CTRL_W<17> {
        CCLK_CTRL_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn mask_data0(&mut self) -> MASK_DATA0_W<31> {
        MASK_DATA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_clkdiv](index.html) module"]
pub struct SMHC_CLKDIV_SPEC;
impl crate::RegisterSpec for SMHC_CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_clkdiv::R](R) reader structure"]
impl crate::Readable for SMHC_CLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_clkdiv::W](W) writer structure"]
impl crate::Writable for SMHC_CLKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_clkdiv to value 0"]
impl crate::Resettable for SMHC_CLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
