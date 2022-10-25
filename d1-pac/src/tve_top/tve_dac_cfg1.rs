#[doc = "Register `tve_dac_cfg1` reader"]
pub struct R(crate::R<TVE_DAC_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_DAC_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_DAC_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_DAC_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_dac_cfg1` writer"]
pub struct W(crate::W<TVE_DAC_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_DAC_CFG1_SPEC>;
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
impl From<crate::W<TVE_DAC_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_DAC_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ref1_sel` reader - (a_refslct1\\[3:0\\])\n\nThe reference voltage is used for hot plug detect function."]
pub type REF1_SEL_R = crate::FieldReader<u8, REF1_SEL_A>;
#[doc = "(a_refslct1\\[3:0\\])\n\nThe reference voltage is used for hot plug detect function.\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REF1_SEL_A {
    #[doc = "0: 0.50 V"]
    _0_50_V = 0,
    #[doc = "1: 0.55 V"]
    _0_55_V = 1,
    #[doc = "2: 0.60 V"]
    _0_60_V = 2,
    #[doc = "3: 0.65 V"]
    _0_65_V = 3,
    #[doc = "4: 0.70 V"]
    _0_70_V = 4,
    #[doc = "5: 0.75 V"]
    _0_75_V = 5,
    #[doc = "6: 0.80 V"]
    _0_80_V = 6,
    #[doc = "7: 0.85 V"]
    _0_85_V = 7,
    #[doc = "8: 0.90 V"]
    _0_90_V = 8,
    #[doc = "9: 0.95 V"]
    _0_95_V = 9,
    #[doc = "10: 1.00 V"]
    _1_00_V = 10,
    #[doc = "11: 1.05 V"]
    _1_05_V = 11,
    #[doc = "12: 1.10 V"]
    _1_10_V = 12,
    #[doc = "13: 1.15 V"]
    _1_15_V = 13,
    #[doc = "14: 1.20 V"]
    _1_20_V = 14,
    #[doc = "15: 1.25 V"]
    _1_25_V = 15,
}
impl From<REF1_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REF1_SEL_A) -> Self {
        variant as _
    }
}
impl REF1_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REF1_SEL_A {
        match self.bits {
            0 => REF1_SEL_A::_0_50_V,
            1 => REF1_SEL_A::_0_55_V,
            2 => REF1_SEL_A::_0_60_V,
            3 => REF1_SEL_A::_0_65_V,
            4 => REF1_SEL_A::_0_70_V,
            5 => REF1_SEL_A::_0_75_V,
            6 => REF1_SEL_A::_0_80_V,
            7 => REF1_SEL_A::_0_85_V,
            8 => REF1_SEL_A::_0_90_V,
            9 => REF1_SEL_A::_0_95_V,
            10 => REF1_SEL_A::_1_00_V,
            11 => REF1_SEL_A::_1_05_V,
            12 => REF1_SEL_A::_1_10_V,
            13 => REF1_SEL_A::_1_15_V,
            14 => REF1_SEL_A::_1_20_V,
            15 => REF1_SEL_A::_1_25_V,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0_50_V`"]
    #[inline(always)]
    pub fn is_0_50_v(&self) -> bool {
        *self == REF1_SEL_A::_0_50_V
    }
    #[doc = "Checks if the value of the field is `_0_55_V`"]
    #[inline(always)]
    pub fn is_0_55_v(&self) -> bool {
        *self == REF1_SEL_A::_0_55_V
    }
    #[doc = "Checks if the value of the field is `_0_60_V`"]
    #[inline(always)]
    pub fn is_0_60_v(&self) -> bool {
        *self == REF1_SEL_A::_0_60_V
    }
    #[doc = "Checks if the value of the field is `_0_65_V`"]
    #[inline(always)]
    pub fn is_0_65_v(&self) -> bool {
        *self == REF1_SEL_A::_0_65_V
    }
    #[doc = "Checks if the value of the field is `_0_70_V`"]
    #[inline(always)]
    pub fn is_0_70_v(&self) -> bool {
        *self == REF1_SEL_A::_0_70_V
    }
    #[doc = "Checks if the value of the field is `_0_75_V`"]
    #[inline(always)]
    pub fn is_0_75_v(&self) -> bool {
        *self == REF1_SEL_A::_0_75_V
    }
    #[doc = "Checks if the value of the field is `_0_80_V`"]
    #[inline(always)]
    pub fn is_0_80_v(&self) -> bool {
        *self == REF1_SEL_A::_0_80_V
    }
    #[doc = "Checks if the value of the field is `_0_85_V`"]
    #[inline(always)]
    pub fn is_0_85_v(&self) -> bool {
        *self == REF1_SEL_A::_0_85_V
    }
    #[doc = "Checks if the value of the field is `_0_90_V`"]
    #[inline(always)]
    pub fn is_0_90_v(&self) -> bool {
        *self == REF1_SEL_A::_0_90_V
    }
    #[doc = "Checks if the value of the field is `_0_95_V`"]
    #[inline(always)]
    pub fn is_0_95_v(&self) -> bool {
        *self == REF1_SEL_A::_0_95_V
    }
    #[doc = "Checks if the value of the field is `_1_00_V`"]
    #[inline(always)]
    pub fn is_1_00_v(&self) -> bool {
        *self == REF1_SEL_A::_1_00_V
    }
    #[doc = "Checks if the value of the field is `_1_05_V`"]
    #[inline(always)]
    pub fn is_1_05_v(&self) -> bool {
        *self == REF1_SEL_A::_1_05_V
    }
    #[doc = "Checks if the value of the field is `_1_10_V`"]
    #[inline(always)]
    pub fn is_1_10_v(&self) -> bool {
        *self == REF1_SEL_A::_1_10_V
    }
    #[doc = "Checks if the value of the field is `_1_15_V`"]
    #[inline(always)]
    pub fn is_1_15_v(&self) -> bool {
        *self == REF1_SEL_A::_1_15_V
    }
    #[doc = "Checks if the value of the field is `_1_20_V`"]
    #[inline(always)]
    pub fn is_1_20_v(&self) -> bool {
        *self == REF1_SEL_A::_1_20_V
    }
    #[doc = "Checks if the value of the field is `_1_25_V`"]
    #[inline(always)]
    pub fn is_1_25_v(&self) -> bool {
        *self == REF1_SEL_A::_1_25_V
    }
}
#[doc = "Field `ref1_sel` writer - (a_refslct1\\[3:0\\])\n\nThe reference voltage is used for hot plug detect function."]
pub type REF1_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TVE_DAC_CFG1_SPEC, u8, REF1_SEL_A, 4, O>;
impl<'a, const O: u8> REF1_SEL_W<'a, O> {
    #[doc = "0.50 V"]
    #[inline(always)]
    pub fn _0_50_v(self) -> &'a mut W {
        self.variant(REF1_SEL_A::_0_50_V)
    }
    #[doc = "0.55 V"]
    #[inline(always)]
    pub fn _0_55_v(self) -> &'a mut W {
        self.variant(REF1_SEL_A::_0_55_V)
    }
    #[doc = "0.60 V"]
    #[inline(always)]
    pub fn _0_60_v(self) -> &'a mut W {
        self.variant(REF1_SEL_A::_0_60_V)
    }
    #[doc = "0.65 V"]
    #[inline(always)]
    pub fn _0_65_v(self) -> &'a mut W {
        self.variant(REF1_SEL_A::_0_65_V)
    }
    #[doc = "0.70 V"]
    #[inline(always)]
    pub fn _0_70_v(self) -> &'a mut W {
        self.variant(REF1_SEL_A::_0_70_V)
    }
    #[doc = "0.75 V"]
    #[inline(always)]
    pub fn _0_75_v(self) -> &'a mut W {
        self.variant(REF1_SEL_A::_0_75_V)
    }
    #[doc = "0.80 V"]
    #[inline(always)]
    pub fn _0_80_v(self) -> &'a mut W {
        self.variant(REF1_SEL_A::_0_80_V)
    }
    #[doc = "0.85 V"]
    #[inline(always)]
    pub fn _0_85_v(self) -> &'a mut W {
        self.variant(REF1_SEL_A::_0_85_V)
    }
    #[doc = "0.90 V"]
    #[inline(always)]
    pub fn _0_90_v(self) -> &'a mut W {
        self.variant(REF1_SEL_A::_0_90_V)
    }
    #[doc = "0.95 V"]
    #[inline(always)]
    pub fn _0_95_v(self) -> &'a mut W {
        self.variant(REF1_SEL_A::_0_95_V)
    }
    #[doc = "1.00 V"]
    #[inline(always)]
    pub fn _1_00_v(self) -> &'a mut W {
        self.variant(REF1_SEL_A::_1_00_V)
    }
    #[doc = "1.05 V"]
    #[inline(always)]
    pub fn _1_05_v(self) -> &'a mut W {
        self.variant(REF1_SEL_A::_1_05_V)
    }
    #[doc = "1.10 V"]
    #[inline(always)]
    pub fn _1_10_v(self) -> &'a mut W {
        self.variant(REF1_SEL_A::_1_10_V)
    }
    #[doc = "1.15 V"]
    #[inline(always)]
    pub fn _1_15_v(self) -> &'a mut W {
        self.variant(REF1_SEL_A::_1_15_V)
    }
    #[doc = "1.20 V"]
    #[inline(always)]
    pub fn _1_20_v(self) -> &'a mut W {
        self.variant(REF1_SEL_A::_1_20_V)
    }
    #[doc = "1.25 V"]
    #[inline(always)]
    pub fn _1_25_v(self) -> &'a mut W {
        self.variant(REF1_SEL_A::_1_25_V)
    }
}
#[doc = "Field `ref2_sel` reader - (a_refslct2\\[1:0\\])"]
pub type REF2_SEL_R = crate::FieldReader<u8, REF2_SEL_A>;
#[doc = "(a_refslct2\\[1:0\\])\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REF2_SEL_A {
    #[doc = "0: 0.25 V"]
    _0_25_V = 0,
    #[doc = "1: 0.30 V"]
    _0_30_V = 1,
    #[doc = "2: 0.35 V"]
    _0_35_V = 2,
    #[doc = "3: 0.40 V"]
    _0_40_V = 3,
}
impl From<REF2_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REF2_SEL_A) -> Self {
        variant as _
    }
}
impl REF2_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REF2_SEL_A {
        match self.bits {
            0 => REF2_SEL_A::_0_25_V,
            1 => REF2_SEL_A::_0_30_V,
            2 => REF2_SEL_A::_0_35_V,
            3 => REF2_SEL_A::_0_40_V,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0_25_V`"]
    #[inline(always)]
    pub fn is_0_25_v(&self) -> bool {
        *self == REF2_SEL_A::_0_25_V
    }
    #[doc = "Checks if the value of the field is `_0_30_V`"]
    #[inline(always)]
    pub fn is_0_30_v(&self) -> bool {
        *self == REF2_SEL_A::_0_30_V
    }
    #[doc = "Checks if the value of the field is `_0_35_V`"]
    #[inline(always)]
    pub fn is_0_35_v(&self) -> bool {
        *self == REF2_SEL_A::_0_35_V
    }
    #[doc = "Checks if the value of the field is `_0_40_V`"]
    #[inline(always)]
    pub fn is_0_40_v(&self) -> bool {
        *self == REF2_SEL_A::_0_40_V
    }
}
#[doc = "Field `ref2_sel` writer - (a_refslct2\\[1:0\\])"]
pub type REF2_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TVE_DAC_CFG1_SPEC, u8, REF2_SEL_A, 2, O>;
impl<'a, const O: u8> REF2_SEL_W<'a, O> {
    #[doc = "0.25 V"]
    #[inline(always)]
    pub fn _0_25_v(self) -> &'a mut W {
        self.variant(REF2_SEL_A::_0_25_V)
    }
    #[doc = "0.30 V"]
    #[inline(always)]
    pub fn _0_30_v(self) -> &'a mut W {
        self.variant(REF2_SEL_A::_0_30_V)
    }
    #[doc = "0.35 V"]
    #[inline(always)]
    pub fn _0_35_v(self) -> &'a mut W {
        self.variant(REF2_SEL_A::_0_35_V)
    }
    #[doc = "0.40 V"]
    #[inline(always)]
    pub fn _0_40_v(self) -> &'a mut W {
        self.variant(REF2_SEL_A::_0_40_V)
    }
}
#[doc = "Field `ref_int_sel` reader - (A_SEL_DETREF_RES)"]
pub type REF_INT_SEL_R = crate::BitReader<REF_INT_SEL_A>;
#[doc = "(A_SEL_DETREF_RES)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REF_INT_SEL_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<REF_INT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: REF_INT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl REF_INT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REF_INT_SEL_A {
        match self.bits {
            false => REF_INT_SEL_A::DISABLE,
            true => REF_INT_SEL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == REF_INT_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == REF_INT_SEL_A::ENABLE
    }
}
#[doc = "Field `ref_int_sel` writer - (A_SEL_DETREF_RES)"]
pub type REF_INT_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_DAC_CFG1_SPEC, REF_INT_SEL_A, O>;
impl<'a, const O: u8> REF_INT_SEL_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(REF_INT_SEL_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(REF_INT_SEL_A::ENABLE)
    }
}
#[doc = "Field `ref_ext_sel` reader - (A_SEL_DETREF_LDO)"]
pub type REF_EXT_SEL_R = crate::BitReader<REF_EXT_SEL_A>;
#[doc = "(A_SEL_DETREF_LDO)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REF_EXT_SEL_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<REF_EXT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: REF_EXT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl REF_EXT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REF_EXT_SEL_A {
        match self.bits {
            false => REF_EXT_SEL_A::DISABLE,
            true => REF_EXT_SEL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == REF_EXT_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == REF_EXT_SEL_A::ENABLE
    }
}
#[doc = "Field `ref_ext_sel` writer - (A_SEL_DETREF_LDO)"]
pub type REF_EXT_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_DAC_CFG1_SPEC, REF_EXT_SEL_A, O>;
impl<'a, const O: u8> REF_EXT_SEL_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(REF_EXT_SEL_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(REF_EXT_SEL_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - (a_refslct1\\[3:0\\])\n\nThe reference voltage is used for hot plug detect function."]
    #[inline(always)]
    pub fn ref1_sel(&self) -> REF1_SEL_R {
        REF1_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - (a_refslct2\\[1:0\\])"]
    #[inline(always)]
    pub fn ref2_sel(&self) -> REF2_SEL_R {
        REF2_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - (A_SEL_DETREF_RES)"]
    #[inline(always)]
    pub fn ref_int_sel(&self) -> REF_INT_SEL_R {
        REF_INT_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - (A_SEL_DETREF_LDO)"]
    #[inline(always)]
    pub fn ref_ext_sel(&self) -> REF_EXT_SEL_R {
        REF_EXT_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - (a_refslct1\\[3:0\\])\n\nThe reference voltage is used for hot plug detect function."]
    #[inline(always)]
    #[must_use]
    pub fn ref1_sel(&mut self) -> REF1_SEL_W<0> {
        REF1_SEL_W::new(self)
    }
    #[doc = "Bits 4:5 - (a_refslct2\\[1:0\\])"]
    #[inline(always)]
    #[must_use]
    pub fn ref2_sel(&mut self) -> REF2_SEL_W<4> {
        REF2_SEL_W::new(self)
    }
    #[doc = "Bit 8 - (A_SEL_DETREF_RES)"]
    #[inline(always)]
    #[must_use]
    pub fn ref_int_sel(&mut self) -> REF_INT_SEL_W<8> {
        REF_INT_SEL_W::new(self)
    }
    #[doc = "Bit 9 - (A_SEL_DETREF_LDO)"]
    #[inline(always)]
    #[must_use]
    pub fn ref_ext_sel(&mut self) -> REF_EXT_SEL_W<9> {
        REF_EXT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder DAC CFG1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_dac_cfg1](index.html) module"]
pub struct TVE_DAC_CFG1_SPEC;
impl crate::RegisterSpec for TVE_DAC_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_dac_cfg1::R](R) reader structure"]
impl crate::Readable for TVE_DAC_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_dac_cfg1::W](W) writer structure"]
impl crate::Writable for TVE_DAC_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_dac_cfg1 to value 0x023a"]
impl crate::Resettable for TVE_DAC_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x023a;
}
