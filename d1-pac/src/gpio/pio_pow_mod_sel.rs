#[doc = "Register `pio_pow_mod_sel` reader"]
pub type R = crate::R<PIO_POW_MOD_SEL_SPEC>;
#[doc = "Register `pio_pow_mod_sel` writer"]
pub type W = crate::W<PIO_POW_MOD_SEL_SPEC>;
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
    pub const fn variant(&self) -> P_PWR_MOD_SEL_A {
        match self.bits {
            false => P_PWR_MOD_SEL_A::V33,
            true => P_PWR_MOD_SEL_A::V18,
        }
    }
    #[doc = "3.3 V"]
    #[inline(always)]
    pub fn is_v33(&self) -> bool {
        *self == P_PWR_MOD_SEL_A::V33
    }
    #[doc = "1.8 V"]
    #[inline(always)]
    pub fn is_v18(&self) -> bool {
        *self == P_PWR_MOD_SEL_A::V18
    }
}
#[doc = "Field `p_pwr_mod_sel[C,D,E,F,G]` writer - PX_POWER POWER MODE Select"]
pub type P_PWR_MOD_SEL_W<'a, REG> = crate::BitWriter<'a, REG, P_PWR_MOD_SEL_A>;
impl<'a, REG> P_PWR_MOD_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "3.3 V"]
    #[inline(always)]
    pub fn v33(self) -> &'a mut crate::W<REG> {
        self.variant(P_PWR_MOD_SEL_A::V33)
    }
    #[doc = "1.8 V"]
    #[inline(always)]
    pub fn v18(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> VCC_IO_PWR_MOD_SEL_A {
        match self.bits {
            false => VCC_IO_PWR_MOD_SEL_A::V33,
            true => VCC_IO_PWR_MOD_SEL_A::V18,
        }
    }
    #[doc = "3.3 V"]
    #[inline(always)]
    pub fn is_v33(&self) -> bool {
        *self == VCC_IO_PWR_MOD_SEL_A::V33
    }
    #[doc = "1.8 V"]
    #[inline(always)]
    pub fn is_v18(&self) -> bool {
        *self == VCC_IO_PWR_MOD_SEL_A::V18
    }
}
#[doc = "Field `vcc_io_pwr_mod_sel` writer - VCC_IO POWER MODE Select"]
pub type VCC_IO_PWR_MOD_SEL_W<'a, REG> = crate::BitWriter<'a, REG, VCC_IO_PWR_MOD_SEL_A>;
impl<'a, REG> VCC_IO_PWR_MOD_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "3.3 V"]
    #[inline(always)]
    pub fn v33(self) -> &'a mut crate::W<REG> {
        self.variant(VCC_IO_PWR_MOD_SEL_A::V33)
    }
    #[doc = "1.8 V"]
    #[inline(always)]
    pub fn v18(self) -> &'a mut crate::W<REG> {
        self.variant(VCC_IO_PWR_MOD_SEL_A::V18)
    }
}
impl R {
    #[doc = "PX_POWER POWER MODE Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pC_pwr_mod_sel` field"]
    #[inline(always)]
    pub fn p_pwr_mod_sel(&self, n: u8) -> P_PWR_MOD_SEL_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        P_PWR_MOD_SEL_R::new(((self.bits >> (n + 2)) & 1) != 0)
    }
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
    #[doc = "PX_POWER POWER MODE Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pC_pwr_mod_sel` field"]
    #[inline(always)]
    #[must_use]
    pub fn p_pwr_mod_sel(&mut self, n: u8) -> P_PWR_MOD_SEL_W<PIO_POW_MOD_SEL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        P_PWR_MOD_SEL_W::new(self, n + 2)
    }
    #[doc = "Bit 2 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    #[must_use]
    pub fn p_c_pwr_mod_sel(&mut self) -> P_PWR_MOD_SEL_W<PIO_POW_MOD_SEL_SPEC> {
        P_PWR_MOD_SEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    #[must_use]
    pub fn p_d_pwr_mod_sel(&mut self) -> P_PWR_MOD_SEL_W<PIO_POW_MOD_SEL_SPEC> {
        P_PWR_MOD_SEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    #[must_use]
    pub fn p_e_pwr_mod_sel(&mut self) -> P_PWR_MOD_SEL_W<PIO_POW_MOD_SEL_SPEC> {
        P_PWR_MOD_SEL_W::new(self, 4)
    }
    #[doc = "Bit 5 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    #[must_use]
    pub fn p_f_pwr_mod_sel(&mut self) -> P_PWR_MOD_SEL_W<PIO_POW_MOD_SEL_SPEC> {
        P_PWR_MOD_SEL_W::new(self, 5)
    }
    #[doc = "Bit 6 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    #[must_use]
    pub fn p_g_pwr_mod_sel(&mut self) -> P_PWR_MOD_SEL_W<PIO_POW_MOD_SEL_SPEC> {
        P_PWR_MOD_SEL_W::new(self, 6)
    }
    #[doc = "Bit 12 - VCC_IO POWER MODE Select"]
    #[inline(always)]
    #[must_use]
    pub fn vcc_io_pwr_mod_sel(&mut self) -> VCC_IO_PWR_MOD_SEL_W<PIO_POW_MOD_SEL_SPEC> {
        VCC_IO_PWR_MOD_SEL_W::new(self, 12)
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
#[doc = "PIO Group Withstand Voltage Mode Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio_pow_mod_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio_pow_mod_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIO_POW_MOD_SEL_SPEC;
impl crate::RegisterSpec for PIO_POW_MOD_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pio_pow_mod_sel::R`](R) reader structure"]
impl crate::Readable for PIO_POW_MOD_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pio_pow_mod_sel::W`](W) writer structure"]
impl crate::Writable for PIO_POW_MOD_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pio_pow_mod_sel to value 0"]
impl crate::Resettable for PIO_POW_MOD_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
