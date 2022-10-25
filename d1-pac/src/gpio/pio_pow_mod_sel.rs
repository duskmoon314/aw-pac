#[doc = "Register `pio_pow_mod_sel` reader"]
pub struct R(crate::R<PIO_POW_MOD_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO_POW_MOD_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO_POW_MOD_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO_POW_MOD_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pio_pow_mod_sel` writer"]
pub struct W(crate::W<PIO_POW_MOD_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO_POW_MOD_SEL_SPEC>;
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
impl From<crate::W<PIO_POW_MOD_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO_POW_MOD_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `p_pwr_mod_sel[C,D,E,F,G]` reader - PX_POWER POWER MODE Select"]
pub type P_PWR_MOD_SEL_R = crate::BitReader<P_PWR_MOD_SEL_A>;
#[doc = "PX_POWER POWER MODE Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P_PWR_MOD_SEL_A {
    #[doc = "0: 3.3 V"]
    V33 = 0,
    #[doc = "1: 1.8 V"]
    V18 = 1,
}
impl From<P_PWR_MOD_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: P_PWR_MOD_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl P_PWR_MOD_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P_PWR_MOD_SEL_A {
        match self.bits {
            false => P_PWR_MOD_SEL_A::V33,
            true => P_PWR_MOD_SEL_A::V18,
        }
    }
    #[doc = "Checks if the value of the field is `V33`"]
    #[inline(always)]
    pub fn is_v33(&self) -> bool {
        *self == P_PWR_MOD_SEL_A::V33
    }
    #[doc = "Checks if the value of the field is `V18`"]
    #[inline(always)]
    pub fn is_v18(&self) -> bool {
        *self == P_PWR_MOD_SEL_A::V18
    }
}
#[doc = "Field `p_pwr_mod_sel[C,D,E,F,G]` writer - PX_POWER POWER MODE Select"]
pub type P_PWR_MOD_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PIO_POW_MOD_SEL_SPEC, P_PWR_MOD_SEL_A, O>;
impl<'a, const O: u8> P_PWR_MOD_SEL_W<'a, O> {
    #[doc = "3.3 V"]
    #[inline(always)]
    pub fn v33(self) -> &'a mut W {
        self.variant(P_PWR_MOD_SEL_A::V33)
    }
    #[doc = "1.8 V"]
    #[inline(always)]
    pub fn v18(self) -> &'a mut W {
        self.variant(P_PWR_MOD_SEL_A::V18)
    }
}
#[doc = "Field `vcc_io_pwr_mod_sel` reader - VCC_IO POWER MODE Select"]
pub type VCC_IO_PWR_MOD_SEL_R = crate::BitReader<VCC_IO_PWR_MOD_SEL_A>;
#[doc = "VCC_IO POWER MODE Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCC_IO_PWR_MOD_SEL_A {
    #[doc = "0: 3.3 V"]
    V33 = 0,
    #[doc = "1: 1.8 V"]
    V18 = 1,
}
impl From<VCC_IO_PWR_MOD_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: VCC_IO_PWR_MOD_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VCC_IO_PWR_MOD_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCC_IO_PWR_MOD_SEL_A {
        match self.bits {
            false => VCC_IO_PWR_MOD_SEL_A::V33,
            true => VCC_IO_PWR_MOD_SEL_A::V18,
        }
    }
    #[doc = "Checks if the value of the field is `V33`"]
    #[inline(always)]
    pub fn is_v33(&self) -> bool {
        *self == VCC_IO_PWR_MOD_SEL_A::V33
    }
    #[doc = "Checks if the value of the field is `V18`"]
    #[inline(always)]
    pub fn is_v18(&self) -> bool {
        *self == VCC_IO_PWR_MOD_SEL_A::V18
    }
}
#[doc = "Field `vcc_io_pwr_mod_sel` writer - VCC_IO POWER MODE Select"]
pub type VCC_IO_PWR_MOD_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PIO_POW_MOD_SEL_SPEC, VCC_IO_PWR_MOD_SEL_A, O>;
impl<'a, const O: u8> VCC_IO_PWR_MOD_SEL_W<'a, O> {
    #[doc = "3.3 V"]
    #[inline(always)]
    pub fn v33(self) -> &'a mut W {
        self.variant(VCC_IO_PWR_MOD_SEL_A::V33)
    }
    #[doc = "1.8 V"]
    #[inline(always)]
    pub fn v18(self) -> &'a mut W {
        self.variant(VCC_IO_PWR_MOD_SEL_A::V18)
    }
}
impl R {
    #[doc = "Bit 2 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    pub fn p_c_pwr_mod_sel(&self) -> P_PWR_MOD_SEL_R {
        P_PWR_MOD_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    pub fn p_d_pwr_mod_sel(&self) -> P_PWR_MOD_SEL_R {
        P_PWR_MOD_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    pub fn p_e_pwr_mod_sel(&self) -> P_PWR_MOD_SEL_R {
        P_PWR_MOD_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    pub fn p_f_pwr_mod_sel(&self) -> P_PWR_MOD_SEL_R {
        P_PWR_MOD_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    pub fn p_g_pwr_mod_sel(&self) -> P_PWR_MOD_SEL_R {
        P_PWR_MOD_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - VCC_IO POWER MODE Select"]
    #[inline(always)]
    pub fn vcc_io_pwr_mod_sel(&self) -> VCC_IO_PWR_MOD_SEL_R {
        VCC_IO_PWR_MOD_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "PX_POWER POWER MODE Select"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn p_pwr_mod_sel<const O: u8>(&mut self) -> P_PWR_MOD_SEL_W<O> {
        P_PWR_MOD_SEL_W::new(self)
    }
    #[doc = "Bit 2 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    #[must_use]
    pub fn p_c_pwr_mod_sel(&mut self) -> P_PWR_MOD_SEL_W<2> {
        P_PWR_MOD_SEL_W::new(self)
    }
    #[doc = "Bit 3 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    #[must_use]
    pub fn p_d_pwr_mod_sel(&mut self) -> P_PWR_MOD_SEL_W<3> {
        P_PWR_MOD_SEL_W::new(self)
    }
    #[doc = "Bit 4 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    #[must_use]
    pub fn p_e_pwr_mod_sel(&mut self) -> P_PWR_MOD_SEL_W<4> {
        P_PWR_MOD_SEL_W::new(self)
    }
    #[doc = "Bit 5 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    #[must_use]
    pub fn p_f_pwr_mod_sel(&mut self) -> P_PWR_MOD_SEL_W<5> {
        P_PWR_MOD_SEL_W::new(self)
    }
    #[doc = "Bit 6 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    #[must_use]
    pub fn p_g_pwr_mod_sel(&mut self) -> P_PWR_MOD_SEL_W<6> {
        P_PWR_MOD_SEL_W::new(self)
    }
    #[doc = "Bit 12 - VCC_IO POWER MODE Select"]
    #[inline(always)]
    #[must_use]
    pub fn vcc_io_pwr_mod_sel(&mut self) -> VCC_IO_PWR_MOD_SEL_W<12> {
        VCC_IO_PWR_MOD_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIO Group Withstand Voltage Mode Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pow_mod_sel](index.html) module"]
pub struct PIO_POW_MOD_SEL_SPEC;
impl crate::RegisterSpec for PIO_POW_MOD_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio_pow_mod_sel::R](R) reader structure"]
impl crate::Readable for PIO_POW_MOD_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio_pow_mod_sel::W](W) writer structure"]
impl crate::Writable for PIO_POW_MOD_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pio_pow_mod_sel to value 0"]
impl crate::Resettable for PIO_POW_MOD_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
