#[doc = "Register `tv_fill_ctl` reader"]
pub struct R(crate::R<TV_FILL_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_FILL_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_FILL_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_FILL_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_fill_ctl` writer"]
pub struct W(crate::W<TV_FILL_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_FILL_CTL_SPEC>;
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
impl From<crate::W<TV_FILL_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_FILL_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tv_fill_en` reader - TV Fill Enable"]
pub type TV_FILL_EN_R = crate::BitReader<TV_FILL_EN_A>;
#[doc = "TV Fill Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TV_FILL_EN_A {
    #[doc = "0: Bypass"]
    BYPASS = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<TV_FILL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TV_FILL_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TV_FILL_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TV_FILL_EN_A {
        match self.bits {
            false => TV_FILL_EN_A::BYPASS,
            true => TV_FILL_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == TV_FILL_EN_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TV_FILL_EN_A::ENABLE
    }
}
#[doc = "Field `tv_fill_en` writer - TV Fill Enable"]
pub type TV_FILL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TV_FILL_CTL_SPEC, TV_FILL_EN_A, O>;
impl<'a, const O: u8> TV_FILL_EN_W<'a, O> {
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(TV_FILL_EN_A::BYPASS)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TV_FILL_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 31 - TV Fill Enable"]
    #[inline(always)]
    pub fn tv_fill_en(&self) -> TV_FILL_EN_R {
        TV_FILL_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - TV Fill Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tv_fill_en(&mut self) -> TV_FILL_EN_W<31> {
        TV_FILL_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Fill Data Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_fill_ctl](index.html) module"]
pub struct TV_FILL_CTL_SPEC;
impl crate::RegisterSpec for TV_FILL_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_fill_ctl::R](R) reader structure"]
impl crate::Readable for TV_FILL_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_fill_ctl::W](W) writer structure"]
impl crate::Writable for TV_FILL_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_fill_ctl to value 0"]
impl crate::Resettable for TV_FILL_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
