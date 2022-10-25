#[doc = "Register `gp_cs_en` reader"]
pub struct R(crate::R<GP_CS_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_CS_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_CS_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_CS_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gp_cs_en` writer"]
pub struct W(crate::W<GP_CS_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_CS_EN_SPEC>;
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
impl From<crate::W<GP_CS_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_CS_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc_ch_select[0-1]` reader - Analog Input Channel Select"]
pub type ADC_CH_SELECT_R = crate::BitReader<ADC_CH_SELECT_A>;
#[doc = "Analog Input Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_CH_SELECT_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ADC_CH_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_CH_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_CH_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_CH_SELECT_A {
        match self.bits {
            false => ADC_CH_SELECT_A::DISABLE,
            true => ADC_CH_SELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC_CH_SELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC_CH_SELECT_A::ENABLE
    }
}
#[doc = "Field `adc_ch_select[0-1]` writer - Analog Input Channel Select"]
pub type ADC_CH_SELECT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GP_CS_EN_SPEC, ADC_CH_SELECT_A, O>;
impl<'a, const O: u8> ADC_CH_SELECT_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC_CH_SELECT_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC_CH_SELECT_A::ENABLE)
    }
}
#[doc = "Field `adc_ch_cmp_en[0-1]` reader - Channel Compare Enable"]
pub type ADC_CH_CMP_EN_R = crate::BitReader<ADC_CH_CMP_EN_A>;
#[doc = "Channel Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_CH_CMP_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ADC_CH_CMP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_CH_CMP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_CH_CMP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_CH_CMP_EN_A {
        match self.bits {
            false => ADC_CH_CMP_EN_A::DISABLE,
            true => ADC_CH_CMP_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC_CH_CMP_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC_CH_CMP_EN_A::ENABLE
    }
}
#[doc = "Field `adc_ch_cmp_en[0-1]` writer - Channel Compare Enable"]
pub type ADC_CH_CMP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GP_CS_EN_SPEC, ADC_CH_CMP_EN_A, O>;
impl<'a, const O: u8> ADC_CH_CMP_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC_CH_CMP_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC_CH_CMP_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Analog Input Channel Select"]
    #[inline(always)]
    pub unsafe fn adc_ch_select(&self, n: u8) -> ADC_CH_SELECT_R {
        ADC_CH_SELECT_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Analog Input Channel Select"]
    #[inline(always)]
    pub fn adc_ch0_select(&self) -> ADC_CH_SELECT_R {
        ADC_CH_SELECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Input Channel Select"]
    #[inline(always)]
    pub fn adc_ch1_select(&self) -> ADC_CH_SELECT_R {
        ADC_CH_SELECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Channel Compare Enable"]
    #[inline(always)]
    pub unsafe fn adc_ch_cmp_en(&self, n: u8) -> ADC_CH_CMP_EN_R {
        ADC_CH_CMP_EN_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel Compare Enable"]
    #[inline(always)]
    pub fn adc_ch0_cmp_en(&self) -> ADC_CH_CMP_EN_R {
        ADC_CH_CMP_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel Compare Enable"]
    #[inline(always)]
    pub fn adc_ch1_cmp_en(&self) -> ADC_CH_CMP_EN_R {
        ADC_CH_CMP_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Analog Input Channel Select"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn adc_ch_select<const O: u8>(&mut self) -> ADC_CH_SELECT_W<O> {
        ADC_CH_SELECT_W::new(self)
    }
    #[doc = "Bit 0 - Analog Input Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ch0_select(&mut self) -> ADC_CH_SELECT_W<0> {
        ADC_CH_SELECT_W::new(self)
    }
    #[doc = "Bit 1 - Analog Input Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ch1_select(&mut self) -> ADC_CH_SELECT_W<1> {
        ADC_CH_SELECT_W::new(self)
    }
    #[doc = "Channel Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn adc_ch_cmp_en<const O: u8>(&mut self) -> ADC_CH_CMP_EN_W<O> {
        ADC_CH_CMP_EN_W::new(self)
    }
    #[doc = "Bit 16 - Channel Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ch0_cmp_en(&mut self) -> ADC_CH_CMP_EN_W<16> {
        ADC_CH_CMP_EN_W::new(self)
    }
    #[doc = "Bit 17 - Channel Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ch1_cmp_en(&mut self) -> ADC_CH_CMP_EN_W<17> {
        ADC_CH_CMP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPADC Compare and Select Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_cs_en](index.html) module"]
pub struct GP_CS_EN_SPEC;
impl crate::RegisterSpec for GP_CS_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp_cs_en::R](R) reader structure"]
impl crate::Readable for GP_CS_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_cs_en::W](W) writer structure"]
impl crate::Writable for GP_CS_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gp_cs_en to value 0"]
impl crate::Resettable for GP_CS_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
