#[doc = "Register `prs_ch%s_infmt` reader"]
pub struct R(crate::R<PRS_CH_INFMT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRS_CH_INFMT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRS_CH_INFMT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRS_CH_INFMT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `prs_ch%s_infmt` writer"]
pub struct W(crate::W<PRS_CH_INFMT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRS_CH_INFMT_SPEC>;
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
impl From<crate::W<PRS_CH_INFMT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRS_CH_INFMT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `input_fmt` reader - input data format"]
pub type INPUT_FMT_R = crate::FieldReader<u8, INPUT_FMT_A>;
#[doc = "input data format\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUT_FMT_A {
    #[doc = "0: RAW stream"]
    RAW = 0,
    #[doc = "3: YUV422"]
    YUV422 = 3,
    #[doc = "4: YUV420"]
    YUV420 = 4,
}
impl From<INPUT_FMT_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT_FMT_A) -> Self {
        variant as _
    }
}
impl INPUT_FMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPUT_FMT_A> {
        match self.bits {
            0 => Some(INPUT_FMT_A::RAW),
            3 => Some(INPUT_FMT_A::YUV422),
            4 => Some(INPUT_FMT_A::YUV420),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RAW`"]
    #[inline(always)]
    pub fn is_raw(&self) -> bool {
        *self == INPUT_FMT_A::RAW
    }
    #[doc = "Checks if the value of the field is `YUV422`"]
    #[inline(always)]
    pub fn is_yuv422(&self) -> bool {
        *self == INPUT_FMT_A::YUV422
    }
    #[doc = "Checks if the value of the field is `YUV420`"]
    #[inline(always)]
    pub fn is_yuv420(&self) -> bool {
        *self == INPUT_FMT_A::YUV420
    }
}
#[doc = "Field `input_fmt` writer - input data format"]
pub type INPUT_FMT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRS_CH_INFMT_SPEC, u8, INPUT_FMT_A, 4, O>;
impl<'a, const O: u8> INPUT_FMT_W<'a, O> {
    #[doc = "RAW stream"]
    #[inline(always)]
    pub fn raw(self) -> &'a mut W {
        self.variant(INPUT_FMT_A::RAW)
    }
    #[doc = "YUV422"]
    #[inline(always)]
    pub fn yuv422(self) -> &'a mut W {
        self.variant(INPUT_FMT_A::YUV422)
    }
    #[doc = "YUV420"]
    #[inline(always)]
    pub fn yuv420(self) -> &'a mut W {
        self.variant(INPUT_FMT_A::YUV420)
    }
}
impl R {
    #[doc = "Bits 0:3 - input data format"]
    #[inline(always)]
    pub fn input_fmt(&self) -> INPUT_FMT_R {
        INPUT_FMT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - input data format"]
    #[inline(always)]
    #[must_use]
    pub fn input_fmt(&mut self) -> INPUT_FMT_W<0> {
        INPUT_FMT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parser Channel\\[i\\] Input Format Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prs_ch_infmt](index.html) module"]
pub struct PRS_CH_INFMT_SPEC;
impl crate::RegisterSpec for PRS_CH_INFMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prs_ch_infmt::R](R) reader structure"]
impl crate::Readable for PRS_CH_INFMT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prs_ch_infmt::W](W) writer structure"]
impl crate::Writable for PRS_CH_INFMT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets prs_ch%s_infmt to value 0x03"]
impl crate::Resettable for PRS_CH_INFMT_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
