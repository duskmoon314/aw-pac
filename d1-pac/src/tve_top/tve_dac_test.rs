#[doc = "Register `tve_dac_test` reader"]
pub struct R(crate::R<TVE_DAC_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_DAC_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_DAC_TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_DAC_TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_dac_test` writer"]
pub struct W(crate::W<TVE_DAC_TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_DAC_TEST_SPEC>;
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
impl From<crate::W<TVE_DAC_TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_DAC_TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dac_test_enable` reader - "]
pub type DAC_TEST_ENABLE_R = crate::BitReader<DAC_TEST_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC_TEST_ENABLE_A {
    #[doc = "1: Repeat DAC data from DAC sram"]
    R_EPEAT = 1,
}
impl From<DAC_TEST_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DAC_TEST_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC_TEST_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAC_TEST_ENABLE_A> {
        match self.bits {
            true => Some(DAC_TEST_ENABLE_A::R_EPEAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `R_EPEAT`"]
    #[inline(always)]
    pub fn is_r_epeat(&self) -> bool {
        *self == DAC_TEST_ENABLE_A::R_EPEAT
    }
}
#[doc = "Field `dac_test_enable` writer - "]
pub type DAC_TEST_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_DAC_TEST_SPEC, DAC_TEST_ENABLE_A, O>;
impl<'a, const O: u8> DAC_TEST_ENABLE_W<'a, O> {
    #[doc = "Repeat DAC data from DAC sram"]
    #[inline(always)]
    pub fn r_epeat(self) -> &'a mut W {
        self.variant(DAC_TEST_ENABLE_A::R_EPEAT)
    }
}
#[doc = "Field `dac_test_sel` reader - "]
pub type DAC_TEST_SEL_R = crate::FieldReader<u8, DAC_TEST_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC_TEST_SEL_A {
    #[doc = "0: DAC0"]
    DAC0 = 0,
}
impl From<DAC_TEST_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC_TEST_SEL_A) -> Self {
        variant as _
    }
}
impl DAC_TEST_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAC_TEST_SEL_A> {
        match self.bits {
            0 => Some(DAC_TEST_SEL_A::DAC0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DAC0`"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        *self == DAC_TEST_SEL_A::DAC0
    }
}
#[doc = "Field `dac_test_sel` writer - "]
pub type DAC_TEST_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_DAC_TEST_SPEC, u8, DAC_TEST_SEL_A, 2, O>;
impl<'a, const O: u8> DAC_TEST_SEL_W<'a, O> {
    #[doc = "DAC0"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut W {
        self.variant(DAC_TEST_SEL_A::DAC0)
    }
}
#[doc = "Field `dac_test_length` reader - DAC TEST DATA LENGTH"]
pub type DAC_TEST_LENGTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `dac_test_length` writer - DAC TEST DATA LENGTH"]
pub type DAC_TEST_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_DAC_TEST_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dac_test_enable(&self) -> DAC_TEST_ENABLE_R {
        DAC_TEST_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dac_test_sel(&self) -> DAC_TEST_SEL_R {
        DAC_TEST_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:25 - DAC TEST DATA LENGTH"]
    #[inline(always)]
    pub fn dac_test_length(&self) -> DAC_TEST_LENGTH_R {
        DAC_TEST_LENGTH_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dac_test_enable(&mut self) -> DAC_TEST_ENABLE_W<0> {
        DAC_TEST_ENABLE_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn dac_test_sel(&mut self) -> DAC_TEST_SEL_W<4> {
        DAC_TEST_SEL_W::new(self)
    }
    #[doc = "Bits 16:25 - DAC TEST DATA LENGTH"]
    #[inline(always)]
    #[must_use]
    pub fn dac_test_length(&mut self) -> DAC_TEST_LENGTH_W<16> {
        DAC_TEST_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder DAC TEST Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_dac_test](index.html) module"]
pub struct TVE_DAC_TEST_SPEC;
impl crate::RegisterSpec for TVE_DAC_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_dac_test::R](R) reader structure"]
impl crate::Readable for TVE_DAC_TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_dac_test::W](W) writer structure"]
impl crate::Writable for TVE_DAC_TEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_dac_test to value 0"]
impl crate::Resettable for TVE_DAC_TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
