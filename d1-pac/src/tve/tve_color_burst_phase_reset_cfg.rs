#[doc = "Register `tve_color_burst_phase_reset_cfg` reader"]
pub type R = crate::R<TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC>;
#[doc = "Register `tve_color_burst_phase_reset_cfg` writer"]
pub type W = crate::W<TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC>;
#[doc = "Field `color_phase_reset` reader - Color burst phase period selection\n\nThese bits select the number of fields or lines after which the color burst phase is reset to its initial value as specified by the ChromaPhase parameter, This parameter is application only for interlaced video."]
pub type COLOR_PHASE_RESET_R = crate::FieldReader<COLOR_PHASE_RESET_A>;
#[doc = "Color burst phase period selection\n\nThese bits select the number of fields or lines after which the color burst phase is reset to its initial value as specified by the ChromaPhase parameter, This parameter is application only for interlaced video.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COLOR_PHASE_RESET_A {
    #[doc = "0: 8 field"]
    LINE8 = 0,
    #[doc = "1: 4 field"]
    LINE4 = 1,
    #[doc = "2: 2 lines"]
    LINE2 = 2,
    #[doc = "3: only once"]
    ONLY_ONCE = 3,
}
impl From<COLOR_PHASE_RESET_A> for u8 {
    #[inline(always)]
    fn from(variant: COLOR_PHASE_RESET_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COLOR_PHASE_RESET_A {
    type Ux = u8;
}
impl COLOR_PHASE_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COLOR_PHASE_RESET_A {
        match self.bits {
            0 => COLOR_PHASE_RESET_A::LINE8,
            1 => COLOR_PHASE_RESET_A::LINE4,
            2 => COLOR_PHASE_RESET_A::LINE2,
            3 => COLOR_PHASE_RESET_A::ONLY_ONCE,
            _ => unreachable!(),
        }
    }
    #[doc = "8 field"]
    #[inline(always)]
    pub fn is_line8(&self) -> bool {
        *self == COLOR_PHASE_RESET_A::LINE8
    }
    #[doc = "4 field"]
    #[inline(always)]
    pub fn is_line4(&self) -> bool {
        *self == COLOR_PHASE_RESET_A::LINE4
    }
    #[doc = "2 lines"]
    #[inline(always)]
    pub fn is_line2(&self) -> bool {
        *self == COLOR_PHASE_RESET_A::LINE2
    }
    #[doc = "only once"]
    #[inline(always)]
    pub fn is_only_once(&self) -> bool {
        *self == COLOR_PHASE_RESET_A::ONLY_ONCE
    }
}
#[doc = "Field `color_phase_reset` writer - Color burst phase period selection\n\nThese bits select the number of fields or lines after which the color burst phase is reset to its initial value as specified by the ChromaPhase parameter, This parameter is application only for interlaced video."]
pub type COLOR_PHASE_RESET_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COLOR_PHASE_RESET_A>;
impl<'a, REG> COLOR_PHASE_RESET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 field"]
    #[inline(always)]
    pub fn line8(self) -> &'a mut crate::W<REG> {
        self.variant(COLOR_PHASE_RESET_A::LINE8)
    }
    #[doc = "4 field"]
    #[inline(always)]
    pub fn line4(self) -> &'a mut crate::W<REG> {
        self.variant(COLOR_PHASE_RESET_A::LINE4)
    }
    #[doc = "2 lines"]
    #[inline(always)]
    pub fn line2(self) -> &'a mut crate::W<REG> {
        self.variant(COLOR_PHASE_RESET_A::LINE2)
    }
    #[doc = "only once"]
    #[inline(always)]
    pub fn only_once(self) -> &'a mut crate::W<REG> {
        self.variant(COLOR_PHASE_RESET_A::ONLY_ONCE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Color burst phase period selection\n\nThese bits select the number of fields or lines after which the color burst phase is reset to its initial value as specified by the ChromaPhase parameter, This parameter is application only for interlaced video."]
    #[inline(always)]
    pub fn color_phase_reset(&self) -> COLOR_PHASE_RESET_R {
        COLOR_PHASE_RESET_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Color burst phase period selection\n\nThese bits select the number of fields or lines after which the color burst phase is reset to its initial value as specified by the ChromaPhase parameter, This parameter is application only for interlaced video."]
    #[inline(always)]
    #[must_use]
    pub fn color_phase_reset(
        &mut self,
    ) -> COLOR_PHASE_RESET_W<TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC> {
        COLOR_PHASE_RESET_W::new(self, 0)
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
#[doc = "TV Encoder Color Burst Phase Reset Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_color_burst_phase_reset_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_color_burst_phase_reset_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC;
impl crate::RegisterSpec for TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_color_burst_phase_reset_cfg::R`](R) reader structure"]
impl crate::Readable for TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_color_burst_phase_reset_cfg::W`](W) writer structure"]
impl crate::Writable for TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_color_burst_phase_reset_cfg to value 0"]
impl crate::Resettable for TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
