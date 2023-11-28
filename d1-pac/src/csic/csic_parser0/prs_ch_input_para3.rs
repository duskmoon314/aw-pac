#[doc = "Register `prs_ch%s_input_para3` reader"]
pub type R = crate::R<PRS_CH_INPUT_PARA3_SPEC>;
#[doc = "Field `input_x` reader - "]
pub type INPUT_X_R = crate::FieldReader<u16>;
#[doc = "Field `input_y` reader - "]
pub type INPUT_Y_R = crate::FieldReader<u16>;
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
#[doc = "Parser Channel\\[i\\] Input Parameter3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch_input_para3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRS_CH_INPUT_PARA3_SPEC;
impl crate::RegisterSpec for PRS_CH_INPUT_PARA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prs_ch_input_para3::R`](R) reader structure"]
impl crate::Readable for PRS_CH_INPUT_PARA3_SPEC {}
#[doc = "`reset()` method sets prs_ch%s_input_para3 to value 0"]
impl crate::Resettable for PRS_CH_INPUT_PARA3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
