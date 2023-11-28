#[doc = "Register `pio_pow_vol_sel_ctl` reader"]
pub type R = crate::R<PIO_POW_VOL_SEL_CTL_SPEC>;
#[doc = "Register `pio_pow_vol_sel_ctl` writer"]
pub type W = crate::W<PIO_POW_VOL_SEL_CTL_SPEC>;
#[doc = "Field `vcc_pf_pwr_vol_sel` reader - VCC_PF Power Voltage Select Control"]
pub type VCC_PF_PWR_VOL_SEL_R = crate::BitReader<VCC_PF_PWR_VOL_SEL_A>;
#[doc = "VCC_PF Power Voltage Select Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCC_PF_PWR_VOL_SEL_A {
    #[doc = "0: 1.8V"]
    V18 = 0,
    #[doc = "1: 3.3V"]
    V33 = 1,
}
impl From<VCC_PF_PWR_VOL_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: VCC_PF_PWR_VOL_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VCC_PF_PWR_VOL_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCC_PF_PWR_VOL_SEL_A {
        match self.bits {
            false => VCC_PF_PWR_VOL_SEL_A::V18,
            true => VCC_PF_PWR_VOL_SEL_A::V33,
        }
    }
    #[doc = "1.8V"]
    #[inline(always)]
    pub fn is_v18(&self) -> bool {
        *self == VCC_PF_PWR_VOL_SEL_A::V18
    }
    #[doc = "3.3V"]
    #[inline(always)]
    pub fn is_v33(&self) -> bool {
        *self == VCC_PF_PWR_VOL_SEL_A::V33
    }
}
#[doc = "Field `vcc_pf_pwr_vol_sel` writer - VCC_PF Power Voltage Select Control"]
pub type VCC_PF_PWR_VOL_SEL_W<'a, REG> = crate::BitWriter<'a, REG, VCC_PF_PWR_VOL_SEL_A>;
impl<'a, REG> VCC_PF_PWR_VOL_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1.8V"]
    #[inline(always)]
    pub fn v18(self) -> &'a mut crate::W<REG> {
        self.variant(VCC_PF_PWR_VOL_SEL_A::V18)
    }
    #[doc = "3.3V"]
    #[inline(always)]
    pub fn v33(self) -> &'a mut crate::W<REG> {
        self.variant(VCC_PF_PWR_VOL_SEL_A::V33)
    }
}
impl R {
    #[doc = "Bit 0 - VCC_PF Power Voltage Select Control"]
    #[inline(always)]
    pub fn vcc_pf_pwr_vol_sel(&self) -> VCC_PF_PWR_VOL_SEL_R {
        VCC_PF_PWR_VOL_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VCC_PF Power Voltage Select Control"]
    #[inline(always)]
    #[must_use]
    pub fn vcc_pf_pwr_vol_sel(&mut self) -> VCC_PF_PWR_VOL_SEL_W<PIO_POW_VOL_SEL_CTL_SPEC> {
        VCC_PF_PWR_VOL_SEL_W::new(self, 0)
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
#[doc = "PIO Group Power Voltage Select Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio_pow_vol_sel_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio_pow_vol_sel_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIO_POW_VOL_SEL_CTL_SPEC;
impl crate::RegisterSpec for PIO_POW_VOL_SEL_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pio_pow_vol_sel_ctl::R`](R) reader structure"]
impl crate::Readable for PIO_POW_VOL_SEL_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pio_pow_vol_sel_ctl::W`](W) writer structure"]
impl crate::Writable for PIO_POW_VOL_SEL_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pio_pow_vol_sel_ctl to value 0"]
impl crate::Resettable for PIO_POW_VOL_SEL_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
