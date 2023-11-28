#[doc = "Register `ac_adc_dap_ctr` reader"]
pub type R = crate::R<AC_ADC_DAP_CTR_SPEC>;
#[doc = "Register `ac_adc_dap_ctr` writer"]
pub type W = crate::W<AC_ADC_DAP_CTR_SPEC>;
#[doc = "Field `adc_hpf_en[1,0]` reader - ADC HPF\\[i\\] enable control"]
pub type ADC_HPF_EN_R = crate::BitReader<ADC_HPF_EN_A>;
#[doc = "ADC HPF\\[i\\] enable control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_HPF_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC_HPF_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_HPF_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_HPF_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC_HPF_EN_A {
        match self.bits {
            false => ADC_HPF_EN_A::DISABLED,
            true => ADC_HPF_EN_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_HPF_EN_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_HPF_EN_A::ENABLED
    }
}
#[doc = "Field `adc_hpf_en[1,0]` writer - ADC HPF\\[i\\] enable control"]
pub type ADC_HPF_EN_W<'a, REG> = crate::BitWriter<'a, REG, ADC_HPF_EN_A>;
impl<'a, REG> ADC_HPF_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_HPF_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_HPF_EN_A::ENABLED)
    }
}
#[doc = "Field `adc_drc_en[1,0]` reader - ADC DRC\\[i\\] enable control"]
pub type ADC_DRC_EN_R = crate::BitReader<ADC_DRC_EN_A>;
#[doc = "ADC DRC\\[i\\] enable control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_DRC_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC_DRC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DRC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_DRC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC_DRC_EN_A {
        match self.bits {
            false => ADC_DRC_EN_A::DISABLED,
            true => ADC_DRC_EN_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_DRC_EN_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_DRC_EN_A::ENABLED
    }
}
#[doc = "Field `adc_drc_en[1,0]` writer - ADC DRC\\[i\\] enable control"]
pub type ADC_DRC_EN_W<'a, REG> = crate::BitWriter<'a, REG, ADC_DRC_EN_A>;
impl<'a, REG> ADC_DRC_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DRC_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DRC_EN_A::ENABLED)
    }
}
#[doc = "Field `adc_dap_en[1,0]` reader - DAP\\[i\\] for ADC enable\n\nDAP0 control ADC1/2\n\nDAP1 control ADC3"]
pub type ADC_DAP_EN_R = crate::BitReader<ADC_DAP_EN_A>;
#[doc = "DAP\\[i\\] for ADC enable\n\nDAP0 control ADC1/2\n\nDAP1 control ADC3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_DAP_EN_A {
    #[doc = "0: Bypassed"]
    BYPASSED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC_DAP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DAP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_DAP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC_DAP_EN_A {
        match self.bits {
            false => ADC_DAP_EN_A::BYPASSED,
            true => ADC_DAP_EN_A::ENABLED,
        }
    }
    #[doc = "Bypassed"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == ADC_DAP_EN_A::BYPASSED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_DAP_EN_A::ENABLED
    }
}
#[doc = "Field `adc_dap_en[1,0]` writer - DAP\\[i\\] for ADC enable\n\nDAP0 control ADC1/2\n\nDAP1 control ADC3"]
pub type ADC_DAP_EN_W<'a, REG> = crate::BitWriter<'a, REG, ADC_DAP_EN_A>;
impl<'a, REG> ADC_DAP_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bypassed"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DAP_EN_A::BYPASSED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DAP_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "ADC HPF\\[i\\] enable control\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `adc_hpf1_en` field"]
    #[inline(always)]
    pub fn adc_hpf_en(&self, n: u8) -> ADC_HPF_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ADC_HPF_EN_R::new(((self.bits >> (n * 4 + 24)) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC HPF\\[i\\] enable control"]
    #[inline(always)]
    pub fn adc_hpf1_en(&self) -> ADC_HPF_EN_R {
        ADC_HPF_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC HPF\\[i\\] enable control"]
    #[inline(always)]
    pub fn adc_hpf0_en(&self) -> ADC_HPF_EN_R {
        ADC_HPF_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "ADC DRC\\[i\\] enable control\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `adc_drc1_en` field"]
    #[inline(always)]
    pub fn adc_drc_en(&self, n: u8) -> ADC_DRC_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ADC_DRC_EN_R::new(((self.bits >> (n * 4 + 25)) & 1) != 0)
    }
    #[doc = "Bit 25 - ADC DRC\\[i\\] enable control"]
    #[inline(always)]
    pub fn adc_drc1_en(&self) -> ADC_DRC_EN_R {
        ADC_DRC_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - ADC DRC\\[i\\] enable control"]
    #[inline(always)]
    pub fn adc_drc0_en(&self) -> ADC_DRC_EN_R {
        ADC_DRC_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "DAP\\[i\\] for ADC enable\n\nDAP0 control ADC1/2\n\nDAP1 control ADC3\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `adc_dap1_en` field"]
    #[inline(always)]
    pub fn adc_dap_en(&self, n: u8) -> ADC_DAP_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ADC_DAP_EN_R::new(((self.bits >> (n * 4 + 27)) & 1) != 0)
    }
    #[doc = "Bit 27 - DAP\\[i\\] for ADC enable\n\nDAP0 control ADC1/2\n\nDAP1 control ADC3"]
    #[inline(always)]
    pub fn adc_dap1_en(&self) -> ADC_DAP_EN_R {
        ADC_DAP_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - DAP\\[i\\] for ADC enable\n\nDAP0 control ADC1/2\n\nDAP1 control ADC3"]
    #[inline(always)]
    pub fn adc_dap0_en(&self) -> ADC_DAP_EN_R {
        ADC_DAP_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "ADC HPF\\[i\\] enable control\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `adc_hpf1_en` field"]
    #[inline(always)]
    #[must_use]
    pub fn adc_hpf_en(&mut self, n: u8) -> ADC_HPF_EN_W<AC_ADC_DAP_CTR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ADC_HPF_EN_W::new(self, n * 4 + 24)
    }
    #[doc = "Bit 24 - ADC HPF\\[i\\] enable control"]
    #[inline(always)]
    #[must_use]
    pub fn adc_hpf1_en(&mut self) -> ADC_HPF_EN_W<AC_ADC_DAP_CTR_SPEC> {
        ADC_HPF_EN_W::new(self, 24)
    }
    #[doc = "Bit 28 - ADC HPF\\[i\\] enable control"]
    #[inline(always)]
    #[must_use]
    pub fn adc_hpf0_en(&mut self) -> ADC_HPF_EN_W<AC_ADC_DAP_CTR_SPEC> {
        ADC_HPF_EN_W::new(self, 28)
    }
    #[doc = "ADC DRC\\[i\\] enable control\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `adc_drc1_en` field"]
    #[inline(always)]
    #[must_use]
    pub fn adc_drc_en(&mut self, n: u8) -> ADC_DRC_EN_W<AC_ADC_DAP_CTR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ADC_DRC_EN_W::new(self, n * 4 + 25)
    }
    #[doc = "Bit 25 - ADC DRC\\[i\\] enable control"]
    #[inline(always)]
    #[must_use]
    pub fn adc_drc1_en(&mut self) -> ADC_DRC_EN_W<AC_ADC_DAP_CTR_SPEC> {
        ADC_DRC_EN_W::new(self, 25)
    }
    #[doc = "Bit 29 - ADC DRC\\[i\\] enable control"]
    #[inline(always)]
    #[must_use]
    pub fn adc_drc0_en(&mut self) -> ADC_DRC_EN_W<AC_ADC_DAP_CTR_SPEC> {
        ADC_DRC_EN_W::new(self, 29)
    }
    #[doc = "DAP\\[i\\] for ADC enable\n\nDAP0 control ADC1/2\n\nDAP1 control ADC3\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `adc_dap1_en` field"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dap_en(&mut self, n: u8) -> ADC_DAP_EN_W<AC_ADC_DAP_CTR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ADC_DAP_EN_W::new(self, n * 4 + 27)
    }
    #[doc = "Bit 27 - DAP\\[i\\] for ADC enable\n\nDAP0 control ADC1/2\n\nDAP1 control ADC3"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dap1_en(&mut self) -> ADC_DAP_EN_W<AC_ADC_DAP_CTR_SPEC> {
        ADC_DAP_EN_W::new(self, 27)
    }
    #[doc = "Bit 31 - DAP\\[i\\] for ADC enable\n\nDAP0 control ADC1/2\n\nDAP1 control ADC3"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dap0_en(&mut self) -> ADC_DAP_EN_W<AC_ADC_DAP_CTR_SPEC> {
        ADC_DAP_EN_W::new(self, 31)
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
#[doc = "ADC DAP Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_dap_ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_dap_ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_ADC_DAP_CTR_SPEC;
impl crate::RegisterSpec for AC_ADC_DAP_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_adc_dap_ctr::R`](R) reader structure"]
impl crate::Readable for AC_ADC_DAP_CTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_adc_dap_ctr::W`](W) writer structure"]
impl crate::Writable for AC_ADC_DAP_CTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_adc_dap_ctr to value 0"]
impl crate::Resettable for AC_ADC_DAP_CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
