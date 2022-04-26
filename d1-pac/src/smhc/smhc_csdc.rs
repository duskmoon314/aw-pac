#[doc = "Register `SMHC_CSDC` reader"]
pub struct R(crate::R<SMHC_CSDC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_CSDC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_CSDC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_CSDC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_CSDC` writer"]
pub struct W(crate::W<SMHC_CSDC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_CSDC_SPEC>;
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
impl From<crate::W<SMHC_CSDC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_CSDC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRC_DET_PARA_A {
    #[doc = "6: `110`"]
    HS400 = 6,
    #[doc = "3: `11`"]
    OTHER = 3,
}
impl From<CRC_DET_PARA_A> for u8 {
    #[inline(always)]
    fn from(variant: CRC_DET_PARA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRC_DET_PARA` reader - "]
pub struct CRC_DET_PARA_R(crate::FieldReader<u8>);
impl CRC_DET_PARA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CRC_DET_PARA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRC_DET_PARA_A> {
        match self.bits {
            6 => Some(CRC_DET_PARA_A::HS400),
            3 => Some(CRC_DET_PARA_A::OTHER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HS400`"]
    #[inline(always)]
    pub fn is_hs400(&self) -> bool {
        **self == CRC_DET_PARA_A::HS400
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        **self == CRC_DET_PARA_A::OTHER
    }
}
impl core::ops::Deref for CRC_DET_PARA_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_DET_PARA` writer - "]
pub struct CRC_DET_PARA_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_DET_PARA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_DET_PARA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn hs400(self) -> &'a mut W {
        self.variant(CRC_DET_PARA_A::HS400)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn other(self) -> &'a mut W {
        self.variant(CRC_DET_PARA_A::OTHER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn crc_det_para(&self) -> CRC_DET_PARA_R {
        CRC_DET_PARA_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn crc_det_para(&mut self) -> CRC_DET_PARA_W {
        CRC_DET_PARA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Status Detect Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_csdc](index.html) module"]
pub struct SMHC_CSDC_SPEC;
impl crate::RegisterSpec for SMHC_CSDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_csdc::R](R) reader structure"]
impl crate::Readable for SMHC_CSDC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_csdc::W](W) writer structure"]
impl crate::Writable for SMHC_CSDC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_CSDC to value 0"]
impl crate::Resettable for SMHC_CSDC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
