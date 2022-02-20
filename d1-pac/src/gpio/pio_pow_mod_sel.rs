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
#[doc = "VCC_IO POWER MODE Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `VCC_IO_PWR_MOD_SEL` reader - VCC_IO POWER MODE Select"]
pub struct VCC_IO_PWR_MOD_SEL_R(crate::FieldReader<bool, VCC_IO_PWR_MOD_SEL_A>);
impl VCC_IO_PWR_MOD_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCC_IO_PWR_MOD_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == VCC_IO_PWR_MOD_SEL_A::V33
    }
    #[doc = "Checks if the value of the field is `V18`"]
    #[inline(always)]
    pub fn is_v18(&self) -> bool {
        **self == VCC_IO_PWR_MOD_SEL_A::V18
    }
}
impl core::ops::Deref for VCC_IO_PWR_MOD_SEL_R {
    type Target = crate::FieldReader<bool, VCC_IO_PWR_MOD_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCC_IO_PWR_MOD_SEL` writer - VCC_IO POWER MODE Select"]
pub struct VCC_IO_PWR_MOD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VCC_IO_PWR_MOD_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCC_IO_PWR_MOD_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
#[doc = "PX_POWER POWER MODE Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Fields `P(0-4)_PWR_MOD_SEL` reader - PX_POWER POWER MODE Select"]
pub struct P_PWR_MOD_SEL_R(crate::FieldReader<bool, P_PWR_MOD_SEL_A>);
impl P_PWR_MOD_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P_PWR_MOD_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == P_PWR_MOD_SEL_A::V33
    }
    #[doc = "Checks if the value of the field is `V18`"]
    #[inline(always)]
    pub fn is_v18(&self) -> bool {
        **self == P_PWR_MOD_SEL_A::V18
    }
}
impl core::ops::Deref for P_PWR_MOD_SEL_R {
    type Target = crate::FieldReader<bool, P_PWR_MOD_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `P(0-4)_PWR_MOD_SEL` writer - PX_POWER POWER MODE Select"]
pub struct P_PWR_MOD_SEL_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> P_PWR_MOD_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P_PWR_MOD_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
#[doc = "Fields `P(0-4)_PWR_MOD_SEL` const generic writer - PX_POWER POWER MODE Select"]
pub struct P_PWR_MOD_SEL_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> P_PWR_MOD_SEL_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P_PWR_MOD_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = "Bit 12 - VCC_IO POWER MODE Select"]
    #[inline(always)]
    pub fn vcc_io_pwr_mod_sel(&self) -> VCC_IO_PWR_MOD_SEL_R {
        VCC_IO_PWR_MOD_SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "PX_POWER POWER MODE Select"]
    #[inline(always)]
    pub unsafe fn p_pwr_mod_sel(&self, n: usize) -> P_PWR_MOD_SEL_R {
        P_PWR_MOD_SEL_R::new(((self.bits >> (n + 2)) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    pub fn pc_pwr_mod_sel(&self) -> P_PWR_MOD_SEL_R {
        P_PWR_MOD_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    pub fn pd_pwr_mod_sel(&self) -> P_PWR_MOD_SEL_R {
        P_PWR_MOD_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    pub fn pe_pwr_mod_sel(&self) -> P_PWR_MOD_SEL_R {
        P_PWR_MOD_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    pub fn pf_pwr_mod_sel(&self) -> P_PWR_MOD_SEL_R {
        P_PWR_MOD_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    pub fn pg_pwr_mod_sel(&self) -> P_PWR_MOD_SEL_R {
        P_PWR_MOD_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - VCC_IO POWER MODE Select"]
    #[inline(always)]
    pub fn vcc_io_pwr_mod_sel(&mut self) -> VCC_IO_PWR_MOD_SEL_W {
        VCC_IO_PWR_MOD_SEL_W { w: self }
    }
    #[doc = "PX_POWER POWER MODE Select"]
    #[inline(always)]
    pub unsafe fn p_pwr_mod_sel(&mut self, n: usize) -> P_PWR_MOD_SEL_W {
        P_PWR_MOD_SEL_W {
            w: self,
            offset: n + 2,
        }
    }
    #[doc = "Bit 2 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    pub fn pc_pwr_mod_sel(&mut self) -> P_PWR_MOD_SEL_CGW<2> {
        P_PWR_MOD_SEL_CGW { w: self }
    }
    #[doc = "Bit 3 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    pub fn pd_pwr_mod_sel(&mut self) -> P_PWR_MOD_SEL_CGW<3> {
        P_PWR_MOD_SEL_CGW { w: self }
    }
    #[doc = "Bit 4 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    pub fn pe_pwr_mod_sel(&mut self) -> P_PWR_MOD_SEL_CGW<4> {
        P_PWR_MOD_SEL_CGW { w: self }
    }
    #[doc = "Bit 5 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    pub fn pf_pwr_mod_sel(&mut self) -> P_PWR_MOD_SEL_CGW<5> {
        P_PWR_MOD_SEL_CGW { w: self }
    }
    #[doc = "Bit 6 - PX_POWER POWER MODE Select"]
    #[inline(always)]
    pub fn pg_pwr_mod_sel(&mut self) -> P_PWR_MOD_SEL_CGW<6> {
        P_PWR_MOD_SEL_CGW { w: self }
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
}
#[doc = "`reset()` method sets pio_pow_mod_sel to value 0"]
impl crate::Resettable for PIO_POW_MOD_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
