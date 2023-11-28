#[doc = "Register `prs_ch%s_infmt` reader"]
pub type R = crate::R<PRS_CH_INFMT_SPEC>;
#[doc = "Register `prs_ch%s_infmt` writer"]
pub type W = crate::W<PRS_CH_INFMT_SPEC>;
#[doc = "Field `input_fmt` reader - input data format"]
pub type INPUT_FMT_R = crate::FieldReader<INPUT_FMT_A>;
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
impl crate::FieldSpec for INPUT_FMT_A {
    type Ux = u8;
}
impl INPUT_FMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INPUT_FMT_A> {
        match self.bits {
            0 => Some(INPUT_FMT_A::RAW),
            3 => Some(INPUT_FMT_A::YUV422),
            4 => Some(INPUT_FMT_A::YUV420),
            _ => None,
        }
    }
    #[doc = "RAW stream"]
    #[inline(always)]
    pub fn is_raw(&self) -> bool {
        *self == INPUT_FMT_A::RAW
    }
    #[doc = "YUV422"]
    #[inline(always)]
    pub fn is_yuv422(&self) -> bool {
        *self == INPUT_FMT_A::YUV422
    }
    #[doc = "YUV420"]
    #[inline(always)]
    pub fn is_yuv420(&self) -> bool {
        *self == INPUT_FMT_A::YUV420
    }
}
#[doc = "Field `input_fmt` writer - input data format"]
pub type INPUT_FMT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, INPUT_FMT_A>;
impl<'a, REG> INPUT_FMT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RAW stream"]
    #[inline(always)]
    pub fn raw(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT_FMT_A::RAW)
    }
    #[doc = "YUV422"]
    #[inline(always)]
    pub fn yuv422(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT_FMT_A::YUV422)
    }
    #[doc = "YUV420"]
    #[inline(always)]
    pub fn yuv420(self) -> &'a mut crate::W<REG> {
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
    pub fn input_fmt(&mut self) -> INPUT_FMT_W<PRS_CH_INFMT_SPEC> {
        INPUT_FMT_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Parser Channel\\[i\\] Input Format Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch_infmt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prs_ch_infmt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRS_CH_INFMT_SPEC;
impl crate::RegisterSpec for PRS_CH_INFMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prs_ch_infmt::R`](R) reader structure"]
impl crate::Readable for PRS_CH_INFMT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prs_ch_infmt::W`](W) writer structure"]
impl crate::Writable for PRS_CH_INFMT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets prs_ch%s_infmt to value 0x03"]
impl crate::Resettable for PRS_CH_INFMT_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
