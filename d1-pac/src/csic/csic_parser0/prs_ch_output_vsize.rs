#[doc = "Register `prs_ch%s_output_vsize` reader"]
pub type R = crate::R<PRS_CH_OUTPUT_VSIZE_SPEC>;
#[doc = "Register `prs_ch%s_output_vsize` writer"]
pub type W = crate::W<PRS_CH_OUTPUT_VSIZE_SPEC>;
#[doc = "Field `ver_start` reader - Vertical line start. Data is valid from this line."]
pub type VER_START_R = crate::FieldReader<u16>;
#[doc = "Field `ver_start` writer - Vertical line start. Data is valid from this line."]
pub type VER_START_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `ver_len` reader - Valid line number of a frame."]
pub type VER_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `ver_len` writer - Valid line number of a frame."]
pub type VER_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Vertical line start. Data is valid from this line."]
    #[inline(always)]
    pub fn ver_start(&self) -> VER_START_R {
        VER_START_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Valid line number of a frame."]
    #[inline(always)]
    pub fn ver_len(&self) -> VER_LEN_R {
        VER_LEN_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Vertical line start. Data is valid from this line."]
    #[inline(always)]
    #[must_use]
    pub fn ver_start(&mut self) -> VER_START_W<PRS_CH_OUTPUT_VSIZE_SPEC> {
        VER_START_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Valid line number of a frame."]
    #[inline(always)]
    #[must_use]
    pub fn ver_len(&mut self) -> VER_LEN_W<PRS_CH_OUTPUT_VSIZE_SPEC> {
        VER_LEN_W::new(self, 16)
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
#[doc = "Parser Channel\\[i\\] Output Vertical Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch_output_vsize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prs_ch_output_vsize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRS_CH_OUTPUT_VSIZE_SPEC;
impl crate::RegisterSpec for PRS_CH_OUTPUT_VSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prs_ch_output_vsize::R`](R) reader structure"]
impl crate::Readable for PRS_CH_OUTPUT_VSIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prs_ch_output_vsize::W`](W) writer structure"]
impl crate::Writable for PRS_CH_OUTPUT_VSIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets prs_ch%s_output_vsize to value 0x02d0_0000"]
impl crate::Resettable for PRS_CH_OUTPUT_VSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x02d0_0000;
}
