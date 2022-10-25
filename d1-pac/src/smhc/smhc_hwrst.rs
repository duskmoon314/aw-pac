#[doc = "Register `smhc_hwrst` reader"]
pub struct R(crate::R<SMHC_HWRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_HWRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_HWRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_HWRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `smhc_hwrst` writer"]
pub struct W(crate::W<SMHC_HWRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_HWRST_SPEC>;
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
impl From<crate::W<SMHC_HWRST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_HWRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hw_rst` reader - "]
pub type HW_RST_R = crate::BitReader<HW_RST_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HW_RST_A {
    #[doc = "0: Active mode"]
    ACTIVE = 0,
    #[doc = "1: Reset"]
    RESET = 1,
}
impl From<HW_RST_A> for bool {
    #[inline(always)]
    fn from(variant: HW_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl HW_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HW_RST_A {
        match self.bits {
            false => HW_RST_A::ACTIVE,
            true => HW_RST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == HW_RST_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == HW_RST_A::RESET
    }
}
#[doc = "Field `hw_rst` writer - "]
pub type HW_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_HWRST_SPEC, HW_RST_A, O>;
impl<'a, const O: u8> HW_RST_W<'a, O> {
    #[doc = "Active mode"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(HW_RST_A::ACTIVE)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(HW_RST_A::RESET)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hw_rst(&self) -> HW_RST_R {
        HW_RST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn hw_rst(&mut self) -> HW_RST_W<0> {
        HW_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_hwrst](index.html) module"]
pub struct SMHC_HWRST_SPEC;
impl crate::RegisterSpec for SMHC_HWRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_hwrst::R](R) reader structure"]
impl crate::Readable for SMHC_HWRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_hwrst::W](W) writer structure"]
impl crate::Writable for SMHC_HWRST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_hwrst to value 0"]
impl crate::Resettable for SMHC_HWRST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
