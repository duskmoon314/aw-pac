#[doc = "Register `SMHC_INTMASK` reader"]
pub struct R(crate::R<SMHC_INTMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_INTMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_INTMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_INTMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_INTMASK` writer"]
pub struct W(crate::W<SMHC_INTMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_INTMASK_SPEC>;
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
impl From<crate::W<SMHC_INTMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_INTMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARD_REMOVAL_INT_EN` reader - Card Removed Interrupt Enable"]
pub struct CARD_REMOVAL_INT_EN_R(crate::FieldReader<bool, bool>);
impl CARD_REMOVAL_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARD_REMOVAL_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARD_REMOVAL_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_REMOVAL_INT_EN` writer - Card Removed Interrupt Enable"]
pub struct CARD_REMOVAL_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_REMOVAL_INT_EN_W<'a> {
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
#[doc = "Field `CARD_INSERT_INT_EN` reader - Card Inserted Interrupt Enable"]
pub struct CARD_INSERT_INT_EN_R(crate::FieldReader<bool, bool>);
impl CARD_INSERT_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARD_INSERT_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARD_INSERT_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_INSERT_INT_EN` writer - Card Inserted Interrupt Enable"]
pub struct CARD_INSERT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_INSERT_INT_EN_W<'a> {
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
#[doc = "Field `SDIO_INT_EN` reader - SDIO Interrupt Enable"]
pub struct SDIO_INT_EN_R(crate::FieldReader<bool, bool>);
impl SDIO_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_INT_EN` writer - SDIO Interrupt Enable"]
pub struct SDIO_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_INT_EN_W<'a> {
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
#[doc = "Field `DEE_INT_EN` reader - Data End-bit Error Interrupt Enable"]
pub struct DEE_INT_EN_R(crate::FieldReader<bool, bool>);
impl DEE_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEE_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEE_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEE_INT_EN` writer - Data End-bit Error Interrupt Enable"]
pub struct DEE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEE_INT_EN_W<'a> {
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
#[doc = "Field `ACD_INT_EN` reader - Auto Command Done Interrupt Enable"]
pub struct ACD_INT_EN_R(crate::FieldReader<bool, bool>);
impl ACD_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACD_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACD_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACD_INT_EN` writer - Auto Command Done Interrupt Enable"]
pub struct ACD_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACD_INT_EN_W<'a> {
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
#[doc = "Field `DSE_BC_INT_EN` reader - Data Start Error Interrupt Enable"]
pub struct DSE_BC_INT_EN_R(crate::FieldReader<bool, bool>);
impl DSE_BC_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSE_BC_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSE_BC_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSE_BC_INT_EN` writer - Data Start Error Interrupt Enable"]
pub struct DSE_BC_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSE_BC_INT_EN_W<'a> {
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
#[doc = "Field `CB_IW_INT_EN` reader - Command Busy and Illegal Write Interrupt Enable"]
pub struct CB_IW_INT_EN_R(crate::FieldReader<bool, bool>);
impl CB_IW_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CB_IW_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CB_IW_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CB_IW_INT_EN` writer - Command Busy and Illegal Write Interrupt Enable"]
pub struct CB_IW_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CB_IW_INT_EN_W<'a> {
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
#[doc = "Field `FU_FO_INT_EN` reader - FIFO Underrun/Overflow Interrupt Enable"]
pub struct FU_FO_INT_EN_R(crate::FieldReader<bool, bool>);
impl FU_FO_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FU_FO_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FU_FO_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FU_FO_INT_EN` writer - FIFO Underrun/Overflow Interrupt Enable"]
pub struct FU_FO_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FU_FO_INT_EN_W<'a> {
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
#[doc = "Field `DSTO_VSD_INT_EN` reader - Data Starvation Timeout/V1.8 Switch Done Interrupt Enable"]
pub struct DSTO_VSD_INT_EN_R(crate::FieldReader<bool, bool>);
impl DSTO_VSD_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSTO_VSD_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSTO_VSD_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSTO_VSD_INT_EN` writer - Data Starvation Timeout/V1.8 Switch Done Interrupt Enable"]
pub struct DSTO_VSD_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTO_VSD_INT_EN_W<'a> {
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
#[doc = "Field `DTO_BDS_INT_EN` reader - Data Timeout/Boot Data Start Interrupt Enable"]
pub struct DTO_BDS_INT_EN_R(crate::FieldReader<bool, bool>);
impl DTO_BDS_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTO_BDS_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTO_BDS_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTO_BDS_INT_EN` writer - Data Timeout/Boot Data Start Interrupt Enable"]
pub struct DTO_BDS_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTO_BDS_INT_EN_W<'a> {
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
#[doc = "Field `RTO_BACK_INT_EN` reader - Response Timeout/Boot ACK Received Interrupt Enable"]
pub struct RTO_BACK_INT_EN_R(crate::FieldReader<bool, bool>);
impl RTO_BACK_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTO_BACK_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTO_BACK_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTO_BACK_INT_EN` writer - Response Timeout/Boot ACK Received Interrupt Enable"]
pub struct RTO_BACK_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTO_BACK_INT_EN_W<'a> {
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
#[doc = "Field `DCE_INT_EN` reader - Data CRC Error Interrupt Enable"]
pub struct DCE_INT_EN_R(crate::FieldReader<bool, bool>);
impl DCE_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCE_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCE_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCE_INT_EN` writer - Data CRC Error Interrupt Enable"]
pub struct DCE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCE_INT_EN_W<'a> {
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
#[doc = "Field `RCE_INT_EN` reader - Response CRC Error Interrupt Enable"]
pub struct RCE_INT_EN_R(crate::FieldReader<bool, bool>);
impl RCE_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCE_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCE_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCE_INT_EN` writer - Response CRC Error Interrupt Enable"]
pub struct RCE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCE_INT_EN_W<'a> {
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
#[doc = "Field `DRR_INT_EN` reader - Data Receive Request Interrupt Enable"]
pub struct DRR_INT_EN_R(crate::FieldReader<bool, bool>);
impl DRR_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRR_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRR_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRR_INT_EN` writer - Data Receive Request Interrupt Enable"]
pub struct DRR_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DRR_INT_EN_W<'a> {
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
#[doc = "Field `DTR_INT_EN` reader - Data Transmit Request Interrupt Enable"]
pub struct DTR_INT_EN_R(crate::FieldReader<bool, bool>);
impl DTR_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTR_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTR_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTR_INT_EN` writer - Data Transmit Request Interrupt Enable"]
pub struct DTR_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTR_INT_EN_W<'a> {
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
#[doc = "Field `DTC_INT_EN` reader - Data Transfer Complete Interrupt Enable"]
pub struct DTC_INT_EN_R(crate::FieldReader<bool, bool>);
impl DTC_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTC_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTC_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTC_INT_EN` writer - Data Transfer Complete Interrupt Enable"]
pub struct DTC_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTC_INT_EN_W<'a> {
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
#[doc = "Field `CC_INT_EN` reader - Command Complete Interrupt Enable"]
pub struct CC_INT_EN_R(crate::FieldReader<bool, bool>);
impl CC_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC_INT_EN` writer - Command Complete Interrupt Enable"]
pub struct CC_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_INT_EN_W<'a> {
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
#[doc = "Field `RE_INT_EN` reader - Response Error Interrupt Enable"]
pub struct RE_INT_EN_R(crate::FieldReader<bool, bool>);
impl RE_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RE_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RE_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RE_INT_EN` writer - Response Error Interrupt Enable"]
pub struct RE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_INT_EN_W<'a> {
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
    #[doc = "Bit 31 - Card Removed Interrupt Enable"]
    #[inline(always)]
    pub fn card_removal_int_en(&self) -> CARD_REMOVAL_INT_EN_R {
        CARD_REMOVAL_INT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Card Inserted Interrupt Enable"]
    #[inline(always)]
    pub fn card_insert_int_en(&self) -> CARD_INSERT_INT_EN_R {
        CARD_INSERT_INT_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 16 - SDIO Interrupt Enable"]
    #[inline(always)]
    pub fn sdio_int_en(&self) -> SDIO_INT_EN_R {
        SDIO_INT_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Data End-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn dee_int_en(&self) -> DEE_INT_EN_R {
        DEE_INT_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Auto Command Done Interrupt Enable"]
    #[inline(always)]
    pub fn acd_int_en(&self) -> ACD_INT_EN_R {
        ACD_INT_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Data Start Error Interrupt Enable"]
    #[inline(always)]
    pub fn dse_bc_int_en(&self) -> DSE_BC_INT_EN_R {
        DSE_BC_INT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Command Busy and Illegal Write Interrupt Enable"]
    #[inline(always)]
    pub fn cb_iw_int_en(&self) -> CB_IW_INT_EN_R {
        CB_IW_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - FIFO Underrun/Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fu_fo_int_en(&self) -> FU_FO_INT_EN_R {
        FU_FO_INT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Starvation Timeout/V1.8 Switch Done Interrupt Enable"]
    #[inline(always)]
    pub fn dsto_vsd_int_en(&self) -> DSTO_VSD_INT_EN_R {
        DSTO_VSD_INT_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Data Timeout/Boot Data Start Interrupt Enable"]
    #[inline(always)]
    pub fn dto_bds_int_en(&self) -> DTO_BDS_INT_EN_R {
        DTO_BDS_INT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Response Timeout/Boot ACK Received Interrupt Enable"]
    #[inline(always)]
    pub fn rto_back_int_en(&self) -> RTO_BACK_INT_EN_R {
        RTO_BACK_INT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Data CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn dce_int_en(&self) -> DCE_INT_EN_R {
        DCE_INT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Response CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn rce_int_en(&self) -> RCE_INT_EN_R {
        RCE_INT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Receive Request Interrupt Enable"]
    #[inline(always)]
    pub fn drr_int_en(&self) -> DRR_INT_EN_R {
        DRR_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Transmit Request Interrupt Enable"]
    #[inline(always)]
    pub fn dtr_int_en(&self) -> DTR_INT_EN_R {
        DTR_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dtc_int_en(&self) -> DTC_INT_EN_R {
        DTC_INT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn cc_int_en(&self) -> CC_INT_EN_R {
        CC_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Response Error Interrupt Enable"]
    #[inline(always)]
    pub fn re_int_en(&self) -> RE_INT_EN_R {
        RE_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Card Removed Interrupt Enable"]
    #[inline(always)]
    pub fn card_removal_int_en(&mut self) -> CARD_REMOVAL_INT_EN_W {
        CARD_REMOVAL_INT_EN_W { w: self }
    }
    #[doc = "Bit 30 - Card Inserted Interrupt Enable"]
    #[inline(always)]
    pub fn card_insert_int_en(&mut self) -> CARD_INSERT_INT_EN_W {
        CARD_INSERT_INT_EN_W { w: self }
    }
    #[doc = "Bit 16 - SDIO Interrupt Enable"]
    #[inline(always)]
    pub fn sdio_int_en(&mut self) -> SDIO_INT_EN_W {
        SDIO_INT_EN_W { w: self }
    }
    #[doc = "Bit 15 - Data End-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn dee_int_en(&mut self) -> DEE_INT_EN_W {
        DEE_INT_EN_W { w: self }
    }
    #[doc = "Bit 14 - Auto Command Done Interrupt Enable"]
    #[inline(always)]
    pub fn acd_int_en(&mut self) -> ACD_INT_EN_W {
        ACD_INT_EN_W { w: self }
    }
    #[doc = "Bit 13 - Data Start Error Interrupt Enable"]
    #[inline(always)]
    pub fn dse_bc_int_en(&mut self) -> DSE_BC_INT_EN_W {
        DSE_BC_INT_EN_W { w: self }
    }
    #[doc = "Bit 12 - Command Busy and Illegal Write Interrupt Enable"]
    #[inline(always)]
    pub fn cb_iw_int_en(&mut self) -> CB_IW_INT_EN_W {
        CB_IW_INT_EN_W { w: self }
    }
    #[doc = "Bit 11 - FIFO Underrun/Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fu_fo_int_en(&mut self) -> FU_FO_INT_EN_W {
        FU_FO_INT_EN_W { w: self }
    }
    #[doc = "Bit 10 - Data Starvation Timeout/V1.8 Switch Done Interrupt Enable"]
    #[inline(always)]
    pub fn dsto_vsd_int_en(&mut self) -> DSTO_VSD_INT_EN_W {
        DSTO_VSD_INT_EN_W { w: self }
    }
    #[doc = "Bit 9 - Data Timeout/Boot Data Start Interrupt Enable"]
    #[inline(always)]
    pub fn dto_bds_int_en(&mut self) -> DTO_BDS_INT_EN_W {
        DTO_BDS_INT_EN_W { w: self }
    }
    #[doc = "Bit 8 - Response Timeout/Boot ACK Received Interrupt Enable"]
    #[inline(always)]
    pub fn rto_back_int_en(&mut self) -> RTO_BACK_INT_EN_W {
        RTO_BACK_INT_EN_W { w: self }
    }
    #[doc = "Bit 7 - Data CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn dce_int_en(&mut self) -> DCE_INT_EN_W {
        DCE_INT_EN_W { w: self }
    }
    #[doc = "Bit 6 - Response CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn rce_int_en(&mut self) -> RCE_INT_EN_W {
        RCE_INT_EN_W { w: self }
    }
    #[doc = "Bit 5 - Data Receive Request Interrupt Enable"]
    #[inline(always)]
    pub fn drr_int_en(&mut self) -> DRR_INT_EN_W {
        DRR_INT_EN_W { w: self }
    }
    #[doc = "Bit 4 - Data Transmit Request Interrupt Enable"]
    #[inline(always)]
    pub fn dtr_int_en(&mut self) -> DTR_INT_EN_W {
        DTR_INT_EN_W { w: self }
    }
    #[doc = "Bit 3 - Data Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dtc_int_en(&mut self) -> DTC_INT_EN_W {
        DTC_INT_EN_W { w: self }
    }
    #[doc = "Bit 2 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn cc_int_en(&mut self) -> CC_INT_EN_W {
        CC_INT_EN_W { w: self }
    }
    #[doc = "Bit 1 - Response Error Interrupt Enable"]
    #[inline(always)]
    pub fn re_int_en(&mut self) -> RE_INT_EN_W {
        RE_INT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_intmask](index.html) module"]
pub struct SMHC_INTMASK_SPEC;
impl crate::RegisterSpec for SMHC_INTMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_intmask::R](R) reader structure"]
impl crate::Readable for SMHC_INTMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_intmask::W](W) writer structure"]
impl crate::Writable for SMHC_INTMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_INTMASK to value 0"]
impl crate::Resettable for SMHC_INTMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
