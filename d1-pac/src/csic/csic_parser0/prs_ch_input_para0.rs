#[doc = "Register `prs_ch%s_input_para0` reader"]
pub type R = crate::R<PRS_CH_INPUT_PARA0_SPEC>;
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
    pub const fn variant(&self) -> INPUT_SRC_TYPE_A {
        match self.bits {
            false => INPUT_SRC_TYPE_A::PROGRESS,
            true => INPUT_SRC_TYPE_A::INTERLACE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_progress(&self) -> bool {
        *self == INPUT_SRC_TYPE_A::PROGRESS
    }
    #[doc = "`1`"]
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
#[doc = "Parser Channel\\[i\\] Input Parameter0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch_input_para0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRS_CH_INPUT_PARA0_SPEC;
impl crate::RegisterSpec for PRS_CH_INPUT_PARA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prs_ch_input_para0::R`](R) reader structure"]
impl crate::Readable for PRS_CH_INPUT_PARA0_SPEC {}
#[doc = "`reset()` method sets prs_ch%s_input_para0 to value 0"]
impl crate::Resettable for PRS_CH_INPUT_PARA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
