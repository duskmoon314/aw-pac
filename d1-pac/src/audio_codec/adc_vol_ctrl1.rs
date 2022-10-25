#[doc = "Register `adc_vol_ctrl1` reader"]
pub struct R(crate::R<ADC_VOL_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_VOL_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_VOL_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_VOL_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adc_vol_ctrl1` writer"]
pub struct W(crate::W<ADC_VOL_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_VOL_CTRL1_SPEC>;
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
impl From<crate::W<ADC_VOL_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_VOL_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc_vol[1-3]` reader - ADC\\[i\\] channel volume (-119.25 dB To 71.25 dB, 0.75 dB/Step)"]
pub type ADC_VOL_R = crate::FieldReader<u8, ADC_VOL_A>;
#[doc = "ADC\\[i\\] channel volume (-119.25 dB To 71.25 dB, 0.75 dB/Step)\n\nValue on reset: 160"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_VOL_A {
    #[doc = "0: Mute"]
    M_UTE = 0,
    #[doc = "1: -119.25 dB ..."]
    N_119_25DB = 1,
    #[doc = "160: 0 dB"]
    _0DB = 160,
    #[doc = "255: 71.25 dB"]
    _71_25DB = 255,
}
impl From<ADC_VOL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_VOL_A) -> Self {
        variant as _
    }
}
impl ADC_VOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_VOL_A> {
        match self.bits {
            0 => Some(ADC_VOL_A::M_UTE),
            1 => Some(ADC_VOL_A::N_119_25DB),
            160 => Some(ADC_VOL_A::_0DB),
            255 => Some(ADC_VOL_A::_71_25DB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `M_UTE`"]
    #[inline(always)]
    pub fn is_m_ute(&self) -> bool {
        *self == ADC_VOL_A::M_UTE
    }
    #[doc = "Checks if the value of the field is `N_119_25DB`"]
    #[inline(always)]
    pub fn is_n_119_25db(&self) -> bool {
        *self == ADC_VOL_A::N_119_25DB
    }
    #[doc = "Checks if the value of the field is `_0DB`"]
    #[inline(always)]
    pub fn is_0db(&self) -> bool {
        *self == ADC_VOL_A::_0DB
    }
    #[doc = "Checks if the value of the field is `_71_25DB`"]
    #[inline(always)]
    pub fn is_71_25db(&self) -> bool {
        *self == ADC_VOL_A::_71_25DB
    }
}
#[doc = "Field `adc_vol[1-3]` writer - ADC\\[i\\] channel volume (-119.25 dB To 71.25 dB, 0.75 dB/Step)"]
pub type ADC_VOL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC_VOL_CTRL1_SPEC, u8, ADC_VOL_A, 8, O>;
impl<'a, const O: u8> ADC_VOL_W<'a, O> {
    #[doc = "Mute"]
    #[inline(always)]
    pub fn m_ute(self) -> &'a mut W {
        self.variant(ADC_VOL_A::M_UTE)
    }
    #[doc = "-119.25 dB ..."]
    #[inline(always)]
    pub fn n_119_25db(self) -> &'a mut W {
        self.variant(ADC_VOL_A::N_119_25DB)
    }
    #[doc = "0 dB"]
    #[inline(always)]
    pub fn _0db(self) -> &'a mut W {
        self.variant(ADC_VOL_A::_0DB)
    }
    #[doc = "71.25 dB"]
    #[inline(always)]
    pub fn _71_25db(self) -> &'a mut W {
        self.variant(ADC_VOL_A::_71_25DB)
    }
}
impl R {
    #[doc = "ADC\\[i\\] channel volume (-119.25 dB To 71.25 dB, 0.75 dB/Step)"]
    #[inline(always)]
    pub unsafe fn adc_vol(&self, n: u8) -> ADC_VOL_R {
        ADC_VOL_R::new(((self.bits >> ((n - 1) * 8)) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - ADC\\[i\\] channel volume (-119.25 dB To 71.25 dB, 0.75 dB/Step)"]
    #[inline(always)]
    pub fn adc1_vol(&self) -> ADC_VOL_R {
        ADC_VOL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ADC\\[i\\] channel volume (-119.25 dB To 71.25 dB, 0.75 dB/Step)"]
    #[inline(always)]
    pub fn adc2_vol(&self) -> ADC_VOL_R {
        ADC_VOL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ADC\\[i\\] channel volume (-119.25 dB To 71.25 dB, 0.75 dB/Step)"]
    #[inline(always)]
    pub fn adc3_vol(&self) -> ADC_VOL_R {
        ADC_VOL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "ADC\\[i\\] channel volume (-119.25 dB To 71.25 dB, 0.75 dB/Step)"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn adc_vol<const O: u8>(&mut self) -> ADC_VOL_W<O> {
        ADC_VOL_W::new(self)
    }
    #[doc = "Bits 0:7 - ADC\\[i\\] channel volume (-119.25 dB To 71.25 dB, 0.75 dB/Step)"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_vol(&mut self) -> ADC_VOL_W<0> {
        ADC_VOL_W::new(self)
    }
    #[doc = "Bits 8:15 - ADC\\[i\\] channel volume (-119.25 dB To 71.25 dB, 0.75 dB/Step)"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_vol(&mut self) -> ADC_VOL_W<8> {
        ADC_VOL_W::new(self)
    }
    #[doc = "Bits 16:23 - ADC\\[i\\] channel volume (-119.25 dB To 71.25 dB, 0.75 dB/Step)"]
    #[inline(always)]
    #[must_use]
    pub fn adc3_vol(&mut self) -> ADC_VOL_W<16> {
        ADC_VOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Volume Control1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_vol_ctrl1](index.html) module"]
pub struct ADC_VOL_CTRL1_SPEC;
impl crate::RegisterSpec for ADC_VOL_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_vol_ctrl1::R](R) reader structure"]
impl crate::Readable for ADC_VOL_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_vol_ctrl1::W](W) writer structure"]
impl crate::Writable for ADC_VOL_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets adc_vol_ctrl1 to value 0xa0a0_a0a0"]
impl crate::Resettable for ADC_VOL_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0xa0a0_a0a0;
}
