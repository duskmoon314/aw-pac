#[doc = "Register `smhc_tmout` reader"]
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
#[doc = "Register `smhc_tmout` writer"]
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
#[doc = "Field `rto_lmt` reader - Response Timeout Limit"]
pub type RTO_LMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rto_lmt` writer - Response Timeout Limit"]
pub type RTO_LMT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMHC_TMOUT_SPEC, u8, u8, 8, O>;
#[doc = "Field `dto_lmt` reader - Data Iimeout Limit"]
pub type DTO_LMT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `dto_lmt` writer - Data Iimeout Limit"]
pub type DTO_LMT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMHC_TMOUT_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - Response Timeout Limit"]
    #[inline(always)]
    pub fn rto_lmt(&self) -> RTO_LMT_R {
        RTO_LMT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Data Iimeout Limit"]
    #[inline(always)]
    pub fn dto_lmt(&self) -> DTO_LMT_R {
        DTO_LMT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - Response Timeout Limit"]
    #[inline(always)]
    #[must_use]
    pub fn rto_lmt(&mut self) -> RTO_LMT_W<0> {
        RTO_LMT_W::new(self)
    }
    #[doc = "Bits 8:31 - Data Iimeout Limit"]
    #[inline(always)]
    #[must_use]
    pub fn dto_lmt(&mut self) -> DTO_LMT_W<8> {
        DTO_LMT_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_tmout to value 0"]
impl crate::Resettable for SMHC_TMOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
