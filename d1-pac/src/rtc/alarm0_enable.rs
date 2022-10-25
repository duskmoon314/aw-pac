#[doc = "Register `alarm0_enable` reader"]
pub struct R(crate::R<ALARM0_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARM0_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARM0_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARM0_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `alarm0_enable` writer"]
pub struct W(crate::W<ALARM0_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARM0_ENABLE_SPEC>;
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
impl From<crate::W<ALARM0_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARM0_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `alm_0_en` reader - Alarm 0 Enable"]
pub type ALM_0_EN_R = crate::BitReader<ALM_0_EN_A>;
#[doc = "Alarm 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALM_0_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ALM_0_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ALM_0_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALM_0_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALM_0_EN_A {
        match self.bits {
            false => ALM_0_EN_A::DISABLE,
            true => ALM_0_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ALM_0_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ALM_0_EN_A::ENABLE
    }
}
#[doc = "Field `alm_0_en` writer - Alarm 0 Enable"]
pub type ALM_0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALARM0_ENABLE_SPEC, ALM_0_EN_A, O>;
impl<'a, const O: u8> ALM_0_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ALM_0_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ALM_0_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Alarm 0 Enable"]
    #[inline(always)]
    pub fn alm_0_en(&self) -> ALM_0_EN_R {
        ALM_0_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alm_0_en(&mut self) -> ALM_0_EN_W<0> {
        ALM_0_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm 0 Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarm0_enable](index.html) module"]
pub struct ALARM0_ENABLE_SPEC;
impl crate::RegisterSpec for ALARM0_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarm0_enable::R](R) reader structure"]
impl crate::Readable for ALARM0_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarm0_enable::W](W) writer structure"]
impl crate::Writable for ALARM0_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets alarm0_enable to value 0"]
impl crate::Resettable for ALARM0_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
