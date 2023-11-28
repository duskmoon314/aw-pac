#[doc = "Register `gp_cs_en` reader"]
pub type R = crate::R<GP_CS_EN_SPEC>;
#[doc = "Register `gp_cs_en` writer"]
pub type W = crate::W<GP_CS_EN_SPEC>;
#[doc = "Field `adc_ch_select[0-1]` reader - Analog Input Channel Select"]
pub type ADC_CH_SELECT_R = crate::BitReader<ADC_CH_SELECT_A>;
#[doc = "Analog Input Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_CH_SELECT_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ADC_CH_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_CH_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_CH_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC_CH_SELECT_A {
        match self.bits {
            false => ADC_CH_SELECT_A::DISABLE,
            true => ADC_CH_SELECT_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC_CH_SELECT_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC_CH_SELECT_A::ENABLE
    }
}
#[doc = "Field `adc_ch_select[0-1]` writer - Analog Input Channel Select"]
pub type ADC_CH_SELECT_W<'a, REG> = crate::BitWriter<'a, REG, ADC_CH_SELECT_A>;
impl<'a, REG> ADC_CH_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CH_SELECT_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CH_SELECT_A::ENABLE)
    }
}
#[doc = "Field `adc_ch_cmp_en[0-1]` reader - Channel Compare Enable"]
pub type ADC_CH_CMP_EN_R = crate::BitReader<ADC_CH_CMP_EN_A>;
#[doc = "Channel Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_CH_CMP_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ADC_CH_CMP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_CH_CMP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_CH_CMP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC_CH_CMP_EN_A {
        match self.bits {
            false => ADC_CH_CMP_EN_A::DISABLE,
            true => ADC_CH_CMP_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC_CH_CMP_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC_CH_CMP_EN_A::ENABLE
    }
}
#[doc = "Field `adc_ch_cmp_en[0-1]` writer - Channel Compare Enable"]
pub type ADC_CH_CMP_EN_W<'a, REG> = crate::BitWriter<'a, REG, ADC_CH_CMP_EN_A>;
impl<'a, REG> ADC_CH_CMP_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CH_CMP_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CH_CMP_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Analog Input Channel Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `adc_ch0_select` field"]
    #[inline(always)]
    pub fn adc_ch_select(&self, n: u8) -> ADC_CH_SELECT_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ADC_CH_SELECT_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Analog Input Channel Select"]
    #[inline(always)]
    pub fn adc_ch0_select(&self) -> ADC_CH_SELECT_R {
        ADC_CH_SELECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Input Channel Select"]
    #[inline(always)]
    pub fn adc_ch1_select(&self) -> ADC_CH_SELECT_R {
        ADC_CH_SELECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Channel Compare Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `adc_ch0_cmp_en` field"]
    #[inline(always)]
    pub fn adc_ch_cmp_en(&self, n: u8) -> ADC_CH_CMP_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ADC_CH_CMP_EN_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel Compare Enable"]
    #[inline(always)]
    pub fn adc_ch0_cmp_en(&self) -> ADC_CH_CMP_EN_R {
        ADC_CH_CMP_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel Compare Enable"]
    #[inline(always)]
    pub fn adc_ch1_cmp_en(&self) -> ADC_CH_CMP_EN_R {
        ADC_CH_CMP_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Analog Input Channel Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `adc_ch0_select` field"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ch_select(&mut self, n: u8) -> ADC_CH_SELECT_W<GP_CS_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ADC_CH_SELECT_W::new(self, n)
    }
    #[doc = "Bit 0 - Analog Input Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ch0_select(&mut self) -> ADC_CH_SELECT_W<GP_CS_EN_SPEC> {
        ADC_CH_SELECT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Analog Input Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ch1_select(&mut self) -> ADC_CH_SELECT_W<GP_CS_EN_SPEC> {
        ADC_CH_SELECT_W::new(self, 1)
    }
    #[doc = "Channel Compare Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `adc_ch0_cmp_en` field"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ch_cmp_en(&mut self, n: u8) -> ADC_CH_CMP_EN_W<GP_CS_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ADC_CH_CMP_EN_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - Channel Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ch0_cmp_en(&mut self) -> ADC_CH_CMP_EN_W<GP_CS_EN_SPEC> {
        ADC_CH_CMP_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ch1_cmp_en(&mut self) -> ADC_CH_CMP_EN_W<GP_CS_EN_SPEC> {
        ADC_CH_CMP_EN_W::new(self, 17)
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
#[doc = "GPADC Compare and Select Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_cs_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_cs_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GP_CS_EN_SPEC;
impl crate::RegisterSpec for GP_CS_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp_cs_en::R`](R) reader structure"]
impl crate::Readable for GP_CS_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gp_cs_en::W`](W) writer structure"]
impl crate::Writable for GP_CS_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gp_cs_en to value 0"]
impl crate::Resettable for GP_CS_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
