#[doc = "Register `ac_dac_dpc` reader"]
pub struct R(crate::R<AC_DAC_DPC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_DAC_DPC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_DAC_DPC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_DAC_DPC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_dac_dpc` writer"]
pub struct W(crate::W<AC_DAC_DPC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_DAC_DPC_SPEC>;
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
impl From<crate::W<AC_DAC_DPC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_DAC_DPC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hub_en` reader - Audio Hub Enable\n\nThe bit takes effect only when the EN_DA is set to 1.\n\nSystem Domain: Audio Codec/I2S0/I2S1/I2S2/OWA TXFIFO Hub Enable."]
pub type HUB_EN_R = crate::BitReader<HUB_EN_A>;
#[doc = "Audio Hub Enable\n\nThe bit takes effect only when the EN_DA is set to 1.\n\nSystem Domain: Audio Codec/I2S0/I2S1/I2S2/OWA TXFIFO Hub Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HUB_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<HUB_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HUB_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HUB_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HUB_EN_A {
        match self.bits {
            false => HUB_EN_A::DISABLE,
            true => HUB_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HUB_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HUB_EN_A::ENABLE
    }
}
#[doc = "Field `hub_en` writer - Audio Hub Enable\n\nThe bit takes effect only when the EN_DA is set to 1.\n\nSystem Domain: Audio Codec/I2S0/I2S1/I2S2/OWA TXFIFO Hub Enable."]
pub type HUB_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_DAC_DPC_SPEC, HUB_EN_A, O>;
impl<'a, const O: u8> HUB_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HUB_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HUB_EN_A::ENABLE)
    }
}
#[doc = "Field `dvol` reader - Digital Volume Control: DVC\n\nATT = DVC\\[5:0\\] * (-1.16 dB)\n\n64 steps, -1.16 dB/step"]
pub type DVOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dvol` writer - Digital Volume Control: DVC\n\nATT = DVC\\[5:0\\] * (-1.16 dB)\n\n64 steps, -1.16 dB/step"]
pub type DVOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AC_DAC_DPC_SPEC, u8, u8, 6, O>;
#[doc = "Field `hpf_en` reader - High Pass Filter Enable"]
pub type HPF_EN_R = crate::BitReader<HPF_EN_A>;
#[doc = "High Pass Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPF_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<HPF_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HPF_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HPF_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPF_EN_A {
        match self.bits {
            false => HPF_EN_A::DISABLE,
            true => HPF_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HPF_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HPF_EN_A::ENABLE
    }
}
#[doc = "Field `hpf_en` writer - High Pass Filter Enable"]
pub type HPF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_DAC_DPC_SPEC, HPF_EN_A, O>;
impl<'a, const O: u8> HPF_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HPF_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HPF_EN_A::ENABLE)
    }
}
#[doc = "Field `dwa` reader - DWA Function Disable"]
pub type DWA_R = crate::BitReader<DWA_A>;
#[doc = "DWA Function Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DWA_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<DWA_A> for bool {
    #[inline(always)]
    fn from(variant: DWA_A) -> Self {
        variant as u8 != 0
    }
}
impl DWA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DWA_A {
        match self.bits {
            false => DWA_A::DISABLE,
            true => DWA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DWA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DWA_A::ENABLE
    }
}
#[doc = "Field `dwa` writer - DWA Function Disable"]
pub type DWA_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_DAC_DPC_SPEC, DWA_A, O>;
impl<'a, const O: u8> DWA_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DWA_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DWA_A::ENABLE)
    }
}
#[doc = "Field `modqu` reader - Internal DAC Quantization Levels.\n\nLevels = \\[7*(21 + MODQU\\[3:0\\])\\]/128\n\nDefault levels = 7*21/128 = 1.15"]
pub type MODQU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `modqu` writer - Internal DAC Quantization Levels.\n\nLevels = \\[7*(21 + MODQU\\[3:0\\])\\]/128\n\nDefault levels = 7*21/128 = 1.15"]
pub type MODQU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AC_DAC_DPC_SPEC, u8, u8, 4, O>;
#[doc = "Field `en_da` reader - DAC Digital Part Enable"]
pub type EN_DA_R = crate::BitReader<EN_DA_A>;
#[doc = "DAC Digital Part Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_DA_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<EN_DA_A> for bool {
    #[inline(always)]
    fn from(variant: EN_DA_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_DA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_DA_A {
        match self.bits {
            false => EN_DA_A::DISABLE,
            true => EN_DA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EN_DA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EN_DA_A::ENABLE
    }
}
#[doc = "Field `en_da` writer - DAC Digital Part Enable"]
pub type EN_DA_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_DAC_DPC_SPEC, EN_DA_A, O>;
impl<'a, const O: u8> EN_DA_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EN_DA_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EN_DA_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Audio Hub Enable\n\nThe bit takes effect only when the EN_DA is set to 1.\n\nSystem Domain: Audio Codec/I2S0/I2S1/I2S2/OWA TXFIFO Hub Enable."]
    #[inline(always)]
    pub fn hub_en(&self) -> HUB_EN_R {
        HUB_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 12:17 - Digital Volume Control: DVC\n\nATT = DVC\\[5:0\\] * (-1.16 dB)\n\n64 steps, -1.16 dB/step"]
    #[inline(always)]
    pub fn dvol(&self) -> DVOL_R {
        DVOL_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 18 - High Pass Filter Enable"]
    #[inline(always)]
    pub fn hpf_en(&self) -> HPF_EN_R {
        HPF_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - DWA Function Disable"]
    #[inline(always)]
    pub fn dwa(&self) -> DWA_R {
        DWA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - Internal DAC Quantization Levels.\n\nLevels = \\[7*(21 + MODQU\\[3:0\\])\\]/128\n\nDefault levels = 7*21/128 = 1.15"]
    #[inline(always)]
    pub fn modqu(&self) -> MODQU_R {
        MODQU_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - DAC Digital Part Enable"]
    #[inline(always)]
    pub fn en_da(&self) -> EN_DA_R {
        EN_DA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Audio Hub Enable\n\nThe bit takes effect only when the EN_DA is set to 1.\n\nSystem Domain: Audio Codec/I2S0/I2S1/I2S2/OWA TXFIFO Hub Enable."]
    #[inline(always)]
    #[must_use]
    pub fn hub_en(&mut self) -> HUB_EN_W<0> {
        HUB_EN_W::new(self)
    }
    #[doc = "Bits 12:17 - Digital Volume Control: DVC\n\nATT = DVC\\[5:0\\] * (-1.16 dB)\n\n64 steps, -1.16 dB/step"]
    #[inline(always)]
    #[must_use]
    pub fn dvol(&mut self) -> DVOL_W<12> {
        DVOL_W::new(self)
    }
    #[doc = "Bit 18 - High Pass Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hpf_en(&mut self) -> HPF_EN_W<18> {
        HPF_EN_W::new(self)
    }
    #[doc = "Bit 24 - DWA Function Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dwa(&mut self) -> DWA_W<24> {
        DWA_W::new(self)
    }
    #[doc = "Bits 25:28 - Internal DAC Quantization Levels.\n\nLevels = \\[7*(21 + MODQU\\[3:0\\])\\]/128\n\nDefault levels = 7*21/128 = 1.15"]
    #[inline(always)]
    #[must_use]
    pub fn modqu(&mut self) -> MODQU_W<25> {
        MODQU_W::new(self)
    }
    #[doc = "Bit 31 - DAC Digital Part Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en_da(&mut self) -> EN_DA_W<31> {
        EN_DA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Digital Part Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_dac_dpc](index.html) module"]
pub struct AC_DAC_DPC_SPEC;
impl crate::RegisterSpec for AC_DAC_DPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_dac_dpc::R](R) reader structure"]
impl crate::Readable for AC_DAC_DPC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_dac_dpc::W](W) writer structure"]
impl crate::Writable for AC_DAC_DPC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_dpc to value 0"]
impl crate::Resettable for AC_DAC_DPC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
