#[doc = "Register `ac_adc_dg` reader"]
pub struct R(crate::R<AC_ADC_DG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_ADC_DG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_ADC_DG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_ADC_DG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_adc_dg` writer"]
pub struct W(crate::W<AC_ADC_DG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_ADC_DG_SPEC>;
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
impl From<crate::W<AC_ADC_DG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_ADC_DG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ad_swp1` reader - ADC output channel swap enable (for digital filter)"]
pub type AD_SWP1_R = crate::BitReader<AD_SWP1_A>;
#[doc = "ADC output channel swap enable (for digital filter)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD_SWP1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<AD_SWP1_A> for bool {
    #[inline(always)]
    fn from(variant: AD_SWP1_A) -> Self {
        variant as u8 != 0
    }
}
impl AD_SWP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD_SWP1_A {
        match self.bits {
            false => AD_SWP1_A::DISABLED,
            true => AD_SWP1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD_SWP1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD_SWP1_A::ENABLED
    }
}
#[doc = "Field `ad_swp1` writer - ADC output channel swap enable (for digital filter)"]
pub type AD_SWP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_ADC_DG_SPEC, AD_SWP1_A, O>;
impl<'a, const O: u8> AD_SWP1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD_SWP1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD_SWP1_A::ENABLED)
    }
}
#[doc = "Field `ad_swp2` reader - ADC output channel swap enable (for digital filter)"]
pub type AD_SWP2_R = crate::BitReader<AD_SWP2_A>;
#[doc = "ADC output channel swap enable (for digital filter)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD_SWP2_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<AD_SWP2_A> for bool {
    #[inline(always)]
    fn from(variant: AD_SWP2_A) -> Self {
        variant as u8 != 0
    }
}
impl AD_SWP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD_SWP2_A {
        match self.bits {
            false => AD_SWP2_A::DISABLED,
            true => AD_SWP2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD_SWP2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD_SWP2_A::ENABLED
    }
}
#[doc = "Field `ad_swp2` writer - ADC output channel swap enable (for digital filter)"]
pub type AD_SWP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_ADC_DG_SPEC, AD_SWP2_A, O>;
impl<'a, const O: u8> AD_SWP2_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD_SWP2_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD_SWP2_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 24 - ADC output channel swap enable (for digital filter)"]
    #[inline(always)]
    pub fn ad_swp1(&self) -> AD_SWP1_R {
        AD_SWP1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADC output channel swap enable (for digital filter)"]
    #[inline(always)]
    pub fn ad_swp2(&self) -> AD_SWP2_R {
        AD_SWP2_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - ADC output channel swap enable (for digital filter)"]
    #[inline(always)]
    #[must_use]
    pub fn ad_swp1(&mut self) -> AD_SWP1_W<24> {
        AD_SWP1_W::new(self)
    }
    #[doc = "Bit 25 - ADC output channel swap enable (for digital filter)"]
    #[inline(always)]
    #[must_use]
    pub fn ad_swp2(&mut self) -> AD_SWP2_W<25> {
        AD_SWP2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Debug Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_adc_dg](index.html) module"]
pub struct AC_ADC_DG_SPEC;
impl crate::RegisterSpec for AC_ADC_DG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_adc_dg::R](R) reader structure"]
impl crate::Readable for AC_ADC_DG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_adc_dg::W](W) writer structure"]
impl crate::Writable for AC_ADC_DG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_adc_dg to value 0"]
impl crate::Resettable for AC_ADC_DG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
