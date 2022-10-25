#[doc = "Register `pio_pow_ms_ctl` reader"]
pub struct R(crate::R<PIO_POW_MS_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO_POW_MS_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO_POW_MS_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO_POW_MS_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pio_pow_ms_ctl` writer"]
pub struct W(crate::W<PIO_POW_MS_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO_POW_MS_CTL_SPEC>;
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
impl From<crate::W<PIO_POW_MS_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO_POW_MS_CTL_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> VCC_P_WS_VOL_MOD_SEL_A {
        match self.bits {
            false => VCC_P_WS_VOL_MOD_SEL_A::ENABLE,
            true => VCC_P_WS_VOL_MOD_SEL_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VCC_P_WS_VOL_MOD_SEL_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VCC_P_WS_VOL_MOD_SEL_A::DISABLE
    }
}
#[doc = "Field `vcc_p_ws_vol_mod_sel[C,D,E,F,G]` writer - VCC_PX Withstand Voltage Mode Select Control"]
pub type VCC_P_WS_VOL_MOD_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PIO_POW_MS_CTL_SPEC, VCC_P_WS_VOL_MOD_SEL_A, O>;
impl<'a, const O: u8> VCC_P_WS_VOL_MOD_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VCC_P_WS_VOL_MOD_SEL_A::ENABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
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
    pub fn variant(&self) -> VCCIO_WS_VOL_MOD_SEL_A {
        match self.bits {
            false => VCCIO_WS_VOL_MOD_SEL_A::ENABLE,
            true => VCCIO_WS_VOL_MOD_SEL_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VCCIO_WS_VOL_MOD_SEL_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VCCIO_WS_VOL_MOD_SEL_A::DISABLE
    }
}
#[doc = "Field `vccio_ws_vol_mod_sel` writer - VCC_IO Withstand Voltage Mode Select Control"]
pub type VCCIO_WS_VOL_MOD_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PIO_POW_MS_CTL_SPEC, VCCIO_WS_VOL_MOD_SEL_A, O>;
impl<'a, const O: u8> VCCIO_WS_VOL_MOD_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VCCIO_WS_VOL_MOD_SEL_A::ENABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VCCIO_WS_VOL_MOD_SEL_A::DISABLE)
    }
}
impl R {
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
    #[doc = "VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn vcc_p_ws_vol_mod_sel<const O: u8>(&mut self) -> VCC_P_WS_VOL_MOD_SEL_W<O> {
        VCC_P_WS_VOL_MOD_SEL_W::new(self)
    }
    #[doc = "Bit 2 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    #[must_use]
    pub fn vcc_p_c_ws_vol_mod_sel(&mut self) -> VCC_P_WS_VOL_MOD_SEL_W<2> {
        VCC_P_WS_VOL_MOD_SEL_W::new(self)
    }
    #[doc = "Bit 3 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    #[must_use]
    pub fn vcc_p_d_ws_vol_mod_sel(&mut self) -> VCC_P_WS_VOL_MOD_SEL_W<3> {
        VCC_P_WS_VOL_MOD_SEL_W::new(self)
    }
    #[doc = "Bit 4 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    #[must_use]
    pub fn vcc_p_e_ws_vol_mod_sel(&mut self) -> VCC_P_WS_VOL_MOD_SEL_W<4> {
        VCC_P_WS_VOL_MOD_SEL_W::new(self)
    }
    #[doc = "Bit 5 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    #[must_use]
    pub fn vcc_p_f_ws_vol_mod_sel(&mut self) -> VCC_P_WS_VOL_MOD_SEL_W<5> {
        VCC_P_WS_VOL_MOD_SEL_W::new(self)
    }
    #[doc = "Bit 6 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    #[must_use]
    pub fn vcc_p_g_ws_vol_mod_sel(&mut self) -> VCC_P_WS_VOL_MOD_SEL_W<6> {
        VCC_P_WS_VOL_MOD_SEL_W::new(self)
    }
    #[doc = "Bit 12 - VCC_IO Withstand Voltage Mode Select Control"]
    #[inline(always)]
    #[must_use]
    pub fn vccio_ws_vol_mod_sel(&mut self) -> VCCIO_WS_VOL_MOD_SEL_W<12> {
        VCCIO_WS_VOL_MOD_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIO Group Withstand Voltage Mode Select Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pow_ms_ctl](index.html) module"]
pub struct PIO_POW_MS_CTL_SPEC;
impl crate::RegisterSpec for PIO_POW_MS_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio_pow_ms_ctl::R](R) reader structure"]
impl crate::Readable for PIO_POW_MS_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio_pow_ms_ctl::W](W) writer structure"]
impl crate::Writable for PIO_POW_MS_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pio_pow_ms_ctl to value 0"]
impl crate::Resettable for PIO_POW_MS_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
