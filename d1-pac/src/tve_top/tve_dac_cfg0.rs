#[doc = "Register `tve_dac_cfg0` reader"]
pub struct R(crate::R<TVE_DAC_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_DAC_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_DAC_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_DAC_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_dac_cfg0` writer"]
pub struct W(crate::W<TVE_DAC_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_DAC_CFG0_SPEC>;
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
impl From<crate::W<TVE_DAC_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_DAC_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dac_en` reader - "]
pub type DAC_EN_R = crate::BitReader<DAC_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<DAC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DAC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC_EN_A {
        match self.bits {
            false => DAC_EN_A::DISABLE,
            true => DAC_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DAC_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DAC_EN_A::ENABLE
    }
}
#[doc = "Field `dac_en` writer - "]
pub type DAC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TVE_DAC_CFG0_SPEC, DAC_EN_A, O>;
impl<'a, const O: u8> DAC_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DAC_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DAC_EN_A::ENABLE)
    }
}
#[doc = "Field `bias_ref_int_en` reader - (A_EN_RESREF)"]
pub type BIAS_REF_INT_EN_R = crate::BitReader<BIAS_REF_INT_EN_A>;
#[doc = "(A_EN_RESREF)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIAS_REF_INT_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<BIAS_REF_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BIAS_REF_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BIAS_REF_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIAS_REF_INT_EN_A {
        match self.bits {
            false => BIAS_REF_INT_EN_A::DISABLE,
            true => BIAS_REF_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BIAS_REF_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BIAS_REF_INT_EN_A::ENABLE
    }
}
#[doc = "Field `bias_ref_int_en` writer - (A_EN_RESREF)"]
pub type BIAS_REF_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_DAC_CFG0_SPEC, BIAS_REF_INT_EN_A, O>;
impl<'a, const O: u8> BIAS_REF_INT_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BIAS_REF_INT_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BIAS_REF_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `bias_int_sel` reader - (A_SEL_BIAS_RES)"]
pub type BIAS_INT_SEL_R = crate::BitReader<BIAS_INT_SEL_A>;
#[doc = "(A_SEL_BIAS_RES)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIAS_INT_SEL_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<BIAS_INT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: BIAS_INT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl BIAS_INT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIAS_INT_SEL_A {
        match self.bits {
            false => BIAS_INT_SEL_A::DISABLE,
            true => BIAS_INT_SEL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BIAS_INT_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BIAS_INT_SEL_A::ENABLE
    }
}
#[doc = "Field `bias_int_sel` writer - (A_SEL_BIAS_RES)"]
pub type BIAS_INT_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_DAC_CFG0_SPEC, BIAS_INT_SEL_A, O>;
impl<'a, const O: u8> BIAS_INT_SEL_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BIAS_INT_SEL_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BIAS_INT_SEL_A::ENABLE)
    }
}
#[doc = "Field `bias_ext_sel` reader - (A_SEL_BIAS_ADDA)"]
pub type BIAS_EXT_SEL_R = crate::BitReader<BIAS_EXT_SEL_A>;
#[doc = "(A_SEL_BIAS_ADDA)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIAS_EXT_SEL_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<BIAS_EXT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: BIAS_EXT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl BIAS_EXT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIAS_EXT_SEL_A {
        match self.bits {
            false => BIAS_EXT_SEL_A::DISABLE,
            true => BIAS_EXT_SEL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BIAS_EXT_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BIAS_EXT_SEL_A::ENABLE
    }
}
#[doc = "Field `bias_ext_sel` writer - (A_SEL_BIAS_ADDA)"]
pub type BIAS_EXT_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_DAC_CFG0_SPEC, BIAS_EXT_SEL_A, O>;
impl<'a, const O: u8> BIAS_EXT_SEL_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BIAS_EXT_SEL_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BIAS_EXT_SEL_A::ENABLE)
    }
}
#[doc = "Field `low_bias` reader - 500 uA to 4 mA"]
pub type LOW_BIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `low_bias` writer - 500 uA to 4 mA"]
pub type LOW_BIAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TVE_DAC_CFG0_SPEC, u8, u8, 4, O>;
#[doc = "Field `cali_in` reader - "]
pub type CALI_IN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cali_in` writer - "]
pub type CALI_IN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_DAC_CFG0_SPEC, u16, u16, 10, O>;
#[doc = "Field `dac_clock_invert` reader - "]
pub type DAC_CLOCK_INVERT_R = crate::BitReader<DAC_CLOCK_INVERT_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC_CLOCK_INVERT_A {
    #[doc = "0: Not invert"]
    NOT_INVERT = 0,
    #[doc = "1: Invert"]
    INVERT = 1,
}
impl From<DAC_CLOCK_INVERT_A> for bool {
    #[inline(always)]
    fn from(variant: DAC_CLOCK_INVERT_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC_CLOCK_INVERT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC_CLOCK_INVERT_A {
        match self.bits {
            false => DAC_CLOCK_INVERT_A::NOT_INVERT,
            true => DAC_CLOCK_INVERT_A::INVERT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERT`"]
    #[inline(always)]
    pub fn is_not_invert(&self) -> bool {
        *self == DAC_CLOCK_INVERT_A::NOT_INVERT
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == DAC_CLOCK_INVERT_A::INVERT
    }
}
#[doc = "Field `dac_clock_invert` writer - "]
pub type DAC_CLOCK_INVERT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_DAC_CFG0_SPEC, DAC_CLOCK_INVERT_A, O>;
impl<'a, const O: u8> DAC_CLOCK_INVERT_W<'a, O> {
    #[doc = "Not invert"]
    #[inline(always)]
    pub fn not_invert(self) -> &'a mut W {
        self.variant(DAC_CLOCK_INVERT_A::NOT_INVERT)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(DAC_CLOCK_INVERT_A::INVERT)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dac_en(&self) -> DAC_EN_R {
        DAC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - (A_EN_RESREF)"]
    #[inline(always)]
    pub fn bias_ref_int_en(&self) -> BIAS_REF_INT_EN_R {
        BIAS_REF_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - (A_SEL_BIAS_RES)"]
    #[inline(always)]
    pub fn bias_int_sel(&self) -> BIAS_INT_SEL_R {
        BIAS_INT_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - (A_SEL_BIAS_ADDA)"]
    #[inline(always)]
    pub fn bias_ext_sel(&self) -> BIAS_EXT_SEL_R {
        BIAS_EXT_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 500 uA to 4 mA"]
    #[inline(always)]
    pub fn low_bias(&self) -> LOW_BIAS_R {
        LOW_BIAS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn cali_in(&self) -> CALI_IN_R {
        CALI_IN_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dac_clock_invert(&self) -> DAC_CLOCK_INVERT_R {
        DAC_CLOCK_INVERT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dac_en(&mut self) -> DAC_EN_W<0> {
        DAC_EN_W::new(self)
    }
    #[doc = "Bit 4 - (A_EN_RESREF)"]
    #[inline(always)]
    #[must_use]
    pub fn bias_ref_int_en(&mut self) -> BIAS_REF_INT_EN_W<4> {
        BIAS_REF_INT_EN_W::new(self)
    }
    #[doc = "Bit 8 - (A_SEL_BIAS_RES)"]
    #[inline(always)]
    #[must_use]
    pub fn bias_int_sel(&mut self) -> BIAS_INT_SEL_W<8> {
        BIAS_INT_SEL_W::new(self)
    }
    #[doc = "Bit 9 - (A_SEL_BIAS_ADDA)"]
    #[inline(always)]
    #[must_use]
    pub fn bias_ext_sel(&mut self) -> BIAS_EXT_SEL_W<9> {
        BIAS_EXT_SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - 500 uA to 4 mA"]
    #[inline(always)]
    #[must_use]
    pub fn low_bias(&mut self) -> LOW_BIAS_W<12> {
        LOW_BIAS_W::new(self)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    #[must_use]
    pub fn cali_in(&mut self) -> CALI_IN_W<16> {
        CALI_IN_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn dac_clock_invert(&mut self) -> DAC_CLOCK_INVERT_W<31> {
        DAC_CLOCK_INVERT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder DAC CFG0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_dac_cfg0](index.html) module"]
pub struct TVE_DAC_CFG0_SPEC;
impl crate::RegisterSpec for TVE_DAC_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_dac_cfg0::R](R) reader structure"]
impl crate::Readable for TVE_DAC_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_dac_cfg0::W](W) writer structure"]
impl crate::Writable for TVE_DAC_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_dac_cfg0 to value 0x8000_4200"]
impl crate::Resettable for TVE_DAC_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_4200;
}
