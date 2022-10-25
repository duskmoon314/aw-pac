#[doc = "Register `ths_en` reader"]
pub struct R(crate::R<THS_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THS_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THS_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THS_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ths_en` writer"]
pub struct W(crate::W<THS_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THS_EN_SPEC>;
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
impl From<crate::W<THS_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THS_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ths_en` reader - Enable temperature measurement sensor"]
pub type THS_EN_R = crate::BitReader<THS_EN_A>;
#[doc = "Enable temperature measurement sensor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THS_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<THS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: THS_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl THS_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THS_EN_A {
        match self.bits {
            false => THS_EN_A::DISABLE,
            true => THS_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == THS_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == THS_EN_A::ENABLE
    }
}
#[doc = "Field `ths_en` writer - Enable temperature measurement sensor"]
pub type THS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, THS_EN_SPEC, THS_EN_A, O>;
impl<'a, const O: u8> THS_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(THS_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(THS_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable temperature measurement sensor"]
    #[inline(always)]
    pub fn ths_en(&self) -> THS_EN_R {
        THS_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable temperature measurement sensor"]
    #[inline(always)]
    #[must_use]
    pub fn ths_en(&mut self) -> THS_EN_W<0> {
        THS_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "THS Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ths_en](index.html) module"]
pub struct THS_EN_SPEC;
impl crate::RegisterSpec for THS_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ths_en::R](R) reader structure"]
impl crate::Readable for THS_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ths_en::W](W) writer structure"]
impl crate::Writable for THS_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ths_en to value 0"]
impl crate::Resettable for THS_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
