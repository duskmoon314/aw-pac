#[doc = "Register `SMHC_RINTSTS` reader"]
pub struct R(crate::R<SMHC_RINTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_RINTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_RINTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_RINTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_RINTSTS` writer"]
pub struct W(crate::W<SMHC_RINTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_RINTSTS_SPEC>;
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
impl From<crate::W<SMHC_RINTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_RINTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARD_REMOVAL` reader - Card Removed"]
pub struct CARD_REMOVAL_R(crate::FieldReader<bool>);
impl CARD_REMOVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARD_REMOVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARD_REMOVAL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_REMOVAL` writer - Card Removed"]
pub struct CARD_REMOVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_REMOVAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
#[doc = "Field `CARD_INSERT` reader - Card Inserted"]
pub struct CARD_INSERT_R(crate::FieldReader<bool>);
impl CARD_INSERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARD_INSERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARD_INSERT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_INSERT` writer - Card Inserted"]
pub struct CARD_INSERT_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_INSERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `SDIOI_INT` reader - SDIO Interrupt"]
pub struct SDIOI_INT_R(crate::FieldReader<bool>);
impl SDIOI_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIOI_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIOI_INT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIOI_INT` writer - SDIO Interrupt"]
pub struct SDIOI_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOI_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `DEE` reader - Data End-bit Error"]
pub struct DEE_R(crate::FieldReader<bool>);
impl DEE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEE` writer - Data End-bit Error"]
pub struct DEE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `ACD` reader - Auto Command Done"]
pub struct ACD_R(crate::FieldReader<bool>);
impl ACD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACD` writer - Auto Command Done"]
pub struct ACD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACD_W<'a> {
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
#[doc = "Field `DSE_BC` reader - Data Start Error/Busy Clear"]
pub struct DSE_BC_R(crate::FieldReader<bool>);
impl DSE_BC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSE_BC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSE_BC_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSE_BC` writer - Data Start Error/Busy Clear"]
pub struct DSE_BC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSE_BC_W<'a> {
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
#[doc = "Field `CB_IW` reader - Command Busy and Illegal Write"]
pub struct CB_IW_R(crate::FieldReader<bool>);
impl CB_IW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CB_IW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CB_IW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CB_IW` writer - Command Busy and Illegal Write"]
pub struct CB_IW_W<'a> {
    w: &'a mut W,
}
impl<'a> CB_IW_W<'a> {
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
#[doc = "Field `FU_FO` reader - FIFO Underrun/Overflow"]
pub struct FU_FO_R(crate::FieldReader<bool>);
impl FU_FO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FU_FO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FU_FO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FU_FO` writer - FIFO Underrun/Overflow"]
pub struct FU_FO_W<'a> {
    w: &'a mut W,
}
impl<'a> FU_FO_W<'a> {
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
#[doc = "Field `DSTO_VSD` reader - Data Starvation Timeout/V1.8 Switch Done"]
pub struct DSTO_VSD_R(crate::FieldReader<bool>);
impl DSTO_VSD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSTO_VSD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSTO_VSD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSTO_VSD` writer - Data Starvation Timeout/V1.8 Switch Done"]
pub struct DSTO_VSD_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTO_VSD_W<'a> {
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
#[doc = "Field `DTO_BDS` reader - Data Timeout/Boot Data Start"]
pub struct DTO_BDS_R(crate::FieldReader<bool>);
impl DTO_BDS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTO_BDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTO_BDS_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTO_BDS` writer - Data Timeout/Boot Data Start"]
pub struct DTO_BDS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTO_BDS_W<'a> {
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
#[doc = "Field `RTO_BACK` reader - Response Timeout/Boot ACK Received"]
pub struct RTO_BACK_R(crate::FieldReader<bool>);
impl RTO_BACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTO_BACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTO_BACK_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTO_BACK` writer - Response Timeout/Boot ACK Received"]
pub struct RTO_BACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RTO_BACK_W<'a> {
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
#[doc = "Field `DCE` reader - Data CRC Error"]
pub struct DCE_R(crate::FieldReader<bool>);
impl DCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCE` writer - Data CRC Error"]
pub struct DCE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `RCE` reader - Response CRC Error"]
pub struct RCE_R(crate::FieldReader<bool>);
impl RCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCE` writer - Response CRC Error"]
pub struct RCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RCE_W<'a> {
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
#[doc = "Field `DRR` reader - Data Receive Request"]
pub struct DRR_R(crate::FieldReader<bool>);
impl DRR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRR` writer - Data Receive Request"]
pub struct DRR_W<'a> {
    w: &'a mut W,
}
impl<'a> DRR_W<'a> {
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
#[doc = "Field `DTR` reader - Data Transmit Request"]
pub struct DTR_R(crate::FieldReader<bool>);
impl DTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTR` writer - Data Transmit Request"]
pub struct DTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTR_W<'a> {
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
#[doc = "Field `DTC` reader - Data Transfer Complete"]
pub struct DTC_R(crate::FieldReader<bool>);
impl DTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTC_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTC` writer - Data Transfer Complete"]
pub struct DTC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTC_W<'a> {
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
#[doc = "Field `CC` reader - Command Complete"]
pub struct CC_R(crate::FieldReader<bool>);
impl CC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC` writer - Command Complete"]
pub struct CC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_W<'a> {
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
#[doc = "Field `RE` reader - Response Error"]
pub struct RE_R(crate::FieldReader<bool>);
impl RE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RE` writer - Response Error"]
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
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
impl R {
    #[doc = "Bit 31 - Card Removed"]
    #[inline(always)]
    pub fn card_removal(&self) -> CARD_REMOVAL_R {
        CARD_REMOVAL_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Card Inserted"]
    #[inline(always)]
    pub fn card_insert(&self) -> CARD_INSERT_R {
        CARD_INSERT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 16 - SDIO Interrupt"]
    #[inline(always)]
    pub fn sdioi_int(&self) -> SDIOI_INT_R {
        SDIOI_INT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Data End-bit Error"]
    #[inline(always)]
    pub fn dee(&self) -> DEE_R {
        DEE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Auto Command Done"]
    #[inline(always)]
    pub fn acd(&self) -> ACD_R {
        ACD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Data Start Error/Busy Clear"]
    #[inline(always)]
    pub fn dse_bc(&self) -> DSE_BC_R {
        DSE_BC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Command Busy and Illegal Write"]
    #[inline(always)]
    pub fn cb_iw(&self) -> CB_IW_R {
        CB_IW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - FIFO Underrun/Overflow"]
    #[inline(always)]
    pub fn fu_fo(&self) -> FU_FO_R {
        FU_FO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Starvation Timeout/V1.8 Switch Done"]
    #[inline(always)]
    pub fn dsto_vsd(&self) -> DSTO_VSD_R {
        DSTO_VSD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Data Timeout/Boot Data Start"]
    #[inline(always)]
    pub fn dto_bds(&self) -> DTO_BDS_R {
        DTO_BDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Response Timeout/Boot ACK Received"]
    #[inline(always)]
    pub fn rto_back(&self) -> RTO_BACK_R {
        RTO_BACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Data CRC Error"]
    #[inline(always)]
    pub fn dce(&self) -> DCE_R {
        DCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Response CRC Error"]
    #[inline(always)]
    pub fn rce(&self) -> RCE_R {
        RCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Receive Request"]
    #[inline(always)]
    pub fn drr(&self) -> DRR_R {
        DRR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Transmit Request"]
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Transfer Complete"]
    #[inline(always)]
    pub fn dtc(&self) -> DTC_R {
        DTC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Command Complete"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Response Error"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Card Removed"]
    #[inline(always)]
    pub fn card_removal(&mut self) -> CARD_REMOVAL_W {
        CARD_REMOVAL_W { w: self }
    }
    #[doc = "Bit 30 - Card Inserted"]
    #[inline(always)]
    pub fn card_insert(&mut self) -> CARD_INSERT_W {
        CARD_INSERT_W { w: self }
    }
    #[doc = "Bit 16 - SDIO Interrupt"]
    #[inline(always)]
    pub fn sdioi_int(&mut self) -> SDIOI_INT_W {
        SDIOI_INT_W { w: self }
    }
    #[doc = "Bit 15 - Data End-bit Error"]
    #[inline(always)]
    pub fn dee(&mut self) -> DEE_W {
        DEE_W { w: self }
    }
    #[doc = "Bit 14 - Auto Command Done"]
    #[inline(always)]
    pub fn acd(&mut self) -> ACD_W {
        ACD_W { w: self }
    }
    #[doc = "Bit 13 - Data Start Error/Busy Clear"]
    #[inline(always)]
    pub fn dse_bc(&mut self) -> DSE_BC_W {
        DSE_BC_W { w: self }
    }
    #[doc = "Bit 12 - Command Busy and Illegal Write"]
    #[inline(always)]
    pub fn cb_iw(&mut self) -> CB_IW_W {
        CB_IW_W { w: self }
    }
    #[doc = "Bit 11 - FIFO Underrun/Overflow"]
    #[inline(always)]
    pub fn fu_fo(&mut self) -> FU_FO_W {
        FU_FO_W { w: self }
    }
    #[doc = "Bit 10 - Data Starvation Timeout/V1.8 Switch Done"]
    #[inline(always)]
    pub fn dsto_vsd(&mut self) -> DSTO_VSD_W {
        DSTO_VSD_W { w: self }
    }
    #[doc = "Bit 9 - Data Timeout/Boot Data Start"]
    #[inline(always)]
    pub fn dto_bds(&mut self) -> DTO_BDS_W {
        DTO_BDS_W { w: self }
    }
    #[doc = "Bit 8 - Response Timeout/Boot ACK Received"]
    #[inline(always)]
    pub fn rto_back(&mut self) -> RTO_BACK_W {
        RTO_BACK_W { w: self }
    }
    #[doc = "Bit 7 - Data CRC Error"]
    #[inline(always)]
    pub fn dce(&mut self) -> DCE_W {
        DCE_W { w: self }
    }
    #[doc = "Bit 6 - Response CRC Error"]
    #[inline(always)]
    pub fn rce(&mut self) -> RCE_W {
        RCE_W { w: self }
    }
    #[doc = "Bit 5 - Data Receive Request"]
    #[inline(always)]
    pub fn drr(&mut self) -> DRR_W {
        DRR_W { w: self }
    }
    #[doc = "Bit 4 - Data Transmit Request"]
    #[inline(always)]
    pub fn dtr(&mut self) -> DTR_W {
        DTR_W { w: self }
    }
    #[doc = "Bit 3 - Data Transfer Complete"]
    #[inline(always)]
    pub fn dtc(&mut self) -> DTC_W {
        DTC_W { w: self }
    }
    #[doc = "Bit 2 - Command Complete"]
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W {
        CC_W { w: self }
    }
    #[doc = "Bit 1 - Response Error"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Raw Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_rintsts](index.html) module"]
pub struct SMHC_RINTSTS_SPEC;
impl crate::RegisterSpec for SMHC_RINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_rintsts::R](R) reader structure"]
impl crate::Readable for SMHC_RINTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_rintsts::W](W) writer structure"]
impl crate::Writable for SMHC_RINTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_RINTSTS to value 0"]
impl crate::Resettable for SMHC_RINTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
