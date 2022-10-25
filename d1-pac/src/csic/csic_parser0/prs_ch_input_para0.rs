#[doc = "Register `prs_ch%s_input_para0` reader"]
pub struct R(crate::R<PRS_CH_INPUT_PARA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRS_CH_INPUT_PARA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRS_CH_INPUT_PARA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRS_CH_INPUT_PARA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `input_src_type` reader - "]
pub type INPUT_SRC_TYPE_R = crate::BitReader<INPUT_SRC_TYPE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INPUT_SRC_TYPE_A {
    #[doc = "0: `0`"]
    PROGRESS = 0,
    #[doc = "1: `1`"]
    INTERLACE = 1,
}
impl From<INPUT_SRC_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: INPUT_SRC_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl INPUT_SRC_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT_SRC_TYPE_A {
        match self.bits {
            false => INPUT_SRC_TYPE_A::PROGRESS,
            true => INPUT_SRC_TYPE_A::INTERLACE,
        }
    }
    #[doc = "Checks if the value of the field is `PROGRESS`"]
    #[inline(always)]
    pub fn is_progress(&self) -> bool {
        *self == INPUT_SRC_TYPE_A::PROGRESS
    }
    #[doc = "Checks if the value of the field is `INTERLACE`"]
    #[inline(always)]
    pub fn is_interlace(&self) -> bool {
        *self == INPUT_SRC_TYPE_A::INTERLACE
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn input_src_type(&self) -> INPUT_SRC_TYPE_R {
        INPUT_SRC_TYPE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Parser Channel\\[i\\] Input Parameter0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prs_ch_input_para0](index.html) module"]
pub struct PRS_CH_INPUT_PARA0_SPEC;
impl crate::RegisterSpec for PRS_CH_INPUT_PARA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prs_ch_input_para0::R](R) reader structure"]
impl crate::Readable for PRS_CH_INPUT_PARA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets prs_ch%s_input_para0 to value 0"]
impl crate::Resettable for PRS_CH_INPUT_PARA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
