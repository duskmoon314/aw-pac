#[doc = "Register `tve_dac_map` reader"]
pub struct R(crate::R<TVE_DAC_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_DAC_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_DAC_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_DAC_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_dac_map` writer"]
pub struct W(crate::W<TVE_DAC_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_DAC_MAP_SPEC>;
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
impl From<crate::W<TVE_DAC_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_DAC_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dac_sel` reader - "]
pub type DAC_SEL_R = crate::FieldReader<u8, DAC_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC_SEL_A {
    #[doc = "1: TVE0"]
    TVE0 = 1,
}
impl From<DAC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC_SEL_A) -> Self {
        variant as _
    }
}
impl DAC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAC_SEL_A> {
        match self.bits {
            1 => Some(DAC_SEL_A::TVE0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TVE0`"]
    #[inline(always)]
    pub fn is_tve0(&self) -> bool {
        *self == DAC_SEL_A::TVE0
    }
}
#[doc = "Field `dac_sel` writer - "]
pub type DAC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_DAC_MAP_SPEC, u8, DAC_SEL_A, 2, O>;
impl<'a, const O: u8> DAC_SEL_W<'a, O> {
    #[doc = "TVE0"]
    #[inline(always)]
    pub fn tve0(self) -> &'a mut W {
        self.variant(DAC_SEL_A::TVE0)
    }
}
#[doc = "Field `dac_map` reader - "]
pub type DAC_MAP_R = crate::FieldReader<u8, DAC_MAP_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC_MAP_A {
    #[doc = "0: OUT0"]
    OUT0 = 0,
}
impl From<DAC_MAP_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC_MAP_A) -> Self {
        variant as _
    }
}
impl DAC_MAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAC_MAP_A> {
        match self.bits {
            0 => Some(DAC_MAP_A::OUT0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == DAC_MAP_A::OUT0
    }
}
#[doc = "Field `dac_map` writer - "]
pub type DAC_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_DAC_MAP_SPEC, u8, DAC_MAP_A, 3, O>;
impl<'a, const O: u8> DAC_MAP_W<'a, O> {
    #[doc = "OUT0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(DAC_MAP_A::OUT0)
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dac_sel(&self) -> DAC_SEL_R {
        DAC_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dac_map(&self) -> DAC_MAP_R {
        DAC_MAP_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn dac_sel(&mut self) -> DAC_SEL_W<0> {
        DAC_SEL_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn dac_map(&mut self) -> DAC_MAP_W<4> {
        DAC_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder DAC MAP Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_dac_map](index.html) module"]
pub struct TVE_DAC_MAP_SPEC;
impl crate::RegisterSpec for TVE_DAC_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_dac_map::R](R) reader structure"]
impl crate::Readable for TVE_DAC_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_dac_map::W](W) writer structure"]
impl crate::Writable for TVE_DAC_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_dac_map to value 0"]
impl crate::Resettable for TVE_DAC_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
