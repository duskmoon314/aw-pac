#[doc = "Register `adc_dig_ctrl` reader"]
pub struct R(crate::R<ADC_DIG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_DIG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_DIG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_DIG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adc_dig_ctrl` writer"]
pub struct W(crate::W<ADC_DIG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_DIG_CTRL_SPEC>;
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
impl From<crate::W<ADC_DIG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_DIG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc_channel_en` reader - Bit 3: ADC4 enabled\n\nBit 2: ADC3 enabled\n\nBit 1: ADC2 enabled\n\nBit 0: ADC1 enabled"]
pub type ADC_CHANNEL_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adc_channel_en` writer - Bit 3: ADC4 enabled\n\nBit 2: ADC3 enabled\n\nBit 1: ADC2 enabled\n\nBit 0: ADC1 enabled"]
pub type ADC_CHANNEL_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC_DIG_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `adc1_2_vol_en` reader - ADC1/2 Volume Control Enable"]
pub type ADC1_2_VOL_EN_R = crate::BitReader<ADC1_2_VOL_EN_A>;
#[doc = "ADC1/2 Volume Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC1_2_VOL_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC1_2_VOL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1_2_VOL_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC1_2_VOL_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC1_2_VOL_EN_A {
        match self.bits {
            false => ADC1_2_VOL_EN_A::DISABLED,
            true => ADC1_2_VOL_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC1_2_VOL_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC1_2_VOL_EN_A::ENABLED
    }
}
#[doc = "Field `adc1_2_vol_en` writer - ADC1/2 Volume Control Enable"]
pub type ADC1_2_VOL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC_DIG_CTRL_SPEC, ADC1_2_VOL_EN_A, O>;
impl<'a, const O: u8> ADC1_2_VOL_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC1_2_VOL_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC1_2_VOL_EN_A::ENABLED)
    }
}
#[doc = "Field `adc3_vol_en` reader - ADC3 Volume Control Enable"]
pub type ADC3_VOL_EN_R = crate::BitReader<ADC3_VOL_EN_A>;
#[doc = "ADC3 Volume Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC3_VOL_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC3_VOL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC3_VOL_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC3_VOL_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC3_VOL_EN_A {
        match self.bits {
            false => ADC3_VOL_EN_A::DISABLED,
            true => ADC3_VOL_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC3_VOL_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC3_VOL_EN_A::ENABLED
    }
}
#[doc = "Field `adc3_vol_en` writer - ADC3 Volume Control Enable"]
pub type ADC3_VOL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC_DIG_CTRL_SPEC, ADC3_VOL_EN_A, O>;
impl<'a, const O: u8> ADC3_VOL_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC3_VOL_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC3_VOL_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:2 - Bit 3: ADC4 enabled\n\nBit 2: ADC3 enabled\n\nBit 1: ADC2 enabled\n\nBit 0: ADC1 enabled"]
    #[inline(always)]
    pub fn adc_channel_en(&self) -> ADC_CHANNEL_EN_R {
        ADC_CHANNEL_EN_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 16 - ADC1/2 Volume Control Enable"]
    #[inline(always)]
    pub fn adc1_2_vol_en(&self) -> ADC1_2_VOL_EN_R {
        ADC1_2_VOL_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC3 Volume Control Enable"]
    #[inline(always)]
    pub fn adc3_vol_en(&self) -> ADC3_VOL_EN_R {
        ADC3_VOL_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bit 3: ADC4 enabled\n\nBit 2: ADC3 enabled\n\nBit 1: ADC2 enabled\n\nBit 0: ADC1 enabled"]
    #[inline(always)]
    #[must_use]
    pub fn adc_channel_en(&mut self) -> ADC_CHANNEL_EN_W<0> {
        ADC_CHANNEL_EN_W::new(self)
    }
    #[doc = "Bit 16 - ADC1/2 Volume Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_2_vol_en(&mut self) -> ADC1_2_VOL_EN_W<16> {
        ADC1_2_VOL_EN_W::new(self)
    }
    #[doc = "Bit 17 - ADC3 Volume Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc3_vol_en(&mut self) -> ADC3_VOL_EN_W<17> {
        ADC3_VOL_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Digtial Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dig_ctrl](index.html) module"]
pub struct ADC_DIG_CTRL_SPEC;
impl crate::RegisterSpec for ADC_DIG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_dig_ctrl::R](R) reader structure"]
impl crate::Readable for ADC_DIG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_dig_ctrl::W](W) writer structure"]
impl crate::Writable for ADC_DIG_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets adc_dig_ctrl to value 0"]
impl crate::Resettable for ADC_DIG_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
