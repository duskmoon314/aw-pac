#[doc = "Register `prs_ch%s_input_para2` reader"]
pub type R = crate::R<PRS_CH_INPUT_PARA2_SPEC>;
#[doc = "Field `input_hb` reader - "]
pub type INPUT_HB_R = crate::FieldReader<u16>;
#[doc = "Field `input_vb` reader - "]
pub type INPUT_VB_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn input_hb(&self) -> INPUT_HB_R {
        INPUT_HB_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29"]
    #[inline(always)]
    pub fn input_vb(&self) -> INPUT_VB_R {
        INPUT_VB_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "Parser Channel\\[i\\] Input Parameter2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch_input_para2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRS_CH_INPUT_PARA2_SPEC;
impl crate::RegisterSpec for PRS_CH_INPUT_PARA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prs_ch_input_para2::R`](R) reader structure"]
impl crate::Readable for PRS_CH_INPUT_PARA2_SPEC {}
#[doc = "`reset()` method sets prs_ch%s_input_para2 to value 0"]
impl crate::Resettable for PRS_CH_INPUT_PARA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
