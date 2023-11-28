#[doc = "Register `ac_adc_dg` reader"]
pub type R = crate::R<AC_ADC_DG_SPEC>;
#[doc = "Register `ac_adc_dg` writer"]
pub type W = crate::W<AC_ADC_DG_SPEC>;
#[doc = "Field `ad_swp1` reader - ADC output channel swap enable (for digital filter)"]
pub type AD_SWP1_R = crate::BitReader<AD_SWP1_A>;
#[doc = "ADC output channel swap enable (for digital filter)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD_SWP1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<AD_SWP1_A> for bool {
    #[inline(always)]
    fn from(variant: AD_SWP1_A) -> Self {
        variant as u8 != 0
    }
}
impl AD_SWP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AD_SWP1_A {
        match self.bits {
            false => AD_SWP1_A::DISABLED,
            true => AD_SWP1_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD_SWP1_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD_SWP1_A::ENABLED
    }
}
#[doc = "Field `ad_swp1` writer - ADC output channel swap enable (for digital filter)"]
pub type AD_SWP1_W<'a, REG> = crate::BitWriter<'a, REG, AD_SWP1_A>;
impl<'a, REG> AD_SWP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD_SWP1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD_SWP1_A::ENABLED)
    }
}
#[doc = "Field `ad_swp2` reader - ADC output channel swap enable (for digital filter)"]
pub type AD_SWP2_R = crate::BitReader<AD_SWP2_A>;
#[doc = "ADC output channel swap enable (for digital filter)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD_SWP2_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<AD_SWP2_A> for bool {
    #[inline(always)]
    fn from(variant: AD_SWP2_A) -> Self {
        variant as u8 != 0
    }
}
impl AD_SWP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AD_SWP2_A {
        match self.bits {
            false => AD_SWP2_A::DISABLED,
            true => AD_SWP2_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD_SWP2_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD_SWP2_A::ENABLED
    }
}
#[doc = "Field `ad_swp2` writer - ADC output channel swap enable (for digital filter)"]
pub type AD_SWP2_W<'a, REG> = crate::BitWriter<'a, REG, AD_SWP2_A>;
impl<'a, REG> AD_SWP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD_SWP2_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD_SWP2_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 24 - ADC output channel swap enable (for digital filter)"]
    #[inline(always)]
    pub fn ad_swp1(&self) -> AD_SWP1_R {
        AD_SWP1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADC output channel swap enable (for digital filter)"]
    #[inline(always)]
    pub fn ad_swp2(&self) -> AD_SWP2_R {
        AD_SWP2_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - ADC output channel swap enable (for digital filter)"]
    #[inline(always)]
    #[must_use]
    pub fn ad_swp1(&mut self) -> AD_SWP1_W<AC_ADC_DG_SPEC> {
        AD_SWP1_W::new(self, 24)
    }
    #[doc = "Bit 25 - ADC output channel swap enable (for digital filter)"]
    #[inline(always)]
    #[must_use]
    pub fn ad_swp2(&mut self) -> AD_SWP2_W<AC_ADC_DG_SPEC> {
        AD_SWP2_W::new(self, 25)
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
#[doc = "ADC Debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_dg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_dg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_ADC_DG_SPEC;
impl crate::RegisterSpec for AC_ADC_DG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_adc_dg::R`](R) reader structure"]
impl crate::Readable for AC_ADC_DG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_adc_dg::W`](W) writer structure"]
impl crate::Writable for AC_ADC_DG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_adc_dg to value 0"]
impl crate::Resettable for AC_ADC_DG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
