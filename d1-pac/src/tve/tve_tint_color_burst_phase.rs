#[doc = "Register `tve_tint_color_burst_phase` reader"]
pub type R = crate::R<TVE_TINT_COLOR_BURST_PHASE_SPEC>;
#[doc = "Register `tve_tint_color_burst_phase` writer"]
pub type W = crate::W<TVE_TINT_COLOR_BURST_PHASE_SPEC>;
#[doc = "Field `chroma_phase` reader - Specify the color burst initial phase (ChromaPhase). 8.8 bit unsigned fraction. Units are cycles of the color burst frequency.\n\nThe color burst is set to this phase at the first HSYNC and then reset to the same value at further HSYNCs as specified by the CPhaseRset bits of the EncConfig5 parameter (see above)"]
pub type CHROMA_PHASE_R = crate::FieldReader;
#[doc = "Field `chroma_phase` writer - Specify the color burst initial phase (ChromaPhase). 8.8 bit unsigned fraction. Units are cycles of the color burst frequency.\n\nThe color burst is set to this phase at the first HSYNC and then reset to the same value at further HSYNCs as specified by the CPhaseRset bits of the EncConfig5 parameter (see above)"]
pub type CHROMA_PHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `tint` reader - Specify the tint adjustment of the chroma signal for CVBS and Y/C outputs. The adjustment is effected by setting the sub- carrier phase to the value of this parameter. 8.8 bit unsigned fraction. Units are cycles of the color burst frequency."]
pub type TINT_R = crate::FieldReader;
#[doc = "Field `tint` writer - Specify the tint adjustment of the chroma signal for CVBS and Y/C outputs. The adjustment is effected by setting the sub- carrier phase to the value of this parameter. 8.8 bit unsigned fraction. Units are cycles of the color burst frequency."]
pub type TINT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Specify the color burst initial phase (ChromaPhase). 8.8 bit unsigned fraction. Units are cycles of the color burst frequency.\n\nThe color burst is set to this phase at the first HSYNC and then reset to the same value at further HSYNCs as specified by the CPhaseRset bits of the EncConfig5 parameter (see above)"]
    #[inline(always)]
    pub fn chroma_phase(&self) -> CHROMA_PHASE_R {
        CHROMA_PHASE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Specify the tint adjustment of the chroma signal for CVBS and Y/C outputs. The adjustment is effected by setting the sub- carrier phase to the value of this parameter. 8.8 bit unsigned fraction. Units are cycles of the color burst frequency."]
    #[inline(always)]
    pub fn tint(&self) -> TINT_R {
        TINT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specify the color burst initial phase (ChromaPhase). 8.8 bit unsigned fraction. Units are cycles of the color burst frequency.\n\nThe color burst is set to this phase at the first HSYNC and then reset to the same value at further HSYNCs as specified by the CPhaseRset bits of the EncConfig5 parameter (see above)"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_phase(&mut self) -> CHROMA_PHASE_W<TVE_TINT_COLOR_BURST_PHASE_SPEC> {
        CHROMA_PHASE_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Specify the tint adjustment of the chroma signal for CVBS and Y/C outputs. The adjustment is effected by setting the sub- carrier phase to the value of this parameter. 8.8 bit unsigned fraction. Units are cycles of the color burst frequency."]
    #[inline(always)]
    #[must_use]
    pub fn tint(&mut self) -> TINT_W<TVE_TINT_COLOR_BURST_PHASE_SPEC> {
        TINT_W::new(self, 16)
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
#[doc = "TV Encoder Tint and Color Burst Phase Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_tint_color_burst_phase::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_tint_color_burst_phase::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_TINT_COLOR_BURST_PHASE_SPEC;
impl crate::RegisterSpec for TVE_TINT_COLOR_BURST_PHASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_tint_color_burst_phase::R`](R) reader structure"]
impl crate::Readable for TVE_TINT_COLOR_BURST_PHASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_tint_color_burst_phase::W`](W) writer structure"]
impl crate::Writable for TVE_TINT_COLOR_BURST_PHASE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_tint_color_burst_phase to value 0"]
impl crate::Resettable for TVE_TINT_COLOR_BURST_PHASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
