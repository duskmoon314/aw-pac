#[doc = "Register `tve_low_pass_control` reader"]
pub struct R(crate::R<TVE_LOW_PASS_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_LOW_PASS_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_LOW_PASS_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_LOW_PASS_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_low_pass_control` writer"]
pub struct W(crate::W<TVE_LOW_PASS_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_LOW_PASS_CONTROL_SPEC>;
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
impl From<crate::W<TVE_LOW_PASS_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_LOW_PASS_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `en` reader - LP function enable"]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "LP function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DISABLE,
            true => EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EN_A::ENABLE
    }
}
#[doc = "Field `en` writer - LP function enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TVE_LOW_PASS_CONTROL_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EN_A::ENABLE)
    }
}
#[doc = "Field `enable_deflicker` reader - Enable_deflicker"]
pub type ENABLE_DEFLICKER_R = crate::BitReader<ENABLE_DEFLICKER_A>;
#[doc = "Enable_deflicker\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_DEFLICKER_A {
    #[doc = "0: Disable deflicker"]
    DISABLE = 0,
    #[doc = "1: Enable deflicker"]
    ENABLE = 1,
}
impl From<ENABLE_DEFLICKER_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_DEFLICKER_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_DEFLICKER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_DEFLICKER_A {
        match self.bits {
            false => ENABLE_DEFLICKER_A::DISABLE,
            true => ENABLE_DEFLICKER_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_DEFLICKER_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_DEFLICKER_A::ENABLE
    }
}
#[doc = "Field `enable_deflicker` writer - Enable_deflicker"]
pub type ENABLE_DEFLICKER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_LOW_PASS_CONTROL_SPEC, ENABLE_DEFLICKER_A, O>;
impl<'a, const O: u8> ENABLE_DEFLICKER_W<'a, O> {
    #[doc = "Disable deflicker"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_DEFLICKER_A::DISABLE)
    }
    #[doc = "Enable deflicker"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_DEFLICKER_A::ENABLE)
    }
}
#[doc = "Field `fix_coef_deflicker` reader - Fix_coef_deflicker"]
pub type FIX_COEF_DEFLICKER_R = crate::BitReader<FIX_COEF_DEFLICKER_A>;
#[doc = "Fix_coef_deflicker\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIX_COEF_DEFLICKER_A {
    #[doc = "0: Auto deflicker"]
    A_UTO = 0,
    #[doc = "1: User deflicker"]
    U_SER = 1,
}
impl From<FIX_COEF_DEFLICKER_A> for bool {
    #[inline(always)]
    fn from(variant: FIX_COEF_DEFLICKER_A) -> Self {
        variant as u8 != 0
    }
}
impl FIX_COEF_DEFLICKER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIX_COEF_DEFLICKER_A {
        match self.bits {
            false => FIX_COEF_DEFLICKER_A::A_UTO,
            true => FIX_COEF_DEFLICKER_A::U_SER,
        }
    }
    #[doc = "Checks if the value of the field is `A_UTO`"]
    #[inline(always)]
    pub fn is_a_uto(&self) -> bool {
        *self == FIX_COEF_DEFLICKER_A::A_UTO
    }
    #[doc = "Checks if the value of the field is `U_SER`"]
    #[inline(always)]
    pub fn is_u_ser(&self) -> bool {
        *self == FIX_COEF_DEFLICKER_A::U_SER
    }
}
#[doc = "Field `fix_coef_deflicker` writer - Fix_coef_deflicker"]
pub type FIX_COEF_DEFLICKER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_LOW_PASS_CONTROL_SPEC, FIX_COEF_DEFLICKER_A, O>;
impl<'a, const O: u8> FIX_COEF_DEFLICKER_W<'a, O> {
    #[doc = "Auto deflicker"]
    #[inline(always)]
    pub fn a_uto(self) -> &'a mut W {
        self.variant(FIX_COEF_DEFLICKER_A::A_UTO)
    }
    #[doc = "User deflicker"]
    #[inline(always)]
    pub fn u_ser(self) -> &'a mut W {
        self.variant(FIX_COEF_DEFLICKER_A::U_SER)
    }
}
#[doc = "Field `user_deflicker_coef` reader - User_deflicker_coef\n\nup: coef/32\n\nCenter: 1-coef/16\n\nDown: coef/32"]
pub type USER_DEFLICKER_COEF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `user_deflicker_coef` writer - User_deflicker_coef\n\nup: coef/32\n\nCenter: 1-coef/16\n\nDown: coef/32"]
pub type USER_DEFLICKER_COEF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_LOW_PASS_CONTROL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - LP function enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enable_deflicker"]
    #[inline(always)]
    pub fn enable_deflicker(&self) -> ENABLE_DEFLICKER_R {
        ENABLE_DEFLICKER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fix_coef_deflicker"]
    #[inline(always)]
    pub fn fix_coef_deflicker(&self) -> FIX_COEF_DEFLICKER_R {
        FIX_COEF_DEFLICKER_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - User_deflicker_coef\n\nup: coef/32\n\nCenter: 1-coef/16\n\nDown: coef/32"]
    #[inline(always)]
    pub fn user_deflicker_coef(&self) -> USER_DEFLICKER_COEF_R {
        USER_DEFLICKER_COEF_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LP function enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 8 - Enable_deflicker"]
    #[inline(always)]
    #[must_use]
    pub fn enable_deflicker(&mut self) -> ENABLE_DEFLICKER_W<8> {
        ENABLE_DEFLICKER_W::new(self)
    }
    #[doc = "Bit 9 - Fix_coef_deflicker"]
    #[inline(always)]
    #[must_use]
    pub fn fix_coef_deflicker(&mut self) -> FIX_COEF_DEFLICKER_W<9> {
        FIX_COEF_DEFLICKER_W::new(self)
    }
    #[doc = "Bits 10:13 - User_deflicker_coef\n\nup: coef/32\n\nCenter: 1-coef/16\n\nDown: coef/32"]
    #[inline(always)]
    #[must_use]
    pub fn user_deflicker_coef(&mut self) -> USER_DEFLICKER_COEF_W<10> {
        USER_DEFLICKER_COEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Low Pass Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_low_pass_control](index.html) module"]
pub struct TVE_LOW_PASS_CONTROL_SPEC;
impl crate::RegisterSpec for TVE_LOW_PASS_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_low_pass_control::R](R) reader structure"]
impl crate::Readable for TVE_LOW_PASS_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_low_pass_control::W](W) writer structure"]
impl crate::Writable for TVE_LOW_PASS_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_low_pass_control to value 0"]
impl crate::Resettable for TVE_LOW_PASS_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
