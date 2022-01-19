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
#[doc = "VCC_IO Withstand Voltage Mode Select Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `VCCIO_WS_VOL_MOD_SEL` reader - VCC_IO Withstand Voltage Mode Select Control"]
pub struct VCCIO_WS_VOL_MOD_SEL_R(crate::FieldReader<bool, VCCIO_WS_VOL_MOD_SEL_A>);
impl VCCIO_WS_VOL_MOD_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCCIO_WS_VOL_MOD_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == VCCIO_WS_VOL_MOD_SEL_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == VCCIO_WS_VOL_MOD_SEL_A::DISABLE
    }
}
impl core::ops::Deref for VCCIO_WS_VOL_MOD_SEL_R {
    type Target = crate::FieldReader<bool, VCCIO_WS_VOL_MOD_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCCIO_WS_VOL_MOD_SEL` writer - VCC_IO Withstand Voltage Mode Select Control"]
pub struct VCCIO_WS_VOL_MOD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VCCIO_WS_VOL_MOD_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCCIO_WS_VOL_MOD_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "VCC_PX Withstand Voltage Mode Select Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Fields `VCC_P(0-4)_WS_VOL_MOD_SEL` reader - VCC_PX Withstand Voltage Mode Select Control"]
pub struct VCC_P_WS_VOL_MOD_SEL_R(crate::FieldReader<bool, VCC_P_WS_VOL_MOD_SEL_A>);
impl VCC_P_WS_VOL_MOD_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCC_P_WS_VOL_MOD_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == VCC_P_WS_VOL_MOD_SEL_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == VCC_P_WS_VOL_MOD_SEL_A::DISABLE
    }
}
impl core::ops::Deref for VCC_P_WS_VOL_MOD_SEL_R {
    type Target = crate::FieldReader<bool, VCC_P_WS_VOL_MOD_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `VCC_P(0-4)_WS_VOL_MOD_SEL` writer - VCC_PX Withstand Voltage Mode Select Control"]
pub struct VCC_P_WS_VOL_MOD_SEL_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> VCC_P_WS_VOL_MOD_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCC_P_WS_VOL_MOD_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << self.offset)) | ((value as u32 & 0x01) << self.offset);
        self.w
    }
}
#[doc = "Fields `VCC_P(0-4)_WS_VOL_MOD_SEL` const generic writer - VCC_PX Withstand Voltage Mode Select Control"]
pub struct VCC_P_WS_VOL_MOD_SEL_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> VCC_P_WS_VOL_MOD_SEL_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCC_P_WS_VOL_MOD_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << O)) | ((value as u32 & 0x01) << O);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - VCC_IO Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vccio_ws_vol_mod_sel(&self) -> VCCIO_WS_VOL_MOD_SEL_R {
        VCCIO_WS_VOL_MOD_SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub unsafe fn vcc_p_ws_vol_mod_sel(&self, n: usize) -> VCC_P_WS_VOL_MOD_SEL_R {
        VCC_P_WS_VOL_MOD_SEL_R::new(((self.bits >> (n + 2)) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vcc_pc_ws_vol_mod_sel(&self) -> VCC_P_WS_VOL_MOD_SEL_R {
        VCC_P_WS_VOL_MOD_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vcc_pd_ws_vol_mod_sel(&self) -> VCC_P_WS_VOL_MOD_SEL_R {
        VCC_P_WS_VOL_MOD_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vcc_pe_ws_vol_mod_sel(&self) -> VCC_P_WS_VOL_MOD_SEL_R {
        VCC_P_WS_VOL_MOD_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vcc_pf_ws_vol_mod_sel(&self) -> VCC_P_WS_VOL_MOD_SEL_R {
        VCC_P_WS_VOL_MOD_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vcc_pg_ws_vol_mod_sel(&self) -> VCC_P_WS_VOL_MOD_SEL_R {
        VCC_P_WS_VOL_MOD_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - VCC_IO Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vccio_ws_vol_mod_sel(&mut self) -> VCCIO_WS_VOL_MOD_SEL_W {
        VCCIO_WS_VOL_MOD_SEL_W { w: self }
    }
    #[doc = "VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub unsafe fn vcc_p_ws_vol_mod_sel(&mut self, n: usize) -> VCC_P_WS_VOL_MOD_SEL_W {
        VCC_P_WS_VOL_MOD_SEL_W {
            w: self,
            offset: n + 2,
        }
    }
    #[doc = "Bit 2 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vcc_pc_ws_vol_mod_sel(&mut self) -> VCC_P_WS_VOL_MOD_SEL_CGW<2> {
        VCC_P_WS_VOL_MOD_SEL_CGW { w: self }
    }
    #[doc = "Bit 3 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vcc_pd_ws_vol_mod_sel(&mut self) -> VCC_P_WS_VOL_MOD_SEL_CGW<3> {
        VCC_P_WS_VOL_MOD_SEL_CGW { w: self }
    }
    #[doc = "Bit 4 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vcc_pe_ws_vol_mod_sel(&mut self) -> VCC_P_WS_VOL_MOD_SEL_CGW<4> {
        VCC_P_WS_VOL_MOD_SEL_CGW { w: self }
    }
    #[doc = "Bit 5 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vcc_pf_ws_vol_mod_sel(&mut self) -> VCC_P_WS_VOL_MOD_SEL_CGW<5> {
        VCC_P_WS_VOL_MOD_SEL_CGW { w: self }
    }
    #[doc = "Bit 6 - VCC_PX Withstand Voltage Mode Select Control"]
    #[inline(always)]
    pub fn vcc_pg_ws_vol_mod_sel(&mut self) -> VCC_P_WS_VOL_MOD_SEL_CGW<6> {
        VCC_P_WS_VOL_MOD_SEL_CGW { w: self }
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
}
#[doc = "`reset()` method sets pio_pow_ms_ctl to value 0"]
impl crate::Resettable for PIO_POW_MS_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
