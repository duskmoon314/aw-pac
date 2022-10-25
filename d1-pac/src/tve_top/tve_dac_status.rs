#[doc = "Register `tve_dac_status` reader"]
pub struct R(crate::R<TVE_DAC_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_DAC_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_DAC_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_DAC_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_dac_status` writer"]
pub struct W(crate::W<TVE_DAC_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_DAC_STATUS_SPEC>;
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
impl From<crate::W<TVE_DAC_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_DAC_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dac_status` reader - "]
pub type DAC_STATUS_R = crate::FieldReader<u8, DAC_STATUS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC_STATUS_A {
    #[doc = "0: Unconnected"]
    U_NCONNECTED = 0,
    #[doc = "1: Connected"]
    C_ONNECTED = 1,
    #[doc = "3: Short to ground"]
    S_HORT = 3,
}
impl From<DAC_STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC_STATUS_A) -> Self {
        variant as _
    }
}
impl DAC_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAC_STATUS_A> {
        match self.bits {
            0 => Some(DAC_STATUS_A::U_NCONNECTED),
            1 => Some(DAC_STATUS_A::C_ONNECTED),
            3 => Some(DAC_STATUS_A::S_HORT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `U_NCONNECTED`"]
    #[inline(always)]
    pub fn is_u_nconnected(&self) -> bool {
        *self == DAC_STATUS_A::U_NCONNECTED
    }
    #[doc = "Checks if the value of the field is `C_ONNECTED`"]
    #[inline(always)]
    pub fn is_c_onnected(&self) -> bool {
        *self == DAC_STATUS_A::C_ONNECTED
    }
    #[doc = "Checks if the value of the field is `S_HORT`"]
    #[inline(always)]
    pub fn is_s_hort(&self) -> bool {
        *self == DAC_STATUS_A::S_HORT
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dac_status(&self) -> DAC_STATUS_R {
        DAC_STATUS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder DAC STAUTS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_dac_status](index.html) module"]
pub struct TVE_DAC_STATUS_SPEC;
impl crate::RegisterSpec for TVE_DAC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_dac_status::R](R) reader structure"]
impl crate::Readable for TVE_DAC_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_dac_status::W](W) writer structure"]
impl crate::Writable for TVE_DAC_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_dac_status to value 0"]
impl crate::Resettable for TVE_DAC_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
