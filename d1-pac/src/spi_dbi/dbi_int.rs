#[doc = "Register `DBI_INT` reader"]
pub struct R(crate::R<DBI_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBI_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBI_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBI_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBI_INT` writer"]
pub struct W(crate::W<DBI_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBI_INT_SPEC>;
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
impl From<crate::W<DBI_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBI_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dbi_fifo_empty_int` reader - "]
pub struct DBI_FIFO_EMPTY_INT_R(crate::FieldReader<bool, bool>);
impl DBI_FIFO_EMPTY_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBI_FIFO_EMPTY_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBI_FIFO_EMPTY_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_fifo_empty_int` writer - "]
pub struct DBI_FIFO_EMPTY_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI_FIFO_EMPTY_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `dbi_fifo_full_int` reader - "]
pub struct DBI_FIFO_FULL_INT_R(crate::FieldReader<bool, bool>);
impl DBI_FIFO_FULL_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBI_FIFO_FULL_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBI_FIFO_FULL_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_fifo_full_int` writer - "]
pub struct DBI_FIFO_FULL_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI_FIFO_FULL_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `timer_int` reader - "]
pub struct TIMER_INT_R(crate::FieldReader<bool, bool>);
impl TIMER_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer_int` writer - "]
pub struct TIMER_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `rd_done_int` reader - "]
pub struct RD_DONE_INT_R(crate::FieldReader<bool, bool>);
impl RD_DONE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_DONE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_DONE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rd_done_int` writer - "]
pub struct RD_DONE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_DONE_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `te_int` reader - "]
pub struct TE_INT_R(crate::FieldReader<bool, bool>);
impl TE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `te_int` writer - "]
pub struct TE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `fram_done_int` reader - "]
pub struct FRAM_DONE_INT_R(crate::FieldReader<bool, bool>);
impl FRAM_DONE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRAM_DONE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAM_DONE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fram_done_int` writer - "]
pub struct FRAM_DONE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAM_DONE_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `line_done_int` reader - "]
pub struct LINE_DONE_INT_R(crate::FieldReader<bool, bool>);
impl LINE_DONE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINE_DONE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINE_DONE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `line_done_int` writer - "]
pub struct LINE_DONE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_DONE_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `dbi_fifo_empty_int_en` reader - "]
pub struct DBI_FIFO_EMPTY_INT_EN_R(crate::FieldReader<bool, bool>);
impl DBI_FIFO_EMPTY_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBI_FIFO_EMPTY_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBI_FIFO_EMPTY_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_fifo_empty_int_en` writer - "]
pub struct DBI_FIFO_EMPTY_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI_FIFO_EMPTY_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `dbi_fifo_full_int_en` reader - "]
pub struct DBI_FIFO_FULL_INT_EN_R(crate::FieldReader<bool, bool>);
impl DBI_FIFO_FULL_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBI_FIFO_FULL_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBI_FIFO_FULL_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_fifo_full_int_en` writer - "]
pub struct DBI_FIFO_FULL_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI_FIFO_FULL_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `timer_int_en` reader - "]
pub struct TIMER_INT_EN_R(crate::FieldReader<bool, bool>);
impl TIMER_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer_int_en` writer - "]
pub struct TIMER_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `rd_done_int_en` reader - "]
pub struct RD_DONE_INT_EN_R(crate::FieldReader<bool, bool>);
impl RD_DONE_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_DONE_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_DONE_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rd_done_int_en` writer - "]
pub struct RD_DONE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_DONE_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `te_int_en` reader - "]
pub struct TE_INT_EN_R(crate::FieldReader<bool, bool>);
impl TE_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TE_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TE_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `te_int_en` writer - "]
pub struct TE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `fram_done_int_en` reader - "]
pub struct FRAM_DONE_INT_EN_R(crate::FieldReader<bool, bool>);
impl FRAM_DONE_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRAM_DONE_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAM_DONE_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fram_done_int_en` writer - "]
pub struct FRAM_DONE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAM_DONE_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `line_done_int_en` reader - "]
pub struct LINE_DONE_INT_EN_R(crate::FieldReader<bool, bool>);
impl LINE_DONE_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINE_DONE_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINE_DONE_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `line_done_int_en` writer - "]
pub struct LINE_DONE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_DONE_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dbi_fifo_empty_int(&self) -> DBI_FIFO_EMPTY_INT_R {
        DBI_FIFO_EMPTY_INT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dbi_fifo_full_int(&self) -> DBI_FIFO_FULL_INT_R {
        DBI_FIFO_FULL_INT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn timer_int(&self) -> TIMER_INT_R {
        TIMER_INT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rd_done_int(&self) -> RD_DONE_INT_R {
        RD_DONE_INT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn te_int(&self) -> TE_INT_R {
        TE_INT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fram_done_int(&self) -> FRAM_DONE_INT_R {
        FRAM_DONE_INT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn line_done_int(&self) -> LINE_DONE_INT_R {
        LINE_DONE_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dbi_fifo_empty_int_en(&self) -> DBI_FIFO_EMPTY_INT_EN_R {
        DBI_FIFO_EMPTY_INT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dbi_fifo_full_int_en(&self) -> DBI_FIFO_FULL_INT_EN_R {
        DBI_FIFO_FULL_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn timer_int_en(&self) -> TIMER_INT_EN_R {
        TIMER_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rd_done_int_en(&self) -> RD_DONE_INT_EN_R {
        RD_DONE_INT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn te_int_en(&self) -> TE_INT_EN_R {
        TE_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fram_done_int_en(&self) -> FRAM_DONE_INT_EN_R {
        FRAM_DONE_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn line_done_int_en(&self) -> LINE_DONE_INT_EN_R {
        LINE_DONE_INT_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dbi_fifo_empty_int(&mut self) -> DBI_FIFO_EMPTY_INT_W {
        DBI_FIFO_EMPTY_INT_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dbi_fifo_full_int(&mut self) -> DBI_FIFO_FULL_INT_W {
        DBI_FIFO_FULL_INT_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn timer_int(&mut self) -> TIMER_INT_W {
        TIMER_INT_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rd_done_int(&mut self) -> RD_DONE_INT_W {
        RD_DONE_INT_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn te_int(&mut self) -> TE_INT_W {
        TE_INT_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fram_done_int(&mut self) -> FRAM_DONE_INT_W {
        FRAM_DONE_INT_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn line_done_int(&mut self) -> LINE_DONE_INT_W {
        LINE_DONE_INT_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dbi_fifo_empty_int_en(&mut self) -> DBI_FIFO_EMPTY_INT_EN_W {
        DBI_FIFO_EMPTY_INT_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dbi_fifo_full_int_en(&mut self) -> DBI_FIFO_FULL_INT_EN_W {
        DBI_FIFO_FULL_INT_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn timer_int_en(&mut self) -> TIMER_INT_EN_W {
        TIMER_INT_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rd_done_int_en(&mut self) -> RD_DONE_INT_EN_W {
        RD_DONE_INT_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn te_int_en(&mut self) -> TE_INT_EN_W {
        TE_INT_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fram_done_int_en(&mut self) -> FRAM_DONE_INT_EN_W {
        FRAM_DONE_INT_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn line_done_int_en(&mut self) -> LINE_DONE_INT_EN_W {
        LINE_DONE_INT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBI Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbi_int](index.html) module"]
pub struct DBI_INT_SPEC;
impl crate::RegisterSpec for DBI_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbi_int::R](R) reader structure"]
impl crate::Readable for DBI_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbi_int::W](W) writer structure"]
impl crate::Writable for DBI_INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBI_INT to value 0"]
impl crate::Resettable for DBI_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
