#[doc = "Register `tve_low_pass_control` reader"]
pub type R = crate::R<TVE_LOW_PASS_CONTROL_SPEC>;
#[doc = "Register `tve_low_pass_control` writer"]
pub type W = crate::W<TVE_LOW_PASS_CONTROL_SPEC>;
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
    pub const fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DISABLE,
            true => EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EN_A::ENABLE
    }
}
#[doc = "Field `en` writer - LP function enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN_A>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> ENABLE_DEFLICKER_A {
        match self.bits {
            false => ENABLE_DEFLICKER_A::DISABLE,
            true => ENABLE_DEFLICKER_A::ENABLE,
        }
    }
    #[doc = "Disable deflicker"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_DEFLICKER_A::DISABLE
    }
    #[doc = "Enable deflicker"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_DEFLICKER_A::ENABLE
    }
}
#[doc = "Field `enable_deflicker` writer - Enable_deflicker"]
pub type ENABLE_DEFLICKER_W<'a, REG> = crate::BitWriter<'a, REG, ENABLE_DEFLICKER_A>;
impl<'a, REG> ENABLE_DEFLICKER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable deflicker"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_DEFLICKER_A::DISABLE)
    }
    #[doc = "Enable deflicker"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> FIX_COEF_DEFLICKER_A {
        match self.bits {
            false => FIX_COEF_DEFLICKER_A::A_UTO,
            true => FIX_COEF_DEFLICKER_A::U_SER,
        }
    }
    #[doc = "Auto deflicker"]
    #[inline(always)]
    pub fn is_a_uto(&self) -> bool {
        *self == FIX_COEF_DEFLICKER_A::A_UTO
    }
    #[doc = "User deflicker"]
    #[inline(always)]
    pub fn is_u_ser(&self) -> bool {
        *self == FIX_COEF_DEFLICKER_A::U_SER
    }
}
#[doc = "Field `fix_coef_deflicker` writer - Fix_coef_deflicker"]
pub type FIX_COEF_DEFLICKER_W<'a, REG> = crate::BitWriter<'a, REG, FIX_COEF_DEFLICKER_A>;
impl<'a, REG> FIX_COEF_DEFLICKER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto deflicker"]
    #[inline(always)]
    pub fn a_uto(self) -> &'a mut crate::W<REG> {
        self.variant(FIX_COEF_DEFLICKER_A::A_UTO)
    }
    #[doc = "User deflicker"]
    #[inline(always)]
    pub fn u_ser(self) -> &'a mut crate::W<REG> {
        self.variant(FIX_COEF_DEFLICKER_A::U_SER)
    }
}
#[doc = "Field `user_deflicker_coef` reader - User_deflicker_coef\n\nup: coef/32\n\nCenter: 1-coef/16\n\nDown: coef/32"]
pub type USER_DEFLICKER_COEF_R = crate::FieldReader;
#[doc = "Field `user_deflicker_coef` writer - User_deflicker_coef\n\nup: coef/32\n\nCenter: 1-coef/16\n\nDown: coef/32"]
pub type USER_DEFLICKER_COEF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    pub fn en(&mut self) -> EN_W<TVE_LOW_PASS_CONTROL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 8 - Enable_deflicker"]
    #[inline(always)]
    #[must_use]
    pub fn enable_deflicker(&mut self) -> ENABLE_DEFLICKER_W<TVE_LOW_PASS_CONTROL_SPEC> {
        ENABLE_DEFLICKER_W::new(self, 8)
    }
    #[doc = "Bit 9 - Fix_coef_deflicker"]
    #[inline(always)]
    #[must_use]
    pub fn fix_coef_deflicker(&mut self) -> FIX_COEF_DEFLICKER_W<TVE_LOW_PASS_CONTROL_SPEC> {
        FIX_COEF_DEFLICKER_W::new(self, 9)
    }
    #[doc = "Bits 10:13 - User_deflicker_coef\n\nup: coef/32\n\nCenter: 1-coef/16\n\nDown: coef/32"]
    #[inline(always)]
    #[must_use]
    pub fn user_deflicker_coef(&mut self) -> USER_DEFLICKER_COEF_W<TVE_LOW_PASS_CONTROL_SPEC> {
        USER_DEFLICKER_COEF_W::new(self, 10)
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
#[doc = "TV Encoder Low Pass Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_low_pass_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_low_pass_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_LOW_PASS_CONTROL_SPEC;
impl crate::RegisterSpec for TVE_LOW_PASS_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_low_pass_control::R`](R) reader structure"]
impl crate::Readable for TVE_LOW_PASS_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_low_pass_control::W`](W) writer structure"]
impl crate::Writable for TVE_LOW_PASS_CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_low_pass_control to value 0"]
impl crate::Resettable for TVE_LOW_PASS_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
