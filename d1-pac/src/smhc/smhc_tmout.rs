#[doc = "Register `SMHC_TMOUT` reader"]
pub struct R(crate::R<SMHC_TMOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_TMOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_TMOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_TMOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_TMOUT` writer"]
pub struct W(crate::W<SMHC_TMOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_TMOUT_SPEC>;
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
impl From<crate::W<SMHC_TMOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_TMOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTO_LMT` reader - Data Iimeout Limit"]
pub struct DTO_LMT_R(crate::FieldReader<u32>);
impl DTO_LMT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DTO_LMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTO_LMT_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTO_LMT` writer - Data Iimeout Limit"]
pub struct DTO_LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> DTO_LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Field `RTO_LMT` reader - Response Timeout Limit"]
pub struct RTO_LMT_R(crate::FieldReader<u8>);
impl RTO_LMT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTO_LMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTO_LMT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTO_LMT` writer - Response Timeout Limit"]
pub struct RTO_LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTO_LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - Data Iimeout Limit"]
    #[inline(always)]
    pub fn dto_lmt(&self) -> DTO_LMT_R {
        DTO_LMT_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - Response Timeout Limit"]
    #[inline(always)]
    pub fn rto_lmt(&self) -> RTO_LMT_R {
        RTO_LMT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - Data Iimeout Limit"]
    #[inline(always)]
    pub fn dto_lmt(&mut self) -> DTO_LMT_W {
        DTO_LMT_W { w: self }
    }
    #[doc = "Bits 0:7 - Response Timeout Limit"]
    #[inline(always)]
    pub fn rto_lmt(&mut self) -> RTO_LMT_W {
        RTO_LMT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_tmout](index.html) module"]
pub struct SMHC_TMOUT_SPEC;
impl crate::RegisterSpec for SMHC_TMOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_tmout::R](R) reader structure"]
impl crate::Readable for SMHC_TMOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_tmout::W](W) writer structure"]
impl crate::Writable for SMHC_TMOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_TMOUT to value 0"]
impl crate::Resettable for SMHC_TMOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
