#[doc = "Register `tve_auto_detection_enable` reader"]
pub type R = crate::R<TVE_AUTO_DETECTION_ENABLE_SPEC>;
#[doc = "Register `tve_auto_detection_enable` writer"]
pub type W = crate::W<TVE_AUTO_DETECTION_ENABLE_SPEC>;
#[doc = "Field `dac0_auto_detect_enable` reader - "]
pub type DAC0_AUTO_DETECT_ENABLE_R = crate::BitReader;
#[doc = "Field `dac0_auto_detect_enable` writer - "]
pub type DAC0_AUTO_DETECT_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac0_auto_detect_interrupt_en` reader - "]
pub type DAC0_AUTO_DETECT_INTERRUPT_EN_R = crate::BitReader;
#[doc = "Field `dac0_auto_detect_interrupt_en` writer - "]
pub type DAC0_AUTO_DETECT_INTERRUPT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_auto_detect_mode_sel` reader - "]
pub type DAC_AUTO_DETECT_MODE_SEL_R = crate::BitReader<DAC_AUTO_DETECT_MODE_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC_AUTO_DETECT_MODE_SEL_A {
    #[doc = "0: Old Mode"]
    O_LD = 0,
    #[doc = "1: New Mode"]
    N_EW = 1,
}
impl From<DAC_AUTO_DETECT_MODE_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DAC_AUTO_DETECT_MODE_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC_AUTO_DETECT_MODE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC_AUTO_DETECT_MODE_SEL_A {
        match self.bits {
            false => DAC_AUTO_DETECT_MODE_SEL_A::O_LD,
            true => DAC_AUTO_DETECT_MODE_SEL_A::N_EW,
        }
    }
    #[doc = "Old Mode"]
    #[inline(always)]
    pub fn is_o_ld(&self) -> bool {
        *self == DAC_AUTO_DETECT_MODE_SEL_A::O_LD
    }
    #[doc = "New Mode"]
    #[inline(always)]
    pub fn is_n_ew(&self) -> bool {
        *self == DAC_AUTO_DETECT_MODE_SEL_A::N_EW
    }
}
#[doc = "Field `dac_auto_detect_mode_sel` writer - "]
pub type DAC_AUTO_DETECT_MODE_SEL_W<'a, REG> =
    crate::BitWriter<'a, REG, DAC_AUTO_DETECT_MODE_SEL_A>;
impl<'a, REG> DAC_AUTO_DETECT_MODE_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Old Mode"]
    #[inline(always)]
    pub fn o_ld(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_AUTO_DETECT_MODE_SEL_A::O_LD)
    }
    #[doc = "New Mode"]
    #[inline(always)]
    pub fn n_ew(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_AUTO_DETECT_MODE_SEL_A::N_EW)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dac0_auto_detect_enable(&self) -> DAC0_AUTO_DETECT_ENABLE_R {
        DAC0_AUTO_DETECT_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dac0_auto_detect_interrupt_en(&self) -> DAC0_AUTO_DETECT_INTERRUPT_EN_R {
        DAC0_AUTO_DETECT_INTERRUPT_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dac_auto_detect_mode_sel(&self) -> DAC_AUTO_DETECT_MODE_SEL_R {
        DAC_AUTO_DETECT_MODE_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dac0_auto_detect_enable(
        &mut self,
    ) -> DAC0_AUTO_DETECT_ENABLE_W<TVE_AUTO_DETECTION_ENABLE_SPEC> {
        DAC0_AUTO_DETECT_ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn dac0_auto_detect_interrupt_en(
        &mut self,
    ) -> DAC0_AUTO_DETECT_INTERRUPT_EN_W<TVE_AUTO_DETECTION_ENABLE_SPEC> {
        DAC0_AUTO_DETECT_INTERRUPT_EN_W::new(self, 16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn dac_auto_detect_mode_sel(
        &mut self,
    ) -> DAC_AUTO_DETECT_MODE_SEL_W<TVE_AUTO_DETECTION_ENABLE_SPEC> {
        DAC_AUTO_DETECT_MODE_SEL_W::new(self, 31)
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
#[doc = "TV Encoder Auto Detection Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_auto_detection_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_auto_detection_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_AUTO_DETECTION_ENABLE_SPEC;
impl crate::RegisterSpec for TVE_AUTO_DETECTION_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_auto_detection_enable::R`](R) reader structure"]
impl crate::Readable for TVE_AUTO_DETECTION_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_auto_detection_enable::W`](W) writer structure"]
impl crate::Writable for TVE_AUTO_DETECTION_ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_auto_detection_enable to value 0"]
impl crate::Resettable for TVE_AUTO_DETECTION_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
