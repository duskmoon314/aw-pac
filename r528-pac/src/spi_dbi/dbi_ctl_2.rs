#[doc = "Register `DBI_CTL_2` reader"]
pub struct R(crate::R<DBI_CTL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBI_CTL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBI_CTL_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBI_CTL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBI_CTL_2` writer"]
pub struct W(crate::W<DBI_CTL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBI_CTL_2_SPEC>;
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
impl From<crate::W<DBI_CTL_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBI_CTL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dbi_fifo_drq_en` reader - DBI FIFO DMA Request Enable"]
pub struct DBI_FIFO_DRQ_EN_R(crate::FieldReader<bool, bool>);
impl DBI_FIFO_DRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBI_FIFO_DRQ_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBI_FIFO_DRQ_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_fifo_drq_en` writer - DBI FIFO DMA Request Enable"]
pub struct DBI_FIFO_DRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI_FIFO_DRQ_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `dbi_trig_level` reader - DBI FIFO Empty Request Trigger Level"]
pub struct DBI_TRIG_LEVEL_R(crate::FieldReader<u8, u8>);
impl DBI_TRIG_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBI_TRIG_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBI_TRIG_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_trig_level` writer - DBI FIFO Empty Request Trigger Level"]
pub struct DBI_TRIG_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI_TRIG_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `dbi_sdq_out_sel` reader - DBI SDI PIN Output Select"]
pub struct DBI_SDQ_OUT_SEL_R(crate::FieldReader<bool, bool>);
impl DBI_SDQ_OUT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBI_SDQ_OUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBI_SDQ_OUT_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_sdq_out_sel` writer - DBI SDI PIN Output Select"]
pub struct DBI_SDQ_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI_SDQ_OUT_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `dbi_dcx_sel` reader - DBI DCX PIN Function Select"]
pub struct DBI_DCX_SEL_R(crate::FieldReader<bool, bool>);
impl DBI_DCX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBI_DCX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBI_DCX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_dcx_sel` writer - DBI DCX PIN Function Select"]
pub struct DBI_DCX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI_DCX_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "DBI SDI PIN FUnction Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBI_SDI_SEL_A {
    #[doc = "0: `0`"]
    DBI_SDI = 0,
    #[doc = "1: `1`"]
    DBI_TE = 1,
    #[doc = "2: `10`"]
    DBI_DCX = 2,
}
impl From<DBI_SDI_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DBI_SDI_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `dbi_sdi_sel` reader - DBI SDI PIN FUnction Select"]
pub struct DBI_SDI_SEL_R(crate::FieldReader<u8, DBI_SDI_SEL_A>);
impl DBI_SDI_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBI_SDI_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DBI_SDI_SEL_A> {
        match self.bits {
            0 => Some(DBI_SDI_SEL_A::DBI_SDI),
            1 => Some(DBI_SDI_SEL_A::DBI_TE),
            2 => Some(DBI_SDI_SEL_A::DBI_DCX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DBI_SDI`"]
    #[inline(always)]
    pub fn is_dbi_sdi(&self) -> bool {
        **self == DBI_SDI_SEL_A::DBI_SDI
    }
    #[doc = "Checks if the value of the field is `DBI_TE`"]
    #[inline(always)]
    pub fn is_dbi_te(&self) -> bool {
        **self == DBI_SDI_SEL_A::DBI_TE
    }
    #[doc = "Checks if the value of the field is `DBI_DCX`"]
    #[inline(always)]
    pub fn is_dbi_dcx(&self) -> bool {
        **self == DBI_SDI_SEL_A::DBI_DCX
    }
}
impl core::ops::Deref for DBI_SDI_SEL_R {
    type Target = crate::FieldReader<u8, DBI_SDI_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_sdi_sel` writer - DBI SDI PIN FUnction Select"]
pub struct DBI_SDI_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI_SDI_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBI_SDI_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dbi_sdi(self) -> &'a mut W {
        self.variant(DBI_SDI_SEL_A::DBI_SDI)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn dbi_te(self) -> &'a mut W {
        self.variant(DBI_SDI_SEL_A::DBI_TE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn dbi_dcx(self) -> &'a mut W {
        self.variant(DBI_SDI_SEL_A::DBI_DCX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `te_dbc_sel` reader - TE debounce function select"]
pub struct TE_DBC_SEL_R(crate::FieldReader<bool, bool>);
impl TE_DBC_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TE_DBC_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TE_DBC_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `te_dbc_sel` writer - TE debounce function select"]
pub struct TE_DBC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_DBC_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `te_trig_sel` reader - TE edge trigger select"]
pub struct TE_TRIG_SEL_R(crate::FieldReader<bool, bool>);
impl TE_TRIG_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TE_TRIG_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TE_TRIG_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `te_trig_sel` writer - TE edge trigger select"]
pub struct TE_TRIG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_TRIG_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `te_en` reader - TE Enable"]
pub struct TE_EN_R(crate::FieldReader<bool, bool>);
impl TE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `te_en` writer - TE Enable"]
pub struct TE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_EN_W<'a> {
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
    #[doc = "Bit 15 - DBI FIFO DMA Request Enable"]
    #[inline(always)]
    pub fn dbi_fifo_drq_en(&self) -> DBI_FIFO_DRQ_EN_R {
        DBI_FIFO_DRQ_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - DBI FIFO Empty Request Trigger Level"]
    #[inline(always)]
    pub fn dbi_trig_level(&self) -> DBI_TRIG_LEVEL_R {
        DBI_TRIG_LEVEL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 6 - DBI SDI PIN Output Select"]
    #[inline(always)]
    pub fn dbi_sdq_out_sel(&self) -> DBI_SDQ_OUT_SEL_R {
        DBI_SDQ_OUT_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DBI DCX PIN Function Select"]
    #[inline(always)]
    pub fn dbi_dcx_sel(&self) -> DBI_DCX_SEL_R {
        DBI_DCX_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - DBI SDI PIN FUnction Select"]
    #[inline(always)]
    pub fn dbi_sdi_sel(&self) -> DBI_SDI_SEL_R {
        DBI_SDI_SEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - TE debounce function select"]
    #[inline(always)]
    pub fn te_dbc_sel(&self) -> TE_DBC_SEL_R {
        TE_DBC_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TE edge trigger select"]
    #[inline(always)]
    pub fn te_trig_sel(&self) -> TE_TRIG_SEL_R {
        TE_TRIG_SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - TE Enable"]
    #[inline(always)]
    pub fn te_en(&self) -> TE_EN_R {
        TE_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - DBI FIFO DMA Request Enable"]
    #[inline(always)]
    pub fn dbi_fifo_drq_en(&mut self) -> DBI_FIFO_DRQ_EN_W {
        DBI_FIFO_DRQ_EN_W { w: self }
    }
    #[doc = "Bits 8:14 - DBI FIFO Empty Request Trigger Level"]
    #[inline(always)]
    pub fn dbi_trig_level(&mut self) -> DBI_TRIG_LEVEL_W {
        DBI_TRIG_LEVEL_W { w: self }
    }
    #[doc = "Bit 6 - DBI SDI PIN Output Select"]
    #[inline(always)]
    pub fn dbi_sdq_out_sel(&mut self) -> DBI_SDQ_OUT_SEL_W {
        DBI_SDQ_OUT_SEL_W { w: self }
    }
    #[doc = "Bit 5 - DBI DCX PIN Function Select"]
    #[inline(always)]
    pub fn dbi_dcx_sel(&mut self) -> DBI_DCX_SEL_W {
        DBI_DCX_SEL_W { w: self }
    }
    #[doc = "Bits 3:4 - DBI SDI PIN FUnction Select"]
    #[inline(always)]
    pub fn dbi_sdi_sel(&mut self) -> DBI_SDI_SEL_W {
        DBI_SDI_SEL_W { w: self }
    }
    #[doc = "Bit 2 - TE debounce function select"]
    #[inline(always)]
    pub fn te_dbc_sel(&mut self) -> TE_DBC_SEL_W {
        TE_DBC_SEL_W { w: self }
    }
    #[doc = "Bit 1 - TE edge trigger select"]
    #[inline(always)]
    pub fn te_trig_sel(&mut self) -> TE_TRIG_SEL_W {
        TE_TRIG_SEL_W { w: self }
    }
    #[doc = "Bit 0 - TE Enable"]
    #[inline(always)]
    pub fn te_en(&mut self) -> TE_EN_W {
        TE_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBI Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbi_ctl_2](index.html) module"]
pub struct DBI_CTL_2_SPEC;
impl crate::RegisterSpec for DBI_CTL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbi_ctl_2::R](R) reader structure"]
impl crate::Readable for DBI_CTL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbi_ctl_2::W](W) writer structure"]
impl crate::Writable for DBI_CTL_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBI_CTL_2 to value 0"]
impl crate::Resettable for DBI_CTL_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
