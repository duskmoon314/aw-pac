#[doc = "Register `csic_top_en` reader"]
pub struct R(crate::R<CSIC_TOP_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_TOP_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_TOP_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_TOP_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_top_en` writer"]
pub struct W(crate::W<CSIC_TOP_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_TOP_EN_SPEC>;
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
impl From<crate::W<CSIC_TOP_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_TOP_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `csic_top_en` reader - "]
pub type CSIC_TOP_EN_R = crate::BitReader<CSIC_TOP_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSIC_TOP_EN_A {
    #[doc = "0: Reset and disable the CSIC module"]
    RESET_DISABLE = 0,
    #[doc = "1: Enable the CSIC module"]
    ENABLE = 1,
}
impl From<CSIC_TOP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CSIC_TOP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CSIC_TOP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSIC_TOP_EN_A {
        match self.bits {
            false => CSIC_TOP_EN_A::RESET_DISABLE,
            true => CSIC_TOP_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DISABLE`"]
    #[inline(always)]
    pub fn is_reset_disable(&self) -> bool {
        *self == CSIC_TOP_EN_A::RESET_DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CSIC_TOP_EN_A::ENABLE
    }
}
#[doc = "Field `csic_top_en` writer - "]
pub type CSIC_TOP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSIC_TOP_EN_SPEC, CSIC_TOP_EN_A, O>;
impl<'a, const O: u8> CSIC_TOP_EN_W<'a, O> {
    #[doc = "Reset and disable the CSIC module"]
    #[inline(always)]
    pub fn reset_disable(self) -> &'a mut W {
        self.variant(CSIC_TOP_EN_A::RESET_DISABLE)
    }
    #[doc = "Enable the CSIC module"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CSIC_TOP_EN_A::ENABLE)
    }
}
#[doc = "Field `bist_mode_en` reader - "]
pub type BIST_MODE_EN_R = crate::BitReader<BIST_MODE_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIST_MODE_EN_A {
    #[doc = "0: Closed"]
    C_LOSED = 0,
    #[doc = "1: EN BIST TEST"]
    EN = 1,
}
impl From<BIST_MODE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BIST_MODE_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BIST_MODE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIST_MODE_EN_A {
        match self.bits {
            false => BIST_MODE_EN_A::C_LOSED,
            true => BIST_MODE_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `C_LOSED`"]
    #[inline(always)]
    pub fn is_c_losed(&self) -> bool {
        *self == BIST_MODE_EN_A::C_LOSED
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BIST_MODE_EN_A::EN
    }
}
#[doc = "Field `bist_mode_en` writer - "]
pub type BIST_MODE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSIC_TOP_EN_SPEC, BIST_MODE_EN_A, O>;
impl<'a, const O: u8> BIST_MODE_EN_W<'a, O> {
    #[doc = "Closed"]
    #[inline(always)]
    pub fn c_losed(self) -> &'a mut W {
        self.variant(BIST_MODE_EN_A::C_LOSED)
    }
    #[doc = "EN BIST TEST"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BIST_MODE_EN_A::EN)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn csic_top_en(&self) -> CSIC_TOP_EN_R {
        CSIC_TOP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn bist_mode_en(&self) -> BIST_MODE_EN_R {
        BIST_MODE_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn csic_top_en(&mut self) -> CSIC_TOP_EN_W<0> {
        CSIC_TOP_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn bist_mode_en(&mut self) -> BIST_MODE_EN_W<2> {
        BIST_MODE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC TOP Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_top_en](index.html) module"]
pub struct CSIC_TOP_EN_SPEC;
impl crate::RegisterSpec for CSIC_TOP_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_top_en::R](R) reader structure"]
impl crate::Readable for CSIC_TOP_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_top_en::W](W) writer structure"]
impl crate::Writable for CSIC_TOP_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_top_en to value 0"]
impl crate::Resettable for CSIC_TOP_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
