#[doc = "Register `pio_pow_ms_ctl` reader"]
pub type R = crate::R<PIO_POW_MS_CTL_SPEC>;
#[doc = "Register `pio_pow_ms_ctl` writer"]
pub type W = crate::W<PIO_POW_MS_CTL_SPEC>;
#[doc = "Field `vcc_p_ws_vol_mod_sel[C,D,E,F,G]` reader - VCC_PX Withstand Voltage Mode Select Control"]
pub type VCC_P_WS_VOL_MOD_SEL_R = crate::BitReader<VCC_P_WS_VOL_MOD_SEL_A>;
#[doc = "VCC_PX Withstand Voltage Mode Select Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCC_P_WS_VOL_MOD_SEL_A {
    #[doc = "0: `0`"]
    ENABLE = 0,
    #[doc = "1: `1`"]
    DISABLE = 1,
}
impl From<VCC_P_WS_VOL_MOD_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: VCC_P_WS_VOL_MOD_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VCC_P_WS_VOL_MOD_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCC_P_WS_VOL_MOD_SEL_A {
        match self.bits {
            false => VCC_P_WS_VOL_MOD_SEL_A::ENABLE,
            true => VCC_P_WS_VOL_MOD_SEL_A::DISABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VCC_P_WS_VOL_MOD_SEL_A::ENABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VCC_P_WS_VOL_MOD_SEL_A::DISABLE
    }
}
#[doc = "Field `vcc_p_ws_vol_mod_sel[C,D,E,F,G]` writer - VCC_PX Withstand Voltage Mode Select Control"]
pub type VCC_P_WS_VOL_MOD_SEL_W<'a, REG> = crate::BitWriter<'a, REG, VCC_P_WS_VOL_MOD_SEL_A>;
impl<'a, REG> VCC_P_WS_VOL_MOD_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(VCC_P_WS_VOL_MOD_SEL_A::ENABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(VCC_P_WS_VOL_MOD_SEL_A::DISABLE)
    }
}
#[doc = "Field `vccio_ws_vol_mod_sel` reader - VCC_IO Withstand Voltage Mode Select Control"]
pub type VCCIO_WS_VOL_MOD_SEL_R = crate::BitReader<VCCIO_WS_VOL_MOD_SEL_A>;
#[doc = "VCC_IO Withstand Voltage Mode Select Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCCIO_WS_VOL_MOD_SEL_A {
    #[doc = "0: `0`"]
    ENABLE = 0,
    #[doc = "1: `1`"]
    DISABLE = 1,
}
impl From<VCCIO_WS_VOL_MOD_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: VCCIO_WS_VOL_MOD_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VCCIO_WS_VOL_MOD_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCCIO_WS_VOL_MOD_SEL_A {
        match self.bits {
            false => VCCIO_WS_VOL_MOD_SEL_A::ENABLE,
            true => VCCIO_WS_VOL_MOD_SEL_A::DISABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VCCIO_WS_VOL_MOD_SEL_A::ENABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VCCIO_WS_VOL_MOD_SEL_A::DISABLE
    }
}
#[doc = "Field `vccio_ws_vol_mod_sel` writer - VCC_IO Withstand Voltage Mode Select Control"]
pub type VCCIO_WS_VOL_MOD_SEL_W<'a, REG> = crate::BitWriter<'a, REG, VCCIO_WS_VOL_MOD_SEL_A>;
impl<'a, REG> VCCIO_WS_VOL_MOD_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(VCCIO_WS_VOL_MOD_SEL_A::ENABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(VCCIO_WS_VOL_MOD_SEL_A::DISABLE)
    }
}
impl R {
    #[doc = "VCC_PX Withstand Voltage Mode Select Control\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `vcc_pC_ws_vol_mod_sel` field"]
    #[inline(always)]
    pub fn vcc_p_ws_vol_mod_sel(&self, n: u8) -> VCC_P_WS_VOL_MOD_SEL_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        VCC_P_WS_VOL_MOD_SEL_R::new(((self.bits >> (n + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vcc_p_c_ws_vol_mod_sel(&self) -> VCC_P_WS_VOL_MOD_SEL_R {
        VCC_P_WS_VOL_MOD_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vcc_p_d_ws_vol_mod_sel(&self) -> VCC_P_WS_VOL_MOD_SEL_R {
        VCC_P_WS_VOL_MOD_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vcc_p_e_ws_vol_mod_sel(&self) -> VCC_P_WS_VOL_MOD_SEL_R {
        VCC_P_WS_VOL_MOD_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vcc_p_f_ws_vol_mod_sel(&self) -> VCC_P_WS_VOL_MOD_SEL_R {
        VCC_P_WS_VOL_MOD_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vcc_p_g_ws_vol_mod_sel(&self) -> VCC_P_WS_VOL_MOD_SEL_R {
        VCC_P_WS_VOL_MOD_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - VCC_IO Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vccio_ws_vol_mod_sel(&self) -> VCCIO_WS_VOL_MOD_SEL_R {
        VCCIO_WS_VOL_MOD_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "VCC_PX Withstand Voltage Mode Select Control\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `vcc_pC_ws_vol_mod_sel` field"]
    #[inline(always)]
    #[must_use]
    pub fn vcc_p_ws_vol_mod_sel(&mut self, n: u8) -> VCC_P_WS_VOL_MOD_SEL_W<PIO_POW_MS_CTL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        VCC_P_WS_VOL_MOD_SEL_W::new(self, n + 2)
    }
    #[doc = "Bit 2 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    #[must_use]
    pub fn vcc_p_c_ws_vol_mod_sel(&mut self) -> VCC_P_WS_VOL_MOD_SEL_W<PIO_POW_MS_CTL_SPEC> {
        VCC_P_WS_VOL_MOD_SEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    #[must_use]
    pub fn vcc_p_d_ws_vol_mod_sel(&mut self) -> VCC_P_WS_VOL_MOD_SEL_W<PIO_POW_MS_CTL_SPEC> {
        VCC_P_WS_VOL_MOD_SEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    #[must_use]
    pub fn vcc_p_e_ws_vol_mod_sel(&mut self) -> VCC_P_WS_VOL_MOD_SEL_W<PIO_POW_MS_CTL_SPEC> {
        VCC_P_WS_VOL_MOD_SEL_W::new(self, 4)
    }
    #[doc = "Bit 5 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    #[must_use]
    pub fn vcc_p_f_ws_vol_mod_sel(&mut self) -> VCC_P_WS_VOL_MOD_SEL_W<PIO_POW_MS_CTL_SPEC> {
        VCC_P_WS_VOL_MOD_SEL_W::new(self, 5)
    }
    #[doc = "Bit 6 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    #[must_use]
    pub fn vcc_p_g_ws_vol_mod_sel(&mut self) -> VCC_P_WS_VOL_MOD_SEL_W<PIO_POW_MS_CTL_SPEC> {
        VCC_P_WS_VOL_MOD_SEL_W::new(self, 6)
    }
    #[doc = "Bit 12 - VCC_IO Withstand Voltage Mode Select Control"]
    #[inline(always)]
    #[must_use]
    pub fn vccio_ws_vol_mod_sel(&mut self) -> VCCIO_WS_VOL_MOD_SEL_W<PIO_POW_MS_CTL_SPEC> {
        VCCIO_WS_VOL_MOD_SEL_W::new(self, 12)
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
#[doc = "PIO Group Withstand Voltage Mode Select Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio_pow_ms_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio_pow_ms_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIO_POW_MS_CTL_SPEC;
impl crate::RegisterSpec for PIO_POW_MS_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pio_pow_ms_ctl::R`](R) reader structure"]
impl crate::Readable for PIO_POW_MS_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pio_pow_ms_ctl::W`](W) writer structure"]
impl crate::Writable for PIO_POW_MS_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pio_pow_ms_ctl to value 0"]
impl crate::Resettable for PIO_POW_MS_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
