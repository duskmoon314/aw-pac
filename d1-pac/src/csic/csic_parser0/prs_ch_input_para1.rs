#[doc = "Register `prs_ch%s_input_para1` reader"]
pub type R = crate::R<PRS_CH_INPUT_PARA1_SPEC>;
#[doc = "Field `input_ht` reader - INPUT_HT = INPUT_HB + INPUT_X"]
pub type INPUT_HT_R = crate::FieldReader<u16>;
#[doc = "Field `input_vt` reader - INPUT_VT = INPUT_VB + INPUT_Y"]
pub type INPUT_VT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - INPUT_HT = INPUT_HB + INPUT_X"]
    #[inline(always)]
    pub fn input_ht(&self) -> INPUT_HT_R {
        INPUT_HT_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - INPUT_VT = INPUT_VB + INPUT_Y"]
    #[inline(always)]
    pub fn input_vt(&self) -> INPUT_VT_R {
        INPUT_VT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "Parser Channel\\[i\\] Input Parameter1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch_input_para1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRS_CH_INPUT_PARA1_SPEC;
impl crate::RegisterSpec for PRS_CH_INPUT_PARA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prs_ch_input_para1::R`](R) reader structure"]
impl crate::Readable for PRS_CH_INPUT_PARA1_SPEC {}
#[doc = "`reset()` method sets prs_ch%s_input_para1 to value 0"]
impl crate::Resettable for PRS_CH_INPUT_PARA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
