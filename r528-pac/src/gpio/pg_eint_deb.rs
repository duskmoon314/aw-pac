#[doc = "Register `pg_eint_deb` reader"]
pub struct R(crate::R<PG_EINT_DEB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PG_EINT_DEB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PG_EINT_DEB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PG_EINT_DEB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pg_eint_deb` writer"]
pub struct W(crate::W<PG_EINT_DEB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PG_EINT_DEB_SPEC>;
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
impl From<crate::W<PG_EINT_DEB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PG_EINT_DEB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEB_CLK_PRE_SCALE` reader - Debounce Clock Pre_scale n"]
pub struct DEB_CLK_PRE_SCALE_R(crate::FieldReader<u8, u8>);
impl DEB_CLK_PRE_SCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEB_CLK_PRE_SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEB_CLK_PRE_SCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEB_CLK_PRE_SCALE` writer - Debounce Clock Pre_scale n"]
pub struct DEB_CLK_PRE_SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEB_CLK_PRE_SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "PIO Interrupt Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO_INT_CLK_SELECT_A {
    #[doc = "0: `0`"]
    LOSC_32KHZ = 0,
    #[doc = "1: `1`"]
    HOSC_24MHZ = 1,
}
impl From<PIO_INT_CLK_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PIO_INT_CLK_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO_INT_CLK_SELECT` reader - PIO Interrupt Clock Select"]
pub struct PIO_INT_CLK_SELECT_R(crate::FieldReader<bool, PIO_INT_CLK_SELECT_A>);
impl PIO_INT_CLK_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO_INT_CLK_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO_INT_CLK_SELECT_A {
        match self.bits {
            false => PIO_INT_CLK_SELECT_A::LOSC_32KHZ,
            true => PIO_INT_CLK_SELECT_A::HOSC_24MHZ,
        }
    }
    #[doc = "Checks if the value of the field is `LOSC_32KHZ`"]
    #[inline(always)]
    pub fn is_losc_32khz(&self) -> bool {
        **self == PIO_INT_CLK_SELECT_A::LOSC_32KHZ
    }
    #[doc = "Checks if the value of the field is `HOSC_24MHZ`"]
    #[inline(always)]
    pub fn is_hosc_24mhz(&self) -> bool {
        **self == PIO_INT_CLK_SELECT_A::HOSC_24MHZ
    }
}
impl core::ops::Deref for PIO_INT_CLK_SELECT_R {
    type Target = crate::FieldReader<bool, PIO_INT_CLK_SELECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO_INT_CLK_SELECT` writer - PIO Interrupt Clock Select"]
pub struct PIO_INT_CLK_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO_INT_CLK_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO_INT_CLK_SELECT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn losc_32khz(self) -> &'a mut W {
        self.variant(PIO_INT_CLK_SELECT_A::LOSC_32KHZ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn hosc_24mhz(self) -> &'a mut W {
        self.variant(PIO_INT_CLK_SELECT_A::HOSC_24MHZ)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - Debounce Clock Pre_scale n"]
    #[inline(always)]
    pub fn deb_clk_pre_scale(&self) -> DEB_CLK_PRE_SCALE_R {
        DEB_CLK_PRE_SCALE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 0 - PIO Interrupt Clock Select"]
    #[inline(always)]
    pub fn pio_int_clk_select(&self) -> PIO_INT_CLK_SELECT_R {
        PIO_INT_CLK_SELECT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - Debounce Clock Pre_scale n"]
    #[inline(always)]
    pub fn deb_clk_pre_scale(&mut self) -> DEB_CLK_PRE_SCALE_W {
        DEB_CLK_PRE_SCALE_W { w: self }
    }
    #[doc = "Bit 0 - PIO Interrupt Clock Select"]
    #[inline(always)]
    pub fn pio_int_clk_select(&mut self) -> PIO_INT_CLK_SELECT_W {
        PIO_INT_CLK_SELECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PG External Interrupt Debounce Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_eint_deb](index.html) module"]
pub struct PG_EINT_DEB_SPEC;
impl crate::RegisterSpec for PG_EINT_DEB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pg_eint_deb::R](R) reader structure"]
impl crate::Readable for PG_EINT_DEB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pg_eint_deb::W](W) writer structure"]
impl crate::Writable for PG_EINT_DEB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pg_eint_deb to value 0"]
impl crate::Resettable for PG_EINT_DEB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
