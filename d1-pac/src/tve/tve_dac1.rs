#[doc = "Register `tve_dac1` reader"]
pub struct R(crate::R<TVE_DAC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_DAC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_DAC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_DAC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_dac1` writer"]
pub struct W(crate::W<TVE_DAC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_DAC1_SPEC>;
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
impl From<crate::W<TVE_DAC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_DAC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dac0_src_sel` reader - "]
pub type DAC0_SRC_SEL_R = crate::FieldReader<u8, DAC0_SRC_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC0_SRC_SEL_A {
    #[doc = "0: Composite"]
    C_OMPOSITE = 0,
}
impl From<DAC0_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC0_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl DAC0_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAC0_SRC_SEL_A> {
        match self.bits {
            0 => Some(DAC0_SRC_SEL_A::C_OMPOSITE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `C_OMPOSITE`"]
    #[inline(always)]
    pub fn is_c_omposite(&self) -> bool {
        *self == DAC0_SRC_SEL_A::C_OMPOSITE
    }
}
#[doc = "Field `dac0_src_sel` writer - "]
pub type DAC0_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_DAC1_SPEC, u8, DAC0_SRC_SEL_A, 3, O>;
impl<'a, const O: u8> DAC0_SRC_SEL_W<'a, O> {
    #[doc = "Composite"]
    #[inline(always)]
    pub fn c_omposite(self) -> &'a mut W {
        self.variant(DAC0_SRC_SEL_A::C_OMPOSITE)
    }
}
impl R {
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dac0_src_sel(&self) -> DAC0_SRC_SEL_R {
        DAC0_SRC_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn dac0_src_sel(&mut self) -> DAC0_SRC_SEL_W<4> {
        DAC0_SRC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder DAC Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_dac1](index.html) module"]
pub struct TVE_DAC1_SPEC;
impl crate::RegisterSpec for TVE_DAC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_dac1::R](R) reader structure"]
impl crate::Readable for TVE_DAC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_dac1::W](W) writer structure"]
impl crate::Writable for TVE_DAC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_dac1 to value 0"]
impl crate::Resettable for TVE_DAC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
