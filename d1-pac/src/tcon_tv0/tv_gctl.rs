#[doc = "Register `tv_gctl` reader"]
pub type R = crate::R<TV_GCTL_SPEC>;
#[doc = "Register `tv_gctl` writer"]
pub type W = crate::W<TV_GCTL_SPEC>;
#[doc = "Field `cec_ddc_pad_sel` reader - CEC DDC PAD Select"]
pub type CEC_DDC_PAD_SEL_R = crate::BitReader<CEC_DDC_PAD_SEL_A>;
#[doc = "CEC DDC PAD Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEC_DDC_PAD_SEL_A {
    #[doc = "1: TCON_TV internal pad for cec scl sal"]
    INTERNAL_PAD = 1,
    #[doc = "0: GPIO pad for cec scl sal"]
    GPIO_PAD = 0,
}
impl From<CEC_DDC_PAD_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CEC_DDC_PAD_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CEC_DDC_PAD_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEC_DDC_PAD_SEL_A {
        match self.bits {
            true => CEC_DDC_PAD_SEL_A::INTERNAL_PAD,
            false => CEC_DDC_PAD_SEL_A::GPIO_PAD,
        }
    }
    #[doc = "TCON_TV internal pad for cec scl sal"]
    #[inline(always)]
    pub fn is_internal_pad(&self) -> bool {
        *self == CEC_DDC_PAD_SEL_A::INTERNAL_PAD
    }
    #[doc = "GPIO pad for cec scl sal"]
    #[inline(always)]
    pub fn is_gpio_pad(&self) -> bool {
        *self == CEC_DDC_PAD_SEL_A::GPIO_PAD
    }
}
#[doc = "Field `cec_ddc_pad_sel` writer - CEC DDC PAD Select"]
pub type CEC_DDC_PAD_SEL_W<'a, REG> = crate::BitWriter<'a, REG, CEC_DDC_PAD_SEL_A>;
impl<'a, REG> CEC_DDC_PAD_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TCON_TV internal pad for cec scl sal"]
    #[inline(always)]
    pub fn internal_pad(self) -> &'a mut crate::W<REG> {
        self.variant(CEC_DDC_PAD_SEL_A::INTERNAL_PAD)
    }
    #[doc = "GPIO pad for cec scl sal"]
    #[inline(always)]
    pub fn gpio_pad(self) -> &'a mut crate::W<REG> {
        self.variant(CEC_DDC_PAD_SEL_A::GPIO_PAD)
    }
}
#[doc = "Field `tv_en` reader - When it is disabled, the module will be reset to idle state."]
pub type TV_EN_R = crate::BitReader<TV_EN_A>;
#[doc = "When it is disabled, the module will be reset to idle state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TV_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<TV_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TV_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TV_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TV_EN_A {
        match self.bits {
            false => TV_EN_A::DISABLE,
            true => TV_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TV_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TV_EN_A::ENABLE
    }
}
#[doc = "Field `tv_en` writer - When it is disabled, the module will be reset to idle state."]
pub type TV_EN_W<'a, REG> = crate::BitWriter<'a, REG, TV_EN_A>;
impl<'a, REG> TV_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TV_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TV_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 1 - CEC DDC PAD Select"]
    #[inline(always)]
    pub fn cec_ddc_pad_sel(&self) -> CEC_DDC_PAD_SEL_R {
        CEC_DDC_PAD_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - When it is disabled, the module will be reset to idle state."]
    #[inline(always)]
    pub fn tv_en(&self) -> TV_EN_R {
        TV_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CEC DDC PAD Select"]
    #[inline(always)]
    #[must_use]
    pub fn cec_ddc_pad_sel(&mut self) -> CEC_DDC_PAD_SEL_W<TV_GCTL_SPEC> {
        CEC_DDC_PAD_SEL_W::new(self, 1)
    }
    #[doc = "Bit 31 - When it is disabled, the module will be reset to idle state."]
    #[inline(always)]
    #[must_use]
    pub fn tv_en(&mut self) -> TV_EN_W<TV_GCTL_SPEC> {
        TV_EN_W::new(self, 31)
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
#[doc = "TV Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_gctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_gctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_GCTL_SPEC;
impl crate::RegisterSpec for TV_GCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_gctl::R`](R) reader structure"]
impl crate::Readable for TV_GCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_gctl::W`](W) writer structure"]
impl crate::Writable for TV_GCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_gctl to value 0"]
impl crate::Resettable for TV_GCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
