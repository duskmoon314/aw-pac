#[doc = "Register `prs_ch%s_input_para3` reader"]
pub struct R(crate::R<PRS_CH_INPUT_PARA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRS_CH_INPUT_PARA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRS_CH_INPUT_PARA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRS_CH_INPUT_PARA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `input_x` reader - "]
pub type INPUT_X_R = crate::FieldReader<u16, u16>;
#[doc = "Field `input_y` reader - "]
pub type INPUT_Y_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn input_x(&self) -> INPUT_X_R {
        INPUT_X_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29"]
    #[inline(always)]
    pub fn input_y(&self) -> INPUT_Y_R {
        INPUT_Y_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "Parser Channel\\[i\\] Input Parameter3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prs_ch_input_para3](index.html) module"]
pub struct PRS_CH_INPUT_PARA3_SPEC;
impl crate::RegisterSpec for PRS_CH_INPUT_PARA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prs_ch_input_para3::R](R) reader structure"]
impl crate::Readable for PRS_CH_INPUT_PARA3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets prs_ch%s_input_para3 to value 0"]
impl crate::Resettable for PRS_CH_INPUT_PARA3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
