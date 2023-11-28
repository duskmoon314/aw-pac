#[doc = "Register `prs_ch%s_output_hsize` reader"]
pub type R = crate::R<PRS_CH_OUTPUT_HSIZE_SPEC>;
#[doc = "Register `prs_ch%s_output_hsize` writer"]
pub type W = crate::W<PRS_CH_OUTPUT_HSIZE_SPEC>;
#[doc = "Field `hor_start` reader - Horizontal pixel unit start. Pixel is valid from this pixel."]
pub type HOR_START_R = crate::FieldReader<u16>;
#[doc = "Field `hor_start` writer - Horizontal pixel unit start. Pixel is valid from this pixel."]
pub type HOR_START_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `hor_len` reader - Horizontal pixel unit length. Valid pixel of a line."]
pub type HOR_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `hor_len` writer - Horizontal pixel unit length. Valid pixel of a line."]
pub type HOR_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Horizontal pixel unit start. Pixel is valid from this pixel."]
    #[inline(always)]
    pub fn hor_start(&self) -> HOR_START_R {
        HOR_START_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Horizontal pixel unit length. Valid pixel of a line."]
    #[inline(always)]
    pub fn hor_len(&self) -> HOR_LEN_R {
        HOR_LEN_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Horizontal pixel unit start. Pixel is valid from this pixel."]
    #[inline(always)]
    #[must_use]
    pub fn hor_start(&mut self) -> HOR_START_W<PRS_CH_OUTPUT_HSIZE_SPEC> {
        HOR_START_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Horizontal pixel unit length. Valid pixel of a line."]
    #[inline(always)]
    #[must_use]
    pub fn hor_len(&mut self) -> HOR_LEN_W<PRS_CH_OUTPUT_HSIZE_SPEC> {
        HOR_LEN_W::new(self, 16)
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
#[doc = "Parser Channel\\[i\\] Output Horizontal Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch_output_hsize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prs_ch_output_hsize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRS_CH_OUTPUT_HSIZE_SPEC;
impl crate::RegisterSpec for PRS_CH_OUTPUT_HSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prs_ch_output_hsize::R`](R) reader structure"]
impl crate::Readable for PRS_CH_OUTPUT_HSIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prs_ch_output_hsize::W`](W) writer structure"]
impl crate::Writable for PRS_CH_OUTPUT_HSIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets prs_ch%s_output_hsize to value 0x0500_0000"]
impl crate::Resettable for PRS_CH_OUTPUT_HSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0500_0000;
}
