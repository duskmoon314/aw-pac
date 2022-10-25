#[doc = "Register `pio_pow_vol_sel_ctl` reader"]
pub struct R(crate::R<PIO_POW_VOL_SEL_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO_POW_VOL_SEL_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO_POW_VOL_SEL_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO_POW_VOL_SEL_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pio_pow_vol_sel_ctl` writer"]
pub struct W(crate::W<PIO_POW_VOL_SEL_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO_POW_VOL_SEL_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PIO_POW_VOL_SEL_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO_POW_VOL_SEL_CTL_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> VCC_PF_PWR_VOL_SEL_A {
        match self.bits {
            false => VCC_PF_PWR_VOL_SEL_A::V18,
            true => VCC_PF_PWR_VOL_SEL_A::V33,
        }
    }
    #[doc = "Checks if the value of the field is `V18`"]
    #[inline(always)]
    pub fn is_v18(&self) -> bool {
        *self == VCC_PF_PWR_VOL_SEL_A::V18
    }
    #[doc = "Checks if the value of the field is `V33`"]
    #[inline(always)]
    pub fn is_v33(&self) -> bool {
        *self == VCC_PF_PWR_VOL_SEL_A::V33
    }
}
#[doc = "Field `vcc_pf_pwr_vol_sel` writer - VCC_PF Power Voltage Select Control"]
pub type VCC_PF_PWR_VOL_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PIO_POW_VOL_SEL_CTL_SPEC, VCC_PF_PWR_VOL_SEL_A, O>;
impl<'a, const O: u8> VCC_PF_PWR_VOL_SEL_W<'a, O> {
    #[doc = "1.8V"]
    #[inline(always)]
    pub fn v18(self) -> &'a mut W {
        self.variant(VCC_PF_PWR_VOL_SEL_A::V18)
    }
    #[doc = "3.3V"]
    #[inline(always)]
    pub fn v33(self) -> &'a mut W {
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
    pub fn vcc_pf_pwr_vol_sel(&mut self) -> VCC_PF_PWR_VOL_SEL_W<0> {
        VCC_PF_PWR_VOL_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIO Group Power Voltage Select Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pow_vol_sel_ctl](index.html) module"]
pub struct PIO_POW_VOL_SEL_CTL_SPEC;
impl crate::RegisterSpec for PIO_POW_VOL_SEL_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio_pow_vol_sel_ctl::R](R) reader structure"]
impl crate::Readable for PIO_POW_VOL_SEL_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio_pow_vol_sel_ctl::W](W) writer structure"]
impl crate::Writable for PIO_POW_VOL_SEL_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pio_pow_vol_sel_ctl to value 0"]
impl crate::Resettable for PIO_POW_VOL_SEL_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
