#[doc = "Register `tv_gctl` reader"]
pub struct R(crate::R<TV_GCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_GCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_GCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_GCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_gctl` writer"]
pub struct W(crate::W<TV_GCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_GCTL_SPEC>;
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
impl From<crate::W<TV_GCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_GCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cec_ddc_pad_sel` reader - CEC DDC PAD Select"]
pub type CEC_DDC_PAD_SEL_R = crate::BitReader<CEC_DDC_PAD_SEL_A>;
#[doc = "CEC DDC PAD Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEC_DDC_PAD_SEL_A {
    #[doc = "1: TCON_TV internal pad for cec scl sal"]
    INTERNAL_PAD = 1,
    #[doc = "0: GPIO pad for cec scl sal"]
    GPIO_PAD = 0,
}
impl From<CEC_DDC_PAD_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CEC_DDC_PAD_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CEC_DDC_PAD_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEC_DDC_PAD_SEL_A {
        match self.bits {
            true => CEC_DDC_PAD_SEL_A::INTERNAL_PAD,
            false => CEC_DDC_PAD_SEL_A::GPIO_PAD,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL_PAD`"]
    #[inline(always)]
    pub fn is_internal_pad(&self) -> bool {
        *self == CEC_DDC_PAD_SEL_A::INTERNAL_PAD
    }
    #[doc = "Checks if the value of the field is `GPIO_PAD`"]
    #[inline(always)]
    pub fn is_gpio_pad(&self) -> bool {
        *self == CEC_DDC_PAD_SEL_A::GPIO_PAD
    }
}
#[doc = "Field `cec_ddc_pad_sel` writer - CEC DDC PAD Select"]
pub type CEC_DDC_PAD_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TV_GCTL_SPEC, CEC_DDC_PAD_SEL_A, O>;
impl<'a, const O: u8> CEC_DDC_PAD_SEL_W<'a, O> {
    #[doc = "TCON_TV internal pad for cec scl sal"]
    #[inline(always)]
    pub fn internal_pad(self) -> &'a mut W {
        self.variant(CEC_DDC_PAD_SEL_A::INTERNAL_PAD)
    }
    #[doc = "GPIO pad for cec scl sal"]
    #[inline(always)]
    pub fn gpio_pad(self) -> &'a mut W {
        self.variant(CEC_DDC_PAD_SEL_A::GPIO_PAD)
    }
}
#[doc = "Field `tv_en` reader - When it is disabled, the module will be reset to idle state."]
pub type TV_EN_R = crate::BitReader<TV_EN_A>;
#[doc = "When it is disabled, the module will be reset to idle state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TV_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<TV_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TV_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TV_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TV_EN_A {
        match self.bits {
            false => TV_EN_A::DISABLE,
            true => TV_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TV_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TV_EN_A::ENABLE
    }
}
#[doc = "Field `tv_en` writer - When it is disabled, the module will be reset to idle state."]
pub type TV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TV_GCTL_SPEC, TV_EN_A, O>;
impl<'a, const O: u8> TV_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TV_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TV_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 1 - CEC DDC PAD Select"]
    #[inline(always)]
    pub fn cec_ddc_pad_sel(&self) -> CEC_DDC_PAD_SEL_R {
        CEC_DDC_PAD_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - When it is disabled, the module will be reset to idle state."]
    #[inline(always)]
    pub fn tv_en(&self) -> TV_EN_R {
        TV_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CEC DDC PAD Select"]
    #[inline(always)]
    #[must_use]
    pub fn cec_ddc_pad_sel(&mut self) -> CEC_DDC_PAD_SEL_W<1> {
        CEC_DDC_PAD_SEL_W::new(self)
    }
    #[doc = "Bit 31 - When it is disabled, the module will be reset to idle state."]
    #[inline(always)]
    #[must_use]
    pub fn tv_en(&mut self) -> TV_EN_W<31> {
        TV_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_gctl](index.html) module"]
pub struct TV_GCTL_SPEC;
impl crate::RegisterSpec for TV_GCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_gctl::R](R) reader structure"]
impl crate::Readable for TV_GCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_gctl::W](W) writer structure"]
impl crate::Writable for TV_GCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_gctl to value 0"]
impl crate::Resettable for TV_GCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
